use super::{
    allocator::NodeAllocator,
    node::{Link, Node},
};

pub struct LinkedList<T> {
    head: Link<T>,
    tail: Link<T>,
    size: usize,
    allocator: NodeAllocator,
}

impl<T> LinkedList<T> {
    #[must_use]
    pub const fn new() -> Self {
        Self {
            head: None,
            tail: None,
            size: 0,
            allocator: NodeAllocator::new(),
        }
    }

    pub fn push_front(&mut self, element: T) {
        let mut new_node = Node::new(element);

        new_node.next = self.head;

        let new_node_ptr = self.allocator.allocate(new_node);

        if let Some(mut head) = self.head {
            unsafe { head.as_mut().previous = Some(new_node_ptr) };
        } else {
            self.tail = Some(new_node_ptr);
        }

        self.head = Some(new_node_ptr);
        self.size += 1;
    }

    pub fn push_back(&mut self, element: T) {
        let mut new_node = Node::new(element);

        new_node.previous = self.tail;

        let new_node_ptr = self.allocator.allocate(new_node);

        if let Some(mut tail) = self.tail {
            unsafe { tail.as_mut().next = Some(new_node_ptr) };
        } else {
            self.head = Some(new_node_ptr);
        }

        self.tail = Some(new_node_ptr);
        self.size += 1;
    }

    pub fn pop_front(&mut self) -> Option<T> {
        let old_head = self.head?;

        self.head = unsafe { old_head.as_ref().next };

        if let Some(mut new_head) = self.head {
            unsafe { new_head.as_mut().previous = None };
        } else {
            self.tail = None;
        }

        self.size -= 1;

        let popped_element = self.allocator.deallocate(old_head);

        Some(popped_element)
    }

    pub fn pop_back(&mut self) -> Option<T> {
        let old_tail = self.tail?;

        self.tail = unsafe { old_tail.as_ref().previous };

        if let Some(mut new_tail) = self.tail {
            unsafe { new_tail.as_mut().next = None };
        } else {
            self.head = None;
        }

        self.size -= 1;

        let popped_element = self.allocator.deallocate(old_tail);

        Some(popped_element)
    }
}

impl<T> Default for LinkedList<T> {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod utils {
    use super::{LinkedList, Node};

    pub fn raw_head<T>(list: &LinkedList<T>) -> &Node<T> {
        unsafe { list.head.unwrap().as_ref() }
    }

    pub fn raw_tail<T>(list: &LinkedList<T>) -> &Node<T> {
        unsafe { list.tail.unwrap().as_ref() }
    }
}

#[cfg(test)]
mod list_tests {
    use crate::list::utils::{raw_head, raw_tail};

    use super::LinkedList;

    #[test]
    fn test_create_list() {
        let list = LinkedList::<i32>::new();

        assert!(list.head.is_none());
        assert!(list.tail.is_none());
        assert_eq!(list.size, 0);
    }

    #[test]
    fn test_push_front_one_element() {
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

        assert_eq!(list.size, 1);
    }

    #[test]
    fn test_push_front_two_elements() {
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

        assert_eq!(list.size, 2);
    }

    #[test]
    fn test_push_back_one_element() {
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

        assert_eq!(list.size, 1);
    }

    #[test]
    fn test_push_back_two_elements() {
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

        assert_eq!(list.size, 2);
    }

    #[test]
    fn test_pop_front_empty_list() {
        let mut list = LinkedList::<i32>::new();

        assert!(list.pop_front().is_none());
    }

    #[test]
    fn test_pop_front_one_element() {
        let mut list = LinkedList::<i32>::new();

        list.push_front(1);

        let popped_element = list.pop_front();

        assert_eq!(popped_element, Some(1));

        assert!(list.head.is_none());
        assert!(list.tail.is_none());
        assert_eq!(list.size, 0);
    }

    #[test]
    fn test_pop_front_two_elements() {
        let mut list = LinkedList::<i32>::new();

        list.push_front(1);
        list.push_front(2);

        let first_pop = list.pop_front();

        assert_eq!(first_pop, Some(2));

        let head_ptr = raw_head(&list);

        assert!(head_ptr.previous.is_none());
        assert!(head_ptr.next.is_none());
        assert_eq!(head_ptr.element, 1);

        let tail_ptr = raw_tail(&list);

        assert!(tail_ptr.previous.is_none());
        assert!(tail_ptr.next.is_none());
        assert_eq!(tail_ptr.element, 1);

        let second_pop = list.pop_front();

        assert_eq!(second_pop, Some(1));

        assert!(list.head.is_none());
        assert!(list.tail.is_none());
        assert_eq!(list.size, 0);
    }

    #[test]
    fn test_pop_back_empty_list() {
        let mut list = LinkedList::<i32>::new();

        assert!(list.pop_back().is_none());
    }

    #[test]
    fn test_pop_back_one_element() {
        let mut list = LinkedList::<i32>::new();

        list.push_back(1);

        let popped_element = list.pop_back();

        assert_eq!(popped_element, Some(1));

        assert!(list.head.is_none());
        assert!(list.tail.is_none());
        assert_eq!(list.size, 0);
    }

    #[test]
    fn test_pop_back_two_elements() {
        let mut list = LinkedList::<i32>::new();

        list.push_back(1);
        list.push_back(2);

        let first_pop = list.pop_back();

        assert_eq!(first_pop, Some(2));

        let head_ptr = raw_head(&list);

        assert!(head_ptr.previous.is_none());
        assert!(head_ptr.next.is_none());
        assert_eq!(head_ptr.element, 1);

        let tail_ptr = raw_tail(&list);

        assert!(tail_ptr.previous.is_none());
        assert!(tail_ptr.next.is_none());
        assert_eq!(tail_ptr.element, 1);

        let second_pop = list.pop_back();

        assert_eq!(second_pop, Some(1));

        assert!(list.head.is_none());
        assert!(list.tail.is_none());
        assert_eq!(list.size, 0);
    }
}
