
#!/bin/bash

# build contract wasm
cargo build --target wasm32-unknown-unknown --release
cp target/wasm32-unknown-unknown/release/ledge.wasm res