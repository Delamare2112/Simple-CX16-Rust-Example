# Simple Commander X16 Rust Example  
This project is an example of how to create a simple application that runs on the X16.  

### How to run
1. Make sure that you have Docker installed and working.  
1. Use `./build.sh` to build the application within a Docker container.  
1. You can launch the application in the X16 emulator like so:  
    `x16emu -prg ./target/mos-cx16-none/release/rust-mos-test -run`

### How does this work?
The `build.sh` script starts a docker container including a full [rust-mos](https://github.com/mrk-its/rust-mos) environment.  This way you don't have to build a custom LLVM or Rust-Lang on your host system.  
The [llvm-mos-sdk](https://github.com/XarkLabs/llvm-mos-sdk/tree/cx16_kernal_support) submodule is a fork (not mine) of [llvm-mos-sdk](https://github.com/llvm-mos/llvm-mos-sdk) which includes additional code that generates a "libc" for the Commander X16 containing many X16 specific Kernal calls.  The `build.rs` file builds the SDK (inside of the docker container) and uses rust-bindgen to generate unsafe bindings for use in this Rust project.
