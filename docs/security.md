# Security & Cryptography

## Threat Model
Gupt assumes the network is hostile and the cloud backend is untrusted. The cloud provider (Supabase) acts as a blind relay and directory service.

## Algorithms
- **Identity**: Ed25519 (Digital Signatures)
- **Key Exchange**: X25519 (Elliptic-curve Diffie-Hellman)
- **Session Keys**: HKDF-SHA256 (Key Derivation from X25519 shared secret)
- **Encryption**: XChaCha20-Poly1305 (AEAD with 24-byte extended nonce)
- **Local Key Protection**: Argon2id (PIN-based key derivation)

## Key Management
- Users generate a master keypair upon registration.
- Private keys are encrypted locally using a PIN and Argon2id.
- Public keys are published to the cloud directory.
- Private keys NEVER leave the device.
