use std::collections::VecDeque;

use std::{fmt, u8, usize};

use crate::pager::Pager;

pub const BLOCK_SIZE: usize = 4096;

pub const BTREE_ORDER: usize = 3;
pub const NODE_CAPACITY: usize = 2*BTREE_ORDER - 1;
pub const LEAF_MIN_CAPACITY: usize = BTREE_ORDER - 1;
pub const ARRAY_DEFAULT_VALUE: Link = None;

pub const PAYLOAD_BEGIN: usize = 4;

pub type Link = Option<Node>;
pub type BtreeValue = u8;

#[derive(Debug, Clone)]
pub struct Node { 
    pub disk_pos: u64,
    pub data: Vec<BtreeValue>

    // pub is_root: bool, first byte u8
    // pub is_leaf: bool, second byte u8
    // pub n_keys: usize, third byte u8
    // pub pos_in_disk: usize, fourth byte 8 

    // pub keys: [BtreeValue; NODE_CAPACITY], at fourfh byte and use u16
    // pub pointers: [Link; NODE_CAPACITY + 2],
}

impl Node {
    pub fn is_root(&self) -> bool {
        return self.data[0] == 1
    }

    pub fn set_is_root(&mut self, value: bool) {
        self.data[0] = if value { 1 } else { 0 }
    }

    pub fn is_leaf(&self) -> bool {
        return self.data[1] == 1
    }

    pub fn set_is_leaf(&mut self, value: bool) {
        self.data[1] = if value { 1 } else { 0 }
    }

    // Acces to number of keys
    pub fn set_n_keys(&mut self, value: u8) {
        self.data[2] = value;
    }

    pub fn add_n_keys(&mut self) {
        self.data[2] += 1;
    }

    pub fn sub_n_keys(&mut self) {
        self.data[2] -= 1;
    }

    pub fn n_keys(&self) -> usize {
        return usize::from(self.data[2]);
    }

    // Access to node number
    pub fn node_number(&self) -> BtreeValue {
        return self.data[3];
    }

    pub fn set_node_number(&mut self, value: BtreeValue) {
        self.data[3] = value;
    }

    // Access to keys
    pub fn key(&self, pos: usize) -> BtreeValue {
        return self.data[pos + PAYLOAD_BEGIN];
    }

    pub fn save_key(&mut self, pos: usize, value: BtreeValue) {
        self.data[pos + PAYLOAD_BEGIN] = value;
    }

    // access to children
    pub fn child(&self, pos: usize) -> BtreeValue {
        return self.data[pos + PAYLOAD_BEGIN + NODE_CAPACITY];
    }

    pub fn save_child(&mut self, pos: usize, child_pos: u8) {
        self.data[pos + PAYLOAD_BEGIN + NODE_CAPACITY] = child_pos;
    }

    pub fn is_full() -> bool {
        return false; 
    }

    ////////////////////////////////////////////////////////////////////////
}

impl fmt::Display for Node {
     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let _ = write!(f, "(");
        let mut i: usize = 0;

        while i < self.n_keys() {
            let _ = write!(f, "{}", self.key(i));
            i += 1;

            if i < self.n_keys() {
                let _ = write!(f, ", ");
            }
        }

        write!(f, "; len: {}, root: {}, leaf: {})", self.n_keys(), self.is_root(), self.is_leaf())
    }
}

impl Node {
    pub fn new(is_root: bool, is_leaf: bool, node_number: BtreeValue, disk_pos: u64) -> Self {
        let mut aux = Self { data: vec![0; BLOCK_SIZE], disk_pos };
        aux.set_is_leaf(is_leaf);
        aux.set_is_root(is_root);
        aux.set_node_number(node_number);

        aux
    }

    pub fn load(data: Vec<BtreeValue>, disk_pos: u64) -> Self {
        Self { data, disk_pos }
    }
}


impl Node {

    pub fn find_key(&self, value: BtreeValue) -> usize {
        let mut i: usize = 0;
        while i < self.n_keys() && value > self.key(i)  {
            i += 1;
        }

        return i;
    }

    pub fn push_node_key(&mut self, value: BtreeValue) {
        let mut j: usize = self.n_keys();
        println!("push_node_key {} - {}", self, value);
        while j > 0  && value < self.key(j - 1) {
            self.save_key(j, self.key(j - 1));
            j -= 1;
        }
        self.save_key(j, value);
        self.add_n_keys();
    }

/*
    pub fn remove_key(&mut self, i: usize) {
        let mut j: usize = i;
        while j < self.n_keys() - 1{
            self.save_key(j, self.key(j + 1));
            j += 1;
        }

        j = i + 1;
        while j < self.n_keys() {
            self.save_child(j, self.child(j + 1));
            j += 1;
        }
        self.sub_n_keys();
    }


    pub fn find_previous(&mut self, value: u8) -> u8 {
        if let Some(node) = &self.pointers[self.n_keys] {
            let mut internal_node = node.as_ref().borrow_mut();

            return internal_node.find_previous(value);
        }

        return value;
    }

    pub fn find_next(&mut self, value: u8) -> u8 {
        if let Some(node) = &self.pointers[0] {
            let mut internal_node = node.as_ref().borrow_mut();

            return internal_node.find_next(value);
        }

        return value;
    }

*/

    pub fn clean_pointers(&mut self) {
        let mut j = self.n_keys() + 1;
        while j < NODE_CAPACITY + 1 {
            self.save_child(j, 0);
            j += 1;
        }

        let mut j = self.n_keys();
        while j < NODE_CAPACITY {
            self.save_key(j, 0);
            j += 1;
        }
    }

