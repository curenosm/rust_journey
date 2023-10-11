use crate::crypto::crypto::{apply_cesar, letters_to_nums, nums_to_letters};


#[cfg(test)]
pub mod test_crypto {
  use super::*;

  #[test]
  fn test_apply_cesar() {
    let b = 2;
    let x = 3;
    let c = apply_cesar(b, x);
    assert_eq!(c, 5);
  }

  #[test]
  fn test_apply_cesar_2() {
    let b = 2;
    let x = 4;
    let c = apply_cesar(b, x);
    assert_eq!(c, 6);
  }

  #[test]
  fn test_convert_to_num() {
    let s = "ABC";
    let v = letters_to_nums(s);
    assert_eq!(v, vec![0, 1, 2]);
  }

  #[test]
  fn test_convert_to_str() {
    let v = vec![0, 1, 2];
    let s = nums_to_letters(&v);
    assert_eq!(s, "ABC");
  }

}
