mod reader;
use reader::ByteReader;

fn main() {
    let path = "worlds/small_corruption.wld";
    let bytes = std::fs::read(path).expect("Failed to read file");
    let mut r = ByteReader::new(&bytes);

    let version = r.i32();
    println!("Version: {}", version);

    let magic = r.bytes(7);
    println!("Magic: {}", String::from_utf8_lossy(magic));

    let savefile_type = r.u8();
    println!("Savefile type version: {}", savefile_type);

    let revision = r.u32();
    println!("Revision: {}", revision);

    let is_favorite = r.u64();
    println!("Is Favorite: {}", is_favorite);

    let pointer_count = r.u16();
    println!("Pointer count: {}", pointer_count);

    let mut pointers = vec![];
    for _ in 0..pointer_count {
        pointers.push(r.u32());
    }
    println!("Pointers: {:?}", pointers);
}