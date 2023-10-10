use std::fs::File;
use std::io::{self, Read};

use base64::{Engine as _, engine::general_purpose};
use euclid::{Point2D, Vector2D};
use nalgebra::{Matrix2, Vector2};

mod tests;

fn main() {
  another_function(5);
  print_labeled_measurement(5, 'h');

  let x = Matrix2::new(1.0, 2.0, 3.0, 4.0);
  let y = Vector2::new(5.0, 7.0);
  let product = x * y;

  println!("x * y = {product}");

  let y = {
    let x = 3;
    x + 1
  };

  println!("The value of y is: {y}");

  // Using io read the txt file named "input.txt"
  let mut input = String::new();
  io::stdin().read_line(&mut input).expect("Failed to read line");

  // Read a file and encode it to base64

  // Leyendo un archivo en Rust
  let file_result = File::open("input.txt");

  match file_result {
    Ok(mut file) => {
      // Create a buffer to hold the file contents
      let mut contents = String::new();

      // Read the file contents into the buffer
      match file.read_to_string(&mut contents) {
        Ok(_) => {
          // Print the contents of the file
          println!("{}", contents);
        }
        Err(err) => {
          eprintln!("Error reading file: {}", err);
        }
      }
    }
    Err(err) => {
      eprintln!("Error opening file: {}", err);
    }
  }

  println!("Input: {:?}", input);
  let encoded = general_purpose::STANDARD.encode(b"hello world~");
  println!("Encoded: {}", encoded);
  println!("Decoded: {:?}", general_purpose::STANDARD.decode("aGVsbG8gd29ybGR+Cg==").unwrap());

  let origin_point: Point2D<f64, f64> = Point2D::new(0.0, 0.0);
  let vector = Vector2D::new(3.0, 4.0);
  let point = origin_point + vector;

  println!("The point is: {:?}", point);

  let x = five();

  println!("The value of x is: {x}");
}

fn another_function(x: i32) {
  println!("The value of x is: {x}");
}

fn print_labeled_measurement(value: i32, unit_label: char) {
  println!("The measurement is: {value}{unit_label}");
}

fn five() -> i32 {
  5
}

fn plus_one(x: i32) -> i32 {
  x + 1
}


