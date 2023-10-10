use crate::crypto::crypto::apply_cesar;


#[cfg(test)]
pub mod test_crypto {
  use super::*;

  #[test]
  fn test_apply_cesar() {
    let a = 1;
    let b = 2;
    let x = 3;
    let c = apply_cesar(a, b, x);
    assert_eq!(c, 5);
  }

  #[test]
  fn test_apply_cesar_2() {
    let a = 1;
    let b = 2;
    let x = 4;
    let c = apply_cesar(a, b, x);
    assert_eq!(c, 6);
  }

}