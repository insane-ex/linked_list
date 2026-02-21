#![allow(unused)]

use std::{
    fmt::{self, Display},
    ptr,
};

use crate::node_allocator::{allocate_node, deallocate_node};

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

    pub fn push_back(&mut self, element: T) {
        let mut new_node = Node::new(element);

        new_node.previous = self.tail;

        let node_ptr = allocate_node(new_node);

        if let Some(mut node) = self.tail {
            unsafe { node.as_mut().next = Some(node_ptr) };
        } else {
            self.head = Some(node_ptr);
        }

        self.tail = Some(node_ptr);
        self.length += 1;
    }

    pub fn pop_front(&mut self) -> Option<T> {
        let old_head = self.head?;

        self.head = unsafe { old_head.as_ref().next };

        if let Some(mut node) = self.head {
            unsafe { node.as_mut().previous = None };
        } else {
            self.tail = None;
        }

        let element = unsafe { ptr::read(&raw const old_head.as_ref().element) };

        unsafe { deallocate_node(old_head) };

        self.length -= 1;

        Some(element)
    }

    pub fn pop_back(&mut self) -> Option<T> {
        let old_tail = self.tail?;

        self.tail = unsafe { old_tail.as_ref().previous };

        if let Some(mut node) = self.tail {
            unsafe { node.as_mut().next = None };
        } else {
            self.head = None;
        }

        let element = unsafe { ptr::read(&raw const old_tail.as_ref().element) };

        unsafe { deallocate_node(old_tail) };

        self.length -= 1;

        Some(element)
    }

    pub fn size(&self) -> usize {
        self.length
    }

    pub fn is_empty(&self) -> bool {
        self.length == 0
    }
}

impl<T: Display> Display for LinkedList<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.length == 0 {
            return write!(f, "[]");
        }

        write!(f, "[")?;

        let mut current_node = self.head;

        while let Some(node) = current_node {
            let node_ref = unsafe { node.as_ref() };

            if node_ref.next.is_some() {
                write!(f, "{} <-> ", node_ref.element)?;
            } else {
                write!(f, "{}", node_ref.element)?;
            }

            current_node = node_ref.next;
        }

        write!(f, "]")
    }
}

#[cfg(test)]
mod tests {
    use crate::node::Node;

    use super::LinkedList;

    fn raw_head<T>(list: &LinkedList<T>) -> &Node<T> {
        unsafe { list.head.unwrap().as_ref() }
    }

    fn raw_tail<T>(list: &LinkedList<T>) -> &Node<T> {
        unsafe { list.tail.unwrap().as_ref() }
    }

    #[test]
    fn create_list() {
        let list = LinkedList::<i32>::new();

        assert!(list.head.is_none());
        assert!(list.tail.is_none());
        assert_eq!(list.length, 0);
    }

    #[test]
    fn push_front_one_element() {
        let mut list = LinkedList::<i32>::new();

        list.push_front(1);

        let head_ptr = raw_head(&list);

        assert!(head_ptr.previous.is_none());
        assert!(head_ptr.next.is_none());
        assert_eq!(head_ptr.element, 1);

        let tail_ptr = raw_tail(&list);

        assert!(tail_ptr.previous.is_none());
        assert!(tail_ptr.next.is_none());
        assert_eq!(tail_ptr.element, 1);

        assert_eq!(list.length, 1);
    }

    #[test]
    fn push_front_two_elements() {
        let mut list = LinkedList::<i32>::new();

        list.push_front(1);
        list.push_front(2);

        let head_ptr = raw_head(&list);

        assert!(head_ptr.previous.is_none());
        assert!(head_ptr.next.is_some());
        assert_eq!(head_ptr.element, 2);

        let tail_ptr = raw_tail(&list);

        assert!(tail_ptr.previous.is_some());
        assert!(tail_ptr.next.is_none());
        assert_eq!(tail_ptr.element, 1);

        assert_eq!(list.length, 2);
    }

    #[test]
    fn push_back_one_element() {
        let mut list = LinkedList::<i32>::new();

        list.push_back(1);

        let head_ptr = raw_head(&list);

        assert!(head_ptr.previous.is_none());
        assert!(head_ptr.next.is_none());
        assert_eq!(head_ptr.element, 1);

        let tail_ptr = raw_tail(&list);

        assert!(tail_ptr.previous.is_none());
        assert!(tail_ptr.next.is_none());
        assert_eq!(tail_ptr.element, 1);

        assert_eq!(list.length, 1);
    }

    #[test]
    fn push_back_two_elements() {
        let mut list = LinkedList::<i32>::new();

        list.push_back(1);
        list.push_back(2);

        let head_ptr = raw_head(&list);

        assert!(head_ptr.previous.is_none());
        assert!(head_ptr.next.is_some());
        assert_eq!(head_ptr.element, 1);

        let tail_ptr = raw_tail(&list);

        assert!(tail_ptr.previous.is_some());
        assert!(tail_ptr.next.is_none());
        assert_eq!(tail_ptr.element, 2);

        assert_eq!(list.length, 2);
    }

    #[test]
    fn display_list_pushing_front() {
        let mut list = LinkedList::<i32>::new();

        list.push_front(1);
        list.push_front(2);
        list.push_front(3);

        assert_eq!(format!("{list}"), "[3 <-> 2 <-> 1]");
    }

    #[test]
    fn display_list_pushing_back() {
        let mut list = LinkedList::<i32>::new();

        list.push_back(1);
        list.push_back(2);
        list.push_back(3);

        assert_eq!(format!("{list}"), "[1 <-> 2 <-> 3]");
    }

    #[test]
    fn pop_front_one_element() {
        let mut list = LinkedList::<i32>::new();

        list.push_front(1);

        let popped_element = list.pop_front();

        assert!(popped_element.is_some());
        assert_eq!(popped_element.unwrap(), 1);

        assert!(list.head.is_none());
        assert!(list.tail.is_none());
        assert_eq!(list.length, 0);
    }

    #[test]
    fn pop_front_two_elements() {
        let mut list = LinkedList::<i32>::new();

        list.push_front(1);
        list.push_front(2);

        let first_pop = list.pop_front();

        assert!(first_pop.is_some());
        assert_eq!(first_pop.unwrap(), 2);

        let second_pop = list.pop_front();

        assert!(second_pop.is_some());
        assert_eq!(second_pop.unwrap(), 1);

        assert!(list.head.is_none());
        assert!(list.tail.is_none());
        assert_eq!(list.length, 0);
    }

    #[test]
    fn pop_back_one_element() {
        let mut list = LinkedList::<i32>::new();

        list.push_back(1);

        let popped_element = list.pop_back();

        assert!(popped_element.is_some());
        assert_eq!(popped_element.unwrap(), 1);

        assert!(list.head.is_none());
        assert!(list.tail.is_none());
        assert_eq!(list.length, 0);
    }

    #[test]
    fn pop_back_two_elements() {
        let mut list = LinkedList::<i32>::new();

        list.push_back(1);
        list.push_back(2);

        let first_pop = list.pop_back();

        assert!(first_pop.is_some());
        assert_eq!(first_pop.unwrap(), 2);

        let second_pop = list.pop_back();

        assert!(second_pop.is_some());
        assert_eq!(second_pop.unwrap(), 1);

        assert!(list.head.is_none());
        assert!(list.tail.is_none());
        assert_eq!(list.length, 0);
    }

    #[test]
    fn list_size() {
        let mut list = LinkedList::<i32>::new();

        list.push_front(1);

        assert_eq!(list.size(), 1);
    }
}
