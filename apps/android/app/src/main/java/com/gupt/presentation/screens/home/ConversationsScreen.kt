package com.gupt.presentation.screens.home

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
import androidx.compose.material.icons.filled.Add
import androidx.compose.material.icons.filled.Block
import androidx.compose.material.icons.filled.Close
import androidx.compose.material.icons.filled.Delete
import androidx.compose.material.icons.filled.Search
import androidx.compose.material.icons.filled.Settings
import androidx.compose.material.icons.filled.WbSunny
import androidx.compose.material.icons.filled.DarkMode
import androidx.compose.material3.*
import androidx.compose.runtime.*
import androidx.compose.ui.Alignment
import androidx.compose.ui.Modifier
import androidx.compose.ui.draw.clip
import androidx.compose.ui.draw.shadow
import androidx.compose.ui.text.font.FontWeight
import androidx.compose.ui.text.style.TextOverflow
import androidx.compose.ui.unit.dp
import android.widget.Toast
import androidx.compose.ui.platform.LocalContext
import com.gupt.domain.SessionManager
import com.gupt.domain.NetworkMode
import com.gupt.domain.model.AppUser
import com.gupt.domain.model.BlockedUser
import com.gupt.domain.model.Conversation
import com.gupt.SupabaseConfig
import com.gupt.presentation.theme.PrimaryBlue
import com.gupt.presentation.theme.AccentTeal
import io.github.jan.supabase.postgrest.postgrest
import kotlinx.coroutines.launch

// ConversationPreview moved to domain.model.Conversation

