#!/bin/bash
set -e

echo "Building Rust core for iOS..."

cd "$(dirname "$0")/.."

# Ensure targets are installed
rustup target add aarch64-apple-ios aarch64-apple-ios-sim x86_64-apple-ios

# Build static libraries
cargo build --manifest-path core/ffi/Cargo.toml --target aarch64-apple-ios --release
cargo build --manifest-path core/ffi/Cargo.toml --target aarch64-apple-ios-sim --release
cargo build --manifest-path core/ffi/Cargo.toml --target x86_64-apple-ios --release

# Create universal library for simulator (Intel + ARM)
mkdir -p target/universal-sim/release
lipo -create \
  target/aarch64-apple-ios-sim/release/libgupt_ffi.a \
  target/x86_64-apple-ios/release/libgupt_ffi.a \
  -output target/universal-sim/release/libgupt_ffi.a

# Remove existing XCFramework if it exists
rm -rf apps/ios/GuPTCore.xcframework

# Create XCFramework
xcodebuild -create-xcframework \
  -library target/aarch64-apple-ios/release/libgupt_ffi.a \
  -headers core/ffi/include/ \
  -library target/universal-sim/release/libgupt_ffi.a \
  -headers core/ffi/include/ \
  -output apps/ios/GuPTCore.xcframework

# Generate Swift bindings
echo "Generating Swift bindings..."
cargo run --bin uniffi-bindgen generate \
    core/ffi/src/gupt.udl \
    --language swift \
    --out-dir apps/ios/GuPT/RustBindings/

echo "iOS build complete!"
