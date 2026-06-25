#!/bin/bash
set -e

# Build Rust core for Android architectures using cargo-ndk
echo "Building Rust core for Android..."

cd "$(dirname "$0")/.."

# Ensure targets are installed
rustup target add aarch64-linux-android armv7-linux-androideabi x86_64-linux-android i686-linux-android

# Ensure cargo-ndk is installed
if ! command -v cargo-ndk &> /dev/null; then
    echo "Installing cargo-ndk..."
    cargo install cargo-ndk
fi

# Build for all targets and output to the jniLibs directory
cargo ndk -t arm64-v8a -t armeabi-v7a -t x86_64 -t x86 \
    -o ./apps/android/app/src/main/jniLibs \
    build --manifest-path core/ffi/Cargo.toml --release

# Generate Kotlin bindings using uniffi-bindgen
echo "Generating Kotlin bindings..."
cargo run --bin uniffi-bindgen generate \
    --library ./target/aarch64-linux-android/release/libgupt_ffi.so \
    --language kotlin \
    --out-dir ./apps/android/app/src/main/java/com/gupt/ffi/

echo "Android build complete!"
