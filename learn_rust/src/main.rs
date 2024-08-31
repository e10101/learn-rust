#![allow(unused)]

use std::io;


fn main() {
    // Rust signed integer types: i8, i16, i32, i64, isize
    // Rust unsigned integer types: u8, u16, u32, u64, usize

    println!("Max u32: {}", u32::MAX);
    println!("Max u64: {}", u64::MAX);
    println!("Max u128: {}", u128::MAX);
    println!("Max usize: {}", usize::MAX);
    println!("Max isize: {}", isize::MAX);
    println!("Max f32: {}", f32::MAX);
    println!("Max f64: {}", f64::MAX);
}
