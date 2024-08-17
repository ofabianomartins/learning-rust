use std::collections::VecDeque;
use std::{cell::RefCell, rc::Rc};

use std::{fmt, u8};

pub const BTREE_ORDER: usize = 3;
pub const NODE_CAPACITY: usize = 2*BTREE_ORDER - 1;
pub const LEAF_MIN_CAPACITY: usize = BTREE_ORDER - 1;
pub const ARRAY_DEFAULT_VALUE: Link = None;

pub type Link = Option<Rc<RefCell<Node>>>;
pub type BtreeValue = u8;

#[derive(Debug, Clone)]
pub struct Node { 
    pub is_root: bool,
    pub is_leaf: bool,

    pub n_keys: usize,
    pub keys: [BtreeValue; NODE_CAPACITY],
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

        write!(f, "; len: {}, leaf: {})", self.n_keys, self.is_leaf)
    }
}

impl Node {
    pub fn find_key(&mut self, value: BtreeValue) -> usize {
        let mut i: usize = 0;
        while i < self.n_keys && value > self.keys[i]  {
            i += 1;
        }

        return i;
    }

    pub fn remove_key(&mut self, i: usize) {
        let mut j: usize = i;
        while j < self.n_keys - 1{
            self.keys[j] = self.keys[j + 1];
            j += 1;
        }

        j = i + 1;
        while j < self.n_keys {
            if let Some(pointer_new_node) = &self.pointers[j + 1] {
                self.pointers[j] = Some(pointer_new_node.clone());
            }
            j += 1;
        }
        self.n_keys -= 1;
    }


    pub fn find_previous(&mut self, value: BtreeValue, level: u8) -> u8 {
        let i: usize = self.find_key(value) ;

        if i == self.n_keys && self.is_leaf {
            let aux = self.keys[self.n_keys - 1];
            self.remove(aux, level + 1);
            return aux;
        } 

        if let Some(node) = &self.pointers[i] {
            let mut internal_node = node.as_ref().borrow_mut();

            return internal_node.find_previous(value, level + 1);
        }

        return 0;
    }

    pub fn find_next(&mut self, value: BtreeValue, level: u8) -> u8 {
        let i: usize = self.find_key(value) ;

        if i == 0 && self.is_leaf {
            let aux = self.keys[0];
            self.remove(aux, level);
            return aux;
        } 

        if let Some(node) = &self.pointers[i] {
            let mut internal_node = node.as_ref().borrow_mut();

            return internal_node.find_next(value, level + 1);
        }

        return 0;
    }

    pub fn merge_child_node(&mut self, left: usize, right: usize, key: usize) {
        println!("Merge coluna {} com {} e chave {}", left, right, self.keys[key]);


        if let Some(node) = &self.pointers[left] {
            let mut child = node.as_ref().borrow_mut();

            if let Some(node) = &self.pointers[right] {
                let right_child = node.as_ref().borrow_mut();
                child.push_nonfull(self.keys[key]);

                let mut j = 0;
                while j < right_child.n_keys {
                    let actual_index = j + child.n_keys;
                    child.keys[actual_index] = right_child.keys[j];
                    j += 1;
                } 

                j = 0;
                while j < right_child.n_keys + 1 {
                    if let Some(pointer_new_node) = &right_child.pointers[j + 1] {
                        let actual_index = j + child.n_keys;
                        child.pointers[actual_index] = Some(pointer_new_node.clone());
                    }
                    j += 1;
                }

                // join n_keys
                child.n_keys += right_child.n_keys;
                // child.remove(self.keys[key], 0);
            }
        }

        self.remove_key(key);
    }

