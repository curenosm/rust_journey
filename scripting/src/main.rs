// #!/usr/bin/env rust-script
// #!/usr/bin/env cargo +nightly -Zscript

use funcs::{another, dye_coat, pet_dog, someone_adopts_dog};
use models::{Action, Fruit, Person, Dog};

mod models;
mod funcs;
mod tests;

// import the necessary macros from tokio extern crate tokio;

#[tokio::main]
async fn main() {
  println!("Hello, world!");

  let x = 5; // immutable
  let mut y = 5; // mutable
  dbg!(x, y);
  y = 10;

  // error: cannot assign twice to immutable variable `x`
  // x = 10;

  dbg!(x, y);

  // Structs and enums

  let person = Person {
    name: String::from("John"),
    age: 100,
  };
  println!("{:?}", person.age);
  println!("{:?}", person.name);

  dbg!(person);

  let fruit = Fruit::Apple;
  let banana = Fruit::Banana;
  let orange = Fruit::Orange;

  println!("{:?}", fruit);
  println!("{:?}", banana);
  println!("{:?}", orange);

  dbg!(fruit);

  let action = Action::AddTodo {
    id: 0,
    text: String::from("Hello"),
  };

  let action_1 = Action::RemoveTodo(0);
  println!("{:?}", action_1);

  match action {
    Action::AddTodo { id, text } => {
      dbg!(id, text);
    }
    Action::RemoveTodo(id, ..) => {
      dbg!(id);
      todo!()
    }
  }

  let maybe_some_number = Some(42);
  let our_number: u8;
  our_number = match maybe_some_number {
    Some(number) => number,
    None => 0,
  };

  let our_number_1 = maybe_some_number.unwrap_or(0);

  dbg!(our_number);
  dbg!(our_number_1);


  // Iterators
  for i in 0..10 {
    dbg!(i);
  }

  let alphabet: String = ('a'..='z').collect();
  dbg!(alphabet);

  // Asynchronous programming
  let result = another(5).await;
  dbg!(result);

  // Borrowing
  let mut dog = Dog {
    coat_color: String::from("white")
  };

  pet_dog(&dog);
  dye_coat(&mut dog);
  someone_adopts_dog(dog);
  // Can't pet a dog that has been adopted
  // pet_dog(&dog);
}

/*
enum Option<T> {
    Some(T),
    None,
}
*/
