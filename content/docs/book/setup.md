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

## Import Modules

In Rust, we can import modules by using `use` keyword.
For example, we can import `std::io` module to read input from the user and print output to the console.


```rust
#![allow(unused)]

use std::io;


fn main() {
    println!("What is your name?");
    let mut name: String = String::new();
    let greeting: String = String::from("Nice to meet you!");
    io::stdin()
        .read_line(&mut name)
        .expect("Didn't Receive Input");
    println!("Hello {}! {}", name.trim_end(), greeting);
}
```

Let's break down the code above:

1. `#![allow(unused)]` is an attribute that tells the Rust compiler to suppress warnings about unused code in this file. This is useful during development when you might have temporary unused variables or imports.

2. `use std::io;` imports the input/output functionality from the standard library. This allows us to interact with the console for input and output operations.

3. In the `main()` function:
   - We use `println!()` to output a prompt to the console.
   - We create a mutable `String` variable `name` to store the user's input.
   - We create an immutable `String` variable `greeting` with a predefined message.
   - `io::stdin().read_line(&mut name)` reads a line of input from the user and stores it in the `name` variable.
   - `.expect("Didn't Receive Input")` handles potential errors during input reading.
   - Finally, we use `println!()` again to output a personalized greeting, using `name.trim_end()` to remove any trailing whitespace from the input.

This code demonstrates basic input/output operations, variable declarations, and string manipulation in Rust.

