# Arbuz Child

This is the orbital (child) contract for the Magic Arbuz collection. Each instance represents a unique, on-the-fly generated text prediction. No data is stored — all predictions are computed deterministically from the index.

## Features
- **Deterministic text prediction**: Each token's prediction is generated on demand from its index.
- **No storage overhead**: Nothing is pre-stored, everything is computed in real time.
- **Parent reference**: Each child links to its parent collection.

## Build
```bash
cargo build --target wasm32-unknown-unknown --release
```
The compiled WASM will be in `target/wasm32-unknown-unknown/release/arbuz_child.wasm`.

## Deploy
```bash
oyl alkane new-contract -c ./target/wasm32-unknown-unknown/release/arbuz_child.wasm -data 3,n -p regtest
```

## Usage
- All predictions are generated on-the-fly via contract calls.
- No need to pre-mint or store any data.

## Example: Mint alkake
```bash
oyl alkane execute -data 2,tx,77 -p regtest
```

## Example: Get Prediction
```bash
oyl alkane simulate -p regtest -target "2:tx" -inputs "1000"
```

---
MIT License 