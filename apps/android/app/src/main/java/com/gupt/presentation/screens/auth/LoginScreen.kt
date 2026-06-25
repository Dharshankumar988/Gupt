package com.gupt.presentation.screens.auth

import androidx.compose.foundation.background
import androidx.compose.foundation.layout.*
import androidx.compose.foundation.shape.CircleShape
import androidx.compose.foundation.shape.RoundedCornerShape
import androidx.compose.material.icons.Icons
import androidx.compose.material.icons.filled.Lock
import androidx.compose.material.icons.filled.Email
import androidx.compose.material.icons.filled.VpnKey
import androidx.compose.material3.*
import androidx.compose.runtime.*
import androidx.compose.ui.Alignment
import androidx.compose.ui.Modifier
import androidx.compose.ui.draw.clip
import androidx.compose.ui.draw.shadow
import androidx.compose.ui.graphics.Brush
import androidx.compose.ui.graphics.Color
import androidx.compose.ui.text.font.FontWeight
import androidx.compose.ui.text.input.PasswordVisualTransformation
import androidx.compose.ui.unit.dp
import com.gupt.SupabaseConfig
import com.gupt.domain.model.AppUser
import com.gupt.domain.model.UserProfile
import com.gupt.presentation.theme.PrimaryBlue
import com.gupt.presentation.theme.AccentTeal
import io.github.jan.supabase.postgrest.postgrest
import kotlinx.coroutines.launch

