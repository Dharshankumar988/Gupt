package com.gupt.domain.model

import kotlinx.serialization.Serializable

@Serializable
data class ChatBackup(
    val user_username: String,
    val payload: String, // Using String to represent JSONB for simplicity
    val created_at: String? = null
)
