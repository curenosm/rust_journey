// use crate::crypto::crypto::apply_cesar;
use crate::dynamic_programming::dynamic_programming::fib;

mod ai;
mod crypto;
mod data_structures;
mod dynamic_programming;
mod math;
mod regression;
mod sorting;
mod statistics;
mod strings;


fn main() {
    println!("Hello, world!");
    let result = fib(5);
    println!("result: {}", result)
}