@OptIn(ExperimentalMaterial3Api::class)
@Composable
fun LoginScreen(
    onLoginSuccess: () -> Unit,
    onNavigateToRegister: () -> Unit
) {
    var email by remember { mutableStateOf("") }
    var password by remember { mutableStateOf("") }
    var isLoading by remember { mutableStateOf(false) }
    var errorMessage by remember { mutableStateOf<String?>(null) }
    val scope = rememberCoroutineScope()

    Box(
        modifier = Modifier
            .fillMaxSize()
            .background(
                brush = Brush.verticalGradient(
                    colors = listOf(
                        Color(0xFF0F172A), // Slate 900
                        Color(0xFF020617)  // Slate 950
                    )
                )
            ),
        contentAlignment = Alignment.Center
    ) {
        Column(
            modifier = Modifier
                .fillMaxWidth()
                .padding(horizontal = 32.dp),
            horizontalAlignment = Alignment.CenterHorizontally
        ) {
            // Gupt Logo
            Box(
                modifier = Modifier
                    .size(120.dp)
                    .shadow(
                        elevation = 32.dp, 
                        shape = RoundedCornerShape(36.dp),
                        spotColor = AccentTeal,
                        ambientColor = PrimaryBlue
                    )
                    .clip(RoundedCornerShape(36.dp))
                    .background(
                        brush = Brush.linearGradient(
                            colors = listOf(PrimaryBlue, AccentTeal)
                        )
                    ),
                contentAlignment = Alignment.Center
            ) {
                Icon(
                    Icons.Default.Lock,
                    contentDescription = "Gupt Logo",
                    modifier = Modifier.size(60.dp),
                    tint = Color.White
                )
            }
            
            Spacer(modifier = Modifier.height(24.dp))
            
            Text(
                text = "Welcome to Gupt",
                style = MaterialTheme.typography.headlineMedium.copy(fontWeight = FontWeight.Bold),
                color = MaterialTheme.colorScheme.onBackground
            )
            
            Text(
                text = "Secure. Private. Offline-first.",
                style = MaterialTheme.typography.bodyMedium,
                color = MaterialTheme.colorScheme.onSurfaceVariant
            )
            
            Spacer(modifier = Modifier.height(48.dp))

            OutlinedTextField(
                value = email,
                onValueChange = { email = it },
                label = { Text("Email (or Username)") },
                leadingIcon = { Icon(Icons.Default.Email, contentDescription = "Email") },
                modifier = Modifier.fillMaxWidth(),
                shape = RoundedCornerShape(16.dp),
                singleLine = true,
                colors = OutlinedTextFieldDefaults.colors(
                    focusedBorderColor = PrimaryBlue,
                    unfocusedBorderColor = Color.Transparent,
                    focusedContainerColor = Color(0xFF1E1E1E),
                    unfocusedContainerColor = Color(0xFF2D2D2D)
                )
            )
            
            Spacer(modifier = Modifier.height(16.dp))

            OutlinedTextField(
                value = password,
                onValueChange = { password = it },
                label = { Text("Password") },
                leadingIcon = { Icon(Icons.Default.VpnKey, contentDescription = "Password") },
                visualTransformation = PasswordVisualTransformation(),
                modifier = Modifier.fillMaxWidth(),
                shape = RoundedCornerShape(16.dp),
                singleLine = true,
                colors = OutlinedTextFieldDefaults.colors(
                    focusedBorderColor = PrimaryBlue,
                    unfocusedBorderColor = Color.Transparent,
                    focusedContainerColor = Color(0xFF1E1E1E),
                    unfocusedContainerColor = Color(0xFF2D2D2D)
                )
            )
            
            Spacer(modifier = Modifier.height(24.dp))

            if (errorMessage != null) {
                Text(
                    text = errorMessage!!,
                    color = MaterialTheme.colorScheme.error,
                    style = MaterialTheme.typography.bodySmall
                )
                Spacer(modifier = Modifier.height(16.dp))
            }

            Button(
                onClick = {
                    scope.launch {
                        isLoading = true
                        try {
                            // Custom PostgREST Login Auth using existing 'users' table
                            val usersList = SupabaseConfig.client.postgrest["users"]
                                .select {
                                    filter {
                                        eq("username", email.trim())
                                        eq("password_hash", password)
                                    }
                                }.decodeList<AppUser>()

                            if (usersList.isNotEmpty()) {
                                com.gupt.domain.SessionManager.currentUser = usersList.first()
                                
                                // Fetch user profile for display name & rewards
                                try {
                                    val profiles = SupabaseConfig.client.postgrest["user_profiles"]
                                        .select {
                                            filter { eq("username", email.trim()) }
                                        }.decodeList<UserProfile>()
                                    if (profiles.isNotEmpty()) {
                                        val p = profiles.first()
                                        com.gupt.domain.SessionManager.updateDisplayName(p.display_name)
                                        com.gupt.domain.SessionManager.updateRelayRewards(p.relay_rewards_inr)
                                    }
                                } catch (_: Exception) { /* profile fetch optional */ }
                                
                                onLoginSuccess()
                            } else {
                                errorMessage = "Invalid email or password"
                            }
                        } catch (e: Exception) {
                            val fullMsg = e.message ?: "Login failed"
                            errorMessage = if (fullMsg.contains("row-level security")) {
                                "Supabase RLS is blocking reads. You must run 'CREATE POLICY' for the users table in your Supabase Dashboard."
                            } else {
                                fullMsg.substringBefore("\n").substringBefore("URL:").trim()
                            }
                        } finally {
                            isLoading = false
                        }
                    }
                },
                modifier = Modifier
                    .fillMaxWidth()
                    .height(56.dp)
                    .shadow(8.dp, RoundedCornerShape(16.dp)),
                shape = RoundedCornerShape(16.dp),
                colors = ButtonDefaults.buttonColors(containerColor = PrimaryBlue),
                enabled = !isLoading && email.isNotBlank() && password.isNotBlank()
            ) {
                if (isLoading) {
                    CircularProgressIndicator(
                        modifier = Modifier.size(24.dp),
                        color = Color.White,
                        strokeWidth = 2.dp
                    )
                } else {
                    Text("Login", style = MaterialTheme.typography.titleMedium, color = Color.White)
                }
            }

            Spacer(modifier = Modifier.height(24.dp))

            TextButton(onClick = onNavigateToRegister) {
                Text("Don't have an account? Register", color = PrimaryBlue)
            }
        }
    }
}
