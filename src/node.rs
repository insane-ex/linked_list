use std::ptr::NonNull;

#[allow(unused)]
pub(super) type Link<T> = Option<NonNull<Node<T>>>;

#[allow(unused)]
pub(super) struct Node<T> {
    pub(super) previous: Link<T>,
    pub(super) next: Link<T>,
    pub(super) element: T,
}

#[allow(unused)]
impl<T> Node<T> {
    pub(super) fn new(element: T) -> Self {
        Self {
            previous: None,
            next: None,
            element,
        }
    }
}

#[cfg(test)]
mod node_tests {
    use super::Node;

    #[test]
    fn test_create_node() {
        let node = Node::new(1);

        assert!(node.previous.is_none());
        assert!(node.next.is_none());
        assert_eq!(node.element, 1);
    }
}
