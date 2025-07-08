use std::io::{self, Write, Seek, SeekFrom};

#[derive(Clone)]
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

    /// Overwrite a u32 value at a specific offset in the buffer (little-endian)
    pub fn patch_u32(&mut self, offset: usize, value: u32) {
        let bytes = value.to_le_bytes();
        if offset + 4 > self.buffer.len() {
            self.buffer.resize(offset + 4, 0);
        }
        self.buffer[offset..offset + 4].copy_from_slice(&bytes);
    }

    pub fn datetime(&mut self, datetime_str: &str) {
        // Parse the datetime string back to a u64 value
        // The format is "YYYY-MM-DD HH:MM:SS"
        if datetime_str.starts_with("⚠️") {
            // Handle invalid datetime - write 0
            self.u64(0);
            return;
        }

        // Parse the datetime string
        if let Ok(dt) = chrono::NaiveDateTime::parse_from_str(datetime_str, "%Y-%m-%d %H:%M:%S") {
            // Convert to Unix timestamp
            let unix_secs = dt.timestamp();
            
            // Convert to .NET ticks (1 tick = 100 nanoseconds)
            let unix_ticks = (unix_secs as u64) * 10_000_000; // seconds to ticks
            let net_ticks = unix_ticks + 621355968000000000; // add .NET epoch offset
            
            // Set kind to Unspecified (0) and write the ticks
            let raw = net_ticks & 0x3FFF_FFFF_FFFF_FFFF; // mask top 2 bits for kind
            println!("DEBUG: Writing datetime raw value: 0x{:016x}", raw); // DEBUG: Print raw datetime value
            self.u64(raw);
        } else {
            // If parsing fails, write 0
            println!("DEBUG: Writing datetime raw value: 0x0000000000000000 (parse failed)"); // DEBUG: Print raw datetime value
            self.u64(0);
        }
    }
} 