#![allow(unused)]

use std::{
    alloc::{Layout, alloc, dealloc},
    ptr::{self, NonNull},
};

use super::node::Node;

pub fn allocate_node<T>(node: Node<T>) -> NonNull<Node<T>> {
    let layout = Layout::new::<Node<T>>();
    let node_ptr = unsafe { alloc(layout).cast::<Node<T>>() };

    assert!(!node_ptr.is_null(), "Out of memory");

    unsafe {
        ptr::write(node_ptr, node);

        NonNull::new_unchecked(node_ptr)
    }
}

pub unsafe fn deallocate_node<T>(node: NonNull<Node<T>>) {
    unsafe {
        ptr::drop_in_place(node.as_ptr());

        dealloc(node.as_ptr().cast::<u8>(), Layout::new::<Node<T>>())
    }
}
