package com.gupt.ffi

data class Message(
    val id: String,
    val senderId: String,
    val text: String,
    val timestamp: Long
)

class GuptEngine {
    companion object {
        fun initialize(dbPath: String, userPin: String): GuptEngine {
            return GuptEngine()
        }
    }

    fun getMessages(conversationId: String): List<Message> {
        // Return dummy messages for UI testing
        return listOf(
            Message("1", "user_1", "Welcome to Gupt!", System.currentTimeMillis()),
            Message("2", "me", "This is a secure offline message.", System.currentTimeMillis() + 1000)
        )
    }

    fun sendMessage(conversationId: String, text: String): String {
        return "mock_id"
    }
}
