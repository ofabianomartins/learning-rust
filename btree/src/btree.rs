use std::collections::VecDeque;
use std::{cell::RefCell, rc::Rc};

use crate::node::Node;
use crate::node::Link;

use crate::node::NODE_CAPACITY;

#[derive(Debug, Clone)]
pub struct Btree {
    root: Link,
}

impl Btree {
    pub fn new() -> Self {
        Btree { root: None }
    }

    pub fn push(&mut self, value: u8) {
        println!("Inserting {}", value);

        if let Some(node) = &mut self.root {
            let mut internal_node = node.as_ref().borrow_mut();
            
            if internal_node.n_keys == NODE_CAPACITY {
                let mut new_node = Node::new(true, false);
                new_node.pointers[0] = Some(Rc::new(RefCell::new(internal_node.clone())));
                new_node.split_node(0);
                *internal_node = new_node.clone();
                internal_node.push_nonfull(value);
            } else {
                internal_node.push_nonfull(value);
            }
        } else {
            let mut new_node = Node::new(true, true);
            new_node.push_nonfull(value);
            self.root = Some(Rc::new(RefCell::new(new_node)))
        }
    }

    pub fn find(&mut self, value: u8) -> bool {
        if let Some(node) = &self.root {
            let mut internal_node = node.borrow_mut();

            return internal_node.find(value, 0)
        }

        return false;
    }

    pub fn remove(&mut self, value: u8) -> bool {
        println!("Removing {}", value);

        if let Some(node) = &self.root {
            let mut internal_node = node.borrow_mut();

            return internal_node.remove(value, 0)
        }

        return false;
    }

    pub fn print_tree(&self) {
        let mut queue: VecDeque<Link> = VecDeque::new();

        queue.push_back(self.root.clone());
        while !queue.is_empty() {
            if let Some(node) = &queue.pop_front() {
                let actual_node = node.as_ref().expect("REASON").borrow_mut();

                let mut j = 0;
                while j <= actual_node.n_keys {
                    if let Some(new_pointer_node) = &actual_node.pointers[j] {
                        queue.push_back(Some(new_pointer_node.clone())); 
                    } 
                    j += 1;
                }
                print!("{}|", actual_node);
            } 
        }

        println!("end");
    }

    pub fn is_correct(&mut self) -> bool {
        if let Some(node) = &self.root {
            let mut internal_node = node.borrow_mut();

            return internal_node.is_correct()
        }

        return true;
    }
}

#[cfg(test)]
pub mod tests {
    use super::*;

    #[test]
    fn insert_1_only() {
        let mut queue = Btree::new();

        queue.push(1);

        assert_eq!(queue.find(1), true);
        assert_eq!(queue.find(6), false);
        assert_eq!(queue.find(9), false);
    }

    #[test]
    fn insert_1_and_5() {
        let mut queue = Btree::new();

        queue.push(1);
        assert!(queue.is_correct());
        queue.push(5);
        assert!(queue.is_correct());

        assert_eq!(queue.find(1), true);
        assert_eq!(queue.find(5), true);
        assert_eq!(queue.find(6), false);
        assert_eq!(queue.find(9), false);
    }

    #[test]
    fn insert_1_to_5() {
        let mut queue = Btree::new();

        queue.push(1);
        assert!(queue.is_correct());
        queue.push(2);
        assert!(queue.is_correct());
        queue.push(3);
        assert!(queue.is_correct());
        queue.push(4);
        assert!(queue.is_correct());
        queue.push(5);
        assert!(queue.is_correct());

        assert_eq!(queue.find(1), true);
        assert_eq!(queue.find(2), true);
        assert_eq!(queue.find(3), true);
        assert_eq!(queue.find(5), true);
        assert_eq!(queue.find(6), false);
        assert_eq!(queue.find(9), false);
    }

    #[test]
    fn insert_1_to_10() {
        let mut queue = Btree::new();

        queue.push(1);
        assert!(queue.is_correct());
        queue.push(2);
        assert!(queue.is_correct());
        queue.push(3);
        assert!(queue.is_correct());
        queue.push(4);
        assert!(queue.is_correct());
        queue.push(5);
        assert!(queue.is_correct());
        queue.push(6);
        assert!(queue.is_correct());
        queue.push(7);
        assert!(queue.is_correct());
        queue.push(8);
        assert!(queue.is_correct());
        queue.push(9);
        assert!(queue.is_correct());
        queue.push(10);
        assert!(queue.is_correct());

        assert_eq!(queue.find(1), true);
        assert_eq!(queue.find(2), true);
        assert_eq!(queue.find(3), true);
        assert_eq!(queue.find(5), true);
        assert_eq!(queue.find(6), true);
        assert_eq!(queue.find(9), true);
        assert_eq!(queue.find(11), false);
    }

