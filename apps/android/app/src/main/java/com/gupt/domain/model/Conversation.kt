package com.gupt.domain.model

import kotlinx.serialization.Serializable

@Serializable
data class Conversation(
    val id: String, // This will be the peer's username
    val name: String, // Also the peer's username
    val lastMessage: String,
    val time: String,
    val unreadCount: Int
)
