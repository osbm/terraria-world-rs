use std::fs;
use std::path::Path;
use std::process::Command;
use terraria_world_parser::world::World;

#[test]
fn test_world_roundtrip() {
    // let test_worlds_dir = "tests/test_worlds";
    // read the environment variable for test worlds directory
    let test_worlds_dir = std::env::var("TEST_WORLDS_DIR")
        .unwrap_or_else(|_| "tests/test_worlds".to_string());
    println!("Using test worlds directory: {}", test_worlds_dir);
    // Get all .wld files in the test worlds directory
    let entries = fs::read_dir(test_worlds_dir).expect("Failed to read test worlds directory");
    let wld_files: Vec<_> = entries
        .filter_map(|entry| {
            let entry = entry.ok()?;
            let path = entry.path();
            if path.extension()?.to_str()? == "wld" {
                Some(path)
            } else {
                None
            }
        })
        .collect();

    if wld_files.is_empty() {
        println!("No .wld files found in tests/test_worlds directory");
        return;
    }

    println!("Found {} .wld files for roundtrip testing", wld_files.len());

    for wld_file in wld_files {
        let file_name = wld_file.file_name().unwrap().to_str().unwrap();
        println!("Testing roundtrip for: {}", file_name);

        // Read the original world
        let world = World::from_file(wld_file.to_str().unwrap())
            .expect(&format!("Failed to read world file: {}", file_name));

        // Save as WLD
        let output_wld_path = format!("{}.roundtrip.wld", wld_file.to_str().unwrap());
        world.save_as_wld(&output_wld_path)
            .expect(&format!("Failed to save WLD for: {}", file_name));

        // Compare hashes using bash
        let original_hash = Command::new("sha256sum")
            .arg(wld_file.to_str().unwrap())
            .output()
            .expect("Failed to compute original file hash");

        let output_hash = Command::new("sha256sum")
            .arg(&output_wld_path)
            .output()
            .expect("Failed to compute output file hash");

        let original_hash_lossy = String::from_utf8_lossy(&original_hash.stdout);
        let original_hash_str = original_hash_lossy.split_whitespace().next().unwrap();
        let output_hash_lossy = String::from_utf8_lossy(&output_hash.stdout);
        let output_hash_str = output_hash_lossy.split_whitespace().next().unwrap();

        assert_eq!(
            original_hash_str,
            output_hash_str,
            "Hash mismatch for {} - Original: {}, Output: {}",
            file_name,
            original_hash_str,
            output_hash_str
        );

        println!("✓ Roundtrip test passed for: {} (hash: {})", file_name, original_hash_str);

        // Clean up temporary files
        fs::remove_file(output_wld_path).ok();
    }
}