#[cfg(test)]
pub mod tests {
    use std::collections::BinaryHeap;
    use std::collections::HashMap;
    use std::collections::HashSet;
    use std::collections::LinkedList;
    use std::collections::VecDeque;

    #[test]
    pub fn set_examples() {
        let mut h: HashSet<i32> = HashSet::new();

        for i in 0..100_000 {
            h.insert(i);
        }

        assert_eq!(h.len(), 100_000);
    }

    #[test]
    #[allow(unused_variables)]
    #[allow(unused_mut)]
    pub fn hashmap_examples() {
        let mut h: HashMap<i32, i32> = HashMap::new();

        for i in 0..100_000 {
            h.insert(i, i);
        }

        assert_eq!(h.len(), 100_000);
    }

    #[test]
    pub fn binary_heap_examples() {
        let mut h: BinaryHeap<i32> = BinaryHeap::new();

        for i in 0..100_000 {
            h.push(i);
        }

        assert_eq!(h.len(), 100_000);
    }

    #[test]
    pub fn linked_list_examples() {
        let mut h: LinkedList<i32> = LinkedList::new();

        for i in 0..100_000 {
            h.push_back(i);
        }

        assert_eq!(h.len(), 100_000);
    }

    #[test]
    pub fn vec_deque_examples() {
        let mut h: VecDeque<i32> = VecDeque::new();

        for i in 0..100_000 {
            h.push_back(i);
        }

        assert_eq!(h.len(), 100_000);
    }
}
