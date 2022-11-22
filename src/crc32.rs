use crate::crc::CRC;

const TABLE_SIZE: u32 = 256;
const BYTE_SIZE: u32 = 8;

#[derive(Debug, Clone, Copy)]
pub struct CRC32 {
    pub table: [u32; TABLE_SIZE as usize],
}

impl CRC32 {
    pub fn new() -> Self {
        Self {
            table: [0; TABLE_SIZE as usize],
        }
    }
}

impl CRC for CRC32 {
    type Output = u32;

    fn make_table(&mut self) {
        for i in 0..TABLE_SIZE {
            let mut c = i;
            for _ in 0..BYTE_SIZE {
                c = if (c & 1) == 1 {
                    0xEDB88320 ^ (c >> 1)
                } else {
                    c >> 1
                };
            }
            self.table[i as usize] = c;
        }
    }

    fn calculate(&self, buf: &[u8], len: usize) -> Self::Output {
        let mut c: u32 = 0xFFFFFFFF;
        for element in buf.iter().take(len) {
            let index = (c ^ *element as u32) & 0xFF;
            c = self.table[index as usize] as u32 ^ (c >> 8);
        }
        c ^ 0xFFFFFFFF
    }
}

impl Default for CRC32 {
    fn default() -> Self {
        let mut default = Self::new();
        default.make_table();
        default
    }
}
