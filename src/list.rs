#![allow(unused)]

use std::{
    fmt::{self, Display},
    mem,
    ptr::{self, NonNull},
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

    pub fn contains(&self, element: &T) -> bool
    where
        T: PartialEq,
    {
        let mut current_node = self.head;

        while let Some(node) = current_node {
            let node_ref = unsafe { node.as_ref() };

            if &node_ref.element == element {
                return true;
            }

            current_node = node_ref.next;
        }

        false
    }

    pub fn reverse(&mut self) {
        if self.length <= 1 {
            return;
        }

        let mut current_node = self.head;

        while let Some(mut node) = current_node {
            let next_node = unsafe { node.as_ref().next };

            unsafe {
                node.as_mut().next = node.as_ref().previous;
                node.as_mut().previous = next_node;
            }

            current_node = next_node;
        }

        unsafe { mem::swap(&mut self.head, &mut self.tail) }
    }

    fn remove_node(&mut self, node: NonNull<Node<T>>) {
        let node_ref = unsafe { node.as_ref() };

        if let Some(mut previous) = node_ref.previous {
            unsafe { previous.as_mut().next = node_ref.next }
        } else {
            self.head = node_ref.next;
        }

        if let Some(mut next) = node_ref.next {
            unsafe { next.as_mut().previous = node_ref.previous }
        } else {
            self.tail = node_ref.previous;
        }

        unsafe { deallocate_node(node) };

        self.length -= 1;
    }

    pub fn retain<F>(&mut self, mut predicate: F)
    where
        F: FnMut(&T) -> bool,
    {
        let mut current_node = self.head;

        while let Some(node) = current_node {
            let next_node = unsafe { node.as_ref().next };

            if !predicate(unsafe { &node.as_ref().element }) {
                self.remove_node(node);
            }

            current_node = next_node;
        }
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

impl<T> Drop for LinkedList<T> {
    fn drop(&mut self) {
        while self.pop_front().is_some() {}
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

    #[test]
    fn list_is_empty() {
        let list = LinkedList::<i32>::new();

        assert!(list.is_empty());
    }

    #[test]
    fn list_not_is_empty() {
        let mut list = LinkedList::<i32>::new();

        list.push_front(1);

        assert!(!list.is_empty());
    }

    #[test]
    fn contains_return_true() {
        let mut list = LinkedList::<i32>::new();

        list.push_front(1);

        assert!(list.contains(&1));
    }

    #[test]
    fn contains_return_false() {
        let mut list = LinkedList::<i32>::new();

        list.push_front(2);

        assert!(!list.contains(&1));
    }

    #[test]
    fn reverse_empty_list() {
        let mut list: LinkedList<i32> = LinkedList::new();

        list.reverse();

        assert!(list.pop_front().is_none());
        assert_eq!(list.size(), 0);
    }

    #[test]
    fn reverse_one_element() {
        let mut list = LinkedList::new();

        list.push_back(1);

        list.reverse();

        assert_eq!(list.size(), 1);
        assert_eq!(list.pop_front(), Some(1));
        assert!(list.is_empty());
    }

    #[test]
    fn reverse_two_elements() {
        let mut list = LinkedList::new();

        list.push_back(1);
        list.push_back(2);
        list.reverse();

        assert_eq!(list.size(), 2);
        assert_eq!(list.pop_front(), Some(2));
        assert_eq!(list.pop_front(), Some(1));
        assert!(list.is_empty());
    }

    #[test]
    fn reverse_multiple_elements() {
        let mut list = LinkedList::new();

        for i in 1..=5 {
            list.push_back(i);
        }

        list.reverse();

        for expected in (1..=5).rev() {
            assert_eq!(list.pop_front(), Some(expected));
        }

        assert!(list.is_empty());
    }

    #[test]
    fn reverse_twice_restores_order() {
        let mut list = LinkedList::new();

        for i in 1..=5 {
            list.push_back(i);
        }

        list.reverse();
        list.reverse();

        for expected in 1..=5 {
            assert_eq!(list.pop_front(), Some(expected));
        }

        assert!(list.is_empty());
    }
}
