use crate::crc::CRC;

const TABLE_SIZE: u16 = 256;
const BYTE_SIZE: u16 = 8;

#[derive(Debug, Clone, Copy)]
pub struct CRC16 {
    table: [u16; TABLE_SIZE as usize],
}

impl CRC16 {
    pub fn new() -> Self {
        Self {
            table: [0; TABLE_SIZE as usize],
        }
    }
}

impl CRC for CRC16 {
    type Output = u16;

    fn make_table(&mut self) {
        for i in 0..TABLE_SIZE {
            let mut c = i;
            for _ in 0..BYTE_SIZE {
                c = if (c & 1) == 1 {
                    0xA001 ^ (c >> 1)
                } else {
                    c >> 1
                };
            }
            self.table[i as usize] = c;
        }
    }

    fn calculate(&self, buf: &[u8], len: usize) -> Self::Output {
        let mut c: u16 = 0;
        for element in buf.iter().take(len) {
            let index = (c ^ *element as u16) & 0xFF;
            c = self.table[index as usize] as u16 ^ (c >> 8);
        }
        c
    }
}

impl Default for CRC16 {
    fn default() -> Self {
        Self::new()
    }
}
