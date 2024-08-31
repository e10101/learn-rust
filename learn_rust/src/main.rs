#![allow(unused)]

use std::io;


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