package com.gupt.services

import android.app.Notification
import android.app.NotificationChannel
import android.app.NotificationManager
import android.app.Service
import android.bluetooth.BluetoothAdapter
import android.bluetooth.BluetoothManager
import android.bluetooth.le.ScanCallback
import android.bluetooth.le.ScanResult
import android.bluetooth.le.ScanSettings
import android.content.Context
import android.content.Intent
import android.os.Build
import android.os.IBinder
import android.util.Log
import com.gupt.ffi.GuptEngine
import com.gupt.SupabaseConfig
import com.gupt.domain.SessionManager
import com.gupt.domain.model.EncryptedMessage
import dagger.hilt.android.AndroidEntryPoint
import io.github.jan.supabase.realtime.PostgresAction
import io.github.jan.supabase.realtime.channel
import io.github.jan.supabase.realtime.postgresChangeFlow
import io.github.jan.supabase.realtime.realtime
import io.github.jan.supabase.postgrest.postgrest
import kotlinx.coroutines.*
import kotlinx.serialization.json.Json
import kotlinx.serialization.json.decodeFromJsonElement
import javax.inject.Inject
import java.util.UUID

@AndroidEntryPoint
class BleService : Service() {

    @Inject
    lateinit var guptEngine: GuptEngine

    private var bluetoothAdapter: BluetoothAdapter? = null
    private val GUPT_SERVICE_UUID = UUID.fromString("00006097-0000-1000-8000-00805F9B34FB")
    
    private val serviceScope = CoroutineScope(Dispatchers.IO + SupervisorJob())

    companion object {
        const val CHANNEL_ID = "GuptBleChannel"
        const val MSG_CHANNEL_ID = "GuptMessagesChannel"
        const val NOTIFICATION_ID = 1001
        
        // Exposes realtime proximity state to the UI for automatic transport switching
        val isMeshNetworkActive = kotlinx.coroutines.flow.MutableStateFlow(false)
        
        fun startService(context: Context) {
            val startIntent = Intent(context, BleService::class.java)
            if (Build.VERSION.SDK_INT >= Build.VERSION_CODES.O) {
                context.startForegroundService(startIntent)
            } else {
                context.startService(startIntent)
            }
        }
        
        fun stopService(context: Context) {
            val stopIntent = Intent(context, BleService::class.java)
            context.stopService(stopIntent)
        }
    }

    override fun onCreate() {
        super.onCreate()
        Log.i("BleService", "BleService created.")
        createNotificationChannel()
        
        val bluetoothManager = getSystemService(Context.BLUETOOTH_SERVICE) as BluetoothManager
        bluetoothAdapter = bluetoothManager.adapter
    }

    override fun onStartCommand(intent: Intent?, flags: Int, startId: Int): Int {
        Log.i("BleService", "BleService started. Initiating background BLE scan.")
        
        val notification = createNotification()
        if (Build.VERSION.SDK_INT >= Build.VERSION_CODES.Q) {
            startForeground(NOTIFICATION_ID, notification, android.content.pm.ServiceInfo.FOREGROUND_SERVICE_TYPE_CONNECTED_DEVICE)
        } else {
            startForeground(NOTIFICATION_ID, notification)
        }
        
        startBleScanning()
        startRealtimeListener()

        return START_STICKY
    }

    private fun startRealtimeListener() {
        val currentUser = SessionManager.currentUser?.username ?: return
        
        serviceScope.launch {
            try {
                val channel = SupabaseConfig.client.realtime.channel("global_recipient_$currentUser")
                val changes = channel.postgresChangeFlow<PostgresAction.Insert>(schema = "public") {
                    table = "encrypted_messages"
                    filter = "recipient_id=eq.$currentUser"
                }
                
                SupabaseConfig.client.realtime.connect()
                channel.subscribe()
                
                Log.i("BleService", "Subscribed to Supabase Realtime for $currentUser")
                
                val jsonConfig = Json { ignoreUnknownKeys = true }
                changes.collect { insert ->
                    val newMsg = jsonConfig.decodeFromJsonElement<EncryptedMessage>(insert.record)
                    Log.i("BleService", "New background message received from ${newMsg.sender_id}")
                    
                    val plaintext = try {
                        val decodedStr = String(android.util.Base64.decode(newMsg.encrypted_payload, android.util.Base64.NO_WRAP))
                        try {
                            val payload = jsonConfig.decodeFromString<com.gupt.presentation.viewmodels.MessagePayload>(decodedStr)
                            payload.text
                        } catch(e: Exception) { decodedStr }
                    } catch(e: Exception) { "Encrypted Message" }

                    // Add to SessionManager or update existing
                    if (SessionManager.conversations.any { it.id == newMsg.sender_id }) {
                        SessionManager.updateConversation(newMsg.sender_id, plaintext, 1)
                    } else {
                        // Fetch user profile to add them to contacts
                        try {
                            val usersList = SupabaseConfig.client.postgrest["user_profiles"]
                                .select {
                                    filter { eq("username", newMsg.sender_id) }
                                }.decodeList<com.gupt.domain.model.UserProfile>()
                            
                            if (usersList.isNotEmpty()) {
                                val peer = usersList.first()
                                val newConv = com.gupt.domain.model.Conversation(
                                    id = peer.username,
                                    name = peer.display_name.ifBlank { peer.username },
                                    lastMessage = plaintext,
                                    time = java.text.SimpleDateFormat("HH:mm", java.util.Locale.getDefault()).format(java.util.Date()),
                                    unreadCount = 1,
                                    avatarUrl = peer.avatar_url ?: "https://ui-avatars.com/api/?name=${peer.username}&background=random&color=fff&size=128"
                                )
                                SessionManager.addConversation(newConv)
                            }
                        } catch (e: Exception) {
                            Log.e("BleService", "Failed to fetch user profile for new message", e)
                        }
                    }
                    
                    showNewMessageNotification(newMsg, plaintext)
                }
            } catch (e: Exception) {
                Log.e("BleService", "Realtime listener error", e)
            }
        }
    }

