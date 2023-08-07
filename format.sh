#!/bin/bash

echo "Formatting rust..."
cargo fmt

echo "Formatting py..."
black . -q

echo "Formatting done!"