    pub fn split_node(&mut self, pos: usize, disk_pos: u64, node_number: BtreeValue, pager: &mut Pager) {
        let new_node = &mut Node::new(false, self.is_leaf(), node_number, disk_pos);

        let mut internal_node = pager.read_disk(self.child(pos) as u64);

        new_node.set_is_leaf(internal_node.is_leaf());
        new_node.set_n_keys(LEAF_MIN_CAPACITY as u8);

        let mut j: usize = 0;
        while j < LEAF_MIN_CAPACITY {
            new_node.save_key(j, internal_node.key(j + BTREE_ORDER));
            j += 1;
        }

        if !internal_node.is_leaf() {
            j = 0;
            while j < BTREE_ORDER {
                new_node.save_child(j, internal_node.child(j + BTREE_ORDER));
                j += 1;
            }
        }

        internal_node.set_n_keys(LEAF_MIN_CAPACITY as u8);

        let mut j: usize = self.n_keys() + 1;
        while j > pos + 1 {
            self.save_child(j, self.child(j - 1));
            j -= 1;
        }

        self.save_child(pos + 1, new_node.disk_pos as u8);

        new_node.clean_pointers();
        pager.write_disk(new_node);

        j = self.n_keys();
        while j > pos {
            self.save_key(j, self.key(j - 1));
            j -= 1;
        }

        self.save_key(pos, internal_node.key(LEAF_MIN_CAPACITY));
        internal_node.clean_pointers();
        pager.write_disk(&internal_node);
        self.add_n_keys();
        self.clean_pointers();
        pager.write_disk(self);
    }

}


impl Node {
    pub fn find(&self, value: BtreeValue, _level: u8) -> bool {
        let i: usize = self.find_key(value);

        if i < self.n_keys() && value == self.key(i)  {
            return true
        } 

        if self.is_leaf() {
            return false
        }

        /*
        if let Some(node) = &self.pointers[i] {
            let mut internal_node = node.as_ref().borrow_mut();


            return internal_node.find(value, level + 1)
        }
        */

        return false;
    }


    pub fn push_nonfull(&mut self, value: BtreeValue, pager: &mut Pager) {
        if self.is_leaf() {
            println!("push_nonfull {}", value);
            self.push_node_key(value);
        } else {
            let mut j: usize = self.n_keys();
            while j > 0  && value < self.key(j - 1) {
                j -= 1;
            }
            // j += 1;

            let mut internal_node = pager.read_disk(self.child(j) as u64);

/*
            if internal_node.n_keys() == NODE_CAPACITY {
                self.split_node(j);
                if value > self.key(j) {
                    j += 1;
                }
            }
*/

            internal_node.push_nonfull(value, pager);
        }
    }

    pub fn print_tree(&self) {
        let mut queue: VecDeque<Link> = VecDeque::new();

        print!("{}|", self);
        /*
        let mut j = 0;
        while j <= self.n_keys() {
            if let Some(new_pointer_node) = &self.pointers[j] {
                queue.push_back(Some(new_pointer_node.clone())); 
            } 
            j += 1;
        }
        */

        while !queue.is_empty() {
            if let Some(Some(actual_node)) = queue.pop_front() {

                /*
                let mut j = 0;
                while j <= actual_node.n_keys() {
                    if let Some(new_pointer_node) = &actual_node.pointers[j] {
                        queue.push_back(Some(new_pointer_node.clone())); 
                    } 
                    j += 1;
                }
                */
                print!("{}|", actual_node);
            } 
        }

        println!("end");
    }

    pub fn is_correct(&self) -> bool {
        let mut j = 0;

        if !self.is_leaf() {
            j = 0;
            while j < self.n_keys() + 1{
                /* 
                if let Some(node) = &self.pointers[j] {
                    let actual_node = node.as_ref().borrow_mut();

                    if !actual_node.is_correct() {
                        println!("Sub-árvore com erros {}", actual_node);
                        return false;
                    }
                }
                */
                j += 1;
            }

            j = 0;
            while j < self.n_keys() {
                /* 
                if let Some(node) = &self.pointers[j] {
                    let mut actual_node = node.as_ref().borrow_mut();

                    let left_limit = actual_node.find_previous(self.key(j));

                    if left_limit > self.key(j)  {
                        println!("Sub-árvore valores maiores: {} - {};", self.keys[j], left_limit);
                        actual_node.print_tree();
                        return false;
                    }
                }

                if let Some(node) = &self.pointers[j+1] {
                    let mut actual_node = node.as_ref().borrow_mut();

                    let right_limit = actual_node.find_next(self.key(j));

                    if right_limit < self.key(j)  {
                        println!("Sub-árvore valores menores: {} - {};", self.key(j), right_limit);
                        actual_node.print_tree();
                        return false;
                    }
                }
                */
                j += 1;
            }
        }

        if self.is_root() && self.n_keys() == 0 {
            println!("Raiz com zero elementos {}", self);
            return false;
        }

        if self.n_keys() > 1 {
            while j < self.n_keys() - 1 {
                if !(self.key(j) < self.key(j+1)) {
                    println!("Nó desordenado: {}", self);
                    return false;
                }
                j += 1;
            }
        }

        if !self.is_root() {
            if self.n_keys() < LEAF_MIN_CAPACITY {
                println!("Nó sem o mínimo de elementos: {}", self);
                return false;
            }
        }

        if self.n_keys() > NODE_CAPACITY {
            println!("Nó com mais elemento que o necessário: {}", self);
            return false;
        }

        return true;
    }
}
