use std::io::{self, Write, Seek, SeekFrom};

pub struct ByteWriter {
    buffer: Vec<u8>,
    offset: usize,
}

impl ByteWriter {
    pub fn new() -> Self {
        ByteWriter {
            buffer: Vec::new(),
            offset: 0,
        }
    }

    pub fn into_inner(self) -> Vec<u8> {
        self.buffer
    }

    pub fn offset(&self) -> usize {
        self.offset
    }

    pub fn seek(&mut self, offset: usize) {
        self.offset = offset;
        if self.buffer.len() < offset {
            self.buffer.resize(offset, 0);
        }
    }

    pub fn u8(&mut self, val: u8) {
        if self.offset == self.buffer.len() {
            self.buffer.push(val);
        } else {
            self.buffer[self.offset] = val;
        }
        self.offset += 1;
    }

    pub fn u16(&mut self, val: u16) {
        let bytes = val.to_le_bytes();
        self.bytes(&bytes);
    }

    pub fn u32(&mut self, val: u32) {
        let bytes = val.to_le_bytes();
        self.bytes(&bytes);
    }

    pub fn u64(&mut self, val: u64) {
        let bytes = val.to_le_bytes();
        self.bytes(&bytes);
    }

    pub fn i8(&mut self, val: i8) {
        self.u8(val as u8);
    }

    pub fn i16(&mut self, val: i16) {
        let bytes = val.to_le_bytes();
        self.bytes(&bytes);
    }

    pub fn i32(&mut self, val: i32) {
        let bytes = val.to_le_bytes();
        self.bytes(&bytes);
    }

    pub fn i64(&mut self, val: i64) {
        let bytes = val.to_le_bytes();
        self.bytes(&bytes);
    }

    pub fn f32(&mut self, val: f32) {
        let bytes = val.to_le_bytes();
        self.bytes(&bytes);
    }

    pub fn f64(&mut self, val: f64) {
        let bytes = val.to_le_bytes();
        self.bytes(&bytes);
    }

    pub fn bool(&mut self, val: bool) {
        self.u8(if val { 1 } else { 0 });
    }

    pub fn bits(&mut self, bits: &[bool]) {
        let mut byte = 0u8;
        for (i, &bit) in bits.iter().enumerate().take(8) {
            if bit {
                byte |= 1 << i;
            }
        }
        self.u8(byte);
    }

    pub fn bytes(&mut self, bytes: &[u8]) {
        let end = self.offset + bytes.len();
        if self.buffer.len() < end {
            self.buffer.resize(end, 0);
        }
        self.buffer[self.offset..end].copy_from_slice(bytes);
        self.offset = end;
    }

    pub fn string(&mut self, s: &str) {
        // Terraria uses ULEB128 for string length
        self.uleb128(s.len() as u64);
        self.bytes(s.as_bytes());
    }

    pub fn uuid(&mut self, uuid: &str) {
        // Expects a string in the format xxxxxxxx-xxxx-xxxx-xxxx-xxxxxxxxxxxx
        let hex = uuid.replace("-", "");
        let bytes = (0..16)
            .map(|i| u8::from_str_radix(&hex[2 * i..2 * i + 2], 16).unwrap_or(0))
            .collect::<Vec<u8>>();
        self.bytes(&bytes);
    }

    pub fn uleb128(&mut self, mut value: u64) {
        loop {
            let mut byte = (value & 0x7F) as u8;
            value >>= 7;
            if value != 0 {
                byte |= 0x80;
            }
            self.u8(byte);
            if value == 0 {
                break;
            }
        }
    }
} 