#!/bin/bash

from_dir="packages/core/target/release"

to_dir="packages/cli/crates/rserve/target"

echo "Copying binary from $from_dir to $to_dir...";
cp -R $from_dir $to_dir;
echo "Complete!";
