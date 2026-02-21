#![allow(unused)]

use crate::node_allocator::allocate_node;

use super::node::{Link, Node};

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

    pub fn push_front(&mut self, element: T) {
        let mut new_node = Node::new(element);

        new_node.next = self.head;

        let node_ptr = allocate_node(new_node);

        if let Some(mut node) = self.head {
            unsafe { node.as_mut().previous = Some(node_ptr) };
        } else {
            self.tail = Some(node_ptr);
        }

        self.head = Some(node_ptr);
        self.length += 1;
    }
}
