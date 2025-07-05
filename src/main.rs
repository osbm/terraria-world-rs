

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
}