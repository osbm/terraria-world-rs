use std::fs;
use terraria_world::world::World;

#[test]
fn test_world_roundtrip() {
    let test_worlds_dir = std::env::var("TEST_WORLDS_DIR")
        .expect("TEST_WORLDS_DIR environment variable not set. Please provide the test worlds directory as a flake input.");
    println!("Using test worlds directory: {test_worlds_dir}");
    // Get all .wld files in the test worlds directory
    let entries = fs::read_dir(&test_worlds_dir).expect("Failed to read test worlds directory");
    let wld_files: Vec<_> = entries
        .filter_map(|entry| {
            let entry = entry.ok()?;
            let path = entry.path();
            if path.extension().map(|ext| ext == "wld").unwrap_or(false) {
                Some(path)
            } else {
                None
            }
        })
        .collect();

    if wld_files.is_empty() {
        println!("No .wld files found in $TEST_WORLDS_DIR directory");
        return;
    }

    println!("Found {} .wld files for roundtrip testing", wld_files.len());

    let mut failures = Vec::new();
    for wld_file in wld_files {
        let file_name = wld_file.file_name().unwrap().to_str().unwrap();
        println!("Testing roundtrip for: {file_name}");

        // Read the original world
        let world = World::from_file(wld_file.to_str().unwrap())
            .unwrap_or_else(|_| panic!("Failed to read world file: {file_name}"));
        println!("World name: {}", world.world_name);
        // create the worlds directory if it doesn't exist
        fs::create_dir_all("./worlds").expect("Failed to create worlds directory");

        // Save as WLD
        let output_wld_path = format!("./worlds/{file_name}.roundtrip.wld");
        world
            .save_as_wld(&output_wld_path)
            .unwrap_or_else(|_| panic!("Failed to save WLD for: {file_name}"));
        println!("Saved roundtrip WLD to: {output_wld_path}");

        // Read both files as bytes
        let orig_bytes = fs::read(&wld_file).expect("Failed to read original file bytes");
        let out_bytes = fs::read(&output_wld_path).expect("Failed to read output file bytes");
        let min_len = orig_bytes.len().min(out_bytes.len());
        let mut first_diff = None;
        for i in 0..min_len {
            if orig_bytes[i] != out_bytes[i] {
                first_diff = Some(i);
                break;
            }
        }
        if let Some(idx) = first_diff {
            let percent = (idx as f64) / (orig_bytes.len().max(out_bytes.len()) as f64) * 100.0;
            println!(
                "✗ {}: first difference at byte {}. total bytes {}. The first ({:.2}%) is correct.",
                file_name,
                idx,
                orig_bytes.len().max(out_bytes.len()),
                percent
            );
            failures.push((
                file_name.to_string(),
                idx,
                orig_bytes.len().max(out_bytes.len()),
                percent,
            ));
        } else if orig_bytes.len() != out_bytes.len() {
            let min_len = orig_bytes.len().min(out_bytes.len());
            println!(
                "✗ {}: files are identical for first {} bytes, but lengths differ ({} vs {})",
                file_name,
                min_len,
                orig_bytes.len(),
                out_bytes.len()
            );
            failures.push((
                file_name.to_string(),
                min_len,
                orig_bytes.len().max(out_bytes.len()),
                (min_len as f64) / (orig_bytes.len().max(out_bytes.len()) as f64) * 100.0,
            ));
        } else {
            println!("✓ {file_name}: OK (100%)");
        };
    }
    if !failures.is_empty() {
        println!("\nSummary of roundtrip failures:");
        for (file, idx, len, percent) in &failures {
            println!(
                "  {file}: first difference at byte {idx}. total bytes {len}. The first  ({percent:.2}%) is correct."
            );
        }
        panic!("{} roundtrip test(s) failed", failures.len());
    }
}
