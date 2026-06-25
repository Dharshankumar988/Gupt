package com.gupt.presentation.viewmodels

import androidx.lifecycle.ViewModel
import androidx.lifecycle.viewModelScope
import com.gupt.ffi.GuptEngine
import com.gupt.ffi.Message
import kotlinx.coroutines.flow.MutableStateFlow
import kotlinx.coroutines.flow.StateFlow
import kotlinx.coroutines.flow.asStateFlow
import kotlinx.coroutines.launch
import dagger.hilt.android.lifecycle.HiltViewModel
import javax.inject.Inject

@HiltViewModel
class ChatViewModel @Inject constructor(
    private val guptEngine: GuptEngine
) : ViewModel() {

    private val _uiState = MutableStateFlow<ChatUiState>(ChatUiState.Loading)
    val uiState: StateFlow<ChatUiState> = _uiState.asStateFlow()

    fun loadConversation(conversationId: String) {
        viewModelScope.launch {
            _uiState.value = ChatUiState.Loading
            try {
                // Call Rust Engine via UniFFI
                val messages = guptEngine.getMessages(conversationId)
                
                _uiState.value = ChatUiState.Success(
                    conversationId = conversationId,
                    messages = messages
                )
            } catch (e: Exception) {
                _uiState.value = ChatUiState.Error(e.message ?: "Unknown FFI error")
            }
        }
    }

    fun sendMessage(conversationId: String, text: String) {
        viewModelScope.launch {
            try {
                // Call Rust Engine via UniFFI
                val msgId = guptEngine.sendMessage(conversationId, text)
                // Reload messages to update UI
                loadConversation(conversationId)
            } catch (e: Exception) {
                _uiState.value = ChatUiState.Error(e.message ?: "Failed to send message")
            }
        }
    }
}

sealed class ChatUiState {
    object Loading : ChatUiState()
    data class Success(
        val conversationId: String,
        val messages: List<Message> // Now using the actual FFI mapped type
    ) : ChatUiState()
    data class Error(val message: String) : ChatUiState()
}
