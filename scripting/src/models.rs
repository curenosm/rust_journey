
#[derive(Debug)]
pub(crate) struct Person {
  pub(crate) name: String,
  pub(crate) age: u8,
}

#[derive(Debug)]
pub(crate) enum Fruit {
  Apple,
  Banana,
  Orange,
}

#[derive(Debug)]
pub(crate) enum Action {
  AddTodo {
    id: u32,
    text: String,
  },
  RemoveTodo(u32),
}

#[derive(Debug)]
pub(crate) struct Dog {
  pub(crate) coat_color: String,
}
