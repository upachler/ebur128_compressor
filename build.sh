#!/bin/bash

echo "building binary with cargo..."
cargo build

echo "creating LV2 bundle..."
./bundle.sh