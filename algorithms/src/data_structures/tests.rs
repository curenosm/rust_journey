use crate::data_structures::graph::Graph;
use crate::data_structures::list::LinkedList;


#[cfg(test)]
pub mod test_list {
  use super::*;

  #[test]
  fn test_list() {
    let mut list = LinkedList::<i32>::new();
  }

  #[test]
  fn test_list_2() {
    let mut list = LinkedList::<i32>::new();
  }

}

#[cfg(test)]
pub mod test_graph {
  use super::*;

  #[test]
  fn test_graph() {
    let mut graph = Graph::new();
  }
}
