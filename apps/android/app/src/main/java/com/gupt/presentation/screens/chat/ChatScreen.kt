package com.gupt.presentation.screens.chat

import android.net.Uri
import androidx.activity.compose.rememberLauncherForActivityResult
import androidx.activity.result.contract.ActivityResultContracts
import androidx.compose.animation.*
import androidx.compose.foundation.background
import androidx.compose.foundation.clickable
import androidx.compose.foundation.ExperimentalFoundationApi
import androidx.compose.foundation.combinedClickable
import androidx.compose.foundation.layout.*
import androidx.compose.foundation.lazy.LazyColumn
import androidx.compose.foundation.lazy.items
import androidx.compose.foundation.shape.CircleShape
import androidx.compose.foundation.shape.RoundedCornerShape
import androidx.compose.material.icons.Icons
import androidx.compose.material.icons.filled.ArrowBack
import androidx.compose.material.icons.filled.Close
import androidx.compose.material.icons.filled.Delete
import androidx.compose.material.icons.filled.Image
import androidx.compose.material.icons.filled.Reply
import androidx.compose.material.icons.filled.Send
import androidx.compose.material3.*
import androidx.compose.runtime.*
import androidx.compose.ui.Alignment
import androidx.compose.ui.Modifier
import androidx.compose.ui.draw.clip
import androidx.compose.ui.graphics.Color
import androidx.compose.ui.layout.ContentScale
import androidx.compose.ui.text.font.FontWeight
import androidx.compose.ui.text.style.TextOverflow
import androidx.compose.ui.unit.dp
import coil.compose.AsyncImage
import com.gupt.presentation.theme.PrimaryBlue
import androidx.hilt.navigation.compose.hiltViewModel
import com.gupt.presentation.viewmodels.ChatViewModel

enum class TransportRoute { BLE, WIFI_DIRECT, CLOUD, MESH }

data class MessageModel(
    val id: String,
    val text: String,
    val isFromMe: Boolean,
    val time: String,
    val transportUsed: TransportRoute,
    val imageUrl: String? = null,
    val replyToMessage: MessageModel? = null
)

