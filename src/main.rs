

fn main() {
    // worlds/small_corruption.wld
    let path = "worlds/small_corruption.wld";

    // Read the file into a vector of bytes
    let bytes = std::fs::read(path)
        .expect("Failed to read the file");

    println!("File size: {} bytes", bytes.len());

    // read first 4 bytes as u32
    let mut header = [0u8; 4];
    header.copy_from_slice(&bytes[0..4]);
    let header_value = u32::from_le_bytes(header);
    println!("Header value: {}", header_value);

    // relogic magic string is the next 7 bytes
    let magic_string = &bytes[4..11];
    let magic_str = String::from_utf8_lossy(magic_string);
    println!("Magic string: {}", magic_str);

    // read the next uint 1 byte
    let version = bytes[11];
    println!("Savefile type version: {}", version);

    // revision is the next 4 bytes
    let mut revision_bytes = [0u8; 4];
    revision_bytes.copy_from_slice(&bytes[12..16]);
    let revision = u32::from_le_bytes(revision_bytes);
    println!("Revision: {}", revision);

    // is favorite number is the next 8 bytes
    let is_favorite_bytes = &bytes[16..24];
    let is_favorite = u64::from_le_bytes(is_favorite_bytes.try_into().expect("Slice with incorrect length"));
    println!("Is favorite: {}", is_favorite);

    //         pointers = Pointers(*[f.int4() for _ in range(f.int2())])
    let pointer_range = u16::from_le_bytes([bytes[24], bytes[25]]);
    println!("Pointer range: {}", pointer_range);

    //         pointers = Pointers(*[f.int4() for _ in range(f.int2())])
    let mut pointers = Vec::new();
    for i in 0..pointer_range {
        let start = 26 + (i as usize * 4);
        let end = start + 4;
        let mut pointer_bytes = [0u8; 4];
        pointer_bytes.copy_from_slice(&bytes[start..end]);
        let pointer = u32::from_le_bytes(pointer_bytes);
        pointers.push(pointer);
        println!("Pointer {}: {}", i, pointer);
    }
    println!("Total pointers: {}", pointers.len());
    println!("Pointers: {:?}", pointers);

    // Read the next 4 bytes as u32


}