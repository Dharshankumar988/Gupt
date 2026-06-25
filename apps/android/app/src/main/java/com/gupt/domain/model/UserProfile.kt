package com.gupt.domain.model

import kotlinx.serialization.Serializable

@Serializable
data class UserProfile(
    val username: String,
    val display_name: String = "",
    val avatar_url: String? = null,
    val relay_rewards_inr: Double = 0.0,
    val is_online: Boolean = false
)
