use std::fs::File;
use std::io::Read;
use crate::crypto::constants::des_cons::{
  KEY_56, PC1, PC2, P, S_BOXES as S, IP, FP, E
};

/// Primer vamos a necesitar función para leer un archivo.txt pasado
/// como parámetro a la función y devolvemos una lista de strings
/// con cada linea del archivo.
pub fn read_file(file_name: &str) -> Vec<String> {
  let mut file = File::open(file_name)
    .expect("No se pudo abrir el archivo");
  let mut contents = String::new();

  file.read_to_string(&mut contents)
    .expect("No se pudo leer el archivo");

  let lines: Vec<String> = contents
    .split("\n")
    .map(|s| s.to_string())
    .collect::<Vec<String>>();

  lines
}


/// Luego vamos a tener una función que traduce un string a binario
/// y devuelve un string con el binario.
pub fn string_to_binary(string: &str) -> String {
  let mut binary_string = String::new();
  for c in string.chars() {
    let bin = format!("{:b}", c as u8);
    binary_string.push_str(&bin);
  }
  binary_string
}


/// Además vamos a tener un función que separe en trozos una
/// cadena binaria que pasemos.
pub fn chunk(string: &str, chunk_size: usize) -> Vec<String> {
  let mut chunks: Vec<String> = Vec::new();
  let mut chunk = String::new();
  for c in string.chars() {
    chunk.push(c);
    if chunk.len() == chunk_size {
      chunks.push(chunk);
      chunk = String::new();
    }
  }
  chunks
}

/// Ahora vamos a tener una función que traduce un string binario
/// a su representación como char (recordando hacer esto cada
/// 8 bits).
pub fn binary_to_string(binary_string: &str) -> String {
  let mut string = String::new();
  for chunk in chunk(binary_string, 8) {
    let bin = u8::from_str_radix(&chunk, 2).unwrap();
    let c = char::from(bin);
    string.push(c);
  }
  string
}


/// De la misma forma, vamos a tener una función representa el proceso
/// completo de cifrado DES (para ello recibimos un string y una clave
/// y devolvemos el string cifrado).

pub fn des(string_to_cipher: &str) -> String {
  // First we need to convert the string to binary
  let binary_string = string_to_binary(string_to_cipher);

  // After that we need to chunk the binary string into 64-bit blocks
  let chunks = chunk(&binary_string, 64);

  // Now we need the 16 keys
  let keys = generate_k_keys(&KEY_56);

  // Apply the initial permutation
  let mut ciphered_chunks: Vec<String> = Vec::new();

  for chunk in chunks {
    let chunk_ciphered = apply_permutation(&chunk, &IP);
    ciphered_chunks.push(chunk_ciphered);
  }

  // Apply the 16 rounds with the 16 keys
  for i in 0..16 {
    let mut new_ciphered_chunks: Vec<String> = Vec::new();
    for chunk in ciphered_chunks {
      let chunk_ciphered = apply_round(&chunk, &keys[i]);
      new_ciphered_chunks.push(chunk_ciphered);
    }
    ciphered_chunks = new_ciphered_chunks;
  }

  // Apply the final permutation
  let mut ciphered_string = String::new();
  for chunk in ciphered_chunks {
    let chunk_ciphered = apply_permutation(
      &chunk,
      &FP);

    ciphered_string.push_str(&chunk_ciphered);
  }

  // Then convert the binary string to a string
  let string_ciphered = binary_to_string(&ciphered_string);

  string_ciphered
}

/// Función para aplicar en cada uno de los rounds
/// después de la permutación inicial.
pub fn apply_round(chunk: &str, key: &str) -> String {
  // First we need to split the chunk into two halves
  let (chunk_left, chunk_right) = split_key(chunk);

  // Now we need to expand the right half
  let chunk_right_expanded = apply_permutation(
    &chunk_right,
    &E);

  // Now we need to XOR the expanded right half with the key
  let chunk_right_expanded_xor_key = xor(
    &chunk_right_expanded,
    &key);

  // Now we need to apply the S-boxes
  let chunk_right_expanded_xor_key_sbox = apply_sboxes(
    &chunk_right_expanded_xor_key);

  // Now we need to apply the permutation P
  let permuted = apply_permutation(
    &chunk_right_expanded_xor_key_sbox,
    &P,
  );

  // Now we need to XOR the result with the left half
  let result = xor(
    &permuted,
    &chunk_left,
  );

  result
}

