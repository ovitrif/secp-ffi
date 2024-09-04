# secp-ffi

## Generate Bindings 
```sh
# using a library file - recommended:
cargo build --release
cargo run --bin uniffi-bindgen generate --library target/release/libsecpffi.dylib --language kotlin --out-dir out
#cargo run --bin uniffi-bindgen generate --library target/release/libsecpffi.so --language kotlin --out-dir out
# --no-format # to minify

# with UDL file:
cargo run --features=uniffi/cli --bin uniffi-bindgen generate src/secp.udl --language kotlin
```

## Advanced UDL Example
```uniffi-dl
namespace secp {};
interface Example {
    string greet(string name);
};
```
