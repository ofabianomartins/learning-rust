use std::{cell::RefCell, rc::Rc, cell::RefMut};

use std::fmt;

pub const BTREE_ORDER: usize = 2;
pub const NODE_CAPACITY: usize = 2*BTREE_ORDER - 1;
pub const LEAF_MIN_CAPACITY: usize = BTREE_ORDER - 1;
pub const ARRAY_DEFAULT_VALUE: Link = None;

type Link = Option<Rc<RefCell<Node>>>;

#[derive(Debug, Clone)]
pub struct Node { 
    pub is_root: bool,
    pub is_leaf: bool,

    pub n_keys: usize,
    pub keys: [u8; NODE_CAPACITY],
    pub pointers: [Link; NODE_CAPACITY + 1],
}

impl fmt::Display for Node {
     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "(");
        let mut i: usize = 0;

        while i < self.n_keys {
            write!(f, "{}", self.keys[i]);
            i += 1;

            if i != self.n_keys - 1 {
                write!(f, ", ");
            }
        }

        write!(f, ")")
    }
}

impl Node {
    pub fn new(is_root: bool, is_leaf: bool) -> Self {
        Self { 
            is_root, 
            is_leaf,

            n_keys: 0,
            keys: [0; NODE_CAPACITY],
            pointers: [ARRAY_DEFAULT_VALUE; NODE_CAPACITY + 1]
        }
    }

    pub fn find(&mut self, value: u8) -> bool {
        let mut i: usize = 0;
        while i < self.n_keys && value > self.keys[i]  {
            i += 1;
        }

        if i < self.n_keys && value == self.keys[i]  {
            return true
        } 

        if self.is_leaf {
            return false
        }

        if let Some(node) = &self.pointers[i] {
            let mut internal_node = node.as_ref().borrow_mut();

            return internal_node.find(value)
        }

        return false;
    }

    pub fn push_nonfull(&mut self, value: u8) {
        let mut j: usize = self.n_keys;
        println!("Inserting nonfull {} ", j);

        if self.is_leaf {
            while j >= 0 && j < (self.keys[j] as usize){
                self.keys[j+1] = self.keys[j];
                j -= 1;
            }
            self.keys[j] = value;
            self.n_keys += 1;
        } else {
            while j >= 1 && j < (self.keys[j] as usize) {
                j -= 1;
            }
            j += 1;
            if let Some(node) = &self.pointers[j] {
                let mut internal_node = node.as_ref().borrow_mut();

                if internal_node.n_keys == NODE_CAPACITY {
                    self.split_node(j);
                    if value > self.keys[j] {
                        j += 1;
                    }
                }
                internal_node.push_nonfull(value);
            }
        }
    }

    pub fn split_node(&mut self, pos: usize) {
        if let Some(node) = &self.pointers[pos] {
            println!("Spliting node");

            let mut internal_node = node.as_ref().borrow_mut();
            let new_node = &mut Node::new(false, true);

            new_node.is_leaf = internal_node.is_leaf;
            new_node.n_keys = LEAF_MIN_CAPACITY;

            let mut j = 0;
            while j < LEAF_MIN_CAPACITY {
                new_node.keys[j] = internal_node.keys[j + LEAF_MIN_CAPACITY];
                j += 1;
            }

            if !internal_node.is_leaf {
                j = 0;
                while j < BTREE_ORDER {
                    // new_node.pointers[j] = internal_node.pointers[j + LEAF_MIN_CAPACITY];
                    j += 1;
                }
            }

            internal_node.n_keys = LEAF_MIN_CAPACITY;

            j = self.n_keys + 1;
            while j > pos + 1 {
                //self.pointers[j+1] = self.pointers[j];
                j -= 1;
            }

            // self.pointers[pos + 1] = Some(Rc::new(RefCell::new(new_node)));

            j = self.n_keys;
            while j > pos {
                self.keys[j+1] = self.keys[j];
                j -= 1;
            }

            self.keys[pos] = internal_node.keys[LEAF_MIN_CAPACITY];

            self.n_keys += 1;
        }
    }

}

#[derive(Debug, Clone)]
pub struct Btree {
    root: Link,
}

impl Btree {
    pub fn new() -> Self {
        Btree { root: None }
    }

    pub fn push(&mut self, value: u8) {
        match &self.root {
            None => {
                let mut new_node = Node::new(true, true);
                new_node.push_nonfull(value);
                self.root = Some(Rc::new(RefCell::new(new_node)))
            },
            Some(node) => {
                let mut internal_node = node.as_ref().borrow_mut();
                
                if internal_node.n_keys == NODE_CAPACITY {
                    let mut new_node = Node::new(true, false);
                    new_node.pointers[0] = Some(Rc::new(RefCell::new(internal_node.clone())));
                    new_node.split_node(1);
                    new_node.push_nonfull(value);
                } else {
                    internal_node.push_nonfull(value);
                }
            }
        }
    }

    pub fn find(&mut self, value: u8) -> bool {
        if let Some(node) = &self.root {
            let mut internal_node = node.borrow_mut();

            return internal_node.find(value)
        }

        return false;
    }

    pub fn print_tree(&self) {
//        let mut queue: VecDeque<Node> = VecDeque::new();
//        match &self.root {
//            None => { println!("Empty tree"); } ,
//            Some(node) => {
//                let mut internal_node = node.borrow_mut();
//
//                queue.push_back(internal_node);
//            }
//        }
//
//        while(!queue.is_empty()) {
//            let actual = queue.pop_front().unwrap();
//
//            let i:usize = 0;
//            while( i < actual.n_keys ) {
//                if let Some(mut node) = &self.pointers[i] {
//
//                    queue.push_back(node.borrow_mut());
//                }
//            }
//        }
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
}