    pub fn move_left_child_element(&mut self, child_key: usize, key: usize) {
        println!("Move chave {} de coluna {} para {} ", self.keys[key], child_key, child_key + 1);

        let parent_value = self.keys[child_key];

        if let Some(node) = &self.pointers[child_key] {
            let mut child = node.as_ref().borrow_mut();

            let new_value = child.find_previous(parent_value, 0);

            self.keys[child_key] = new_value;

            if let Some(node) = &self.pointers[child_key + 1] {
                let mut right_child = node.as_ref().borrow_mut();

                let mut j: usize = right_child.n_keys;
                while j > 0{
                    right_child.keys[j] = right_child.keys[j - 1];
                    j += 1;
                }

                j = 0;
                while j < self.n_keys + 1 {
                    if let Some(pointer_new_node) = &right_child.pointers[j] {
                        right_child.pointers[j + 1] = Some(pointer_new_node.clone());
                    }
                    j += 1;
                }

                right_child.keys[0] =  parent_value;

                if let Some(pointer_new_node) = &child.pointers[key] {
                    right_child.pointers[0] = Some(pointer_new_node.clone());
                }
                right_child.n_keys += 1;

                child.n_keys -= 1;
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
                new_node.keys[j] = internal_node.keys[j + BTREE_ORDER];
                j += 1;
            }

            if !internal_node.is_leaf {
                j = 0;
                while j < BTREE_ORDER {
                    if let Some(pointer_new_node) = &internal_node.pointers[j + BTREE_ORDER] {
                        new_node.pointers[j] = Some(pointer_new_node.clone());
                    }
                    j += 1;
                }
            }

            internal_node.n_keys = LEAF_MIN_CAPACITY;
        }

        let mut j: usize = self.n_keys + 1;
        while j > pos + 1 {
            if let Some(pointer_new_node) = &self.pointers[j - 1] {
                self.pointers[j] = Some(pointer_new_node.clone());
            }
            j -= 1;
        }

        self.pointers[pos + 1] = Some(Rc::new(RefCell::new(new_node.clone())));

        if let Some(node) = &self.pointers[pos] {
            let internal_node = node.as_ref().borrow_mut();

            j = self.n_keys;
            while j > pos {
                self.keys[j] = self.keys[j - 1];
                j -= 1;
            }

            self.keys[pos] = internal_node.keys[LEAF_MIN_CAPACITY].clone();
        }
        self.n_keys += 1;
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

    pub fn find(&mut self, value: BtreeValue, level: u8) -> bool {
        let i: usize = self.find_key(value);

        println!("find {} level {} - {}", value, level, self);

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

    pub fn push_nonfull(&mut self, value: BtreeValue) {
        // println!("push_nonfull {}", value);
        if self.is_leaf {
            let mut j: usize = self.n_keys;
            while j > 0  && value < self.keys[j - 1] {
                self.keys[j] = self.keys[j - 1];
                j -= 1;
            }
            self.keys[j] = value;
            self.n_keys += 1;
        } else {
            let mut j: usize = self.n_keys;
            while j > 0  && value < self.keys[j - 1] {
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

    pub fn remove(&mut self, value: BtreeValue, level: u8) -> bool {
        let i: usize = self.find_key(value);
        println!("remove {} level {} key {} n_keys {} leaf {}", value, level, i, self.n_keys, self.is_leaf);

        if value == self.keys[i] && self.is_leaf {
            println!("Case 1");
            self.remove_key(i);
            return true;
        } 

        if value == self.keys[i] && !self.is_leaf {
            if let Some(node) = &self.pointers[i] {
                let mut left_child = node.as_ref().borrow_mut();

                if left_child.n_keys >= BTREE_ORDER {
                    println!("Case 2-a");
                    let new_value = left_child.find_previous(value, level + 1);

                    self.keys[i] = new_value;
                    return true;
                } 
            }

            if let Some(node) = &self.pointers[i + 1] {
                let mut right_child = node.as_ref().borrow_mut();

                if right_child.n_keys >= BTREE_ORDER {
                    println!("Case 2-b");
                    let new_value = right_child.find_next(value, level + 1);
                    
                    self.keys[i] = new_value;
                    return true;
                } 
            }

            println!("Case 2-c");
            self.merge_child_node(i, i+1, i);

            return true;
        } 

        if value != self.keys[i] && !self.is_leaf {
            let mut left_child_min: bool = false;
            let mut right_child_min: bool = false;

            if let Some(node) = &self.pointers[i] {
                let mut child = node.as_ref().borrow_mut();

                println!("Descendo!");
                child.remove(value, level + 1);

                if child.n_keys <= LEAF_MIN_CAPACITY {
                    if i < self.n_keys {
                        if let Some(node) = &self.pointers[i + 1] {
                            let mut right_child = node.as_ref().borrow_mut();

                            if right_child.n_keys > LEAF_MIN_CAPACITY {
                                println!("Case 3-a right");

                                let new_value = right_child.find_next(value, level + 1);

                                child.push_nonfull(self.keys[i]);
                                self.keys[i] = new_value;

                                child.remove(value, level + 1);
                                right_child.remove(new_value, level + 1);
                                return true;
                            } else {
                                right_child_min = true;
                            }
                        } 
                    } 

                    if i > 0 {
                        if let Some(node) = &self.pointers[i - 1] {
                            let mut left_child = node.as_ref().borrow_mut();

                            if left_child.n_keys > LEAF_MIN_CAPACITY {
                                println!("Case 3-a left");

                                // self.move_child_element(i-1, i, i - 1);


                                println!("new_values {} - {}", new_value, self.keys[i - 1]);
                                child.push_nonfull(self.keys[i - 1]);

                                child.remove(value, level + 1);
                                left_child.remove(new_value, level + 1);
                                return true;
                            } else {
                                left_child_min = true;
                            }
                        } 
                    }
                }
            }

            if left_child_min || right_child_min {
                if left_child_min {
                    println!("Caso 3b left!");
                    if i == self.n_keys {
                        self.merge_child_node(i - 1, i, i - 1);

                        if let Some(node) = &self.pointers[i - 1] {
                            let mut child = node.as_ref().borrow_mut();

                            child.remove(value, level + 1);
                        }
                    } else {
                        self.merge_child_node(i - 1, i, i - 1);

                        if let Some(node) = &self.pointers[i] {
                            let mut child = node.as_ref().borrow_mut();

                            child.remove(value, level + 1);
                        }
                    }
                    self.print_tree();

                }

                if right_child_min && !left_child_min {
                    println!("Caso 3b right!");
                    if i == self.n_keys {
                        self.merge_child_node(i - 1, i, i - 1);

                        if let Some(node) = &self.pointers[i - 1] {
                            let mut child = node.as_ref().borrow_mut();

                            child.remove(value, level + 1);
                        }
                    } else {
                        self.merge_child_node(i - 1, i, i - 1);

                        if let Some(node) = &self.pointers[i] {
                            let mut child = node.as_ref().borrow_mut();

                            child.remove(value, level + 1);
                        }
                    }
                }
            }
        }

        return false;
    }

    pub fn print_tree(&self) {
        let mut queue: VecDeque<Link> = VecDeque::new();

        print!("{}|", self);
        let mut j = 0;
        while j <= self.n_keys {
            if let Some(new_pointer_node) = &self.pointers[j] {
                queue.push_back(Some(new_pointer_node.clone())); 
            } 
            j += 1;
        }

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

    pub fn is_correct(&self) -> bool {
        let mut j = 0;

        if !self.is_leaf {
            j = 0;
            while j < self.n_keys + 1{
                if let Some(node) = &self.pointers[j] {
                    let actual_node = node.as_ref().borrow_mut();

                    if !actual_node.is_correct() {
                        println!("Sub-árvore com erros {}", actual_node);
                        return false;
                    }
                }
                j += 1;
            }
        }

        if self.n_keys > 1 {
            while j < self.n_keys - 1 {
                if !(self.keys[j] < self.keys[j+1]) {
                    println!("Nó desordenado: {}", self);
                    return false;
                }
                j += 1;
            }
        }

        if !self.is_root {
            if self.n_keys < LEAF_MIN_CAPACITY {
                println!("Nó sem o mínimo de elementos: {}", self);
                return false;
            }
        }

        if self.n_keys > NODE_CAPACITY {
            println!("Nó com mais elemento que o necessário: {}", self);
            return false;
        }

        return true;
    }
}
