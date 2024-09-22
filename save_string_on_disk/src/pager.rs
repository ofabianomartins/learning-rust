use std::fs::File;
use std::io::{Read, Seek, SeekFrom, Write};

pub const BLOCK_SIZE: usize = 4096;

#[derive(Debug)]
pub struct Pager { 
    pub file: File
}

impl Pager {
    pub fn new(file: File) -> Self {
        Self { file }
    }

    pub fn read_disk(&mut self, pos: u64) -> [u8; BLOCK_SIZE] {
        let mut buffer = [0u8; BLOCK_SIZE];

        let _ = self.file.seek(SeekFrom::Start(pos * (BLOCK_SIZE as u64)));
        match self.file.read(&mut buffer) {
            Ok(_) => buffer,
            Err(err) => { 
                println!("{}", err);
                buffer
            }
        }
    }

    pub fn write_disk(&mut self, pos: u64, buffer: &[u8; BLOCK_SIZE]) {
        let _ = self.file.seek(SeekFrom::Start(pos * (BLOCK_SIZE as u64)));
        self.file.write(buffer).expect("buffer overflow");
    }

}
