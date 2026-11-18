#!/bin/bash

cargo build --workspace --bins --release

OS_INFO=$(uname -moprsv)

echo -e "OS: $OS_INFO"
echo -e

./target/release/initialize
./target/release/readable
./target/release/writeable