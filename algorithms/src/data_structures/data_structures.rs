#[cfg(test)]
pub mod tests {

    use std::collections::BinaryHeap;
    use std::collections::HashMap;
    use std::collections::HashSet;
    use std::collections::LinkedList;
    use std::collections::VecDeque;

    #[test]
    pub fn set_examples() {
        // Create a new empty HashSet
        let mut h: HashSet<i32> = HashSet::new();

        // Insert some values
        for i in 0..100_000_000 {
            h.insert(i);
        }

        // Check for the size
        assert_eq!(h.len(), 100_000_000);
    }

    #[test]
    #[allow(unused_variables)]
    #[allow(unused_mut)]
    pub fn hashmap_examples() {
        // Create a new empty HashMap
        let mut h: HashMap<i32, i32> = HashMap::new();

        // Insert some values
        for i in 0..100_000_000 {
            h.insert(i, i);
        }

        // Check for the size
        assert_eq!(h.len(), 100_000_000);
    }

    #[test]
    pub fn binary_heap_examples() {
        // Create a new empty BinaryHeap
        let mut h: BinaryHeap<i32> = BinaryHeap::new();

        // Insert some values
        // Add a hundred million elements to the queue
        for i in 0..100_000_000 {
            h.push(i);
        }

        // Check for the size
        assert_eq!(h.len(), 100_000_000);
    }

    #[test]
    pub fn linked_list_examples() {
        // Create a new empty LinkedList
        let mut h: LinkedList<i32> = LinkedList::new();

        // Add a hundred million elements to the queue
        for i in 0..100_000_000 {
            h.push_back(i);
        }

        // Check for the size
        assert_eq!(h.len(), 100_000_000);
    }

    #[test]
    pub fn vec_deque_examples() {
        // Create a new empty VecDeque
        let mut h: VecDeque<i32> = VecDeque::new();

        // Add a hundred million elements to the queue
        for i in 0..100_000_000 {
            h.push_back(i);
        }

        // Check for the size
        assert_eq!(h.len(), 100_000_000);
    }
}
