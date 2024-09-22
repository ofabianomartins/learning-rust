use std::usize;

use crate::pager::Pager;
use crate::pager::BLOCK_SIZE;


#[derive(Debug)]
pub struct Buffer {
    pub total_of_pages: u64,
    pub number_of_page: u64,
    pub buffer: [u8; BLOCK_SIZE],
    pub buffer_length: u32,
    pub pager: Pager,
}

impl Buffer {
    pub fn new(pager: Pager) -> Self {
        Self { 
            buffer: [0u8; BLOCK_SIZE], 
            total_of_pages: 0, 
            number_of_page: 0, 
            buffer_length: 4,
            pager 
        }
    }

    pub fn read_from_file(&mut self) {
        self.buffer = self.pager.read_disk(0);

        let mut buf = [0u8; 4];
        buf[..4].copy_from_slice(&self.buffer[0..4]);
        let number_of_words = u32::from_be_bytes(buf);

        let mut word_pos: u32 = 4;

        for _iter in 0..number_of_words {
            let mut buf = [0u8; 4];
            buf[..4].copy_from_slice(&self.buffer[(word_pos as usize)..((word_pos + 4u32) as usize)]);
            let index_of_word = u32::from_be_bytes(buf);

            word_pos += 4u32 + index_of_word;
        }

        self.buffer_length = word_pos;
    }

    pub fn save_on_buffer(&mut self, data: &str) {
        let len: u32 = data.len() as u32;
        let index = self.buffer_length;

        self.buffer[(index as usize)..((index + 4) as usize)].copy_from_slice(&len.to_be_bytes());
        self.buffer[((index + 4) as usize)..((index + len + 4) as usize)].copy_from_slice(data.as_bytes());

        self.buffer_length += 4u32 + len;

        let mut buf = [0u8; 4];
        buf[..4].copy_from_slice(&self.buffer[0..4]);
        let number_of_words = u32::from_be_bytes(buf);
        self.buffer[0..4].copy_from_slice(&(number_of_words + 1).to_be_bytes());

        self.pager.write_disk(0, &self.buffer);
    }

    pub fn read_from_buffer(&mut self, pos: u32) -> &str {
        let mut buf = [0u8; 4];
        buf[..4].copy_from_slice(&self.buffer[0..4]);
        let number_of_words = u32::from_be_bytes(buf);

        let mut word_pos: u32 = 4;

        for iter in 0..number_of_words {
            let mut buf = [0u8; 4];
            buf[..4].copy_from_slice(&self.buffer[(word_pos as usize)..((word_pos + 4) as usize)]);
            let index_of_word = u32::from_be_bytes(buf);

            word_pos += 4u32;
            let s = std::str::from_utf8(
                &self.buffer[(word_pos as usize)..((word_pos + index_of_word) as usize)]
            ).unwrap();

            if iter == pos {
                return s;
            } else {
                word_pos += index_of_word;
            }
        }

        return &"Position not found!";
    }
}


