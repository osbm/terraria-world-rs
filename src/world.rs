mod reader;
use reader::ByteReader;


pub struct World {
    pub version: i32,
    pub magic: String,
    pub savefile_type: u8,
    pub revision: u32,
    pub is_favorite: u64,
    pub pointers: Vec<u32>,
    pub tile_frame_important: Vec<u8>,
}


impl World {
    pub fn from_file(path: &str) -> std::io::Result<Self> {
        let bytes = std::fs::read(path)?;
        let mut r = ByteReader::new(&bytes);

        let version = r.i32();
        let magic = String::from_utf8_lossy(r.bytes(7)).to_string();
        let savefile_type = r.u8();
        let revision = r.u32();
        let is_favorite = r.u64();
        let pointer_count = r.u16();

        let mut pointers = vec![];
        for _ in 0..pointer_count {
            pointers.push(r.u32());
        }

        let tile_frame_important_size = (r.i16() + 7) / 8;
        let tile_frame_important = r.bytes(tile_frame_important_size as usize).to_vec();

        Ok(Self {
            version,
            magic,
            savefile_type,
            revision,
            is_favorite,
            pointers,
            tile_frame_important,
        })
    }


    pub fn remove_corruption(mut self) -> Self {
        println!("Removing corruption...");
        // raise unimplemented error
        unimplemented!("Corruption removal is not implemented yet.");
    }

    pub fn save_to(&self, path: &str) -> std::io::Result<()> {
        println!("Saving to {path}...");
        unimplemented!("Saving functionality is not implemented yet.");
    }
}