@OptIn(ExperimentalMaterial3Api::class, ExperimentalFoundationApi::class)
@Composable
fun ChatScreen(
    username: String,
    onNavigateBack: () -> Unit,
    viewModel: ChatViewModel = hiltViewModel()
) {
    var inputText by remember { mutableStateOf("") }
    var selectedImageUri by remember { mutableStateOf<Uri?>(null) }
    var replyingTo by remember { mutableStateOf<MessageModel?>(null) }
    var selectedMessage by remember { mutableStateOf<MessageModel?>(null) }
    
    // Load conversation when screen opens
    LaunchedEffect(username) {
        viewModel.loadConversation(username)
    }

    // Auto-Switching based on Proximity State
    val isMeshActive by com.gupt.services.BleService.isMeshNetworkActive.collectAsState()
    val activeTransport = if (isMeshActive) TransportRoute.BLE else TransportRoute.CLOUD

    // State holding dynamic messages from ViewModel
    val messages by viewModel.messages.collectAsState()

    val photoPickerLauncher = rememberLauncherForActivityResult(
        contract = ActivityResultContracts.GetContent()
    ) { uri: Uri? ->
        selectedImageUri = uri
    }

    Scaffold(
        topBar = {
            if (selectedMessage != null) {
                TopAppBar(
                    title = { Text("1 Selected") },
                    navigationIcon = {
                        IconButton(onClick = { selectedMessage = null }) {
                            Icon(Icons.Default.Close, contentDescription = "Cancel")
                        }
                    },
                    actions = {
                        IconButton(onClick = { 
                            replyingTo = selectedMessage
                            selectedMessage = null
                        }) {
                            Icon(Icons.Default.Reply, contentDescription = "Reply")
                        }
                        IconButton(onClick = { 
                            selectedMessage?.id?.let { viewModel.deleteMessage(it) }
                            selectedMessage = null
                        }) {
                            Icon(Icons.Default.Delete, contentDescription = "Delete")
                        }
                    },
                    colors = TopAppBarDefaults.topAppBarColors(
                        containerColor = MaterialTheme.colorScheme.surfaceVariant
                    )
                )
            } else {
                TopAppBar(
                    title = { 
                        Row(verticalAlignment = Alignment.CenterVertically) {
                            val conv = com.gupt.domain.SessionManager.conversations.find { it.id == username }
                            val dpUrl = conv?.avatarUrl ?: "https://ui-avatars.com/api/?name=\${username}&background=random&color=fff&size=128"
                            
                            AsyncImage(
                                model = dpUrl,
                                contentDescription = "Profile Picture",
                                modifier = Modifier
                                    .size(36.dp)
                                    .clip(CircleShape),
                                contentScale = ContentScale.Crop
                            )
                            Spacer(modifier = Modifier.width(12.dp))
                            Text(conv?.name ?: username) 
                        }
                    },
                    navigationIcon = {
                        IconButton(onClick = onNavigateBack) {
                            Icon(Icons.Default.ArrowBack, contentDescription = "Back")
                        }
                    },
                    colors = TopAppBarDefaults.topAppBarColors(
                        containerColor = MaterialTheme.colorScheme.surface
                    )
                )
            }
        },
        bottomBar = {
            Column {
                // Reply Preview Box
                AnimatedVisibility(visible = replyingTo != null) {
                    replyingTo?.let { replyMsg ->
                        Row(
                            modifier = Modifier
                                .fillMaxWidth()
                                .background(MaterialTheme.colorScheme.surfaceVariant)
                                .padding(8.dp),
                            verticalAlignment = Alignment.CenterVertically
                        ) {
                            Column(modifier = Modifier.weight(1f)) {
                                Text(
                                    "Replying to ${if(replyMsg.isFromMe) "yourself" else username}",
                                    color = PrimaryBlue,
                                    style = MaterialTheme.typography.labelMedium
                                )
                                Text(
                                    replyMsg.text.ifBlank { "Image" },
                                    maxLines = 1,
                                    overflow = TextOverflow.Ellipsis,
                                    style = MaterialTheme.typography.bodyMedium
                                )
                            }
                            IconButton(onClick = { replyingTo = null }) {
                                Icon(Icons.Default.Close, contentDescription = "Cancel Reply")
                            }
                        }
                    }
                }

                // Image Preview
                AnimatedVisibility(visible = selectedImageUri != null) {
                    Box(modifier = Modifier.padding(8.dp)) {
                        AsyncImage(
                            model = selectedImageUri,
                            contentDescription = "Selected Image",
                            modifier = Modifier
                                .height(150.dp)
                                .clip(RoundedCornerShape(8.dp)),
                            contentScale = ContentScale.Crop
                        )
                        IconButton(
                            onClick = { selectedImageUri = null },
                            modifier = Modifier
                                .align(Alignment.TopEnd)
                                .background(Color.Black.copy(alpha = 0.5f), CircleShape)
                        ) {
                            Icon(Icons.Default.Close, contentDescription = "Remove", tint = Color.White)
                        }
                    }
                }

                ChatInputBar(
                    text = inputText,
                    onTextChange = { inputText = it },
                    onAttachImage = { photoPickerLauncher.launch("image/*") },
                    onSend = {
                        viewModel.sendMessage(
                            text = inputText,
                            imageUrl = selectedImageUri?.toString(),
                            replyTo = replyingTo,
                            transport = activeTransport
                        )
                        
                        // Reset input state
                        inputText = ""
                        selectedImageUri = null
                        replyingTo = null
                    }
                )
            }
        }
    ) { padding ->
        if (messages.isEmpty()) {
            Box(
                modifier = Modifier
                    .fillMaxSize()
                    .padding(padding),
                contentAlignment = Alignment.Center
            ) {
                Text(
                    text = "No messages yet. Send a secure message!",
                    color = MaterialTheme.colorScheme.onSurfaceVariant
                )
            }
        } else {
            LazyColumn(
                modifier = Modifier
                    .fillMaxSize()
                    .padding(padding)
                    .background(MaterialTheme.colorScheme.background),
                reverseLayout = true
            ) {
                items(messages, key = { it.id }) { msg ->
                    MessageBubble(
                        message = msg,
                        isSelected = selectedMessage?.id == msg.id,
                        onLongClick = { selectedMessage = msg },
                        username = username
                    )
                }
            }
        }
    }
}

