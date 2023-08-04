#!/bin/bash

echo "Formatting cpp..."
find ./src -name '*.cpp' -o -name '*.h' | xargs clang-format --style=file -i

echo "Formatting py..."
black . -q

echo "Formatting done!"