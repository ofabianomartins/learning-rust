use std::fs::File;
use std::io::{Read, Seek, SeekFrom, Write};
// use std::mem::size_of;

use crate::node::Node;
use crate::node::BLOCK_SIZE;

#[derive(Debug)]
pub struct Pager { 
    pub file: File
}

impl Pager {
    pub fn new(file: File) -> Self {
        Self { file }
    }

    pub fn calculate_offset(disk: u64) -> u64{
        // calculate the nº of bytes a node has
        // let size_of_node: u64 = (size_of(u8) * 4) + (size_of(element) * order-1) + (sizeof(int) * order);

        // return size_of_btNode * disk;    // calculate the position of the node in the file
        return (BLOCK_SIZE as u64) * disk;   
    }

    pub fn read_disk(&mut self, pos: u64) -> Node {
        let mut buffer = vec![0; BLOCK_SIZE];

        self.file.seek(SeekFrom::Start(pos));
        match self.file.read(&mut buffer) {
            Ok(_) => Node::load(buffer, pos),
            Err(err) => { 
                println!("{}", err);
                Node::load(buffer, pos)
            }
        }
    }

    pub fn write_disk(&mut self, node: &Node) {
        self.file.seek(SeekFrom::Start(node.disk_pos));
        self.file.write(&node.data).expect("buffer overflow");
    }

}
