package com.gupt.presentation.navigation

import androidx.compose.runtime.Composable
import androidx.navigation.NavHostController
import androidx.navigation.compose.NavHost
import androidx.navigation.compose.composable
import androidx.navigation.compose.rememberNavController
import com.gupt.presentation.screens.chat.ChatScreen
import com.gupt.presentation.screens.home.ConversationsScreen
import com.gupt.presentation.screens.mesh.MeshDiscoveryScreen
import com.gupt.presentation.screens.onboarding.OnboardingScreen
import com.gupt.presentation.screens.auth.LoginScreen
import com.gupt.presentation.screens.auth.RegisterScreen

sealed class Screen(val route: String) {
    object Login : Screen("login")
    object Register : Screen("register")
    object Onboarding : Screen("onboarding")
    object Home : Screen("home")
    object Splash : Screen("splash")
    object Chat : Screen("chat/{username}") {
        fun createRoute(username: String) = "chat/$username"
    }
    object MeshDiscovery : Screen("mesh_discovery")
    object Profile : Screen("profile")
}

@Composable
fun GuptNavGraph(
    navController: NavHostController = rememberNavController(),
    startDestination: String = Screen.Splash.route
) {
    NavHost(
        navController = navController,
        startDestination = startDestination
    ) {
        composable(route = Screen.Splash.route) {
            com.gupt.presentation.screens.splash.SplashScreen(
                onSplashFinished = {
                    val nextRoute = if (com.gupt.domain.SessionManager.currentUser != null) Screen.Home.route else Screen.Login.route
                    navController.navigate(nextRoute) {
                        popUpTo(Screen.Splash.route) { inclusive = true }
                    }
                }
            )
        }

        composable(route = Screen.Login.route) {
            LoginScreen(
                onLoginSuccess = {
                    navController.navigate(Screen.Home.route) {
                        popUpTo(Screen.Login.route) { inclusive = true }
                    }
                },
                onNavigateToRegister = {
                    navController.navigate(Screen.Register.route)
                }
            )
        }
        
        composable(route = Screen.Register.route) {
            RegisterScreen(
                onRegisterSuccess = {
                    navController.navigate(Screen.Home.route) {
                        popUpTo(Screen.Register.route) { inclusive = true }
                        popUpTo(Screen.Login.route) { inclusive = true }
                    }
                },
                onNavigateToLogin = {
                    navController.popBackStack()
                }
            )
        }

        composable(route = Screen.Onboarding.route) {
            OnboardingScreen(
                onOnboardingComplete = {
                    navController.navigate(Screen.Home.route) {
                        popUpTo(Screen.Onboarding.route) { inclusive = true }
                    }
                }
            )
        }
        
        composable(route = Screen.Home.route) {
            ConversationsScreen(
                onNavigateToChat = { username ->
                    navController.navigate(Screen.Chat.createRoute(username))
                },
                onNavigateToSettings = {
                    navController.navigate(Screen.Profile.route)
                }
            )
        }
        
        composable(route = Screen.Chat.route) { backStackEntry ->
            val username = backStackEntry.arguments?.getString("username") ?: return@composable
            ChatScreen(
                username = username,
                onNavigateBack = { navController.popBackStack() }
            )
        }
        
        composable(route = Screen.MeshDiscovery.route) {
            MeshDiscoveryScreen(
                onNavigateBack = { navController.popBackStack() },
                onLogout = {
                    com.gupt.domain.SessionManager.logout()
                    navController.navigate(Screen.Login.route) {
                        popUpTo(0) { inclusive = true } // Clear entire backstack
                    }
                },
                onNavigateToChat = { username ->
                    navController.navigate(Screen.Chat.createRoute(username))
                }
            )
        }
        
        composable(route = Screen.Profile.route) {
            com.gupt.presentation.screens.profile.ProfileScreen(
                onNavigateBack = { navController.popBackStack() },
                onNavigateToMesh = { navController.navigate(Screen.MeshDiscovery.route) },
                onLogout = {
                    com.gupt.domain.SessionManager.logout()
                    navController.navigate(Screen.Login.route) {
                        popUpTo(0) { inclusive = true } // Clear entire backstack
                    }
                }
            )
        }
    }
}
