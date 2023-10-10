use crate::math::complex::complex::Complex;


pub struct Matrix {
  pub rows: usize,
  pub cols: usize,
  pub data: Vec<Complex>,
}

impl Matrix {
  fn new(rows: usize, cols: usize) -> Matrix {
    Matrix {
      rows,
      cols,
      data: vec![Complex { real: 0.0, imag: 0.0 }; rows * cols],
    }
  }

  fn add(&self, other: &Matrix) -> Matrix {
    let mut result = Matrix::new(self.rows, self.cols);

    for i in 0..self.rows {
      for j in 0..self.cols {
        let a = self.data[i * self.cols + j];
        let b = other.data[i * self.cols + j];
        result.data[i * self.cols + j] = Complex {
          real: a.real + b.real,
          imag: a.imag + b.imag,
        };
      }
    }

    result
  }
}

#[cfg(test)]
pub mod matrices_test {
  use super::*;

  # [test]
  fn test_add() {
    let a = Matrix::new(2, 2);
    let b = Matrix::new(2, 2);

    let c = a.add(&b);

    assert_eq!(c.data[0].real, 0.0);
    assert_eq!(c.data[0].imag, 0.0);
    assert_eq!(c.data[1].real, 0.0);
    assert_eq!(c.data[1].imag, 0.0);
    assert_eq!(c.data[2].real, 0.0);
    assert_eq!(c.data[2].imag, 0.0);
    assert_eq!(c.data[3].real, 0.0);
    assert_eq!(c.data[3].imag, 0.0);
  }

}
