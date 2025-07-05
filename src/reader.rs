pub struct ByteReader<'a> {
    data: &'a [u8],
    offset: usize,
}

impl<'a> ByteReader<'a> {
    pub fn new(data: &'a [u8]) -> Self {
        ByteReader { data, offset: 0 }
    }

    pub fn u8(&mut self) -> u8 {
        let val = self.data[self.offset];
        self.offset += 1;
        val
    }

    pub fn u16(&mut self) -> u16 {
        let val = u16::from_le_bytes(self.data[self.offset..self.offset + 2].try_into().unwrap());
        self.offset += 2;
        val
    }

    pub fn u32(&mut self) -> u32 {
        let val = u32::from_le_bytes(self.data[self.offset..self.offset + 4].try_into().unwrap());
        self.offset += 4;
        val
    }

    pub fn u64(&mut self) -> u64 {
        let val = u64::from_le_bytes(self.data[self.offset..self.offset + 8].try_into().unwrap());
        self.offset += 8;
        val
    }

    pub fn bool(&mut self) -> bool {
        self.u8() != 0
    }

    pub fn bytes(&mut self, count: usize) -> &'a [u8] {
        let slice = &self.data[self.offset..self.offset + count];
        self.offset += count;
        slice
    }

    pub fn offset(&self) -> usize {
        self.offset
    }

    pub fn seek(&mut self, offset: usize) {
        self.offset = offset;
    }
}
