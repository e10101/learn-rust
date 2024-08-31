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

Output:

```
What is your name?
Eason
Hello Eason! Nice to meet you!
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

Output:

```
I'm 48 and I want $1000000
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


## Data Types

### Number Types

In Rust, we have following number types:

  - Unsigned Integer Types: `u8`, `u16`, `u32`, `u64`, `u128`, `usize`
  - Signed Integer Types: `i8`, `i16`, `i32`, `i64`, `i128`, `isize`
  - Floating-Point Types: `f32`, `f64`

```rust
fn main() {
    println!("Max u32: {}", u32::MAX);
    println!("Max u64: {}", u64::MAX);
    println!("Max u128: {}", u128::MAX);
    println!("Max usize: {}", usize::MAX);
    println!("Max isize: {}", isize::MAX);
    println!("Max f32: {}", f32::MAX);
    println!("Max f64: {}", f64::MAX);
}
```

The output of above codes is:

```
Max u32: 4294967295
Max u64: 18446744073709551615
Max u128: 340282366920938463463374607431768211455
Max usize: 18446744073709551615
Max isize: 9223372036854775807
Max f32: 340282350000000000000000000000000000000
Max f64: 179769313486231570000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000
```

The `usize` and `isize` types are specific to the target operating system's architecture. On a 64-bit system, `usize` and `isize` are both 64 bits, while on a 32-bit system, they are 32 bits.

#### Floating-Point Precision

And we can see `f32` and `f64` are floating-point types with different levels of precision. From the output above:

- `f32` has a maximum value of approximately 3.4 × 10^38
- `f64` has a maximum value of approximately 1.8 × 10^308

This demonstrates that `f64` provides significantly more precision and a much larger range than `f32`. The `f64` type is often the default choice for floating-point numbers in Rust when you need high precision, while `f32` can be used when memory usage is a concern and lower precision is acceptable.


### Boolean Type

In Rust, we have `bool` type to represent boolean values. It can be either `true` or `false`.

```rust
fn main() {
    let is_true = true;
    let is_false: bool = false;

    println!("is_true: {}", is_true);
    println!("is_false: {}", is_false);
}
```

Output:

```
is_true: true
is_false: false
```

### Character Type

In Rust, we have `char` type to represent a single character. It is a 4-byte (32-bit) value that corresponds to a Unicode Scalar Value.

```rust
fn main() {
    let a: char = 'a';
    let heart_eyed_cat: char = '😻';

    println!("a: {}", a);
    println!("heart_eyed_cat: {}", heart_eyed_cat);
}
```

Output:

```
a: a
heart_eyed_cat: 😻
```

## Math

In Rust, we can use following math operations:

- Addition: `+`
- Subtraction: `-`
- Multiplication: `*`
- Division: `/`
- Remainder: `%`

For example:

```rust
fn main() {
    let num_1: f32 = 1.1111111111111111111111;
    println!("f32: {}", num_1 + 0.1111111111111111111111);

    let num_2: f64 = 1.1111111111111111111111;
    println!("f64: {}", num_2 + 0.1111111111111111111111);

    let mut num_3: u32 = 5;
    let num_4: u32 = 4;
    println!("5 + 4 = {}", num_3 + num_4);
    println!("5 - 4 = {}", num_3 - num_4);
    println!("5 * 4 = {}", num_3 * num_4);
    println!("5 / 4 = {}", num_3 / num_4);
    println!("5 % 4 = {}", num_3 % num_4);

    num_3 += 1;
    println!("num_3 += 1: {}", num_3);
}
```

Output:

```
f32: 1.2222223
f64: 1.2222222222222223
5 + 4 = 9
5 - 4 = 1
5 * 4 = 20
5 / 4 = 1
5 % 4 = 1
num_3 += 1: 6
```


In this example, we can observe several important aspects of Rust's math operations:

1. **Floating-point precision**: The `f32` and `f64` types demonstrate different levels of precision. The `f32` type (32-bit float) shows less precision in the result compared to the `f64` type (64-bit float).

2. **Integer division**: When dividing integers (5 / 4), Rust performs integer division, which truncates the result to the nearest integer. That's why 5 / 4 equals 1, not 1.25.

3. **Remainder operation**: The `%` operator calculates the remainder of a division. In this case, 5 % 4 equals 1, as 1 is the remainder when 5 is divided by 4.

4. **Compound assignment**: The `+=` operator is used to add a value to a variable and assign the result back to the same variable. This is a shorthand for `num_3 = num_3 + 1`.

These operations showcase Rust's strong typing system and its behavior with different numeric types, which helps in writing more predictable and efficient code.


