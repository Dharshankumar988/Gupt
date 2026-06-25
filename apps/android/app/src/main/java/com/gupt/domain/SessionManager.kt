package com.gupt.domain

import android.content.Context
import android.content.SharedPreferences
import com.gupt.domain.model.AppUser
import androidx.compose.runtime.mutableStateOf
import androidx.compose.runtime.getValue
import androidx.compose.runtime.setValue
import com.gupt.domain.model.Conversation
import kotlinx.serialization.encodeToString
import kotlinx.serialization.json.Json

enum class NetworkMode {
    ONLINE_CLOUD,
    OFFLINE_MESH
}

object SessionManager {
    private const val PREF_NAME = "gupt_session"
    private const val KEY_USERNAME = "username"
    private const val KEY_PASSWORD_HASH = "password_hash"
    private const val KEY_IS_DARK_MODE = "is_dark_mode"
    private const val KEY_PROFILE_IMAGE_URI = "profile_image_uri"
    private const val KEY_CONVERSATIONS = "saved_conversations"
    private const val KEY_DISPLAY_NAME = "display_name"

    private var sharedPreferences: SharedPreferences? = null

    var isDarkMode by mutableStateOf(true)
        private set

    var profileImageUri: String? by mutableStateOf(null)
        private set

    var currentNetworkMode by mutableStateOf(NetworkMode.ONLINE_CLOUD)

    var displayName by mutableStateOf("")
        private set

    var relayRewards by mutableStateOf(0.0)
        private set

    var conversations by mutableStateOf<List<Conversation>>(emptyList())
        private set

    var currentUser: AppUser? = null
        set(value) {
            field = value
            sharedPreferences?.edit()?.apply {
                if (value != null) {
                    putString(KEY_USERNAME, value.username)
                    putString(KEY_PASSWORD_HASH, value.password_hash)
                } else {
                    remove(KEY_USERNAME)
                    remove(KEY_PASSWORD_HASH)
                }
                apply()
            }
        }

    fun init(context: Context) {
        sharedPreferences = context.getSharedPreferences(PREF_NAME, Context.MODE_PRIVATE)
        val username = sharedPreferences?.getString(KEY_USERNAME, null)
        val passwordHash = sharedPreferences?.getString(KEY_PASSWORD_HASH, null)

        if (username != null && passwordHash != null) {
            currentUser = AppUser(username, passwordHash)
        }

        isDarkMode = sharedPreferences?.getBoolean(KEY_IS_DARK_MODE, true) ?: true
        profileImageUri = sharedPreferences?.getString(KEY_PROFILE_IMAGE_URI, null)
        displayName = sharedPreferences?.getString(KEY_DISPLAY_NAME, "") ?: ""

        val convsJson = sharedPreferences?.getString(KEY_CONVERSATIONS, null)
        if (convsJson != null) {
            try {
                conversations = Json.decodeFromString(convsJson)
            } catch (e: Exception) {
                e.printStackTrace()
            }
        }
    }

    fun addConversation(conversation: Conversation) {
        if (!conversations.any { it.id == conversation.id }) {
            conversations = listOf(conversation) + conversations
            saveConversations()
        }
    }

    fun removeConversation(id: String) {
        conversations = conversations.filterNot { it.id == id }
        saveConversations()
    }

    private fun saveConversations() {
        val json = Json.encodeToString(conversations)
        sharedPreferences?.edit()?.putString(KEY_CONVERSATIONS, json)?.apply()
    }

    fun updateThemeMode(dark: Boolean) {
        isDarkMode = dark
        sharedPreferences?.edit()?.putBoolean(KEY_IS_DARK_MODE, dark)?.apply()
    }

    fun setProfileImage(uri: String) {
        profileImageUri = uri
        sharedPreferences?.edit()?.putString(KEY_PROFILE_IMAGE_URI, uri)?.apply()
    }

    fun updateDisplayName(name: String) {
        displayName = name
        sharedPreferences?.edit()?.putString(KEY_DISPLAY_NAME, name)?.apply()
    }

    fun updateUsername(newUsername: String) {
        currentUser = currentUser?.copy(username = newUsername)
    }

    fun updateRelayRewards(amount: Double) {
        relayRewards = amount
    }

    fun logout() {
        currentUser = null
    }
}
