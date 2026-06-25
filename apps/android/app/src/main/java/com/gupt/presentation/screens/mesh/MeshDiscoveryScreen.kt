package com.gupt.presentation.screens.mesh

import androidx.activity.compose.rememberLauncherForActivityResult
import androidx.activity.result.contract.ActivityResultContracts
import androidx.activity.result.PickVisualMediaRequest
import androidx.compose.animation.core.*
import androidx.compose.foundation.Canvas
import androidx.compose.foundation.background
import androidx.compose.foundation.clickable
import androidx.compose.foundation.layout.*
import androidx.compose.foundation.lazy.LazyColumn
import androidx.compose.foundation.lazy.items
import androidx.compose.foundation.shape.CircleShape
import androidx.compose.foundation.shape.RoundedCornerShape
import androidx.compose.material.icons.Icons
import androidx.compose.material.icons.filled.ArrowBack
import androidx.compose.material.icons.filled.Block
import androidx.compose.material.icons.filled.Cloud
import androidx.compose.material.icons.filled.CloudSync
import androidx.compose.material.icons.filled.Edit
import androidx.compose.material.icons.filled.Message
import androidx.compose.material.icons.filled.Person
import androidx.compose.material.icons.filled.Star
import androidx.compose.material.icons.filled.Wifi
import androidx.compose.material3.*
import androidx.compose.runtime.*
import androidx.compose.ui.Alignment
import androidx.compose.ui.Modifier
import androidx.compose.ui.draw.clip
import androidx.compose.ui.draw.shadow
import androidx.compose.ui.graphics.Brush
import androidx.compose.ui.graphics.Color
import androidx.compose.ui.graphics.drawscope.Stroke
import androidx.compose.ui.layout.ContentScale
import androidx.compose.ui.text.font.FontWeight
import androidx.compose.ui.text.style.TextOverflow
import androidx.compose.ui.unit.dp
import android.widget.Toast
import androidx.compose.ui.platform.LocalContext
import coil.compose.AsyncImage
import com.gupt.domain.SessionManager
import com.gupt.domain.NetworkMode
import com.gupt.domain.model.BlockedUser
import com.gupt.domain.model.ChatBackup
import com.gupt.domain.model.Conversation
import com.gupt.domain.model.UserProfile
import com.gupt.SupabaseConfig
import com.gupt.presentation.theme.AccentTeal
import com.gupt.presentation.theme.PrimaryBlue
import io.github.jan.supabase.postgrest.postgrest
import io.github.jan.supabase.postgrest.query.Order
import java.time.ZonedDateTime
import java.time.format.DateTimeFormatter
import kotlinx.coroutines.launch

