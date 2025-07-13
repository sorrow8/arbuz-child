## Build
```bash
cargo build --target wasm32-unknown-unknown --release
```

## Deploy
```bash
oyl alkane new-contract -c ./target/wasm32-unknown-unknown/release/arbuz_child.wasm -data 3,index -p regtest
```

## Gen block
```bash
oyl regtest genBlocks -c -p regtest
```

## Trace
```bash
oyl alkane trace -params '{"txid":"txid","vout":3}' -p regtest
```