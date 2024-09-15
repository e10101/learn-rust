---
weight: 5
bookFlatSection: true
title: "Axum Setup"
---


# Setup Environment

## Install Rust

You can use following command to install Rust in Mac OS.

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

Then verify the installation by running following command.

```bash
rustc --version
```

You will see the version information of Rust. For example,

```
rustc 1.81.0 (eeb90cda1 2024-09-04)
```

## Install Cargo

Cargo is the package manager for Rust. You can use following command to install Cargo.

```bash
cargo --version
```

You will see the version information of Cargo. For example,

```
cargo 1.81.0 (2dbb1af80 2024-08-20)
```

## Setup Project

You can use following command to setup a new Axum project.

```bash
cargo new axum-project
```

This will create a new directory called `axum-project` with the following structure.

```
axum-project/
├── Cargo.toml
├── src
│   ├── main.rs
```

The `main.rs` is the entry point of the application.

## Setup Dependencies

You can use following command to setup dependencies.

```bash
cargo add axum
```

This will add the `axum` crate to the dependencies.


```toml
[dependencies]
axum = "0.7.5"
```

## Run Project

You can use following command to run the project.

```bash
cargo run
```