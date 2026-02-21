#![allow(unused)]

use super::node::Link;

pub struct LinkedList<T> {
    head: Link<T>,
    tail: Link<T>,
    length: usize,
}

impl<T> LinkedList<T> {
    pub const fn new() -> Self {
        Self {
            head: None,
            tail: None,
            length: 0,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::LinkedList;

    #[test]
    fn create_list() {
        let list = LinkedList::<i32>::new();

        assert!(list.head.is_none());
        assert!(list.tail.is_none());
        assert_eq!(list.length, 0);
    }
}
