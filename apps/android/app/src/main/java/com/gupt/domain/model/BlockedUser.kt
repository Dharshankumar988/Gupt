package com.gupt.domain.model

import kotlinx.serialization.Serializable

@Serializable
data class BlockedUser(
    val blocker_username: String,
    val blocked_username: String
)
