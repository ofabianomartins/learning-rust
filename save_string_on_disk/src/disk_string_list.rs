use crate::buffer::Buffer;

#[derive(Debug)]
pub struct DiskStringList {
    pub buffer: Buffer
}

impl DiskStringList {
    pub fn new(buffer: Buffer) -> Self {
        Self { buffer }
    }

    pub fn save_string(&mut self, data: &str) {
        self.buffer.save_on_buffer(data);
    }

    pub fn read_string(&mut self, pos: u32) -> &str {
        return self.buffer.read_from_buffer(pos);
    }

    pub fn load_file(&mut self) {
        self.buffer.read_from_file();
    }
}


