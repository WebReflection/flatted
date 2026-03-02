#!/bin/bash

# 1) Go to the golang directory and run a make build
echo "Building flatted binary..."
cd golang || exit 1
make build || exit 1
cd ..

# 2) normalize sha256sum test/65518.json and verify the hash
ORIGINAL_HASH="c76a5329a11de440d28f8d8c4b37aafaa61bca9f1eb41a904b3d46312d5ab565"
ACTUAL_HASH=$(cat test/65518.json | jq --sort-keys -r . | sha256sum | awk '{ print $1 }')

echo "Verifying checksum for test/65518.json..."
if [ "$ACTUAL_HASH" == "$ORIGINAL_HASH" ]; then
    echo "Checksum verified successfully."
else
    echo "Checksum mismatch!"
    echo "Expected: $ORIGINAL_HASH"
    echo "Actual:   $ACTUAL_HASH"
    exit 1
fi

# 3) Create a temp file, flatten the input, and verify the output hash
TEMP_FILE=$(mktemp)
trap 'rm -f "$TEMP_FILE"' EXIT

echo "Flattening test/65518.json and verifying output..."
cat test/65518.json | ./golang/flatted | jq --sort-keys -r . > "$TEMP_FILE"

EXPECTED_OUTPUT_HASH="feacd401744cea2e8597b41ddb3bad1fe6e77e306979529ddea9bc72e3f30a14"
ACTUAL_OUTPUT_HASH=$(sha256sum "$TEMP_FILE" | awk '{ print $1 }')

if [ "$ACTUAL_OUTPUT_HASH" == "$EXPECTED_OUTPUT_HASH" ]; then
    echo "Output checksum verified successfully."
else
    echo "Output checksum mismatch!"
    echo "Expected: $EXPECTED_OUTPUT_HASH"
    echo "Actual:   $ACTUAL_OUTPUT_HASH"
    exit 1
fi

# 4) Unflatten the temp file and verify it matches the original normalized hash
echo "Unflattening and verifying round-trip integrity..."
ROUNDTRIP_HASH=$(cat "$TEMP_FILE" | ./golang/flatted -d | jq --sort-keys -r . | sha256sum | awk '{ print $1 }')

if [ "$ROUNDTRIP_HASH" == "$ORIGINAL_HASH" ]; then
    echo "Round-trip verification successful."
else
    echo "Round-trip verification failed!"
    echo "Expected: $ORIGINAL_HASH"
    echo "Actual:   $ROUNDTRIP_HASH"
    exit 1
fi