/// Función para aplicar el XOR entre dos cadenas binarias
pub fn xor(chunk1: &str, chunk2: &str) -> String {
  let mut chunk_xor = String::new();
  for i in 0..chunk1.len() {
    let bit1 = chunk1.chars().nth(i).unwrap();
    let bit2 = chunk2.chars().nth(i).unwrap();
    chunk_xor.push(if bit1 == bit2 { '0' } else { '1' })
  }
  chunk_xor
}


/// Función para aplicar todas las S-boxes
/// a un chunk de 48 bits.
pub fn apply_sboxes(chunk: &str) -> String {
  // First we need to split the chunk into 8 blocks of 6 bits
  let chunks = chunk(chunk, 6);

  // Now we need to apply each S-box
  let mut chunk_sboxed = String::new();
  for i in 0..8 {
    let chunk_sboxed_i = apply_sbox(&chunks[i], i);
    chunk_sboxed.push_str(&chunk_sboxed_i);
  }

  chunk_sboxed
}

/// Función para aplicar una S-box a un chunk de 6 bits
/// y devolver un chunk de 4 bits.
pub fn apply_sbox(chunk: &str, i: usize) -> String {
  // First we need to get the row and column
  let row = format!(
    "{}{}",
    chunk.chars().nth(0).unwrap(),
    chunk.chars().nth(5).unwrap()
  );
  let row = usize::from_str_radix(&row, 2).unwrap();

  // Take from the second up to the fifth chars of the chunk
  let column = chunk
    .chars()
    .skip(1)
    .take(4)
    .collect::<String>();
  let column = usize::from_str_radix(&column, 2).unwrap();

  // Now we need to get the value from the S-box
  let value = match i {
    0..=7 => S[i][row][column],
    _ => panic!("Invalid S-box")
  };

  // Now we need to convert the value to binary
  let value = format!("{:b}", value);

  // Now we need to pad the value with zeros
  let mut value = value;
  while value.len() < 4 {
    value = format!("0{}", value);
  }

  value
}

/// Función para completar una clave de 56 bits
/// a una de 64 bits.
pub fn complete_key(key_56: &str) -> String {
  let mut key_64 = String::new();
  // Make sure you are counting the numbers of 1s
  // in the key
  let mut count_1s = 0;
  for i in 0..64 {

    // Each 8 bits we need to add a the parity bit
    if i % 8 == 0 {
      // which is a 1 if the number of 1s is even
      // and a 0 if the number of 1s is odd
      key_64.push(if count_1s % 2 == 0 { '0' } else { '1' });
    } else {
      key_64.push(key_56.chars().nth(i).unwrap());
      if key_56.chars().nth(i).unwrap() == '1' {
        count_1s += 1;
      }
    }
  }

  key_64
}

/// Función para generar las 16 claves K
/// para cada uno de los rounds.
pub fn generate_k_keys(key_56: &str) -> Vec<String> {
  // First we need to apply the PC-1 permutation
  let key_56_pc1 = apply_permutation(
    key_56,
    &PC1);

  // Now we need to split the key into two halves
  let (key_left, key_right) = split_key(&key_56_pc1);

  // Now we need to generate the 16 keys
  let mut keys: Vec<String> = Vec::new();

  for i in 0..16 {
    // First we need to apply the left shift
    let key_l_shifted = left_shift(&key_left, i);
    let key_r_shifted = left_shift(&key_right, i);

    // Now we need to apply the PC-2
    let key_48 = apply_permutation(
      &format!(
        "{}{}",
        key_l_shifted,
        key_r_shifted
      ),
      &PC2,
    );

    // Now we need to add the key to the vector
    keys.push(key_48);
  }

  keys
}


/// Función para girar a la izquierda una clave, esto significa
/// que el bit más a la izquierda pasa a ser el ultimo ahora y
/// el resto de bits se desplazan a la izquierda.
pub fn left_shift(key: &str, i: usize) -> String {
  let mut key_shifted = String::new();
  for j in 0..28 {
    let index = (j + i) % 28;
    key_shifted.push(key.chars().nth(index).unwrap());
  }
  key_shifted
}

/// Función para dividir una clave en dos mitades.
pub fn split_key(key: &str) -> (String, String) {
  let key_left = key
    .chars()
    .take(key.len() / 2)
    .collect::<String>();
  let key_right = key
    .chars()
    .skip(key.len() / 2)
    .collect::<String>();
  
  (key_left, key_right)
}


/// Función para aplicar una permutación a un string.
pub fn apply_permutation(key: &str, permutation: &[usize]) -> String {
  let mut key_permuted = String::new();
  for i in 0..permutation.len() {
    let index = permutation[i];
    key_permuted.push(key.chars().nth(index).unwrap());
  }
  key_permuted
}
