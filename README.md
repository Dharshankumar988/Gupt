# <div align="center">

<img src="https://capsule-render.vercel.app/api?type=waving&color=gradient&height=220&section=header&text=Gupt&fontSize=90&animation=fadeIn"/>

[![Typing SVG](https://readme-typing-svg.herokuapp.com?font=Fira+Code\&weight=600\&size=24\&duration=3000\&pause=1000\&color=36BCF7\&center=true\&vCenter=true\&width=700\&lines=Hybrid+Adaptive+Communication;Offline+First+Messaging;End-to-End+Encrypted;Rust+Powered.+Native+Performance.)](https://git.io/typing-svg)

### **Private. Secure. Offline. Adaptive.**

*A next-generation messaging platform that intelligently communicates over Bluetooth, Wi-Fi Direct, Mesh Networks, and the Internet—without requiring users to think about connectivity.*

<br>

![Rust](https://img.shields.io/badge/Rust-Core%20Engine-orange?style=for-the-badge\&logo=rust)
![Kotlin](https://img.shields.io/badge/Kotlin-Android-purple?style=for-the-badge\&logo=kotlin)
![Swift](https://img.shields.io/badge/Swift-iOS-blue?style=for-the-badge\&logo=swift)
![Encryption](https://img.shields.io/badge/End--to--End-Encrypted-success?style=for-the-badge\&logo=lock)

</div>

---

# 📥 Latest Android Build

```
apps/android/app/build/outputs/apk/debug/Gupt.apk
```

---

# 🌍 Why Gupt?

Messaging applications today depend entirely on the Internet.

When the network is unavailable, overloaded, censored, or destroyed, communication simply stops.

**Gupt changes that.**

Instead of relying on a single communication channel, Gupt continuously adapts to whatever connectivity exists around you.

Whether you're:

* 🏔 Hiking in remote mountains
* 🎵 At a packed concert
* 🚨 In a disaster zone
* 🚆 Underground with no signal
* ✈ Traveling without data

Gupt automatically discovers the best available path and delivers your encrypted messages.

**No configuration. No manual switching. No compromises.**

---

# ✨ Key Features

## 🚀 Adaptive Communication

Gupt intelligently switches between:

* Bluetooth Low Energy (BLE)
* Wi-Fi Direct
* Delay-Tolerant Mesh Networking
* Internet Relay

The transition is completely transparent to users.

---

## 🔒 End-to-End Encryption

Security is built into every layer.

* X25519 Key Exchange
* Ed25519 Digital Signatures
* XChaCha20-Poly1305 Encryption
* Argon2id Password Hashing

Private keys never leave your device.

The backend cannot decrypt messages.

Ever.

---

## 🌐 Offline-First Architecture

Unlike traditional messengers, Gupt treats the Internet as **optional**.

Messages continue moving through nearby devices until they reach their destination.

Even if both users are offline.

---

## 🔋 Battery Optimized

Adaptive scanning prevents unnecessary Bluetooth and Wi-Fi usage.

The routing engine constantly balances:

* Connectivity
* Battery consumption
* Signal strength
* Reliability

---

## 🤝 Delay-Tolerant Mesh

Messages don't fail.

If the recipient isn't reachable immediately:

* Messages are securely stored
* Forwarded through nearby trusted devices
* Delivered once connectivity is restored

---

# 🏗 Architecture

```
                    ┌──────────────────────┐
                    │   Android (Compose)  │
                    └──────────┬───────────┘
                               │
                    ┌──────────▼───────────┐
                    │      Rust Core       │
                    │──────────────────────│
                    │ Adaptive Routing     │
                    │ Cryptography         │
                    │ Mesh Networking      │
                    │ Local Storage        │
                    └──────────┬───────────┘
                               │
                ┌──────────────┼──────────────┐
                │              │              │
              BLE        Wi-Fi Direct     Internet
                │              │              │
                └──────────────┼──────────────┘
                               │
                     Rust Axum Backend
                               │
                     Supabase PostgreSQL
```

---

# ⚙ Technology Stack

| Layer             | Technologies                                     |
| ----------------- | ------------------------------------------------ |
| **Android**       | Kotlin • Jetpack Compose                         |
| **iOS**           | Swift • SwiftUI                                  |
| **Core Engine**   | Rust • UniFFI                                    |
| **Storage**       | SQLCipher                                        |
| **Cryptography**  | Ed25519 • X25519 • XChaCha20-Poly1305 • Argon2id |
| **Backend**       | Rust • Axum • SQLx                               |
| **Database**      | Supabase PostgreSQL                              |
| **Notifications** | UnifiedPush                                      |

---

# 📂 Project Structure

```text
gupt
│
├── apps
│   ├── android
│   └── ios
│
├── backend
│   ├── api
│   ├── relay
│   └── database
│
├── core
│   ├── crypto
│   ├── mesh
│   ├── routing
│   └── storage
│
└── docs
```

---

# 🚀 Getting Started

## Requirements

* Rust 1.75+
* Android Studio
* Android SDK 34+
* Xcode 16+
* cargo-ndk
* Supabase Account

---

## Clone

```bash
git clone https://github.com/yourusername/gupt.git

cd gupt
```

---

## Build

```bash
cargo build --workspace
```

---

## Run Tests

```bash
cargo test --workspace
```

---

# 🛡 Design Principles

### 🔐 Security First

Trust mathematics—not servers.

---

### 🕵 Privacy First

The cloud stores encrypted blobs.

It never sees your conversations.

---

### 📡 Offline First

Internet connectivity is a bonus.

Communication shouldn't depend on it.

---

### ⚡ Native Performance

No Electron.

No WebViews.

Pure Rust with native Android and iOS interfaces.

---

### ❤️ Simplicity

Users send messages.

Gupt handles everything else.

---

# 🚀 Future Roadmap

* [ ] Group Mesh Messaging
* [ ] Voice Messages
* [ ] File Sharing
* [ ] Multi-hop Mesh Optimization
* [ ] Desktop Client
* [ ] Web Companion
* [ ] Cross-device Sync
* [ ] Self-hosted Relay Support

---

# 🤝 Contributing

Contributions are always welcome.

Whether it's:

* Bug reports
* Feature requests
* Documentation
* Code improvements

Every contribution helps make communication more resilient.

---

# 📜 License

Distributed under the **MIT License**.

---

<div align="center">

### **Communication should never depend on infrastructure.**

**Built with ❤️ using Rust for a more resilient world.**

</div>
