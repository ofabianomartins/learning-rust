use std::{cell::RefCell, rc::Rc};

use std::fmt;

pub const BTREE_ORDER: usize = 2;
pub const NODE_CAPACITY: usize = 2*BTREE_ORDER - 1;
pub const LEAF_MIN_CAPACITY: usize = BTREE_ORDER - 1;
pub const ARRAY_DEFAULT_VALUE: Link = None;

pub type Link = Option<Rc<RefCell<Node>>>;

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
        let _ = write!(f, "(");
        let mut i: usize = 0;

        while i < self.n_keys {
            let _ = write!(f, "{}", self.keys[i]);
            i += 1;

            if i < self.n_keys {
                let _ = write!(f, ", ");
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

    pub fn find(&mut self, value: u8, level: u8) -> bool {
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

            return internal_node.find(value, level + 1)
        }

        return false;
    }

    pub fn remove(&mut self, value: u8, level: u8) -> bool {
        return false;
    }

    pub fn push_nonfull(&mut self, value: u8) {
        let mut j: usize = self.n_keys;

        if self.is_leaf {
            while value < self.keys[j] {
                self.keys[j+1] = self.keys[j];
                j -= 1;
            }
            self.keys[j] = value;
            self.n_keys += 1;
        } else {
            while value < self.keys[j] {
                j -= 1;
            }
            // j += 1;

            let mut n_keys: usize = 0;
            if let Some(node) = &self.pointers[j] {
                let internal_node = node.as_ref().borrow_mut();

                n_keys = internal_node.n_keys;
            }


            if n_keys == NODE_CAPACITY {
                self.split_node(j);
                if value > self.keys[j] {
                    j += 1;
                }
            }

            if let Some(node) = &self.pointers[j] {
                let mut internal_node = node.as_ref().borrow_mut();

                internal_node.push_nonfull(value);
            }
        }
    }

    pub fn split_node(&mut self, pos: usize) {
        let new_node = &mut Node::new(false, true);

        if let Some(node) = &self.pointers[pos] {
            let mut internal_node = node.as_ref().borrow_mut();

            new_node.is_leaf = internal_node.is_leaf;
            new_node.n_keys = LEAF_MIN_CAPACITY;

            let mut j: usize = 0;
            while j < LEAF_MIN_CAPACITY {
                new_node.keys[j] = internal_node.keys[j + LEAF_MIN_CAPACITY + 1];
                j += 1;
            }

            if !internal_node.is_leaf {
                j = 0;
                while j < BTREE_ORDER {
                    if let Some(pointer_new_node) = &internal_node.pointers[j + LEAF_MIN_CAPACITY + 1] {
                        new_node.pointers[j] = Some(pointer_new_node.clone());
                    }
                    j += 1;
                }
            }

            internal_node.n_keys = LEAF_MIN_CAPACITY;
        }

        let mut j: usize = self.n_keys + 1;
        while j > pos + 1 {
            if let Some(pointer_new_node) = &self.pointers[j] {
                self.pointers[j + 1] = Some(pointer_new_node.clone());
            }
            j -= 1;
        }

        self.pointers[pos + 1] = Some(Rc::new(RefCell::new(new_node.clone())));

        if let Some(node) = &self.pointers[pos] {
            let internal_node = node.as_ref().borrow_mut();

            j = self.n_keys;
            while j > pos {
                self.keys[j+1] = self.keys[j];
                j -= 1;
            }

            self.keys[pos] = internal_node.keys[LEAF_MIN_CAPACITY].clone();
        }
        self.n_keys += 1;
    }

}
