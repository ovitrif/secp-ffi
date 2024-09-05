# secp-ffi

## Generate Bindings 
First build for release
```sh
cargo build --release
````

### Library mode (recommended):
#### Kotlin
```sh
# generate bindings
cargo run --bin uniffi-bindgen generate --library target/release/libsecpffi.dylib --language kotlin --out-dir example/kotlin/app/src/main/kotlin/ --no-format
# Update binary
cp target/release/libsecpffi.dylib example/kotlin/app/src/main/resources/libsecpffi.dylib
```

#### Swift
```sh
#cargo run --bin uniffi-bindgen generate --library target/release/libsecpffi.so --language kotlin --out-dir out --no-format
```

### Single UDL mode (alternative):
```sh
cargo run --features=uniffi/cli --bin uniffi-bindgen generate src/secp.udl --language kotlin
```

## Example Apps
### Kotlin
```sh
idea example/kotlin
```
