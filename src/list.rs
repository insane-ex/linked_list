use crate::allocator::Allocator;

use super::node::Link;

#[allow(unused)]
pub struct LinkedList<T> {
    head: Link<T>,
    tail: Link<T>,
    size: usize,
    allocator: Allocator<T>,
}

impl<T> LinkedList<T> {
    pub fn new() -> Self {
        Self {
            head: None,
            tail: None,
            size: 0,
            allocator: Allocator::new(),
        }
    }
}

#[cfg(test)]
mod list_tests {
    use super::LinkedList;

    #[test]
    fn test_create_list() {
        let list = LinkedList::<i32>::new();

        assert!(list.head.is_none());
        assert!(list.tail.is_none());
        assert_eq!(list.size, 0);
    }
}
