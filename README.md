# This Is Magic Arbuz Factory Contract

<p align="center">
  <img src="./arbuz.png" alt="ARBUZ Logo">
</p>

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
oyl alkane trace -params '{"txid":"txid","vout":3}' -p network
```