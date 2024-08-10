use std::{cell::RefCell, rc::Rc};

use std::fmt;

type Link = Option<Rc<RefCell<Node>>>;

pub const BTREE_ORDER: usize = 3;
pub const ARRAY_DEFAULT_VALUE: Link = None;

#[derive(Debug, Clone)]
pub enum NodeType {
    Internal,
    Leaf, 
    Unexpected
}

#[derive(Debug, Clone)]
pub struct Node { 
    pub node_type: NodeType,
    pub is_root: bool,
    pub is_leaf: bool,

    pub n_keys: usize,
    pub keys: [u8; BTREE_ORDER - 1],
    pub pointers: [Link; BTREE_ORDER - 1],
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
            node_type: NodeType::Leaf, 

            n_keys: 0,
            keys: [0; BTREE_ORDER - 1],
            pointers: [ARRAY_DEFAULT_VALUE; BTREE_ORDER - 1]
        }
    }

    pub fn new_with_value(is_root: bool, is_leaf: bool, value: u8) -> Self {
        let mut obj = Self { 
            is_root, 
            is_leaf,
            node_type: NodeType::Leaf, 

            n_keys: 0,
            keys: [0; BTREE_ORDER - 1],
            pointers: [ARRAY_DEFAULT_VALUE; BTREE_ORDER - 1]
        };

        obj.keys[0] = value;
        obj.n_keys = 1;

        obj
    }

    pub fn push(&mut self, value: u8) {
        let mut i: usize = 0;
        while i < self.n_keys && value > self.keys[i]  {
            i += 1;
        }

        println!("value: {}; n_keys: {}; indice {}; {}", value, self.n_keys, i, BTREE_ORDER - 1 );

        if (self.is_root || self.is_leaf) && (i < (BTREE_ORDER - 1)) {
            println!("Inserting in leaf or root");
            self.keys[i] = value;
            self.n_keys += 1;
            return
        }

        if self.is_leaf && i == BTREE_ORDER - 1 {
            println!("Spliting in leaf");

            // let mut new_node = Node::new(false, true, value);



        } 

        return 
        if !self.is_leaf {
            match &self.pointers[i] {
                None => { 
                    self.pointers[i] = Some(Rc::new(RefCell::new(Node::new(false, true))))
                },
                Some(node) => {
                    let mut internal_node = node.borrow_mut();

                    return internal_node.push(value)
                }
            }
        } 
    }

    pub fn push_non_full(&mut self) {

    }

    pub fn split_node(&mut self, pos: usize, order: u8) {
        match &self.pointers[pos] {
            None => { },
            Some(node) => {
                let mut internal_node = node.borrow_mut();

                let mut new_node = &mut Node::new(false, true);
                new_node.n_keys = BTREE_ORDER - 1;

                Node::_deslocate_keys_up(new_node, &internal_node, 0, BTREE_ORDER -1, 0, BTREE_ORDER );

                if internal_node.is_leaf {
                    Node::_deslocate_keys_up(new_node, &internal_node, 0, BTREE_ORDER, 0, BTREE_ORDER);
                }
                internal_node.n_keys = BTREE_ORDER - 1;

                // self.pointers[pos+1] = Some(Rc::new(RefCell::new((*new_node).clone())));
                
                // Node::_deslocate_keys_down(&self, self, self.n_keys - 1, pos - 1, 1, 0);

                self.keys[pos] = internal_node.keys[BTREE_ORDER - 1];
                self.n_keys += 1;
            }
        }
    }

    pub fn _deslocate_keys_up(to: &mut Node, from: &Node, beg: usize, end: usize, padding_to: usize, padding_from: usize) {
        let mut j: usize = beg;
        while j < end {
            to.keys[j + padding_to] = from.keys[j + padding_from];
            j += 1;
        }
    }

    pub fn _deslocate_keys_down(to: &mut Node, from: &Node, beg: usize, end: usize, padding_to: usize, padding_from: usize) {
        let mut j: usize = beg;
        while j > end {
            to.keys[j + padding_to] = from.keys[j + padding_from];
            j -= 1;
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

        match &self.pointers[i] {
            None => false,
            Some(node) => {
                let mut internal_node = node.borrow_mut();

                return internal_node.find(value)
            }
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
                self.root = Some(Rc::new(RefCell::new(Node::new_with_value(true, true, value))))
            },
            Some(node) => {
                let mut internal_node = node.borrow_mut();

                internal_node.push(value);
            }
        }
    }

    pub fn find(&mut self, value: u8) -> bool {
        match &self.root {
            None => false,
            Some(node) => {
                let mut internal_node = node.borrow_mut();

                return internal_node.find(value)
            }
        }
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
    fn insert_1() {
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
}
