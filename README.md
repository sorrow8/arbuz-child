# This Is Magic Arbuz Factory Contract

<p align="center">
  <img src="./arbuz_logo.svg" alt="ARBUZ Logo">
</p>

### For Local Testing:
## Build
```bash
cargo build --target wasm32-unknown-unknown --release
```

## Deploy
```bash
oyl alkane new-contract -c ./target/wasm32-unknown-unknown/release/arbuz_child.wasm -data 3,tx -p regtest
```

## Trace
```bash
oyl alkane trace -params '{"txid":"txid","vout":3}' -p regtest
```