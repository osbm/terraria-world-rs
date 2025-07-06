use crate::reader::ByteReader;

mod pointers;
use pointers::Pointers;

#[derive(Debug)]
pub struct World {
    pub version_integer: i32,
    pub magic: String,
    pub savefile_type: u8,
    pub revision: u32,
    pub is_favorite: u64,
    pub pointer_count: u16,
    pub pointer_vector: Vec<u32>,
    pub tile_frame_important_size: i16,
    pub tile_frame_important: Vec<u8>,
}

impl World {
    pub fn from_file(path: &str) -> std::io::Result<Self> {
        let bytes = std::fs::read(path)?;
        let mut r = ByteReader::new(&bytes);

        let version_integer = r.i32();
        let magic = String::from_utf8_lossy(r.bytes(7)).to_string();
        let savefile_type = r.u8();
        let revision = r.u32();
        let is_favorite = r.u64();
        let pointer_count = r.u16();
        let mut pointer_vector = vec![];
        for _ in 0..pointer_count {
            pointer_vector.push(r.u32());
        }
        let tile_frame_important_size = (r.i16() + 7) / 8;
        let tile_frame_important = r.bytes(tile_frame_important_size as usize).to_vec();

        Ok(Self {
            version_integer,
            magic,
            savefile_type,
            revision,
            is_favorite,
            pointer_count,
            pointer_vector,
            tile_frame_important_size,
            tile_frame_important,
        })
    }

    pub fn version(&self) ->  &str {
        if self.version_integer != 279 {
            eprintln!("⚠️ Warning: This parser was tested only on version 279 (1.4.4.9). Parsed version is {}", self.version_integer);
        }

        return match self.version_integer {
            12 => "1.0.5", // unconfirmed
            20 => "1.0.6", // unconfirmed
            22 => "1.0.6.1", // unconfirmed
            37 => "1.1.1", // unconfirmed
            39 => "1.1.2", // unconfirmed
            67 => "1.2", // unconfirmed
            71 => "1.2.0.3.1", // unconfirmed
            72 => "1.2.1.1", // unconfirmed
            73 => "1.2.1.2", // unconfirmed
            77 => "1.2.2", // unconfirmed
            94 => "1.2.3.1", // unconfirmed
            101 => "1.2.4", // unconfirmed
            102 => "1.2.4.1", // unconfirmed
            140 | 146 => "1.3.0.1", // unconfirmed
            147 => "1.3.0.2", // unconfirmed
            149 => "1.3.0.3", // unconfirmed
            151 => "1.3.0.4", // unconfirmed
            153 => "1.3.0.5", // unconfirmed
            154 => "1.3.0.6", // unconfirmed
            155 => "1.3.0.7", // unconfirmed
            156 => "1.3.0.8", // unconfirmed
            168 => "1.3.1", // unconfirmed
            169 => "1.3.1.1", // unconfirmed
            170 => "1.3.2", // unconfirmed
            173 => "1.3.2.1", // unconfirmed
            174 => "1.3.3", // unconfirmed
            175 => "1.3.3.1", // unconfirmed
            176 => "1.3.3.2", // unconfirmed
            177 => "1.3.3.3", // unconfirmed
            178 => "1.3.4", // unconfirmed
            185 => "1.3.4.1", // unconfirmed
            186 => "1.3.4.2", // unconfirmed
            187 => "1.3.4.3", // unconfirmed
            188 => "1.3.4.4", // unconfirmed
            191 => "1.3.5", // unconfirmed
            192 => "1.3.5.1", // unconfirmed
            193 => "1.3.5.2", // unconfirmed
            194 => "1.3.5.3", // unconfirmed
            225 => "1.4.0.1", // unconfirmed
            226 => "1.4.0.2", // unconfirmed
            227 => "1.4.0.3", // unconfirmed
            228 => "1.4.0.4", // unconfirmed
            230 => "1.4.0.5", // unconfirmed
            238 => "1.4.2.3", // unconfirmed
            274 => "1.4.4.5", // unconfirmed
            278 => "1.4.4.8", // unconfirmed
            279 => "1.4.4.9",
            _ => "Unknon version",
        };
    }

    pub fn pointers(&self) -> Pointers {
        Pointers::from_vector(&self.pointer_vector)
    }

    pub fn remove_corruption(self) -> Self {
        println!("Removing corruption...");
        // raise unimplemented error
        unimplemented!("Corruption removal is not implemented yet.");
    }

    pub fn save_to(&self, path: &str) -> std::io::Result<()> {
        println!("Saving to {path}...");
        unimplemented!("Saving functionality is not implemented yet.");
    }
}