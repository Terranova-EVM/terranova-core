## Project Description

Terranova is an implementation of the Ethereum virtual machine (EVM) which targets Web Assembly for compilation. It allows for the execution of arbitrary EVM bytecode on WASM-compliant architectures, writing resultant state changes to a database that persists across multiple executions.

## Building and Testing

Build the wasm binary:
```sh
# this will produce a wasm artifact in ./target/wasm32-unknown-unknown/release/YOUR_NAME_HERE.wasm
cargo wasm
```

Run unit tests:
```sh
# runs unit tests in ./src/testing.rs
RUST_BACKTRACE=1 cargo unit-test
```



