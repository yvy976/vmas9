#!/bin/bash

# Path to input value for stdin-based programs
INPUT="5"

# Directory containing the test files
TEST_DIR="tests"

# Loop through all .v files
for file in "$TEST_DIR"/*.v; do
    echo "Testing: $file"

    # Run both VMs with input piped in
    MY_OUTPUT=$(echo "$INPUT" | cargo run --quiet -- "$file")
    REF_OUTPUT=$(echo "$INPUT" | "$TEST_DIR"/machine "$file")

    # Compare outputs
    if diff <(echo "$MY_OUTPUT") <(echo "$REF_OUTPUT") > /dev/null; then
        echo "✅ Match"
    else
        echo "❌ Mismatch in $file"
        echo "---- Your Output ----"
        echo "$MY_OUTPUT"
        echo "---- Reference Output ----"
        echo "$REF_OUTPUT"
    fi

    echo "----------------------------"
done

