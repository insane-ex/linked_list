#![allow(unused)]

use std::ptr::NonNull;

type Link<T> = Option<NonNull<Node<T>>>;

struct Node<T> {
    previous: Link<T>,
    next: Link<T>,
    element: T,
}

impl<T> Node<T> {
    pub const fn new(element: T) -> Self {
        Self {
            previous: None,
            next: None,
            element,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Node;

    #[test]
    fn create_node() {
        let node = Node::new(1);

        assert!(node.previous.is_none());
        assert!(node.next.is_none());
        assert_eq!(node.element, 1);
    }
}
