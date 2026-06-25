package com.gupt.presentation.viewmodels

import android.util.Base64
import android.util.Log
import androidx.lifecycle.ViewModel
import androidx.lifecycle.viewModelScope
import com.gupt.SupabaseConfig
import com.gupt.domain.SessionManager
import com.gupt.domain.model.EncryptedMessage
import com.gupt.presentation.screens.chat.MessageModel
import com.gupt.presentation.screens.chat.TransportRoute
import dagger.hilt.android.lifecycle.HiltViewModel
import io.github.jan.supabase.postgrest.postgrest
import io.github.jan.supabase.realtime.PostgresAction
import io.github.jan.supabase.realtime.channel
import io.github.jan.supabase.realtime.postgresChangeFlow
import io.github.jan.supabase.realtime.realtime
import kotlinx.coroutines.flow.MutableStateFlow
import kotlinx.coroutines.flow.StateFlow
import kotlinx.coroutines.flow.asStateFlow
import kotlinx.coroutines.launch
import kotlinx.serialization.Serializable
import kotlinx.serialization.encodeToString
import kotlinx.serialization.json.Json
import kotlinx.serialization.json.decodeFromJsonElement
import kotlinx.serialization.json.jsonPrimitive
import java.text.SimpleDateFormat
import java.util.Date
import java.util.Locale
import java.util.UUID
import javax.inject.Inject

@Serializable
data class MessagePayload(
    val text: String,
    val replyToText: String? = null,
    val replyToIsMe: Boolean? = null
)

@HiltViewModel
class ChatViewModel @Inject constructor() : ViewModel() {

    private val _messages = MutableStateFlow<List<MessageModel>>(emptyList())
    val messages: StateFlow<List<MessageModel>> = _messages.asStateFlow()

    private var currentPeer: String? = null
    private val jsonConfig = Json { ignoreUnknownKeys = true }

    fun loadConversation(peerUsername: String) {
        currentPeer = peerUsername
        viewModelScope.launch {
            try {
                val currentUser = SessionManager.currentUser?.username ?: return@launch
                val convId = listOf(currentUser, peerUsername).sorted().joinToString("_")

                // 1. Fetch historical messages
                val historical = SupabaseConfig.client.postgrest["encrypted_messages"]
                    .select {
                        filter { eq("conversation_id", convId) }
                    }.decodeList<EncryptedMessage>()

                val uiMessages = historical.sortedByDescending { it.created_at ?: "" }.map { it.toMessageModel(currentUser) }
                _messages.value = uiMessages

                // 2. Subscribe to realtime updates
                val channel = SupabaseConfig.client.realtime.channel("chat_$convId")
                
                val insertChanges = channel.postgresChangeFlow<PostgresAction.Insert>(schema = "public") {
                    table = "encrypted_messages"
                    filter = "conversation_id=eq.$convId"
                }

                val deleteChanges = channel.postgresChangeFlow<PostgresAction.Delete>(schema = "public") {
                    table = "encrypted_messages"
                    filter = "conversation_id=eq.$convId"
                }

                SupabaseConfig.client.realtime.connect()
                channel.subscribe()

                launch {
                    insertChanges.collect { insert ->
                        val newMsg = jsonConfig.decodeFromJsonElement<EncryptedMessage>(insert.record)
                        if (_messages.value.none { it.id == newMsg.id }) {
                            _messages.value = listOf(newMsg.toMessageModel(currentUser)) + _messages.value
                        }
                    }
                }

                launch {
                    deleteChanges.collect { delete ->
                        val deletedId = delete.oldRecord["id"]?.jsonPrimitive?.content
                        if (deletedId != null) {
                            _messages.value = _messages.value.filterNot { it.id == deletedId }
                        }
                    }
                }
            } catch (e: Exception) {
                Log.e("ChatViewModel", "Error loading messages", e)
            }
        }
    }