    #[test]
    fn insert_1_to_10_to_remove() {
        let mut queue = Btree::new();

        queue.push(1);
        assert!(queue.is_correct());
        queue.push(2);
        assert!(queue.is_correct());
        queue.push(3);
        assert!(queue.is_correct());
        queue.push(4);
        assert!(queue.is_correct());
        queue.push(5);
        assert!(queue.is_correct());
        queue.push(6);
        assert!(queue.is_correct());
        queue.push(7);
        assert!(queue.is_correct());
        queue.push(8);
        assert!(queue.is_correct());
        queue.push(9);
        assert!(queue.is_correct());
        queue.push(10);
        assert!(queue.is_correct());
        queue.print_tree();

        queue.remove(2);
        assert!(queue.is_correct());
        queue.print_tree();
        queue.remove(5);
        assert!(queue.is_correct());
        queue.print_tree();
        queue.remove(7);
        assert!(queue.is_correct());
        queue.print_tree();

        assert_eq!(queue.find(1), true);
        assert_eq!(queue.find(5), false);
        assert_eq!(queue.find(7), false);
        assert_eq!(queue.find(11), false);
    }

    #[test]
    fn insert_1_to_10_to_remove_internal() {
        let mut queue = Btree::new();

        queue.push(1);
        assert!(queue.is_correct());
        queue.push(2);
        assert!(queue.is_correct());
        queue.push(3);
        assert!(queue.is_correct());
        queue.push(4);
        assert!(queue.is_correct());
        queue.push(5);
        assert!(queue.is_correct());
        queue.push(6);
        assert!(queue.is_correct());
        queue.push(7);
        assert!(queue.is_correct());
        queue.push(8);
        assert!(queue.is_correct());
        queue.push(9);
        assert!(queue.is_correct());
        queue.push(10);
        assert!(queue.is_correct());
        queue.print_tree();

        queue.remove(8);
        assert!(queue.is_correct());
        queue.print_tree();
        queue.remove(10);
        assert!(queue.is_correct());
        queue.print_tree();

        assert_eq!(queue.find(1), true);
        assert_eq!(queue.find(8), false);
        assert_eq!(queue.find(10), false);
        assert_eq!(queue.find(11), false);
    }

    #[test]
    fn insert_1_to_10_and_remove_38() {
        let mut queue = Btree::new();

        let mut j = 26;

        while j > 14 {
            queue.push(j*2);
            assert!(queue.is_correct());
            j -= 1;
        }
        queue.print_tree();

        queue.remove(38);
        assert!(queue.is_correct());
        queue.print_tree();

        assert_eq!(queue.find(46), true);
        assert_eq!(queue.find(34), true);
        assert_eq!(queue.find(42), true);
        assert_eq!(queue.find(36), true);
        assert_eq!(queue.find(40), true);
        assert_eq!(queue.find(38), false);
    }

    #[test]
    fn insert_1_to_26_and_remove_22() {
        let mut queue = Btree::new();

        let mut j = 26;

        while j > 0 {
            queue.push(j);
            assert!(queue.is_correct());
            j -= 1;
        }
        queue.print_tree();

        queue.remove(22);
        queue.print_tree();
        assert!(queue.is_correct());

        assert_eq!(queue.find(26), true);
        assert_eq!(queue.find(22), false);
    }

    #[test]
    fn insert_1_to_10_and_remove_32() {
        let mut queue = Btree::new();

        let mut j = 26;

        while j > 0 {
            queue.push(j*2);
            assert!(queue.is_correct());
            j -= 1;
        }

        queue.push(23);
        assert!(queue.is_correct());
        queue.print_tree();

        queue.remove(32);
        assert!(queue.is_correct());
        queue.print_tree();

        assert_eq!(queue.find(46), true);
        assert_eq!(queue.find(34), true);
        assert_eq!(queue.find(42), true);
        assert_eq!(queue.find(36), true);
        assert_eq!(queue.find(40), true);
        assert_eq!(queue.find(32), false);
    }
}
