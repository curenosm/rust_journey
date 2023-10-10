use crate::math::complex::complex::Complex;
use crate::math::complex::complex::Polar;

#[cfg(test)]
pub mod test_complex {
  use super::*;

  #[test]
  fn test_add() {
    let a = Complex::new(1.0, 2.0);
    let b = Complex::new(3.0, 4.0);
    let c = a + b;
    assert_eq!(c.real, 4.0);
    assert_eq!(c.imag, 6.0);
  }

  #[test]
  fn test_sub() {
    let a = Complex::new(1.0, 2.0);
    let b = Complex::new(3.0, 4.0);
    let c = a - b;
    assert_eq!(c.real, -2.0);
    assert_eq!(c.imag, -2.0);
  }

  #[test]
  fn test_mul() {
    let a = Complex::new(1.0, 2.0);
    let b = Complex::new(3.0, 4.0);
    let c = a * b;
    assert_eq!(c.real, -5.0);
    assert_eq!(c.imag, 10.0);
  }

  #[test]
  fn test_div() {
    let a = Complex::new(1.0, 2.0);
    let b = Complex::new(3.0, 4.0);
    let c = a / b;
    assert_eq!(c.real, 0.44);
    assert_eq!(c.imag, 0.08);
  }

  #[test]
  fn test_polar_to_complex() {
    let a = Polar::new(1.0, 0.0);
    let b = a.to_complex();
    assert_eq!(b.real, 1.0);
    assert_eq!(b.imag, 0.0);
  }

  #[test]

  fn test_complex_to_polar() {
    let a = Complex::new(1.0, 0.0);
    let b = a.to_polar();
    assert_eq!(b.magnitude, 1.0);
    assert_eq!(b.phase, 0.0);
  }

  #[test]
  fn test_complex_to_polar_2() {
    let a = Complex::new(0.0, 1.0);
    let b = a.to_polar();
    assert_eq!(b.magnitude, 1.0);
    assert_eq!(b.phase, std::f64::consts::FRAC_PI_2);
  }

  #[test]
  fn test_complex_to_polar_3() {
    let a = Complex::new(-1.0, 0.0);
    let b = a.to_polar();
    assert_eq!(b.magnitude, 1.0);
    assert_eq!(b.phase, std::f64::consts::PI);
  }

  #[test]
  fn test_conjugate() {
    let a = Complex::new(1.0, 2.0);
    let b = a.conjugate();
    assert_eq!(b.real, 1.0);
    assert_eq!(b.imag, -2.0);
  }

  #[test]
  fn test_conjugate_polat() {
    let a = Polar::new(1.0, 2.0);
    let b = a.conjugate();
    assert_eq!(b.magnitude, 1.0);
    assert_eq!(b.phase, -2.0);
  }

  #[test]
  fn test_from_polar() {
    let a = Polar::new(1.0, 0.0);
    let b = Complex::from_polar(a);
    assert_eq!(b.real, 1.0);
    assert_eq!(b.imag, 0.0);
  }

  #[test]
  fn test_magnitude() {
    let a = Complex::new(3.0, 4.0);
    let b = a.magnitude();
    assert_eq!(b, 5.0);
  }

  #[test]
  fn test_phase() {
    let a = Complex::new(1.0, 1.0);
    let b = a.phase();
    assert_eq!(b, std::f64::consts::FRAC_PI_4);
  }

}
