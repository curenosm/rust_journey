pub struct ListNode<T> {
    data: T,
    next: Option<Box<ListNode<T>>>,
    prev: Option<Box<ListNode<T>>>,
}

pub struct LinkedList<T> {
    head: Option<Box<ListNode<T>>>,
    tail: Option<Box<ListNode<T>>>,
    size: usize,
    last: Option<Box<ListNode<T>>>,
}
