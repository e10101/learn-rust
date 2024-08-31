## Setup Rust

```shell
# Create a new Rust project
cargo new learn_rust

# Change directory to the new folder
cd learn_rust
```

In the created project's `src/main.rs` file, type following codes:

```rust
fn main() {
    println!("Hello, world!");
}
```

Then run your code with the `Run` button from VSCode's `rust-analyzer` extension, or using follow command:

```shell
cargo run --bin learn_rust
```