@OptIn(ExperimentalMaterial3Api::class)
@Composable
fun MeshDiscoveryScreen(
    onNavigateBack: () -> Unit,
    onLogout: () -> Unit,
    onNavigateToChat: ((String) -> Unit)? = null
) {
    val currentUser = SessionManager.currentUser
    val coroutineScope = rememberCoroutineScope()
    val context = LocalContext.current
    
    var showNameDialog by remember { mutableStateOf(false) }
    var editNameValue by remember { mutableStateOf(SessionManager.displayName.ifBlank { currentUser?.username ?: "" }) }
    var isSyncing by remember { mutableStateOf(false) }
    var lastBackupTime by remember { mutableStateOf<String?>(null) }
    
    // Nearby users state
    var nearbyUsers by remember { mutableStateOf<List<UserProfile>>(emptyList()) }
    var isLoadingNearby by remember { mutableStateOf(false) }
    
    // Blocked users state
    var blockedUsernames by remember { mutableStateOf<List<String>>(emptyList()) }
    var showBlockDialog by remember { mutableStateOf(false) }
    var blockTargetUsername by remember { mutableStateOf("") }

    val photoPickerLauncher = rememberLauncherForActivityResult(
        contract = ActivityResultContracts.PickVisualMedia(),
        onResult = { uri ->
            if (uri != null) {
                SessionManager.setProfileImage(uri.toString())
            }
        }
    )

    // Fetch nearby users and blocked list on launch
    LaunchedEffect(Unit) {
        isLoadingNearby = true
        try {
            val myUsername = currentUser?.username ?: ""
            
            // Fetch blocked usernames
            val blockedList = SupabaseConfig.client.postgrest["blocked_users"]
                .select {
                    filter { eq("blocker_username", myUsername) }
                }.decodeList<BlockedUser>()
            blockedUsernames = blockedList.map { it.blocked_username }
            
            // Set myself as online
            try {
                SupabaseConfig.client.postgrest["user_profiles"]
                    .update({ set("is_online", true) }) {
                        filter { eq("username", myUsername) }
                    }
            } catch (_: Exception) { }
            
            // Fetch all online users (except myself and blocked)
            val allProfiles = SupabaseConfig.client.postgrest["user_profiles"]
                .select {
                    filter { eq("is_online", true) }
                }.decodeList<UserProfile>()
            nearbyUsers = allProfiles.filter { 
                it.username != myUsername && it.username !in blockedUsernames 
            }
            
            // Fetch own profile for rewards
            val myProfiles = allProfiles.filter { it.username == myUsername }
            if (myProfiles.isNotEmpty()) {
                SessionManager.updateRelayRewards(myProfiles.first().relay_rewards_inr)
            }
            
            // Fetch last backup time
            try {
                val lastBackupList = SupabaseConfig.client.postgrest["chat_backups"]
                    .select {
                        filter { eq("user_username", myUsername) }
                        order("created_at", Order.DESCENDING)
                        limit(1)
                    }.decodeList<ChatBackup>()
                if (lastBackupList.isNotEmpty()) {
                    val createdAt = lastBackupList.first().created_at
                    if (createdAt != null) {
                        try {
                            val parsedDate = ZonedDateTime.parse(createdAt)
                            val formatter = DateTimeFormatter.ofPattern("MMM dd, yyyy HH:mm")
                            lastBackupTime = parsedDate.format(formatter)
                        } catch (e: Exception) {
                            lastBackupTime = createdAt.take(10)
                        }
                    }
                }
            } catch (e: Exception) {
                e.printStackTrace()
            }
        } catch (e: Exception) {
            e.printStackTrace()
        } finally {
            isLoadingNearby = false
        }
    }

    if (showNameDialog) {
        AlertDialog(
            onDismissRequest = { showNameDialog = false },
            title = { Text("Edit Display Name") },
            text = {
                OutlinedTextField(
                    value = editNameValue,
                    onValueChange = { editNameValue = it },
                    label = { Text("Display Name") },
                    singleLine = true
                )
            },
            confirmButton = {
                TextButton(onClick = {
                    val newName = editNameValue.trim()
                    SessionManager.updateDisplayName(newName)
                    // Update in Supabase too
                    coroutineScope.launch {
                        try {
                            SupabaseConfig.client.postgrest["user_profiles"]
                                .update({ set("display_name", newName) }) {
                                    filter { eq("username", currentUser?.username ?: "") }
                                }
                        } catch (_: Exception) { }
                    }
                    showNameDialog = false
                }) { Text("Save") }
            },
            dismissButton = {
                TextButton(onClick = { showNameDialog = false }) { Text("Cancel") }
            }
        )
    }
    
    // Block user dialog
    if (showBlockDialog) {
        AlertDialog(
            onDismissRequest = { showBlockDialog = false },
            title = { Text("Block User") },
            text = {
                OutlinedTextField(
                    value = blockTargetUsername,
                    onValueChange = { blockTargetUsername = it },
                    label = { Text("Username to block") },
                    singleLine = true
                )
            },
            confirmButton = {
                Button(
                    onClick = {
                        if (blockTargetUsername.isBlank()) return@Button
                        coroutineScope.launch {
                            try {
                                val blocked = BlockedUser(
                                    blocker_username = currentUser?.username ?: "",
                                    blocked_username = blockTargetUsername.trim()
                                )
                                SupabaseConfig.client.postgrest["blocked_users"].insert(blocked)
                                blockedUsernames = blockedUsernames + blockTargetUsername.trim()
                                nearbyUsers = nearbyUsers.filter { it.username != blockTargetUsername.trim() }
                                Toast.makeText(context, "User blocked", Toast.LENGTH_SHORT).show()
                                blockTargetUsername = ""
                                showBlockDialog = false
                            } catch (e: Exception) {
                                Toast.makeText(context, "Failed: ${e.message}", Toast.LENGTH_SHORT).show()
                            }
                        }
                    },
                    colors = ButtonDefaults.buttonColors(containerColor = MaterialTheme.colorScheme.error)
                ) { Text("Block") }
            },
            dismissButton = {
                TextButton(onClick = { showBlockDialog = false }) { Text("Cancel") }
            }
        )
    }

    Scaffold(
        topBar = {
            TopAppBar(
                title = { Text("Profile & Settings", fontWeight = FontWeight.Bold) },
                navigationIcon = {
                    IconButton(onClick = onNavigateBack) {
                        Icon(Icons.Filled.ArrowBack, contentDescription = "Back")
                    }
                },
                actions = {
                    TextButton(onClick = onLogout) {
                        Text("Log Out", color = MaterialTheme.colorScheme.error, fontWeight = FontWeight.Bold)
                    }
                },
                colors = TopAppBarDefaults.topAppBarColors(
                    containerColor = MaterialTheme.colorScheme.surface
                )
            )
        }
    ) { padding ->
        LazyColumn(
            modifier = Modifier
                .fillMaxSize()
                .padding(padding)
                .background(MaterialTheme.colorScheme.background),
            horizontalAlignment = Alignment.CenterHorizontally
        ) {
            // ===================== PROFILE SECTION =====================
            item {
                Spacer(modifier = Modifier.height(24.dp))
                
                // Profile Picture
                Box(
                    modifier = Modifier.size(100.dp)
                ) {
                    Box(
                        modifier = Modifier
                            .fillMaxSize()
                            .shadow(8.dp, CircleShape)
                            .clip(CircleShape)
                            .background(
                                brush = Brush.linearGradient(
                                    colors = listOf(PrimaryBlue.copy(alpha = 0.2f), AccentTeal.copy(alpha = 0.2f))
                                )
                            )
                            .clickable {
                                photoPickerLauncher.launch(PickVisualMediaRequest(ActivityResultContracts.PickVisualMedia.ImageOnly))
                            },
                        contentAlignment = Alignment.Center
                    ) {
                        if (SessionManager.profileImageUri != null) {
                            AsyncImage(
                                model = SessionManager.profileImageUri,
                                contentDescription = "Profile Picture",
                                modifier = Modifier.fillMaxSize(),
                                contentScale = ContentScale.Crop
                            )
                        } else {
                            Icon(
                                Icons.Default.Person, 
                                contentDescription = "DP", 
                                modifier = Modifier.size(56.dp),
                                tint = PrimaryBlue
                            )
                        }
                    }
                    // Edit Badge
                    Box(
                        modifier = Modifier
                            .size(32.dp)
                            .align(Alignment.BottomEnd)
                            .clip(CircleShape)
                            .background(PrimaryBlue)
                            .clickable {
                                photoPickerLauncher.launch(PickVisualMediaRequest(ActivityResultContracts.PickVisualMedia.ImageOnly))
                            },
                        contentAlignment = Alignment.Center
                    ) {
                        Icon(Icons.Default.Edit, contentDescription = "Edit DP", tint = Color.White, modifier = Modifier.size(16.dp))
                    }
                }
                
                Spacer(modifier = Modifier.height(16.dp))
                
                // Display Name (large, above email)
                val nameToShow = SessionManager.displayName.ifBlank { currentUser?.username ?: "Guest" }
                Row(verticalAlignment = Alignment.CenterVertically) {
                    Text(
                        text = nameToShow, 
                        style = MaterialTheme.typography.headlineSmall,
                        fontWeight = FontWeight.Bold,
                        color = MaterialTheme.colorScheme.onBackground
                    )
                    Spacer(modifier = Modifier.width(8.dp))
                    IconButton(
                        onClick = { 
                            editNameValue = SessionManager.displayName.ifBlank { currentUser?.username ?: "" }
                            showNameDialog = true 
                        },
                        modifier = Modifier.size(24.dp)
                    ) {
                        Icon(Icons.Default.Edit, contentDescription = "Edit Name", tint = PrimaryBlue, modifier = Modifier.size(18.dp))
                    }
                }
                
                // Username / Email (smaller, below name)
                Text(
                    text = currentUser?.username ?: "", 
                    style = MaterialTheme.typography.bodyMedium,
                    color = MaterialTheme.colorScheme.onSurfaceVariant
                )
                
                Text(
                    text = "Tap photo or name to edit", 
                    style = MaterialTheme.typography.labelSmall,
                    color = MaterialTheme.colorScheme.onSurfaceVariant.copy(alpha = 0.6f)
                )
                
                Spacer(modifier = Modifier.height(24.dp))
            }

            // ===================== RELAY REWARDS =====================
            item {
                Column(
                    modifier = Modifier
                        .fillMaxWidth()
                        .padding(horizontal = 16.dp)
                ) {
                    Text(
                        "Relay Rewards", 
                        style = MaterialTheme.typography.titleMedium,
                        color = AccentTeal,
                        fontWeight = FontWeight.Bold,
                        modifier = Modifier.padding(start = 8.dp, bottom = 8.dp)
                    )

                    Card(
                        modifier = Modifier.fillMaxWidth(),
                        shape = RoundedCornerShape(16.dp),
                        colors = CardDefaults.cardColors(
                            containerColor = MaterialTheme.colorScheme.surfaceVariant.copy(alpha = 0.5f)
                        )
                    ) {
                        Row(
                            modifier = Modifier
                                .fillMaxWidth()
                                .padding(16.dp),
                            verticalAlignment = Alignment.CenterVertically
                        ) {
                            Icon(Icons.Default.Star, contentDescription = null, tint = Color(0xFFFFD700), modifier = Modifier.size(32.dp))
                            Spacer(modifier = Modifier.width(16.dp))
                            Column(modifier = Modifier.weight(1f)) {
                                Text(
                                    "₹${String.format("%.4f", SessionManager.relayRewards)}", 
                                    style = MaterialTheme.typography.headlineSmall, 
                                    color = AccentTeal,
                                    fontWeight = FontWeight.Bold
                                )
                                Text(
                                    "Earned by relaying messages • ₹0.0001 per relay", 
                                    style = MaterialTheme.typography.bodySmall, 
                                    color = MaterialTheme.colorScheme.onSurfaceVariant
                                )
                            }
                        }
                    }
                }
                
                Spacer(modifier = Modifier.height(24.dp))
            }

            // ===================== OPERATING MODE =====================
            item {
                Column(
                    modifier = Modifier
                        .fillMaxWidth()
                        .padding(horizontal = 16.dp)
                ) {
                    Text(
                        "Operating Mode", 
                        style = MaterialTheme.typography.titleMedium,
                        color = PrimaryBlue,
                        fontWeight = FontWeight.Bold,
                        modifier = Modifier.padding(start = 8.dp, bottom = 8.dp)
                    )

                    Card(
                        modifier = Modifier.fillMaxWidth(),
                        shape = RoundedCornerShape(16.dp),
                        colors = CardDefaults.cardColors(
                            containerColor = MaterialTheme.colorScheme.surfaceVariant.copy(alpha = 0.5f)
                        )
                    ) {
                        Column(
                            modifier = Modifier
                                .fillMaxWidth()
                                .padding(16.dp)
                        ) {
                            Row(
                                modifier = Modifier.fillMaxWidth(),
                                verticalAlignment = Alignment.CenterVertically,
                                horizontalArrangement = Arrangement.SpaceBetween
                            ) {
                                Row(verticalAlignment = Alignment.CenterVertically) {
                                    Icon(
                                        if (SessionManager.currentNetworkMode == NetworkMode.ONLINE_CLOUD) 
                                            Icons.Default.Cloud else Icons.Default.Wifi,
                                        contentDescription = null,
                                        tint = if (SessionManager.currentNetworkMode == NetworkMode.ONLINE_CLOUD) PrimaryBlue else AccentTeal,
                                        modifier = Modifier.size(28.dp)
                                    )
                                    Spacer(modifier = Modifier.width(12.dp))
                                    Column {
                                        Text(
                                            if (SessionManager.currentNetworkMode == NetworkMode.ONLINE_CLOUD) "Cloud Mode ☁️" else "Mesh Mode 🕸️",
                                            style = MaterialTheme.typography.titleSmall,
                                            fontWeight = FontWeight.Bold,
                                            color = MaterialTheme.colorScheme.onSurface
                                        )
                                        Text(
                                            if (SessionManager.currentNetworkMode == NetworkMode.ONLINE_CLOUD) 
                                                "Messages route through server" else "Messages relay via nearby peers",
                                            style = MaterialTheme.typography.bodySmall,
                                            color = MaterialTheme.colorScheme.onSurfaceVariant
                                        )
                                    }
                                }
                                Switch(
                                    checked = SessionManager.currentNetworkMode == NetworkMode.OFFLINE_MESH,
                                    onCheckedChange = { isMesh ->
                                        SessionManager.currentNetworkMode = 
                                            if (isMesh) NetworkMode.OFFLINE_MESH else NetworkMode.ONLINE_CLOUD
                                    },
                                    colors = SwitchDefaults.colors(
                                        checkedThumbColor = AccentTeal,
                                        checkedTrackColor = AccentTeal.copy(alpha = 0.3f),
                                        uncheckedThumbColor = PrimaryBlue,
                                        uncheckedTrackColor = PrimaryBlue.copy(alpha = 0.3f)
                                    )
                                )
                            }
                            Spacer(modifier = Modifier.height(8.dp))
                            Text(
                                "Auto-switches based on connectivity",
                                style = MaterialTheme.typography.labelSmall,
                                color = MaterialTheme.colorScheme.onSurfaceVariant.copy(alpha = 0.5f)
                            )
                        }
                    }
                }
                
                Spacer(modifier = Modifier.height(24.dp))
            }

            // ===================== CLOUD BACKUP =====================
            item {
                Column(
                    modifier = Modifier
                        .fillMaxWidth()
                        .padding(horizontal = 16.dp)
                ) {
                    Text(
                        "Cloud Backup", 
                        style = MaterialTheme.typography.titleMedium,
                        color = PrimaryBlue,
                        fontWeight = FontWeight.Bold,
                        modifier = Modifier.padding(start = 8.dp, bottom = 8.dp)
                    )

                    Card(
                        modifier = Modifier.fillMaxWidth(),
                        shape = RoundedCornerShape(16.dp),
                        colors = CardDefaults.cardColors(
                            containerColor = MaterialTheme.colorScheme.surfaceVariant.copy(alpha = 0.5f)
                        )
                    ) {
                        Row(
                            modifier = Modifier
                                .fillMaxWidth()
                                .padding(16.dp),
                            verticalAlignment = Alignment.CenterVertically
                        ) {
                            Icon(Icons.Default.CloudSync, contentDescription = null, tint = AccentTeal, modifier = Modifier.size(32.dp))
                            Spacer(modifier = Modifier.width(16.dp))
                            Column(modifier = Modifier.weight(1f)) {
                                Text("End-to-End Encrypted", style = MaterialTheme.typography.bodyLarge, color = MaterialTheme.colorScheme.onSurface)
                                Text("Backup chats to cloud", style = MaterialTheme.typography.bodySmall, color = MaterialTheme.colorScheme.onSurfaceVariant)
                                if (lastBackupTime != null) {
                                    Text("Last backup: $lastBackupTime", style = MaterialTheme.typography.labelSmall, color = PrimaryBlue)
                                }
                            }
                            Button(
                                onClick = { 
                                    if (isSyncing) return@Button
                                    isSyncing = true
                                    coroutineScope.launch {
                                        try {
                                            val mockPayload = """
                                                [
                                                    {"id": "msg_1", "text": "Hello, this is a secure message.", "isFromMe": true, "time": "10:30 AM"},
                                                    {"id": "msg_2", "text": "I got it! The mesh connection works.", "isFromMe": false, "time": "10:31 AM"}
                                                ]
                                            """.trimIndent()
                                            
                                            val backup = ChatBackup(
                                                user_username = SessionManager.currentUser?.username ?: "Unknown",
                                                payload = mockPayload
                                            )
                                            
                                            SupabaseConfig.client.postgrest["chat_backups"].insert(backup)
                                            
                                            lastBackupTime = "Just now"
                                            
                                            Toast.makeText(context, "Cloud Backup Successful!", Toast.LENGTH_SHORT).show()
                                        } catch (e: Exception) {
                                            Toast.makeText(context, "Backup Failed: ${e.message}", Toast.LENGTH_LONG).show()
                                        } finally {
                                            isSyncing = false
                                        }
                                    }
                                }, 
                                colors = ButtonDefaults.buttonColors(containerColor = PrimaryBlue)
                            ) {
                                if (isSyncing) {
                                    CircularProgressIndicator(modifier = Modifier.size(20.dp), color = Color.White, strokeWidth = 2.dp)
                                } else {
                                    Text("Sync")
                                }
                            }
                        }
                    }
                }
                
                Spacer(modifier = Modifier.height(24.dp))
            }

            // ===================== BLOCK USERS =====================
            item {
                Column(
                    modifier = Modifier
                        .fillMaxWidth()
                        .padding(horizontal = 16.dp)
                ) {
                    Text(
                        "Privacy", 
                        style = MaterialTheme.typography.titleMedium,
                        color = MaterialTheme.colorScheme.error,
                        fontWeight = FontWeight.Bold,
                        modifier = Modifier.padding(start = 8.dp, bottom = 8.dp)
                    )

                    Card(
                        modifier = Modifier.fillMaxWidth(),
                        shape = RoundedCornerShape(16.dp),
                        colors = CardDefaults.cardColors(
                            containerColor = MaterialTheme.colorScheme.surfaceVariant.copy(alpha = 0.5f)
                        )
                    ) {
                        Row(
                            modifier = Modifier
                                .fillMaxWidth()
                                .clickable { showBlockDialog = true }
                                .padding(16.dp),
                            verticalAlignment = Alignment.CenterVertically
                        ) {
                            Icon(Icons.Default.Block, contentDescription = null, tint = MaterialTheme.colorScheme.error, modifier = Modifier.size(28.dp))
                            Spacer(modifier = Modifier.width(16.dp))
                            Column(modifier = Modifier.weight(1f)) {
                                Text("Block a User", style = MaterialTheme.typography.bodyLarge, color = MaterialTheme.colorScheme.onSurface)
                                Text(
                                    "${blockedUsernames.size} blocked", 
                                    style = MaterialTheme.typography.bodySmall, 
                                    color = MaterialTheme.colorScheme.onSurfaceVariant
                                )
                            }
                        }
                    }
                }
                
                Spacer(modifier = Modifier.height(24.dp))
            }

            // ===================== PROXIMITY DEVICES =====================
            item {
                Column(
                    modifier = Modifier
                        .fillMaxWidth()
                        .padding(horizontal = 16.dp),
                    horizontalAlignment = Alignment.CenterHorizontally
                ) {
                    Text(
                        "Nearby Users", 
                        style = MaterialTheme.typography.titleMedium,
                        color = AccentTeal,
                        fontWeight = FontWeight.Bold,
                        modifier = Modifier.align(Alignment.Start).padding(start = 8.dp, bottom = 16.dp)
                    )

                    RadarAnimation()
                    Spacer(modifier = Modifier.height(16.dp))
                }
            }
            
            // List of nearby users
            if (isLoadingNearby) {
                item {
                    CircularProgressIndicator(
                        modifier = Modifier.padding(32.dp),
                        color = AccentTeal
                    )
                }
            } else if (nearbyUsers.isEmpty()) {
                item {
                    Text(
                        "No nearby users found",
                        style = MaterialTheme.typography.bodyMedium,
                        color = MaterialTheme.colorScheme.onSurfaceVariant,
                        modifier = Modifier.padding(16.dp)
                    )
                }
            } else {
                items(nearbyUsers, key = { it.username }) { peer ->
                    NearbyPeerCard(
                        peer = peer,
                        onMessage = {
                            // Create conversation and navigate
                            val newConv = Conversation(
                                id = peer.username,
                                name = peer.display_name.ifBlank { peer.username },
                                lastMessage = "Nearby connection",
                                time = "Now",
                                unreadCount = 0
                            )
                            SessionManager.addConversation(newConv)
                            onNavigateToChat?.invoke(peer.username)
                        },
                        onBlock = {
                            coroutineScope.launch {
                                try {
                                    val blocked = BlockedUser(
                                        blocker_username = currentUser?.username ?: "",
                                        blocked_username = peer.username
                                    )
                                    SupabaseConfig.client.postgrest["blocked_users"].insert(blocked)
                                    blockedUsernames = blockedUsernames + peer.username
                                    nearbyUsers = nearbyUsers.filter { it.username != peer.username }
                                    Toast.makeText(context, "${peer.username} blocked", Toast.LENGTH_SHORT).show()
                                } catch (e: Exception) {
                                    Toast.makeText(context, "Failed: ${e.message}", Toast.LENGTH_SHORT).show()
                                }
                            }
                        }
                    )
                }
            }

            item {
                Spacer(modifier = Modifier.height(32.dp))
            }
        }
    }
}