    fun sendMessage(text: String, imageUrl: String? = null, replyTo: MessageModel? = null, transport: TransportRoute = TransportRoute.CLOUD) {
        val peer = currentPeer ?: return
        val currentUser = SessionManager.currentUser?.username ?: return
        
        viewModelScope.launch {
            try {
                val convId = listOf(currentUser, peer).sorted().joinToString("_")
                val newId = UUID.randomUUID().toString()

                val payloadObj = MessagePayload(
                    text = text.ifBlank { "[IMAGE_ONLY]" },
                    replyToText = replyTo?.text,
                    replyToIsMe = replyTo?.isFromMe
                )
                val payloadString = jsonConfig.encodeToString(payloadObj)
                val encryptedPayload = Base64.encodeToString(payloadString.toByteArray(), Base64.NO_WRAP)
                val nonce = Base64.encodeToString("dummy_nonce_123456789012".toByteArray(), Base64.NO_WRAP)

                val optimisticMsg = MessageModel(
                    id = newId,
                    text = payloadObj.text,
                    isFromMe = true,
                    time = SimpleDateFormat("HH:mm", Locale.getDefault()).format(Date()),
                    transportUsed = transport,
                    replyToMessage = replyTo
                )
                _messages.value = listOf(optimisticMsg) + _messages.value

                @Serializable
                data class InsertMessage(
                    val id: String,
                    val conversation_id: String,
                    val sender_id: String,
                    val recipient_id: String,
                    val encrypted_payload: String,
                    val nonce: String,
                    val transport_used: String
                )

                val insertMsg = InsertMessage(
                    id = newId,
                    conversation_id = convId,
                    sender_id = currentUser,
                    recipient_id = peer,
                    encrypted_payload = encryptedPayload,
                    nonce = nonce,
                    transport_used = transport.name
                )

                SupabaseConfig.client.postgrest["encrypted_messages"].insert(insertMsg)
            } catch (e: Exception) {
                Log.e("ChatViewModel", "Failed to send message", e)
            }
        }
    }

    fun deleteMessage(messageId: String) {
        viewModelScope.launch {
            try {
                // Optimistic UI Delete
                _messages.value = _messages.value.filterNot { it.id == messageId }
                
                // Network Delete
                SupabaseConfig.client.postgrest["encrypted_messages"].delete {
                    filter { eq("id", messageId) }
                }
            } catch (e: Exception) {
                Log.e("ChatViewModel", "Failed to delete message", e)
            }
        }
    }

    private fun EncryptedMessage.toMessageModel(currentUsername: String): MessageModel {
        var plaintext = "Error decrypting"
        var replyModel: MessageModel? = null

        try {
            val decodedStr = String(Base64.decode(this.encrypted_payload, Base64.NO_WRAP))
            // Attempt to parse as MessagePayload JSON
            try {
                val payload = jsonConfig.decodeFromString<MessagePayload>(decodedStr)
                plaintext = payload.text
                if (payload.replyToText != null) {
                    replyModel = MessageModel(
                        id = "reply_dummy",
                        text = payload.replyToText,
                        isFromMe = payload.replyToIsMe ?: false,
                        time = "",
                        transportUsed = TransportRoute.CLOUD
                    )
                }
            } catch (e: Exception) {
                // Backwards compatibility for raw plaintext string from earlier
                plaintext = decodedStr
            }
        } catch (e: Exception) {}

        val sdf = SimpleDateFormat("HH:mm", Locale.getDefault())
        val timeStr = if (this.created_at != null) {
            try {
                val date = SimpleDateFormat("yyyy-MM-dd'T'HH:mm:ss", Locale.getDefault()).parse(this.created_at)
                if (date != null) sdf.format(date) else "Unknown"
            } catch (e: Exception) { "Unknown" }
        } else {
            sdf.format(Date())
        }

        return MessageModel(
            id = this.id,
            text = plaintext,
            isFromMe = this.sender_id == currentUsername,
            time = timeStr,
            transportUsed = try { TransportRoute.valueOf(this.transport_used) } catch(e: Exception) { TransportRoute.CLOUD },
            imageUrl = null,
            replyToMessage = replyModel
        )
    }
}
