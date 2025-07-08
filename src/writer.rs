
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
        // Helper function to write default date (April 7, 2001)
        let write_default_date = |writer: &mut Self, reason: &str| {
            println!("⚠️ WARNING: {}, using default date (April 7, 2001)", reason);

            // Create default date: April 7, 2001 00:00:00
            let default_date = chrono::NaiveDate::from_ymd_opt(2001, 4, 7)
                .unwrap()
                .and_hms_opt(0, 0, 0)
                .unwrap();

            // Convert to Unix timestamp
            let unix_secs = default_date.and_utc().timestamp();

            // Convert to .NET ticks (1 tick = 100 nanoseconds)
            let unix_ticks = (unix_secs as u64) * 10_000_000;
            let net_ticks = unix_ticks + 621355968000000000; // add .NET epoch offset

            // Hardcode kind bits to 10 (binary) = 2 (decimal) = "Local"
            let raw = (0b10u64 << 62) | (net_ticks & 0x3FFF_FFFF_FFFF_FFFF);
            writer.u64(raw);
        };

        // Handle invalid datetime strings or parsing failures
        if datetime_str.starts_with("⚠️") {
            write_default_date(self, "Invalid datetime string");
            return;
        }

        // Parse the datetime string with fractional seconds
        if let Ok(dt) = chrono::NaiveDateTime::parse_from_str(datetime_str, "%Y-%m-%d %H:%M:%S%.f")
        {
            // Convert to Unix timestamp with nanosecond precision
            let unix_secs = dt.and_utc().timestamp();
            let unix_nanos = dt.and_utc().timestamp_subsec_nanos();

            // Convert to .NET ticks (1 tick = 100 nanoseconds)
            let unix_ticks = (unix_secs as u64) * 10_000_000 + (unix_nanos as u64) / 100;
            let net_ticks = unix_ticks + 621355968000000000; // add .NET epoch offset

            // Hardcode kind bits to 10 (binary) = 2 (decimal) = "Local"
            let raw = (0b10u64 << 62) | (net_ticks & 0x3FFF_FFFF_FFFF_FFFF);
            self.u64(raw);
        } else {
            write_default_date(
                self,
                &format!("Failed to parse datetime '{}'", datetime_str),
            );
        }
    }
}
