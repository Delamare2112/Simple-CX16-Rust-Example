#!/bin/sh
cd "$1"
cmake /home/mos/local/llvm-mos-sdk -DPLATFORM=cx16
cmake --build . --target cx16-examples