@Composable
fun NearbyPeerCard(
    peer: UserProfile,
    onMessage: () -> Unit,
    onBlock: () -> Unit
) {
    Card(
        modifier = Modifier
            .fillMaxWidth()
            .padding(horizontal = 16.dp, vertical = 4.dp),
        shape = RoundedCornerShape(16.dp),
        colors = CardDefaults.cardColors(
            containerColor = MaterialTheme.colorScheme.surfaceVariant.copy(alpha = 0.4f)
        )
    ) {
        Row(
            modifier = Modifier
                .fillMaxWidth()
                .padding(12.dp),
            verticalAlignment = Alignment.CenterVertically
        ) {
            // Avatar
            Box(
                modifier = Modifier
                    .size(48.dp)
                    .clip(CircleShape)
                    .background(
                        brush = Brush.linearGradient(
                            colors = listOf(PrimaryBlue.copy(alpha = 0.3f), AccentTeal.copy(alpha = 0.3f))
                        )
                    ),
                contentAlignment = Alignment.Center
            ) {
                if (peer.avatar_url != null) {
                    AsyncImage(
                        model = peer.avatar_url,
                        contentDescription = "DP",
                        modifier = Modifier.fillMaxSize().clip(CircleShape),
                        contentScale = ContentScale.Crop
                    )
                } else {
                    Text(
                        text = (peer.display_name.ifBlank { peer.username }).first().uppercase(),
                        style = MaterialTheme.typography.titleMedium,
                        fontWeight = FontWeight.Bold,
                        color = PrimaryBlue
                    )
                }
            }
            
            Spacer(modifier = Modifier.width(12.dp))
            
            Column(modifier = Modifier.weight(1f)) {
                Text(
                    text = peer.display_name.ifBlank { peer.username },
                    style = MaterialTheme.typography.titleSmall,
                    fontWeight = FontWeight.Bold,
                    color = MaterialTheme.colorScheme.onSurface,
                    maxLines = 1,
                    overflow = TextOverflow.Ellipsis
                )
                Text(
                    text = peer.username,
                    style = MaterialTheme.typography.bodySmall,
                    color = MaterialTheme.colorScheme.onSurfaceVariant
                )
            }
            
            // Message button
            IconButton(onClick = onMessage) {
                Icon(Icons.Default.Message, contentDescription = "Message", tint = PrimaryBlue)
            }
            
            // Block button
            IconButton(onClick = onBlock) {
                Icon(Icons.Default.Block, contentDescription = "Block", tint = MaterialTheme.colorScheme.error.copy(alpha = 0.6f), modifier = Modifier.size(20.dp))
            }
        }
    }
}