@OptIn(ExperimentalFoundationApi::class)
@Composable
fun MessageBubble(
    message: MessageModel,
    isSelected: Boolean,
    onLongClick: () -> Unit,
    username: String
) {
    val alignment = if (message.isFromMe) Alignment.CenterEnd else Alignment.CenterStart
    val bubbleColor = if (message.isFromMe) PrimaryBlue else MaterialTheme.colorScheme.surfaceVariant
    val textColor = if (message.isFromMe) MaterialTheme.colorScheme.onPrimary else MaterialTheme.colorScheme.onSurfaceVariant
    val shape = if (message.isFromMe) {
        RoundedCornerShape(16.dp, 16.dp, 4.dp, 16.dp)
    } else {
        RoundedCornerShape(16.dp, 16.dp, 16.dp, 4.dp)
    }

    Box(
        modifier = Modifier
            .fillMaxWidth()
            .background(if (isSelected) MaterialTheme.colorScheme.primaryContainer.copy(alpha = 0.3f) else Color.Transparent)
            .combinedClickable(
                onClick = {},
                onLongClick = onLongClick
            )
            .padding(horizontal = 16.dp, vertical = 4.dp),
        contentAlignment = alignment
    ) {
        Column(
            horizontalAlignment = if (message.isFromMe) Alignment.End else Alignment.Start
        ) {
            Box(
                modifier = Modifier
                    .clip(shape)
                    .background(bubbleColor)
                    .padding(4.dp)
            ) {
                Column(modifier = Modifier.padding(horizontal = 12.dp, vertical = 8.dp)) {
                    
                    // Reply box
                    message.replyToMessage?.let { reply ->
                        Box(
                            modifier = Modifier
                                .fillMaxWidth(0.8f)
                                .clip(RoundedCornerShape(8.dp))
                                .background(Color.Black.copy(alpha = 0.2f))
                                .padding(8.dp)
                        ) {
                            Column {
                                Text(
                                    text = if (reply.isFromMe) "You" else username,
                                    color = if (message.isFromMe) Color.White else PrimaryBlue,
                                    fontWeight = FontWeight.Bold,
                                    style = MaterialTheme.typography.labelSmall
                                )
                                Text(
                                    text = reply.text.ifBlank { "Image" },
                                    color = textColor.copy(alpha = 0.8f),
                                    maxLines = 1,
                                    overflow = TextOverflow.Ellipsis,
                                    style = MaterialTheme.typography.bodySmall
                                )
                            }
                        }
                        Spacer(modifier = Modifier.height(4.dp))
                    }

                    // Image
                    if (message.imageUrl != null) {
                        AsyncImage(
                            model = message.imageUrl,
                            contentDescription = "Message Image",
                            modifier = Modifier
                                .fillMaxWidth(0.8f)
                                .height(200.dp)
                                .clip(RoundedCornerShape(8.dp)),
                            contentScale = ContentScale.Crop
                        )
                        Spacer(modifier = Modifier.height(4.dp))
                    }

                    if (message.text.isNotBlank()) {
                        Text(
                            text = message.text,
                            color = textColor,
                            style = MaterialTheme.typography.bodyLarge
                        )
                    }
                }
            }
            
            Spacer(modifier = Modifier.height(4.dp))
            
            Row(
                verticalAlignment = Alignment.CenterVertically,
                horizontalArrangement = Arrangement.spacedBy(4.dp)
            ) {
                Text(
                    text = message.time,
                    style = MaterialTheme.typography.labelSmall,
                    color = MaterialTheme.colorScheme.onBackground.copy(alpha = 0.5f)
                )
                val transportIcon = when (message.transportUsed) {
                    TransportRoute.BLE -> "ᛒ"
                    TransportRoute.WIFI_DIRECT -> "🛜"
                    TransportRoute.CLOUD -> "☁️"
                    TransportRoute.MESH -> "🕸️"
                }
                Text(
                    text = transportIcon,
                    style = MaterialTheme.typography.labelSmall
                )
            }
        }
    }
}

@OptIn(ExperimentalMaterial3Api::class)
@Composable
fun ChatInputBar(
    text: String,
    onTextChange: (String) -> Unit,
    onAttachImage: () -> Unit,
    onSend: () -> Unit
) {
    Row(
        modifier = Modifier
            .fillMaxWidth()
            .background(MaterialTheme.colorScheme.surface)
            .padding(8.dp),
        verticalAlignment = Alignment.CenterVertically
    ) {
        IconButton(onClick = onAttachImage) {
            Icon(Icons.Default.Image, contentDescription = "Attach Image", tint = MaterialTheme.colorScheme.primary)
        }

        OutlinedTextField(
            value = text,
            onValueChange = onTextChange,
            placeholder = { Text("Message...") },
            modifier = Modifier.weight(1f),
            shape = RoundedCornerShape(24.dp),
            colors = OutlinedTextFieldDefaults.colors(
                unfocusedBorderColor = Color.Transparent,
                focusedBorderColor = Color.Transparent,
                unfocusedContainerColor = MaterialTheme.colorScheme.surfaceVariant.copy(alpha = 0.5f),
                focusedContainerColor = MaterialTheme.colorScheme.surfaceVariant.copy(alpha = 0.8f)
            ),
            maxLines = 4
        )
        
        Spacer(modifier = Modifier.width(8.dp))
        
        IconButton(
            onClick = onSend,
            modifier = Modifier
                .size(48.dp)
                .clip(CircleShape)
                .background(PrimaryBlue)
        ) {
            Icon(
                Icons.Default.Send,
                contentDescription = "Send",
                tint = MaterialTheme.colorScheme.onPrimary
            )
        }
    }
}
