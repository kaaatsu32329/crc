pub trait CRC {
    type Output;

    fn make_table(&mut self);
    fn calculate(&self, buf: &[u8], len: usize) -> Self::Output;
}
