#!/bin/bash

# Integration test runner for terraria-world-parser-rust
# This script runs the Python integration test to generate reference data,
# then runs the Rust integration tests to compare against that data.

set -e

echo "Running integration tests..."

# Function to find world files
find_world_files() {
    local world_files=()
    
    # Check for world files in the current directory
    for file in *.wld; do
        if [[ -f "$file" ]]; then
            world_files+=("$file")
        fi
    done
    
    # If no world files found, check TERRARIA_WORLD_PATH environment variable
    if [[ ${#world_files[@]} -eq 0 ]]; then
        if [[ -n "$TERRARIA_WORLD_PATH" ]]; then
            for file in "$TERRARIA_WORLD_PATH"/*.wld; do
                if [[ -f "$file" ]]; then
                    world_files+=("$file")
                fi
            done
        fi
    fi
    
    echo "${world_files[@]}"
}

# Find world files
world_files=($(find_world_files))

if [[ ${#world_files[@]} -eq 0 ]]; then
    echo "Warning: No test world files found."
    echo "You can:"
    echo "  1. Place .wld files in the current directory"
    echo "  2. Set TERRARIA_WORLD_PATH environment variable"
    echo "  3. Download test worlds from: https://github.com/osbm/terraria-worlds"
    echo "Skipping integration tests."
    exit 0
fi

echo "Found ${#world_files[@]} world file(s): ${world_files[*]}"

# Generate reference data for all world files
echo "Generating reference data with lihzahrd..."
for world_file in "${world_files[@]}"; do
    echo "Processing: $world_file"
    
    # Run Python integration test to generate reference data
    python3 tests/integration_test.py "$world_file"
    
    if [[ $? -ne 0 ]]; then
        echo "Failed to generate reference data for $world_file"
        exit 1
    fi
done

echo "Reference data generated successfully for all world files."

# Run Rust integration tests
echo "Running Rust integration tests..."
cargo test --test integration_tests -- --nocapture

if [[ $? -eq 0 ]]; then
    echo "Integration tests passed!"
else
    echo "Integration tests failed!"
    exit 1
fi

# Optional: Clean up reference files
if [[ "$1" == "--cleanup" ]]; then
    echo "Cleaning up reference files..."
    for world_file in "${world_files[@]}"; do
        reference_file="${world_file}.lihzahrd_reference.json"
        if [[ -f "$reference_file" ]]; then
            rm "$reference_file"
            echo "Removed: $reference_file"
        fi
    done
fi 