#!/bin/bash

# Create output directory if it doesn't exist
mkdir -p output_tests

# Find all docx/DOCX files in samples directory (bash compatible)
files=()
while IFS= read -r -d '' file; do
    files+=("$file")
done < <(find samples -maxdepth 1 -type f \( -iname "*.docx" \) -print0 | sort -z)

# Build project first
echo "Building project..."
cargo build --release

# Create output directory
mkdir -p output_tests/samples || { echo "Build failed"; exit 1; }

echo "Starting tests..."
echo "----------------------------------------"

for input_file in "${files[@]}"; do
    if [ ! -f "$input_file" ]; then
        echo "⚠️  File not found: $input_file"
        continue
    fi
    
    output_file="output_tests/${input_file%.*}.md"
    echo "Processing: $input_file"
    
    if cargo run --release -- "$input_file" "$output_file"; then
        echo "✅ Success"
    else
        echo "❌ Failed"
    fi
    echo "----------------------------------------"
done