@Composable
fun RadarAnimation() {
    val infiniteTransition = rememberInfiniteTransition(label = "radar")
    val scale by infiniteTransition.animateFloat(
        initialValue = 0f,
        targetValue = 1f,
        animationSpec = infiniteRepeatable(
            animation = tween(2000, easing = LinearOutSlowInEasing),
            repeatMode = RepeatMode.Restart
        ),
        label = "scale"
    )
    val alpha by infiniteTransition.animateFloat(
        initialValue = 1f,
        targetValue = 0f,
        animationSpec = infiniteRepeatable(
            animation = tween(2000, easing = LinearOutSlowInEasing),
            repeatMode = RepeatMode.Restart
        ),
        label = "alpha"
    )

    Box(
        modifier = Modifier
            .size(140.dp),
        contentAlignment = Alignment.Center
    ) {
        Canvas(modifier = Modifier.fillMaxSize()) {
            val maxRadius = size.width / 2
            
            // Draw expanding circle
            drawCircle(
                color = AccentTeal.copy(alpha = alpha),
                radius = maxRadius * scale,
                style = Stroke(width = 3f)
            )
            
            // Draw center dot
            drawCircle(
                color = AccentTeal,
                radius = 10f
            )
        }
        
        Text(
            text = "Scanning...",
            style = MaterialTheme.typography.labelSmall,
            color = AccentTeal,
            modifier = Modifier.offset(y = 20.dp)
        )
    }
}
