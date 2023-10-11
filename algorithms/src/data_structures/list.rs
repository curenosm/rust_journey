
#[derive(Debug)]
pub struct ListNode<T> {
    pub data: T,
    pub next: Option<Box<ListNode<T>>>,
    pub prev: Option<Box<ListNode<T>>>,
}


#[derive(Debug)]
pub struct LinkedList<T> {
    pub head: Option<Box<ListNode<T>>>,
    pub tail: Option<Box<ListNode<T>>>,
    pub size: usize,
    pub last: Option<Box<ListNode<T>>>,
}

impl<T> LinkedList<T> {

    pub fn new() -> Self {
        LinkedList::<T> {
            head: None,
            tail: None,
            size: 0,
            last: None,
        }
    }


}
