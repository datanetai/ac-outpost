#!/bin/bash

# Initialize the file
echo "" > rust.md

# Find all the Rust files in the current directory and its subdirectories.
# and iterate over the files
find . -name "*.rs" | while read file; do
    # Check if the file name matches the exclusion patterns
    # ['test', 'schema']
    if [[ $file != *test* && $file != *schema* ]]; then
        # Print the file name
        echo "Processing file: $file"
        # Append the file name to the output file
        echo "### FILE: $(basename $file)" >> rust.md
        # Append the file content to the output file
        cat $file >> rust.md
    fi
done
