#!/bin/bash

from_dir="packages/core/target/release/rsrv"
to_path="packages/cli/crates/rsrv"

if [ -f $to_path ]; then
  echo "Purging existing binary file at $to_path";
  rm $to_path;
fi

echo "Copying binary from $from_dir to $to_path...";
cp $from_dir $to_path;
echo "Complete!";
