package com.gupt.presentation.screens.splash

import androidx.compose.animation.core.*
import androidx.compose.foundation.Image
import androidx.compose.foundation.background
import androidx.compose.foundation.layout.*
import androidx.compose.material3.MaterialTheme
import androidx.compose.material3.Text
import androidx.compose.runtime.*
import androidx.compose.ui.Alignment
import androidx.compose.ui.Modifier
import androidx.compose.ui.draw.alpha
import androidx.compose.ui.draw.scale
import androidx.compose.ui.draw.clip
import androidx.compose.foundation.shape.CircleShape
import androidx.compose.ui.layout.ContentScale
import androidx.compose.ui.res.painterResource
import androidx.compose.ui.text.font.FontWeight
import androidx.compose.ui.unit.dp
import com.gupt.R
import kotlinx.coroutines.delay

@Composable
fun SplashScreen(
    onSplashFinished: () -> Unit
) {
    // Animation states
    var startAnimation by remember { mutableStateOf(false) }
    var showImage by remember { mutableStateOf(false) }

    // Gupt Logo Alpha Animation
    val alphaAnim by animateFloatAsState(
        targetValue = if (startAnimation) 1f else 0f,
        animationSpec = tween(durationMillis = 1000)
    )

    // DK Astronaut Pop-up Animation
    val scaleAnim by animateFloatAsState(
        targetValue = if (showImage) 1f else 0.5f,
        animationSpec = spring(
            dampingRatio = Spring.DampingRatioMediumBouncy,
            stiffness = Spring.StiffnessLow
        )
    )
    val imageAlphaAnim by animateFloatAsState(
        targetValue = if (showImage) 1f else 0f,
        animationSpec = tween(durationMillis = 800)
    )

    LaunchedEffect(key1 = true) {
        // Fade in Gupt Logo
        startAnimation = true
        delay(1200) // Wait for logo to settle
        
        // Pop up the astronaut image
        showImage = true
        delay(2000) // Hold the image
        
        // Navigate away
        onSplashFinished()
    }

    Box(
        modifier = Modifier
            .fillMaxSize()
            .background(MaterialTheme.colorScheme.background),
        contentAlignment = Alignment.Center
    ) {
        Column(
            horizontalAlignment = Alignment.CenterHorizontally,
            verticalArrangement = Arrangement.Center
        ) {
            // Gupt Logo
            Text(
                text = "Gupt",
                style = MaterialTheme.typography.displayLarge.copy(
                    fontWeight = FontWeight.Bold,
                    color = MaterialTheme.colorScheme.primary
                ),
                modifier = Modifier.alpha(alphaAnim)
            )

            Spacer(modifier = Modifier.height(40.dp))

            // Pop-up Image and "By" text
            if (showImage || scaleAnim > 0.5f) {
                Text(
                    text = "By",
                    style = MaterialTheme.typography.titleMedium.copy(
                        color = MaterialTheme.colorScheme.onBackground.copy(alpha = 0.7f)
                    ),
                    modifier = Modifier.alpha(imageAlphaAnim)
                )

                Spacer(modifier = Modifier.height(16.dp))

                Image(
                    painter = painterResource(id = R.drawable.dk_astronaut),
                    contentDescription = "DK Astronaut",
                    contentScale = ContentScale.Crop,
                    modifier = Modifier
                        .size(250.dp)
                        .scale(scaleAnim)
                        .alpha(imageAlphaAnim)
                        .clip(CircleShape)
                )
            }
        }
    }
}
