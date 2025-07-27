## Build
```bash
cargo build --target wasm32-unknown-unknown --release
```

## Deploy
```bash
oyl alkane new-contract -c ./target/wasm32-unknown-unknown/release/arbuz_child.wasm -data 3,427 -p regtest
```

## Gen block
```bash
oyl regtest genBlocks -c -p regtest
```

## Trace
```bash
oyl alkane trace -params '{"txid":"cdca097ad3872e4ad4a7349bc4101cb488b080059b67892a5872a2549cdfb87a","vout":3}' -p regtest
```