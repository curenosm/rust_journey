// use crate::crypto::crypto::apply_cesar;
use crate::dynamic_programming::dynamic_programming::fib;
use crate::math::extras::extras::generate_fractal_triangle;

mod ai;
mod crypto;
mod data_structures;
mod dynamic_programming;
mod math;
mod regression;
mod sorting;
mod statistics;
mod strings;

pub fn main() {
    //! This is my first rust crate

    println!("Hello, world!");
    let result = fib(5);

    generate_fractal_triangle();

    println!("result: {}", result)
}
