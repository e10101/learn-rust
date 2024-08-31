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


## Constants and Shadowing

To define a constant, we use `const` keyword.

In Rust, you can define variable with the same name but different type which is called shadowing.

```rust
fn main() {
    const ONE_MIL: u32 = 1_000_000;
    const PI: f32 = 3.141592;
    let age: &str = "47";
    let mut age: u32 = age.trim().parse()
        .expect("Age wasn't assigned a number");
    age = age + 1;
    println!("I'm {} and I want ${}", age, ONE_MIL);
}
```



Let's break down this code:

1. `const ONE_MIL: u32 = 1_000_000;` defines a constant `ONE_MIL` of type `u32` (32-bit unsigned integer) with a value of one million. The underscore is used for readability and doesn't affect the value.

2. `const PI: f32 = 3.141592;` defines another constant `PI` of type `f32` (32-bit floating-point) with an approximation of pi.

3. `let age: &str = "47";` initially declares `age` as a string slice containing "47".

4. The next line re-declares `age` as a mutable `u32`:
   ```rust
   let mut age: u32 = age.trim().parse()
       .expect("Age wasn't assigned a number");
   ```
   This line trims any whitespace from the string, parses it into a `u32`, and will panic with the given error message if parsing fails.

5. `age = age + 1;` increments the `age` by 1.

6. Finally, `println!("I'm {} and I want ${}", age, ONE_MIL);` prints a formatted string using the `age` and `ONE_MIL` variables.

This code demonstrates constants, variable shadowing, type conversion, mutable variables, and string formatting in Rust.
