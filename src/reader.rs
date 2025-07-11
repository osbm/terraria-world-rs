use chrono::DateTime;

pub struct ByteReader<'a> {
    data: &'a [u8],
    offset: usize,
}

impl<'a> ByteReader<'a> {
    pub fn new(data: &'a [u8]) -> Self {
        ByteReader { data, offset: 0 }
    }

    pub fn u8(&mut self) -> u8 {
        if self.offset >= self.data.len() {
            panic!(
                "Attempted to read u8 at offset {} but data length is {}",
                self.offset,
                self.data.len()
            );
        }
        let val = self.data[self.offset];
        self.offset += 1;
        val
    }

    pub fn u16(&mut self) -> u16 {
        if self.offset + 2 > self.data.len() {
            panic!(
                "Attempted to read u16 at offset {} but data length is {}",
                self.offset,
                self.data.len()
            );
        }
        let val = u16::from_le_bytes(self.data[self.offset..self.offset + 2].try_into().unwrap());
        self.offset += 2;
        val
    }

    pub fn u32(&mut self) -> u32 {
        if self.offset + 4 > self.data.len() {
            panic!(
                "Attempted to read u32 at offset {} but data length is {}",
                self.offset,
                self.data.len()
            );
        }
        let val = u32::from_le_bytes(self.data[self.offset..self.offset + 4].try_into().unwrap());
        self.offset += 4;
        val
    }

    pub fn u64(&mut self) -> u64 {
        if self.offset + 8 > self.data.len() {
            panic!(
                "Attempted to read u64 at offset {} but data length is {}",
                self.offset,
                self.data.len()
            );
        }
        let val = u64::from_le_bytes(self.data[self.offset..self.offset + 8].try_into().unwrap());
        self.offset += 8;
        val
    }

    pub fn i8(&mut self) -> i8 {
        if self.offset >= self.data.len() {
            panic!(
                "Attempted to read i8 at offset {} but data length is {}",
                self.offset,
                self.data.len()
            );
        }
        let val = self.data[self.offset] as i8;
        self.offset += 1;
        val
    }

    pub fn i16(&mut self) -> i16 {
        if self.offset + 2 > self.data.len() {
            panic!(
                "Attempted to read i16 at offset {} but data length is {}",
                self.offset,
                self.data.len()
            );
        }
        let val = i16::from_le_bytes(self.data[self.offset..self.offset + 2].try_into().unwrap());
        self.offset += 2;
        val
    }

    pub fn i32(&mut self) -> i32 {
        if self.offset + 4 > self.data.len() {
            panic!(
                "Attempted to read i32 at offset {} but data length is {}",
                self.offset,
                self.data.len()
            );
        }
        let val = i32::from_le_bytes(self.data[self.offset..self.offset + 4].try_into().unwrap());
        self.offset += 4;
        val
    }

    pub fn i64(&mut self) -> i64 {
        if self.offset + 8 > self.data.len() {
            panic!(
                "Attempted to read i64 at offset {} but data length is {}",
                self.offset,
                self.data.len()
            );
        }
        let val = i64::from_le_bytes(self.data[self.offset..self.offset + 8].try_into().unwrap());
        self.offset += 8;
        val
    }

    pub fn bool(&mut self) -> bool {
        let byte = self.u8();
        // In Terraria world files, any non-zero value is considered true
        byte != 0
    }

    pub fn bits(&mut self) -> Vec<bool> {
        let byte = self.u8(); // read one byte
        (0..8).map(|i| (byte & (1 << i)) != 0).collect()
    }

    pub fn bytes(&mut self, count: usize) -> &'a [u8] {
        if self.offset + count > self.data.len() {
            panic!(
                "Attempted to read {} bytes at offset {} but data length is {}",
                count,
                self.offset,
                self.data.len()
            );
        }
        let slice = &self.data[self.offset..self.offset + count];
        self.offset += count;
        slice
    }

    /// Returns a slice of bytes from the current offset without advancing the offset.
    pub fn peek_bytes(&self, count: usize) -> &'a [u8] {
        if self.offset + count > self.data.len() {
            panic!(
                "Attempted to peek {} bytes at offset {} but data length is {}",
                count,
                self.offset,
                self.data.len()
            );
        }
        &self.data[self.offset..self.offset + count]
    }

    pub fn read_until(&mut self, address: usize) -> Vec<u8> {
        let end = std::cmp::min(address, self.data.len());
        if self.offset >= end {
            return Vec::new(); // Already past the target address
        }
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

    pub fn uuid(&mut self) -> String {
        let bytes = self.bytes(16);
        format!(
            "{:02x}{:02x}{:02x}{:02x}-\
             {:02x}{:02x}-\
             {:02x}{:02x}-\
             {:02x}{:02x}-\
             {:02x}{:02x}{:02x}{:02x}{:02x}{:02x}",
            bytes[0],
            bytes[1],
            bytes[2],
            bytes[3],
            bytes[4],
            bytes[5],
            bytes[6],
            bytes[7],
            bytes[8],
            bytes[9],
            bytes[10],
            bytes[11],
            bytes[12],
            bytes[13],
            bytes[14],
            bytes[15],
        )
    }

    pub fn datetime(&mut self) -> String {
        let raw = self.u64(); // already reads 8 bytes little-endian

        let _kind: u64 = (raw >> 62) & 0b11;
        let ticks: u64 = raw & 0x3FFF_FFFF_FFFF_FFFF; // mask top 2 bits

        // println!("Kind: {}", match kind {
        //     0 => "Unspecified",
        //     1 => "Utc",
        //     2 => "Local",
        //     _ => "⚠️ Reserved/Invalid",
        // });

        // .NET ticks start at 0001-01-01
        let unix_offset: u64 = 621355968000000000;
        if ticks < unix_offset {
            return "⚠️ Before UNIX epoch".to_string();
        }

        let unix_ticks: u64 = ticks - unix_offset;
        let secs: u64 = unix_ticks / 10_000_000;
        let nsecs: u64 = (unix_ticks % 10_000_000) * 100;

        match DateTime::from_timestamp(secs as i64, nsecs as u32) {
            Some(dt) => {
                // Use format with 7 decimal places to preserve .NET tick precision
                // .NET ticks are 100ns intervals, so 7 decimal places gives us the full precision
                dt.format("%Y-%m-%d %H:%M:%S%.f").to_string()
            }
            _ => "⚠️ Invalid datetime".to_string(),
        }
    }

    pub fn f32(&mut self) -> f32 {
        if self.offset + 4 > self.data.len() {
            panic!(
                "Attempted to read f32 at offset {} but data length is {}",
                self.offset,
                self.data.len()
            );
        }
        let bytes = self.bytes(4);
        let val = f32::from_le_bytes(bytes.try_into().unwrap());
        val
    }

    pub fn f64(&mut self) -> f64 {
        if self.offset + 8 > self.data.len() {
            panic!(
                "Attempted to read f64 at offset {} but data length is {}",
                self.offset,
                self.data.len()
            );
        }
        let bytes = self.bytes(8);
        let val = f64::from_le_bytes(bytes.try_into().unwrap());
        val
    }

    pub fn slice_bytes(&self, start: usize, end: usize) -> Vec<u8> {
        if start > end || end > self.data.len() {
            panic!("Invalid slice range: {}..{} (data len {})", start, end, self.data.len());
        }
        self.data[start..end].to_vec()
    }
}
