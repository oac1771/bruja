# Bruja

Bruja network is a decentralized platform which connects users enabling them to share their unused computing power or access resources as needed. Bruja leverages a [substrate](https://substrate.io/) based blockchain as a settlement layer, [ink smart contract](https://use.ink/) to facilitate job requests, and [libp2p](https://docs.rs/libp2p/0.54.1/libp2p/) allow communication between requesters and workers.

# Architecture

The bruja network leverges a [substrate blockchain](crates/node/README.md) and [ink smart contract](crates/catalog/src/lib.rs) as data backends and settlement layer. [Requesters](crates/requester/src/main.rs) submit job request extrinsics to the [Catalog](crates/catalog/src/lib.rs) smart contract. The contract emits a job request event which [Workers](crates/worker/src/main.rs) will respond to. If the workers choose the job request, the will send a propagate a message through the network until the requester is reached. The requester will then send a raw wasm code to the worker so that it can execute it via [Wasmtime](https://docs.rs/wasmtime/26.0.0/wasmtime/).  


# Development
This project levereges [mise](https://mise.jdx.dev/) to manage dev tools. Follow the directions [here](https://mise.jdx.dev/getting-started.html#quickstart) to install.

Install project dependencies
```
mise install
```

Install Cargo Make
```
cargo install --no-default-features cargo-make
```

Install Dev Dependencies
```
cargo make install-dev-dependencies
```


# Build
Run the following command to build bruja project:

```
cargo make cargo-build
```

# Run Locally
Start a local substrate node in one terminal:

```
cargo make local-node
```

In a seperate terminal, instantiate catalog contract with the following command:

```
export CONTRACT_ADDRESS=$(cargo run -p scripts instantiate --suri //Alice --file target/ink/catalog/catalog.contract)
```

In another seperate terminal, start the worker:

```
cargo run -p worker start --address <contract address>
```

In the same terminal which you used to instantiate the contract, run the following command to submit a job to the network:

```
cargo run -p requester submit-job --address $CONTRACT_ADDRESS --path crates/tests/tests/work_bg.wasm --params 10 --func-name foo
```

# Testing

Run the following command to run unit tests:

```
cargo test
```

Run the following command to run smart contract integration tests:

```
cargo make contract-tests
```


To run project integration tests, start a local substrate node in one terminal:

```
cargo make local-node
```

In a seperate terminal, run the following command to perform Requester - Worker Integration tests:

```
cargo make integration-tests
```
