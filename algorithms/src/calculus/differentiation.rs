use crate::complex::complex::Complex;

pub fn approx_derivative(f: &dyn Fn(f64) -> f64, x: f64, h: f64) -> f64 {
  (f(x + h) - f(x)) / h
}

pub fn line_equation(m: f64, b: f64) -> Box<dyn Fn(f64) -> f64> {
  Box::new(move |x| m * x + b)
}

pub fn quadratic_equation(a: f64, b: f64, c: f64) -> Box<dyn Fn(f64) -> f64> {
  Box::new(move |x| a * x.powi(2) + b * x + c)
}

pub fn quadratic_formula(a: f64, b: f64, c: f64) -> (Complex, Complex) {
  let discriminant = discriminant(a, b, c);
  if discriminant < 0.0 {
    let real = -b / (2.0 * a);
    let imaginary = (discriminant.abs()).sqrt() / (2.0 * a);
    let x1 = Complex::new(real, imaginary);
    let x2 = Complex::new(real, -imaginary);
    return (x1, x2);
  }

  (x1, x2)
}


pub fn discriminant(a: f64, b: f64, c: f64) -> f64 {
  b.powi(2) - 4.0 * a * c
}
