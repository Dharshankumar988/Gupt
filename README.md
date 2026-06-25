<div align="center">
  <img src="https://capsule-render.vercel.app/api?type=waving&color=gradient&height=200&section=header&text=Gupt&fontSize=90&animation=fadeIn" />

  <a href="https://git.io/typing-svg"><img src="https://readme-typing-svg.herokuapp.com?font=Fira+Code&weight=600&size=24&duration=3000&pause=1000&color=36BCF7&center=true&vCenter=true&width=600&lines=Hybrid+Adaptive+Communication;No+Internet?+No+Problem.;Secure.+Private.+Relentless.;Written+in+Rust.+Built+for+Everyone." alt="Typing SVG" /></a>
  
  <p><b>Your conversations, unbound by networks.</b></p>

  <!-- Badges -->
  <p>
    <img src="https://img.shields.io/badge/Rust-Core%20Engine-orange?style=for-the-badge&logo=rust" alt="Rust Core" />
    <img src="https://img.shields.io/badge/Kotlin-Android UI-purple?style=for-the-badge&logo=kotlin" alt="Kotlin" />
    <img src="https://img.shields.io/badge/Swift-iOS UI-blue?style=for-the-badge&logo=swift" alt="Swift" />
    <img src="https://img.shields.io/badge/End_to_End-Encrypted-success?style=for-the-badge&logo=lock" alt="E2EE" />
  </p>
</div>

---

### 📥 Download the Latest Build
**Android APK:** `C:\IMP PROJECTS\Gupt\apps\android\app\build\outputs\apk\debug\Gupt.apk`

---

## 🌟 The Vision: Communication Without Borders

Imagine you are in a crowded stadium where the cell towers are overwhelmed. Or perhaps hiking deep in the mountains, miles away from a Wi-Fi signal. Or in a disaster zone where traditional networks have collapsed. 

You need to send a message. You hit **"Send"**.

With traditional apps, the message fails. With **Gupt** ("Secret/Hidden" in Hindi), the message *always* finds a way. 

Gupt is a **hybrid adaptive communication platform** that doesn't just rely on the Internet. It intelligently routes your messages through Bluetooth Low Energy (BLE), Wi-Fi Direct, Delay-Tolerant Mesh Networking, and traditional Internet Relays—completely transparently. You don't choose the network; Gupt finds the most reliable, battery-efficient, and private route for you.

---

## ✨ Why Gupt Feels Like Magic

- 🚀 **Zero-Configuration Routing:** Just type and send. Gupt dynamically hops between BLE, Wi-Fi Direct, and the Cloud based on what's available *right now*.
- 🛡️ **Paranoid-Level Security:** Everything is End-to-End Encrypted (X25519, Ed25519, XChaCha20-Poly1305). Private keys *never* leave your device. The cloud backend is completely blind to your data.
- 🔋 **Battery Conscious:** Adaptive scanning means Gupt isn't draining your battery while hunting for peers.
- 🤝 **Store-and-Forward Mesh:** If your friend isn't nearby, your message can securely hop through trusted intermediary devices until it reaches them.

---

## 🏗️ Architecture Under the Hood

Gupt achieves cross-platform native performance by combining the speed and safety of Rust with the native feel of Kotlin and Swift.

<div align="center">
  <img src="https://raw.githubusercontent.com/andreasbm/readme/master/assets/lines/rainbow.png" alt="rainbow divider" width="100%">
</div>

1. **Native UI (The Face)**: Jetpack Compose for Android and SwiftUI for iOS ensure the app feels right at home on your device.
2. **Rust Core (The Brain)**: A shared Rust engine (via UniFFI) powers the Adaptive Routing, Cryptography, Mesh Logic, and local SQLite storage. Write once, run securely everywhere.
3. **Cloud Backend (The Relay)**: A blazing-fast Axum server running on Rust, backed by Supabase PostgreSQL. It coordinates connections and relays encrypted blobs without ever knowing what they say.

---

## 🧰 The Tech Stack

| Domain | Magic Spells (Tech) |
| :--- | :--- |
| **Android UI** | `<Kotlin>` `<Jetpack Compose>` |
| **iOS UI** | `<Swift>` `<SwiftUI>` |
| **Core Engine** | `<Rust>` `<UniFFI>` `<SQLCipher>` |
| **Cryptography** | `<Ed25519>` `<X25519>` `<XChaCha20-Poly1305>` `<Argon2id>` |
| **Backend API** | `<Rust>` `<Axum>` `<SQLx>` |
| **Database** | `<Supabase PostgreSQL>` |
| **Push Notifications**| `<UnifiedPush>` |

---

## 🗺️ Exploring the Codebase

```text
gupt/
├── 📱 apps/               # The beautiful user interfaces
│   ├── android/        # Kotlin + Jetpack Compose Magic
│   └── ios/            # Swift + SwiftUI Elegance
├── ☁️ backend/            # The blind cloud relay infrastructure
│   ├── api/            # Axum REST API
│   ├── relay/          # Encrypted message broker
│   └── database/       # DB connection & migrations
├── 🧠 core/               # The shared Rust brain (UniFFI)
│   ├── crypto/         # Military-grade cryptography
│   ├── mesh/           # Store-and-forward mesh logic
│   ├── routing/        # The smart adaptive router
│   └── storage/        # Local SQLCipher storage
└── 📚 docs/               # Deep dive documentation
```

---

## 🚀 Getting Started

Ready to build the future of communication? Let's get your environment set up.

### What you'll need:
- 🦀 **Rust 1.75+** (The foundation)
- 🤖 **Android Studio** & Android SDK (API 34+)
- 🍎 **Xcode 16+** (For the Apple ecosystem)
- 📦 **cargo-ndk** (To compile Rust for Android)
- ☁️ **Supabase** account (For the backend database)

### Firing up the engines:

```bash
# Clone the repository
git clone https://github.com/yourusername/gupt.git
cd gupt

# Build the entire Rust workspace 🚀
cargo build --workspace

# Run the test suite to ensure all systems are go ✅
cargo test --workspace
```

---

## 🛡️ The Gupt Philosophy

1. **Security First**: Trust math, not servers.
2. **Privacy First**: We collect nothing. The cloud is a dumb pipe.
3. **Offline First**: The Internet is a luxury, not a requirement.
4. **Simplicity**: Users shouldn't need a networking degree to send a text.
5. **Native Performance**: No electron, no webviews. Just pure speed.

---

<div align="center">
  <img src="https://raw.githubusercontent.com/andreasbm/readme/master/assets/lines/rainbow.png" alt="rainbow divider" width="100%">
  <br>
  <i>Built with ❤️ for a more connected, resilient world.</i>
  <br><br>
  <b>MIT License</b>
</div>
