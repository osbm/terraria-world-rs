# Integration Tests

This directory contains comprehensive integration tests that validate our Rust implementation against the trusted [lihzahrd](https://github.com/Steffo99/lihzahrd) Python library.

## Overview

The integration testing workflow:

1. **Python Reference Generation**: The `integration_test.py` script uses lihzahrd to parse Terraria world files and generate detailed reference data in JSON format.
2. **Rust Validation**: The `integration_tests.rs` file contains comprehensive Rust tests that parse the same world files and compare the results against the Python reference data.
3. **Automated Testing**: The `run_integration_tests.sh` script orchestrates the entire testing process.

## Features

- **Multiple World Support**: Tests can handle multiple world files simultaneously
- **Comprehensive Validation**: Validates world metadata, tile data, wiring, paint, frames, and more
- **Robust Error Handling**: Graceful handling of missing files and parsing errors
- **Detailed Reporting**: Clear error messages with specific tile coordinates
- **Environment Integration**: Supports `TEST_WORLDS_DIR` environment variable
- **Reproducible Results**: Deterministic sampling for consistent test results

## Files

- `integration_test.py` - Enhanced Python script that uses lihzahrd to generate reference data
- `integration_tests.rs` - Comprehensive Rust tests with modular validation functions
- `run_integration_tests.sh` - Automated test runner with support for multiple world files
- `README.md` - This documentation

## Prerequisites

1. **Test World Files**: You need Terraria world files (`.wld`) to test against. You can get these from:
   - [osbm/terraria-worlds](https://github.com/osbm/terraria-worlds) repository
   - Your own Terraria worlds

2. **Nix Environment**: The flake.nix includes the lihzahrd Python package, so you can run tests in a Nix environment.

## Running Tests

### Option 1: Using the automated test runner

```bash
# Test all world files in current directory
./tests/run_integration_tests.sh

# Test with cleanup (removes reference files after testing)
./tests/run_integration_tests.sh --cleanup
```

### Option 2: Manual workflow

1. Generate reference data for specific world files:
   ```bash
   # Single world file
   python3 tests/integration_test.py small_corruption.wld

   # All world files in current directory
   python3 tests/integration_test.py --all

   # With custom output file
   python3 tests/integration_test.py small_corruption.wld -o my_reference.json

   # Verbose output
   python3 tests/integration_test.py small_corruption.wld --verbose
   ```

2. Run Rust integration tests:
   ```bash
   cargo test --test integration_tests

   # With output capture
   cargo test --test integration_tests -- --nocapture
   ```

### Option 3: Using Nix

```bash
# Enter the development shell with all dependencies
nix develop

# Run the integration tests
./tests/run_integration_tests.sh
```

### Option 4: Environment-based world files

```bash
# Set environment variable to point to world files
export TEST_WORLDS_DIR=/path/to/terraria/worlds

# Run tests (will automatically find world files)
./tests/run_integration_tests.sh
```

## What the Tests Validate

### World Metadata
- Version, name, size, difficulty, world flags
- Spawn and dungeon point coordinates
- Underground and cavern level depths
- All world flags (drunk, for the worthy, anniversary, etc.)

### Tile Frame Important Array
- Array length consistency
- Access to common block types
- Reasonable array size validation

### Individual Tiles
- Block presence, type, paint, frames, active state
- Wall presence, type, paint, properties
- Liquid type and volume
- Wiring (red, blue, green, yellow)
- Illuminant and echo states

### Basic Functionality
- World bounds validation
- Tile accessibility
- Coordinate range checking
- Data consistency

## Test Structure

### Rust Tests

1. **`test_world_parsing_against_lihzahrd`**: Main integration test comparing against Python reference
2. **`test_world_parsing_basic_functionality`**: Basic sanity checks and bounds validation
3. **`test_tile_frame_important_consistency`**: Validates tile frame important array
4. **`test_world_file_validation`**: Tests error handling for invalid files

### Test Utilities

The `test_utils` module provides:
- `load_reference_data()`: Load and parse JSON reference files
- `assert_approx_eq()`: Floating-point comparison with tolerance
- `validate_world_metadata()`: Comprehensive world metadata validation
- `validate_tile_frame_important()`: Tile frame important array validation
- `validate_tile()`: Individual tile data validation
- `get_test_world_files()`: Find world files from multiple sources

## Python Script Features

The `integration_test.py` script provides:
- **Command-line interface** with argparse
- **Multiple world file support** with `--all` flag
- **Flexible output** with custom file names
- **Verbose mode** for debugging
- **Robust error handling** with detailed error messages
- **Comprehensive sampling** from multiple world areas
- **Reproducible results** with seeded random sampling

## Adding New Tests

To add new test cases:

1. **Add new world files** to your test data
2. **Extend validation functions** in `test_utils` for new data types
3. **Add specific test functions** in `integration_tests.rs` for edge cases
4. **Update the Python script** to extract additional data if needed
5. **Consider adding performance benchmarks**

## Troubleshooting

### Missing World Files
If you get "No test world files found":
- Place `.wld` files in the current directory
- Set `TEST_WORLDS_DIR` environment variable
- Download test worlds from [osbm/terraria-worlds](https://github.com/osbm/terraria-worlds)

### Python Import Errors
If lihzahrd is not found:
- Make sure you're in the Nix development shell: `nix develop`
- The flake.nix should provide the lihzahrd Python package

### Test Failures
If tests fail:
- Check that world files are valid and not corrupted
- Verify that both Python and Rust implementations are up to date
- Look at the generated reference JSON files for debugging
- Run with `--verbose` flag for more detailed output

### Performance Issues
For large world files:
- The tests sample a subset of tiles for efficiency
- Consider reducing the number of random samples in the Python script
- Use smaller test worlds for development

## Continuous Integration

The flake.nix includes an integration test check that runs automatically:

```bash
nix flake check
```

This will run the integration tests in a clean Nix environment.

## Example Output

```
Running integration tests...
Found 2 world file(s): small_corruption.wld large_crimson.wld
Generating reference data with lihzahrd...
Processing: small_corruption.wld
Reference data written to: small_corruption.wld.lihzahrd_reference.json
World: Test World
Size: 4200x1200
Sample tiles: 18
Total tiles: 5040000
Processing: large_crimson.wld
Reference data written to: large_crimson.wld.lihzahrd_reference.json
World: Large Test World
Size: 8400x2400
Sample tiles: 18
Total tiles: 20160000
Reference data generated successfully for all world files.
Running Rust integration tests...
Testing world file: small_corruption.wld
Successfully validated 18 tiles for small_corruption.wld
Testing world file: large_crimson.wld
Successfully validated 18 tiles for large_crimson.wld
Integration tests passed!
```