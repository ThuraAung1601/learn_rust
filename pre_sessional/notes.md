## Setup

### Rust install
```
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

### Cargo Commands
- start project
```
cargo new <proj-name>
```
- check version 
```
cargo --version
```
- build your project with 
```
cargo build
```
- run your project with 
```
cargo run
```
- test your project with 
```
cargo test
```
- build documentation for your project with cargo doc
- publish a library to crates.io with cargo publish

For build and run specific Rust file \\
File should be under the folder ./src/bin/
```
cargo run --bin <rs-file>
```