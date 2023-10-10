pub fn apply_cesar(b: i32, x: i32) -> i32 {
  apply_affine(1, b, x)
}

pub fn apply_affine(a: i32, b: i32, x: i32) -> i32 {
  (a * x + b) % 26
}

/// We also have Euclid's extended algorithm, which is used to find the
/// multiplicative inverse of two numbers.
pub fn extended_euclid(a: i32, b: i32) -> (i32, i32, i32) {

  if a == 0 {
    return (b, 0, 1);
  }

  let (gcd, x1, y1) = extended_euclid(b % a, a);
  let x = y1 - (b / a) * x1;
  let y = x1;

  (gcd, x, y)
}

/// Find the inverse of a number modulo n
pub fn inverse(a: i32, n: i32) -> i32 {
  let (gcd, x, _) = extended_euclid(a, n);
  if gcd != 1 {
    panic!("{} has no inverse modulo {}", a, n);
  }
  (x % n + n) % n
}

pub fn unapply_affine(a: i32, b: i32, x: i32) -> i32 {
  (inverse(a, 26) * (x - b)) % 26
}

pub fn unapply_cesar(b: i32, x: i32) -> i32 {
  unapply_affine(1, b, x)
}

/// Given a string, map it to a vector of numbers
pub fn letters_to_nums(s: &str) -> Vec<i32> {
  s.chars().map(|c| (c as u8 - 'A' as u8)).collect()
}

/// Given a vector of numbers, map it to a string
pub fn nums_to_letters(v: &[i32]) -> String {
  v.iter().map(|x| (x + 'A' as i32) as u8 as char).collect()
}

/// Given a string, encode id using the caesar cipher
pub fn caesar_encode(s: &str, b: i32) -> String {
  nums_to_letters(&letters_to_nums(s).iter().map(|x| apply_cesar(b, *x)).collect::<Vec<i32>>())
}

/// Given a string, decode it using the caesar cipher
pub fn caesar_decode(s: &str, b: i32) -> String {
  nums_to_letters(&letters_to_nums(s).iter().map(|x| unapply_cesar(b, *x)).collect::<Vec<i32>>())
}

/// Given a string, encode it using the affine cipher
pub fn affine_encode(s: &str, a: i32, b: i32) -> String {
  nums_to_letters(&letters_to_nums(s).iter().map(|x| apply_affine(a, b, *x)).collect::<Vec<i32>>())
}

/// Given a string, decode it using the affine cipher
pub fn affine_decode(s: &str, a: i32, b: i32) -> String {
  nums_to_letters(&letters_to_nums(s).iter().map(|x| unapply_affine(a, b, *x)).collect::<Vec<i32>>())
}
