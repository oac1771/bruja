# Setup

install project dependencies
```
mise install
```
```
brew install qemu lima docker
```

Install Dev Dependencies
```
cargo make install-dev-dependencies
```

Get node metadata
```
subxt metadata --pallets Contracts  > chain.scale
```


# Reference
info on runtime development including crate strucutre: [here](https://docs.substrate.io/learn/runtime-development/)

info on architecture: [here](https://docs.substrate.io/learn/architecture/)

info on build: [here](https://docs.substrate.io/build/build-process/)

info on subxt codegen: [here](https://docs.rs/subxt/0.37.0/subxt/book/setup/codegen/index.html)

info on cargo-contract: [here](https://use.ink/cargo-contract-cli/)
