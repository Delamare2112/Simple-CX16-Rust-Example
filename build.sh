#!/bin/sh
docker run -it -v .:/home/mos/local mrkits/rust-mos sh -c 'cd local; cargo build --release --target mos-cx16-none -Z build-std=core,alloc'
