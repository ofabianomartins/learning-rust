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
        return false;
    }

    pub fn print_tree(&self) {
        if let Some(node) = &self.root {
            let internal_node = node.as_ref().borrow_mut();
            print!("{}|", internal_node);

            let mut j = 0;
            while j <= internal_node.n_keys {
                if let Some(other_node) = &internal_node.pointers[j] {
                    let aux = other_node.as_ref().borrow();
                    print!("{}|", aux);
                }
                j += 1;
            }
            println!("end");
        } 
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
        queue.push(5);

        assert_eq!(queue.find(1), true);
        assert_eq!(queue.find(5), true);
        assert_eq!(queue.find(6), false);
        assert_eq!(queue.find(9), false);
    }

    #[test]
    fn insert_1_to_5() {
        let mut queue = Btree::new();

        queue.push(1);
        queue.push(2);
        queue.push(3);
        queue.push(4);
        queue.push(5);

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
        queue.push(2);
        queue.push(3);
        queue.push(4);
        queue.push(5);
        queue.push(6);
        queue.push(7);
        queue.push(8);
        queue.push(9);
        queue.push(10);

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
        queue.print_tree();
        queue.push(2);
        queue.print_tree();
        queue.push(3);
        queue.print_tree();
        queue.push(4);
        queue.print_tree();
        queue.push(5);
        queue.print_tree();
        queue.push(6);
        queue.print_tree();
        queue.push(7);
        queue.print_tree();
        queue.push(8);
        queue.print_tree();
        queue.push(9);
        queue.print_tree();
        queue.push(10);

        queue.remove(2);
        queue.remove(3);
        queue.remove(4);
        queue.remove(5);
        queue.remove(6);
        queue.remove(7);

        assert_eq!(queue.find(1), true);
        assert_eq!(queue.find(2), false);
        assert_eq!(queue.find(3), false);
        assert_eq!(queue.find(5), false);
        assert_eq!(queue.find(6), false);
        assert_eq!(queue.find(9), true);
        assert_eq!(queue.find(11), false);
    }
}
