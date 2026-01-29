use std::{
    alloc::{Layout, alloc, dealloc},
    marker::PhantomData,
    ptr::NonNull,
};

use super::node::Node;

#[allow(unused)]
pub(super) struct Allocator<T> {
    _marker: PhantomData<T>,
}

#[allow(unused)]
impl<T> Allocator<T> {
    pub(super) fn new() -> Self {
        Self {
            _marker: PhantomData,
        }
    }

    pub(super) fn allocate(&self, node: Node<T>) -> NonNull<Node<T>> {
        let layout = Layout::new::<Node<T>>();
        let raw_ptr = unsafe { alloc(layout).cast::<Node<T>>() };

        assert!(!raw_ptr.is_null(), "Out of memory");

        unsafe { raw_ptr.write(node) };
        unsafe { NonNull::new_unchecked(raw_ptr) }
    }

    pub(super) fn deallocate(&self, node: NonNull<Node<T>>) {
        let layout = Layout::new::<Node<T>>();
        let raw_ptr = node.as_ptr().cast::<u8>();

        unsafe { dealloc(raw_ptr, layout) }
    }
}

#[cfg(test)]
mod allocator_tests {
    use super::{Allocator, Node};

    #[test]
    fn test_allocate_node() {
        let allocator = Allocator::<i32>::new();
        let node = Node::new(1);
        let node_ptr = allocator.allocate(node);

        unsafe {
            assert!((*node_ptr.as_ptr()).previous.is_none());
            assert!((*node_ptr.as_ptr()).next.is_none());
            assert_eq!((*node_ptr.as_ptr()).element, 1);
        }

        allocator.deallocate(node_ptr);
    }
}
