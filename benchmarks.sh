#!/bin/sh

cargo build --workspace --bins --release

OS_INFO=$(uname -moprsv)

echo "OS: $OS_INFO"
echo;

./target/release/initialize
./target/release/readable
./target/release/writeable
