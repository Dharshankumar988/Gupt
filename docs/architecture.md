# Architecture Overview

## Three-Layer System

1. **Mobile Applications**
   - Pure UI and platform integration.
   - Built with Jetpack Compose (Android) and SwiftUI (iOS).
   - No complex business logic here.

2. **Core Engine (Rust)**
   - Shared business logic.
   - Handles cryptography, routing, SQLite storage, mesh networking.
   - Exposes a unified API via UniFFI.

3. **Cloud Backend (Rust/Axum + Supabase)**
   - Blind encrypted relay.
   - Public key directory.
   - Device registration.
   - Cannot decrypt messages or read metadata beyond what is strictly required for routing.
