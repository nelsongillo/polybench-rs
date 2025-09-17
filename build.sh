#!/bin/bash

# Usage check
if [ $# -ne 1 ]; then
    echo "Usage: $0 (wasm|bmvm|native)"
    exit 1
fi

profile="$1"
addargs=""
files="src/bin/*.rs"

# Validate profile
case "$profile" in
    wasm)
        addargs="--target=wasm32-unknown-unknown"
        ;;
    bmvm)
        addargs="-Zbuild-std=core,alloc,compiler_builtins --target ./bmvm.json"
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
        cargo build --bin "$base" --profile="$profile" --features="$profile" $addargs
  fi
done
