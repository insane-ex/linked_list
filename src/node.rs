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
