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

    pub fn i8(&mut self) -> i8 {
        let val = self.data[self.offset] as i8;
        self.offset += 1;
        val
    }

    pub fn i16(&mut self) -> i16 {
        let val = i16::from_le_bytes(self.data[self.offset..self.offset + 2].try_into().unwrap());
        self.offset += 2;
        val
    }

    pub fn i32(&mut self) -> i32 {
        let val = i32::from_le_bytes(self.data[self.offset..self.offset + 4].try_into().unwrap());
        self.offset += 4;
        val
    }

    pub fn i64(&mut self) -> i64 {
        let val = i64::from_le_bytes(self.data[self.offset..self.offset + 8].try_into().unwrap());
        self.offset += 8;
        val
    }

    pub fn bit(&mut self) -> bool {
        let byte_index = self.offset / 8;
        let bit_index = self.offset % 8;
        let bit = (self.data[byte_index] >> bit_index) & 1;
        self.offset += 1;
        bit != 0
    }

    pub fn bits(&mut self) -> Vec<bool> {
        let byte = self.u8(); // read one byte
        (0..8).map(|i| (byte & (1 << i)) != 0).collect()
    }

    pub fn bytes(&mut self, count: usize) -> &'a [u8] {
        let slice = &self.data[self.offset..self.offset + count];
        self.offset += count;
        slice
    }

    pub fn read_until(&mut self, address: usize) -> Vec<u8> {
        let end = std::cmp::min(address, self.data.len());
        let slice = &self.data[self.offset..end];
        self.offset = end; // update offset to the end of the slice
        slice.to_vec()
    }

    pub fn offset(&self) -> usize {
        self.offset
    }

    pub fn seek(&mut self, offset: usize) {
        self.offset = offset;
    }

    pub fn uleb128(&mut self) -> u64 {
        let mut value = 0u64;
        let mut shift = 0;
        loop {
            let byte = self.u8();
            value |= ((byte & 0x7F) as u64) << shift;
            if (byte & 0x80) == 0 {
                break;
            }
            shift += 7;
        }
        value
    }

    pub fn string(&mut self, size: Option<usize>) -> String {
        let size = size.unwrap_or_else(|| self.uleb128() as usize);
        let bytes = self.bytes(size);
        bytes.iter().map(|&b| b as char).collect() // assuming latin1
    }
}
