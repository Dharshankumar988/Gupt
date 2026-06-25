package com.gupt.domain.model

import kotlinx.serialization.Serializable

@Serializable
data class AppUser(
    val username: String,
    val password_hash: String
)
