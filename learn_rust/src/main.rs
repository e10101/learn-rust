#![allow(unused)]

use rand::Rng;


fn main() {
    let random_num = rand::thread_rng().gen_range(1..=100);
    println!("Random number: {}", random_num);
}