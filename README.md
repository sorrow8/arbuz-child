# This Magic Arbuz Factory Contract

## Build
```bash
cargo build --target wasm32-unknown-unknown --release
```

## Deploy
```bash
oyl alkane new-contract -c ./target/wasm32-unknown-unknown/release/arbuz_child.wasm -data 3,id -p network
```

## Trace
```bash
oyl alkane trace -params '{"txid":"tx_id","vout":3}' -p network
```