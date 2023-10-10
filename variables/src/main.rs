fn main() {
    let x = 5;
    println!("The value of x is: {x}");
    let x = 6;
    println!("The value of x is: {x}");

    let x = 2.0; // f64

    let y: f32 = 3.0; // f32

    // Basic Operations
    let sum = 5 + 10;
    let difference = 95.5 - 4.3;
    let product = 4 * 30;
    let quotient = 56.7 / 32.2;
    let truncated = -5 / 3; // Results in -1
    let remainder = 43 % 5;

    // Booleans
    let t = true;
    let f: bool = false; // with explicit type annotation

    // Rust supports Unicode Scalar Values
    let c = 'z';
    let z: char = 'ℤ'; // with explicit type annotation
    let heart_eyed_cat = '😻';

    // TUPLES
    // let tup = (500, 6.4, 1);
    let tup: (i32, f64, u8) = (500, 6.4, 1);

    let (x, y, z) = tup;

    println!("The value of y is: {y}");

    // Access elements of tuples
    let five_hundred = x.0;
    let six_point_four = x.1;
    let one = x.2;

    // The tuple without any values has a special name, unit. ()

    // ARRAYS
    let a = [1, 2, 3, 4, 5];
    let a: [i32; 5] = [1, 2, 3, 4, 5];
    let months = [
        "January",
        "February",
        "March",
        "April",
        "May",
        "June",
        "July",
        "August",
        "September",
        "October",
        "November",
        "December",
    ];
    let a = [0; 5]; // [0, 0, 0, 0, 0]

    // Accessing
    let first = a[0];
    let second = a[1];

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