    private fun startBleScanning() {
        try {
            val scanner = bluetoothAdapter?.bluetoothLeScanner
            if (scanner == null) {
                Log.e("BleService", "Bluetooth LE Scanner not available.")
                return
            }

            val settings = ScanSettings.Builder()
                .setScanMode(ScanSettings.SCAN_MODE_LOW_POWER)
                .build()

            scanner.startScan(null, settings, scanCallback)
            Log.i("BleService", "BLE Scan started successfully.")
        } catch (e: SecurityException) {
            Log.e("BleService", "Missing Bluetooth Permissions! Scan aborted.", e)
        }
    }

    private val scanCallback = object : ScanCallback() {
        override fun onScanResult(callbackType: Int, result: ScanResult?) {
            result?.let {
                val peerId = it.device.address
                val rssi = it.rssi.toShort()
                
                // Pass real-time proximity to Rust's RoutingEngine
                guptEngine.updatePeerProximity(peerId, rssi)
                Log.d("BleService", "Reported peer $peerId with RSSI $rssi to Rust FFI")
                
                // Auto-switch UI to Mesh mode if signal is reasonably strong
                if (rssi > -80) {
                    isMeshNetworkActive.value = true
                }
            }
        }

        override fun onScanFailed(errorCode: Int) {
            Log.e("BleService", "BLE Scan failed with error: $errorCode")
        }
    }

    override fun onBind(intent: Intent?): IBinder? {
        return null
    }

    override fun onDestroy() {
        Log.i("BleService", "BleService destroyed. Stopping BLE scanning.")
        bluetoothAdapter?.bluetoothLeScanner?.stopScan(scanCallback)
        serviceScope.cancel()
        super.onDestroy()
    }

    private fun createNotificationChannel() {
        if (Build.VERSION.SDK_INT >= Build.VERSION_CODES.O) {
            val manager = getSystemService(NotificationManager::class.java)

            val serviceChannel = NotificationChannel(
                CHANNEL_ID,
                "Gupt Background Connection",
                NotificationManager.IMPORTANCE_LOW
            ).apply {
                description = "Keeps Gupt connected to nearby peers via BLE"
            }
            manager.createNotificationChannel(serviceChannel)
            
            val msgChannel = NotificationChannel(
                MSG_CHANNEL_ID,
                "Gupt Messages",
                NotificationManager.IMPORTANCE_HIGH
            ).apply {
                description = "Notifications for new secure messages"
            }
            manager.createNotificationChannel(msgChannel)
        }
    }

    private fun createNotification(): Notification {
        val builder = if (Build.VERSION.SDK_INT >= Build.VERSION_CODES.O) {
            Notification.Builder(this, CHANNEL_ID)
        } else {
            @Suppress("DEPRECATION")
            Notification.Builder(this)
        }
        
        return builder
            .setContentTitle("Gupt is running")
            .setContentText("Discovering nearby peers securely...")
            // Need a real icon in production
            .setSmallIcon(android.R.drawable.stat_sys_data_bluetooth)
            .build()
    }

    private fun showNewMessageNotification(msg: EncryptedMessage, plaintext: String) {
        val builder = if (Build.VERSION.SDK_INT >= Build.VERSION_CODES.O) {
            Notification.Builder(this, MSG_CHANNEL_ID)
        } else {
            @Suppress("DEPRECATION")
            Notification.Builder(this)
        }
        
        val notification = builder
            .setContentTitle("New message from ${msg.sender_id}")
            .setContentText(plaintext)
            .setSmallIcon(android.R.drawable.stat_notify_chat)
            .setAutoCancel(true)
            .build()
            
        val manager = getSystemService(NotificationManager::class.java)
        manager.notify(msg.id.hashCode(), notification)
    }
}
