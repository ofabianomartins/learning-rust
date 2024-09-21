
use crate::pager::Pager;

#[derive(Debug)]
pub struct DiskStringList {
    pub size: u64,
    pub pager: Pager
}

impl DiskStringList {
    pub fn new(pager: Pager) -> Self {
        Self { size: 0, pager }
    }

    pub fn save_string(&mut self, data: &str) {
        let mut buffer = [0u8; 40];
        let input_bytes = data.as_bytes(); // Convert the string to bytes

        // Copy the string bytes into the buffer, truncating if needed
        let length = std::cmp::min(input_bytes.len(), 40);
        buffer[..length].copy_from_slice(&input_bytes[..length]);

        self.pager.write_disk(self.size, &buffer);
        self.size += 1;
    }

    pub fn read_string(&mut self, pos: u64) -> [u8; 40]{
        return self.pager.read_disk(pos);
    }
}


