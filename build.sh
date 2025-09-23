#!/bin/bash

# Usage check
if [ $# -lt 1 ]; then
    echo "Usage: $0 (wasm|bmvm|native) [NUM_LINKS] "
    exit 1
fi

is_number='^[0-9]+$'
profile="$1"
links="$2"
target=""
files="src/bin/"

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

features=""
matcher=""
if [[ -n "$links" ]]; then
    if ! [[ $links =~ $is_number ]] ; then
        echo "Error: Links must be a number"
        exit 1
    else
        features="--features=links$links,$profile"
    fi
else
    matcher="-not"
    features="--features=$profile"
fi

FILES=()
while IFS= read -r -d $'\0' file; do
    FILES+=("$file")
done < <(find "$files" -maxdepth 1 -type f -name "*.rs" $matcher -name "*link*$profile*" -print0)


# Loop through files in the given directory
for file in "${FILES[@]}"; do
    filename=$(basename "$file")    # strip directory
    base="${filename%.*}"           # strip extension
    cargo build --bin $base --profile=$profile $features $target
done
