package com.gupt.domain.model

import kotlinx.serialization.Serializable
import kotlinx.serialization.SerialName

@Serializable
data class EncryptedMessage(
    val id: String,
    val conversation_id: String,
    val sender_id: String,
    val recipient_id: String,
    val encrypted_payload: String,
    val nonce: String,
    val message_type: String = "text",
    val transport_used: String = "CLOUD",
    val created_at: String? = null
)
