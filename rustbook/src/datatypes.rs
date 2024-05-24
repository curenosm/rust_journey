fn main() {
    let guess: u32 = "42".parse().expect("Not a number!");
    let formatted: u32 = 98_222;
    let hex = 0xff;
    let oct = 0o77;
    let bin = 0b1111_0000;
    let byteu8only = b'A';


    // Punto flotante
    let x = 2.0; // f64
    let y: f32 = 3.0; // f32
}

fn numeric_operations() {

    // addition
    let sum = 5 + 10;

    // subtraction
    let difference = 95.5 - 4.3;

    // multiplication
    let product = 4 * 30;

    // division
    let quotient = 56.7 / 32.2;
    let truncated = -5 / 3; // Results in -1

    // remainder
    let remainder = 43 % 5;
}

fn booleans() {
    let t = true;
    let f: bool = false; // with explicit type annotation
}

fn chars() {
    // 4 bytes - representa un unicode
    let c = 'z';
    let z: char = 'ℤ'; // with explicit type annotation
    let heart_eyed_cat = '😻';
}

fn tipos_compuestos() {
    let tup: (i32, f64, u8) = (500, 6.4, 1);

    // Usamos pattern matching, para
    // desestructurar los valores que contiene
    // una túpla.

    let tup = (500, 6.4, 1);

    let (x, y, z) = tup;

    println!("The value of y is: {y}");

    // O bien índices
    let x = (500, 6.4, 1);

    let five_hundred = x.0;
    let six_point_four = x.1;
    let one = x.2;
}


fn arrays() {
    // Todos del mismo tipo. Alojados en el stack.
    let a = [1, 2, 3, 4, 5];

    // A diferencia de un vector, su tamaño es fijo.
    let months = ["January", "February", "March", "April", "May", "June", "July",
        "August", "September", "October", "November", "December"];

    // Ar-ti-num
    let a: [i32; 5] = [1, 2, 3, 4, 5];

    // Inicializado con un mismo valor.
    let a = [3; 5];

    // Accediendo a los elementos.
    let first = a[0];
    let second = a[1];

}

// Compila correctamente, pero da error en ejecución
use std::io;

fn ooops() {
    let a = [1, 2, 3, 4, 5];

    println!("Please enter an array index.");

    let mut index = String::new();

    io::stdin()
        .read_line(&mut index)
        .expect("Failed to read line");

    let index: usize = index
        .trim()
        .parse()
        .expect("Index entered was not a number");

    let element = a[index];

    println!("The value of the element at index {index} is: {element}");
}
