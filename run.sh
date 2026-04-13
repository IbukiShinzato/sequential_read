##!/bin/zsh

echo "Building binary..."
cargo build --release
BINARY="./target/release/sequential_read"

SIZES=(1024 4096 65536 1048576 1073741824)

echo "Starting benchmarks..."
echo "------------------------------------------------"

for sz in "${SIZES[@]}"; do
    echo -n "Mode: read, Buffer Size: $sz bytes "
    sudo purge
    (time $BINARY read $sz) > /dev/null
    echo "------------------------------------------------"
done

echo -n "Mode: mmap "
sudo purge
(time $BINARY mmap) > /dev/null
echo "------------------------------------------------"