@OptIn(ExperimentalMaterial3Api::class, ExperimentalFoundationApi::class)
@Composable
fun ConversationsScreen(
    onNavigateToChat: (String) -> Unit,
    onNavigateToSettings: () -> Unit
) {
    val conversations = SessionManager.conversations
    var selectedConversation by remember { mutableStateOf<Conversation?>(null) }
    var searchQuery by remember { mutableStateOf("") }
    
    // Search User Dialog State
    var showSearchDialog by remember { mutableStateOf(false) }
    var searchUsername by remember { mutableStateOf("") }
    var isSearching by remember { mutableStateOf(false) }
    val scope = rememberCoroutineScope()
    val context = LocalContext.current

    Scaffold(
        topBar = {
            if (selectedConversation != null) {
                TopAppBar(
                    title = { Text("1 Selected") },
                    navigationIcon = {
                        IconButton(onClick = { selectedConversation = null }) {
                            Icon(Icons.Default.Close, contentDescription = "Cancel")
                        }
                    },
                    actions = {
                        IconButton(onClick = {
                            selectedConversation?.id?.let { convId ->
                                scope.launch {
                                    try {
                                        val blocked = BlockedUser(
                                            blocker_username = SessionManager.currentUser?.username ?: "",
                                            blocked_username = convId
                                        )
                                        SupabaseConfig.client.postgrest["blocked_users"].insert(blocked)
                                        SessionManager.removeConversation(convId)
                                        selectedConversation = null
                                        Toast.makeText(context, "User blocked", Toast.LENGTH_SHORT).show()
                                    } catch (e: Exception) {
                                        Toast.makeText(context, "Failed to block: ${e.message}", Toast.LENGTH_SHORT).show()
                                    }
                                }
                            }
                        }) {
                            Icon(Icons.Default.Block, contentDescription = "Block User", tint = MaterialTheme.colorScheme.error)
                        }
                        IconButton(onClick = { 
                            selectedConversation?.id?.let { SessionManager.removeConversation(it) }
                            selectedConversation = null
                        }) {
                            Icon(Icons.Default.Delete, contentDescription = "Delete Contact")
                        }
                    },
                    colors = TopAppBarDefaults.topAppBarColors(
                        containerColor = MaterialTheme.colorScheme.surfaceVariant
                    )
                )
            } else {
                TopAppBar(
                    title = { Text("Gupt", fontWeight = FontWeight.Bold) },
                    actions = {
                        IconButton(onClick = { 
                            SessionManager.updateThemeMode(!SessionManager.isDarkMode) 
                        }) {
                            Icon(
                                if (SessionManager.isDarkMode) Icons.Default.WbSunny else Icons.Default.DarkMode, 
                                contentDescription = "Toggle Theme"
                            )
                        }
                        IconButton(onClick = onNavigateToSettings) {
                            Icon(Icons.Default.Settings, contentDescription = "Settings & Discovery")
                        }
                    },
                    colors = TopAppBarDefaults.topAppBarColors(
                        containerColor = MaterialTheme.colorScheme.background
                    )
                )
            }
        },
        floatingActionButton = {
            FloatingActionButton(
                onClick = { showSearchDialog = true },
                containerColor = PrimaryBlue,
                contentColor = MaterialTheme.colorScheme.onPrimary,
                modifier = Modifier
                    .shadow(
                        elevation = 24.dp,
                        shape = RoundedCornerShape(16.dp),
                        spotColor = AccentTeal,
                        ambientColor = PrimaryBlue
                    )
            ) {
                Icon(Icons.Default.Add, contentDescription = "New Chat")
            }
        }
    ) { padding ->
        Column(
            modifier = Modifier
                .fillMaxSize()
                .padding(padding)
                .background(MaterialTheme.colorScheme.background)
        ) {
            if (showSearchDialog) {
                AlertDialog(
                    onDismissRequest = { if (!isSearching) showSearchDialog = false },
                    title = { Text("Search User") },
                    text = {
                        OutlinedTextField(
                            value = searchUsername,
                            onValueChange = { searchUsername = it },
                            label = { Text("Enter Username") },
                            singleLine = true,
                            enabled = !isSearching
                        )
                    },
                    confirmButton = {
                        Button(
                            onClick = {
                                if (searchUsername.isBlank()) return@Button
                                isSearching = true
                                scope.launch {
                                    try {
                                        val usersList = SupabaseConfig.client.postgrest["users"]
                                            .select {
                                                filter { eq("username", searchUsername.trim()) }
                                            }.decodeList<AppUser>()

                                        if (usersList.isNotEmpty()) {
                                            val peer = usersList.first()
                                            // Create new persistent conversation
                                            val newConv = Conversation(
                                                id = peer.username,
                                                name = peer.username,
                                                lastMessage = "Connection established",
                                                time = "Just now",
                                                unreadCount = 0
                                            )
                                            SessionManager.addConversation(newConv)
                                            showSearchDialog = false
                                            searchUsername = ""
                                            onNavigateToChat(peer.username)
                                        } else {
                                            Toast.makeText(context, "User not found", Toast.LENGTH_SHORT).show()
                                        }
                                    } catch (e: Exception) {
                                        Toast.makeText(context, "Search failed: ${e.message}", Toast.LENGTH_SHORT).show()
                                    } finally {
                                        isSearching = false
                                    }
                                }
                            },
                            enabled = !isSearching && searchUsername.isNotBlank()
                        ) {
                            if (isSearching) {
                                CircularProgressIndicator(modifier = Modifier.size(20.dp), color = androidx.compose.ui.graphics.Color.White, strokeWidth = 2.dp)
                            } else {
                                Text("Connect")
                            }
                        }
                    },
                    dismissButton = {
                        TextButton(onClick = { showSearchDialog = false }, enabled = !isSearching) {
                            Text("Cancel")
                        }
                    }
                )
            }
            // Search Bar
            OutlinedTextField(
                value = searchQuery,
                onValueChange = { searchQuery = it },
                placeholder = { Text("Search securely...") },
                leadingIcon = { Icon(Icons.Default.Search, contentDescription = null, tint = PrimaryBlue) },
                modifier = Modifier
                    .fillMaxWidth()
                    .padding(horizontal = 20.dp, vertical = 12.dp)
                    .shadow(8.dp, RoundedCornerShape(24.dp)),
                shape = RoundedCornerShape(24.dp),
                colors = OutlinedTextFieldDefaults.colors(
                    unfocusedContainerColor = MaterialTheme.colorScheme.surfaceVariant.copy(alpha = 0.5f),
                    focusedContainerColor = MaterialTheme.colorScheme.surfaceVariant,
                    unfocusedBorderColor = androidx.compose.ui.graphics.Color.Transparent,
                    focusedBorderColor = PrimaryBlue
                )
            )
            
            Spacer(modifier = Modifier.height(16.dp))

            if (conversations.isEmpty()) {
                Box(
                    modifier = Modifier.fillMaxSize()
                ) {
                    Text(
                        "No contacts yet. Tap + to start a secure chat.",
                        color = MaterialTheme.colorScheme.onSurfaceVariant,
                        modifier = Modifier.align(Alignment.Center)
                    )
                    
                    // Mode Indicator
                    Surface(
                        modifier = Modifier.align(Alignment.BottomStart).padding(16.dp),
                        shape = RoundedCornerShape(16.dp),
                        color = if (SessionManager.currentNetworkMode == NetworkMode.ONLINE_CLOUD) PrimaryBlue else AccentTeal
                    ) {
                        Text(
                            text = if (SessionManager.currentNetworkMode == NetworkMode.ONLINE_CLOUD) "Cloud Mode" else "Mesh Mode",
                            modifier = Modifier.padding(horizontal = 12.dp, vertical = 6.dp),
                            style = MaterialTheme.typography.labelSmall,
                            color = androidx.compose.ui.graphics.Color.White
                        )
                    }
                }
            } else {
                Box(modifier = Modifier.fillMaxSize()) {
                    LazyColumn(
                        modifier = Modifier.fillMaxSize()
                    ) {
                        items(conversations, key = { it.id }) { conv ->
                            val isSelected = selectedConversation?.id == conv.id
                            ConversationItem(
                                conversation = conv,
                                isSelected = isSelected,
                                onClick = { 
                                    if (selectedConversation != null) {
                                        selectedConversation = if (isSelected) null else conv
                                    } else {
                                        onNavigateToChat(conv.id) 
                                    }
                                },
                                onLongClick = { selectedConversation = conv }
                            )
                        }
                    }
                    
                    // Mode Indicator
                    Surface(
                        modifier = Modifier.align(Alignment.BottomStart).padding(16.dp),
                        shape = RoundedCornerShape(16.dp),
                        color = if (SessionManager.currentNetworkMode == NetworkMode.ONLINE_CLOUD) PrimaryBlue else AccentTeal
                    ) {
                        Text(
                            text = if (SessionManager.currentNetworkMode == NetworkMode.ONLINE_CLOUD) "Cloud Mode" else "Mesh Mode",
                            modifier = Modifier.padding(horizontal = 12.dp, vertical = 6.dp),
                            style = MaterialTheme.typography.labelSmall,
                            color = androidx.compose.ui.graphics.Color.White
                        )
                    }
                }
            }
        }
    }
}

