#!/bin/bash

from_dir="packages/core/target/release/rserve"

to_dir="packages/cli/crates/rserve/target/release/"

echo "Copying binary from $from_dir to $to_dir...";
cp $from_dir $to_dir;
echo "Complete!";
