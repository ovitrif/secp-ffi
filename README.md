# secp-ffi
rust-secp256k1 uniffi bindings for kotlin and swift

## 1. Build Binaries
```sh
# build in release mode `cargo build --release` # deprecated

# android - arm64-v8a (modern hw) `rustup target add aarch64-linux-android`
export CFLAGS="-D__ANDROID_MIN_SDK_VERSION__=24"
export AR="llvm-ar"
export CARGO_TARGET_AARCH64_LINUX_ANDROID_LINKER="aarch64-linux-android24-clang"
export CC="aarch64-linux-android24-clang"

cargo build --profile release-smaller --target aarch64-linux-android
# TODO android - x86_64 (emulators) 
# TODO android - armeabi-v7a (old 32-bit hw)

# jvm - arm `rustup target add aarch64-apple-darwin`
cargo build --profile release-smaller --target aarch64-apple-darwin

# jvm - x86 `rustup target add x86_64-apple-darwin`
cargo build --profile release-smaller --target x86_64-apple-darwin
````

## 2. Copy Binaries
```sh
# android - arm64-v8a
cp target/aarch64-linux-android/release-smaller/libsecpffi.so example/kotlin/app/src/main/jniLibs/arm64-v8a/libsecpffi.so

# jvm - arm
cp target/aarch64-apple-darwin/release-smaller/libsecpffi.dylib example/kotlin/app/src/main/resources/libsecpffi.dylib
```

## 3. Generate Bindings
### a) Library mode (recommended):
#### Kotlin
```sh
# android - arm64-v8a
cargo run --bin uniffi-bindgen generate --library target/aarch64-linux-android/release-smaller/libsecpffi.so --language kotlin --out-dir example/kotlin/app/src/main/kotlin/ --no-format

# jvm - arm
cargo run --bin uniffi-bindgen generate --library target/aarch64-apple-darwin/release-smaller/libsecpffi.dylib --language kotlin --out-dir example/kotlin/app/src/main/kotlin/ --no-format
```
#### Swift
```sh
# TODO: `rustup target add <arch>`
#cargo run --bin uniffi-bindgen generate --library target/<arch>/<profile>/libsecpffi.so --language swift --out-dir out --no-format
```

### b) Single UDL mode (alternative):
```sh
cargo run --features=uniffi/cli --bin uniffi-bindgen generate src/secp.udl --language kotlin
```

## Example Apps
### Kotlin
```sh
idea example/kotlin
```
