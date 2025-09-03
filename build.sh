#!/bin/bash

# Usage check
if [ $# -ne 1 ]; then
    echo "Usage: $0 (wasm|bmvm|native)"
    exit 1
fi

profile="$1"
target=""
files="src/bin/*.rs"

# Validate profile
case "$profile" in
    wasm)
        target="--target=wasm32-unknown-unknown"
        ;;
    bmvm)
        target="--target=x86_64-unknown-none"
        ;;
    native)
        ;;
    *)
        echo "Error: mode must be one of {wasm|bmvm|native}"
        exit 1
        ;;
esac

# Loop through files in the given directory
for file in $files; do
    echo "$file"
    if [ -f "$file" ]; then
        filename=$(basename "$file")    # strip directory
        base="${filename%.*}"           # strip extension
        echo "$base"
        cargo build --bin "$base" --profile="$profile" --features="$profile" $target
  fi
done