@OptIn(ExperimentalFoundationApi::class)
@Composable
fun ConversationItem(
    conversation: Conversation,
    isSelected: Boolean,
    onClick: () -> Unit,
    onLongClick: () -> Unit
) {
    Row(
        modifier = Modifier
            .fillMaxWidth()
            .background(if (isSelected) PrimaryBlue.copy(alpha = 0.1f) else androidx.compose.ui.graphics.Color.Transparent)
            .combinedClickable(
                onClick = onClick,
                onLongClick = onLongClick
            )
            .padding(horizontal = 20.dp, vertical = 16.dp),
        verticalAlignment = Alignment.CenterVertically
    ) {
        // Avatar Placeholder
        Box(
            modifier = Modifier
                .size(52.dp)
                .clip(CircleShape)
                .background(MaterialTheme.colorScheme.surfaceVariant),
            contentAlignment = Alignment.Center
        ) {
            Text(
                text = conversation.name.firstOrNull()?.toString() ?: "?",
                style = MaterialTheme.typography.titleLarge,
                color = MaterialTheme.colorScheme.onSurfaceVariant
            )
        }
        
        Spacer(modifier = Modifier.width(16.dp))
        
        Column(modifier = Modifier.weight(1f)) {
            Row(
                modifier = Modifier.fillMaxWidth(),
                horizontalArrangement = Arrangement.SpaceBetween
            ) {
                Text(
                    text = conversation.name,
                    style = MaterialTheme.typography.titleMedium,
                    fontWeight = FontWeight.Bold,
                    color = MaterialTheme.colorScheme.onBackground
                )
                Text(
                    text = conversation.time,
                    style = MaterialTheme.typography.labelMedium,
                    color = MaterialTheme.colorScheme.onSurfaceVariant
                )
            }
            
            Spacer(modifier = Modifier.height(4.dp))
            
            Row(
                modifier = Modifier.fillMaxWidth(),
                horizontalArrangement = Arrangement.SpaceBetween,
                verticalAlignment = Alignment.CenterVertically
            ) {
                Text(
                    text = conversation.lastMessage,
                    style = MaterialTheme.typography.bodyMedium,
                    color = MaterialTheme.colorScheme.onSurfaceVariant,
                    maxLines = 1,
                    overflow = TextOverflow.Ellipsis,
                    modifier = Modifier.weight(1f)
                )
                
                if (conversation.unreadCount > 0) {
                    Spacer(modifier = Modifier.width(8.dp))
                    Box(
                        modifier = Modifier
                            .size(24.dp)
                            .clip(CircleShape)
                            .background(PrimaryBlue),
                        contentAlignment = Alignment.Center
                    ) {
                        Text(
                            text = conversation.unreadCount.toString(),
                            style = MaterialTheme.typography.labelMedium,
                            color = MaterialTheme.colorScheme.onPrimary
                        )
                    }
                }
            }
        }
    }
}
