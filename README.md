# Setup

install project dependencies
```
mise install
```
```
brew install qemu lima docker
```

Add wasm target:
```
rustup target add wasm32-unknown-unknown
```

Add x86_target
```
rustup target add x86_64-unknown-linux-musl
```

Install cross compiler toolchains
```
brew install filosottile/musl-cross/musl-cross
```

Add Cargo Make:
```
cargo install --no-default-features --force cargo-make
```

Add Cargo Contract for ink development
```
cargo install --force --locked cargo-contract
```


# Reference
info on runtime development including crate strucutre: [here](https://docs.substrate.io/learn/runtime-development/)

info on architecture: [here](https://docs.substrate.io/learn/architecture/)

info on build: [here](https://docs.substrate.io/build/build-process/)