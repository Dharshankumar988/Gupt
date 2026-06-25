package com.gupt.services

import android.app.Notification
import android.app.NotificationChannel
import android.app.NotificationManager
import android.app.Service
import android.content.Context
import android.content.Intent
import android.os.Build
import android.os.IBinder
import android.util.Log

class BleService : Service() {

    companion object {
        const val CHANNEL_ID = "GuptBleChannel"
        const val NOTIFICATION_ID = 1001
        
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
    }

    override fun onStartCommand(intent: Intent?, flags: Int, startId: Int): Int {
        Log.i("BleService", "BleService started. Initiating background BLE scan & advertise.")
        
        val notification = createNotification()
        
        // Start as a foreground service to ensure background execution on modern Android
        startForeground(NOTIFICATION_ID, notification)
        
        // TODO: Call UniFFI bound Rust method to start BLE transport
        // GuptEngine.startTransport(TransportType.Ble)

        // START_STICKY tells the OS to restart the service if it's killed due to memory pressure
        return START_STICKY
    }

    override fun onBind(intent: Intent?): IBinder? {
        return null // We don't provide binding, it's a started service
    }

    override fun onDestroy() {
        Log.i("BleService", "BleService destroyed. Stopping BLE transport.")
        // TODO: Call UniFFI bound Rust method to stop BLE transport
        // GuptEngine.stopTransport(TransportType.Ble)
        super.onDestroy()
    }

    private fun createNotificationChannel() {
        if (Build.VERSION.SDK_INT >= Build.VERSION_CODES.O) {
            val serviceChannel = NotificationChannel(
                CHANNEL_ID,
                "Gupt Background Connection",
                NotificationManager.IMPORTANCE_LOW
            ).apply {
                description = "Keeps Gupt connected to nearby peers via BLE"
            }
            
            val manager = getSystemService(NotificationManager::class.java)
            manager.createNotificationChannel(serviceChannel)
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
            // .setSmallIcon(R.mipmap.ic_launcher_round)
            .build()
    }
}
