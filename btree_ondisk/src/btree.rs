use std::collections::VecDeque;
use std::fs::File;
use std::fs::OpenOptions;
use std::usize;

use crate::node::Node;
use crate::node::NODE_CAPACITY;
use crate::pager::Pager;

#[derive(Debug)]
pub struct Btree {
    root: Option<Node>,
    pager: Pager,
    node_count: u64
}

impl Btree {
    pub fn new(pager: Pager) -> Self {
        Btree { root: None, pager, node_count: 0 }
    }

    pub fn add_node_count(&mut self) {
        self.node_count += 1;
    }

    pub fn find(&mut self, value: u8) -> bool {
        if let Some(node) = &self.root {
            return node.find(value, 0)
        }

        return false;
    }

    pub fn push(&mut self, value: u8) {
        println!("Inserting {}", value);

        if let Some(internal_node) = &mut self.root {
            if internal_node.n_keys() == NODE_CAPACITY {
                let mut new_node = Node::new(true, false, self.node_count as u8, Pager::calculate_offset(self.node_count));
                new_node.save_child(0, internal_node.disk_pos as u8);
                internal_node.set_is_root(false);
                self.pager.write_disk(internal_node);
                self.add_node_count();
                new_node.split_node(0, Pager::calculate_offset(self.node_count), self.node_count as u8, &mut self.pager);
                self.add_node_count();
                new_node.push_nonfull(value, &mut self.pager);
                self.root = Some(new_node);
            } else {
                internal_node.push_nonfull(value, &mut self.pager);
                self.pager.write_disk(&internal_node);
            }
        } else {
            let mut new_node = Node::new(true, true, 0, 0);
            new_node.push_nonfull(value, &mut self.pager);
            self.root = Some(new_node.clone());
            self.add_node_count();
            self.pager.write_disk(&new_node);
        }
    }

    pub fn print_tree(&mut self) {
        let mut node_check: Vec<bool> = vec![false; self.node_count as usize];
        let mut queue: VecDeque<Option<Node>> = VecDeque::new();

        queue.push_back(self.root.clone());
        if let Some(node) = &self.root {
            node_check[node.node_number() as usize] = true;
        }
        while !queue.is_empty() {
            if let Some(Some(actual_node)) = &queue.pop_front() {
                let mut j = 0;
                while j <= actual_node.n_keys() {
                    let internal_node = self.pager.read_disk(actual_node.child(j) as u64);
                    if !node_check[internal_node.node_number() as usize] {
                        node_check[internal_node.node_number() as usize] = true;
                        queue.push_back(Some(internal_node));
                    }
                    j += 1;
                }
                print!("{}|", actual_node);
            } 
        }

        println!("end");
    }

    pub fn is_correct(&self) -> bool {
        if let Some(node) = &self.root {
            return node.is_correct()
        }

        return true;
    }

    pub fn is_empty(&self) -> bool {
        if let Some(node) = &self.root {
            return node.n_keys() == 0;
        }

        return true;
    }
}

#[cfg(test)]
pub mod tests {
    use super::*;

    #[test]
    fn insert_1_to_10() {
        let file = OpenOptions::new()
            .write(true)    
            .read(true)
            .truncate(true)
            .open("foo_11.txt")
            .unwrap();

        let pager = Pager::new(file);


        let mut queue = Btree::new(pager);

        let mut j: u8 = 1;
        while j < 11 {
            queue.push(j);
            queue.print_tree();
            assert!(queue.is_correct());
            j += 1;
        }

        j = 1;
        while j < 11 {
            assert_eq!(queue.find(j), true);
            j += 1;
        }
    }
}
