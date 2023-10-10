use crate::models::Dog;

pub fn dye_coat(dog: &mut Dog) {
  dog.coat_color = String::from("red");
}

pub fn someone_adopts_dog(dog: Dog) {
  dbg!("dog was adopted", dog);
}

pub fn pet_dog(dog: &Dog) {
  dbg!(&dog.coat_color);
}

pub fn add(left: usize, right: usize) -> usize {
  left + right
}

pub(crate) async fn another(x: u8) -> u8 {
  add(x as usize, 5) as u8
}
