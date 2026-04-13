##!/bin/zsh

FILE="testfile"
if [ ! -f "$FILE" ]; then
    echo "Creating 1GB test file..."
    dd if=/dev/zero of=${FILE} bs=1M count=1024 > /dev/null 2>&1
fi

KB=1024
MB=$(( 1024 * KB ))
GB=$(( 1024 * MB ))

SIZES=(
  $(( 1 * KB ))
  $(( 4 * KB ))
  $(( 64 * KB ))
  $(( 1 * MB ))
)

echo "Building binary..."
cargo clean -q
cargo build --release -q
BINARY="./target/release/sequential_read"

echo "Starting benchmarks..."
echo "------------------------------------------------"

for sz in "${SIZES[@]}"; do
    echo "Mode: read, Buffer Size: $sz bytes "
    sudo purge
    (time $BINARY read $sz) > /dev/null
    echo "------------------------------------------------"
done

echo "Mode: mmap "
sudo purge
(time $BINARY mmap) > /dev/null
echo "------------------------------------------------"
