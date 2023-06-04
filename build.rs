#![feature(exit_status_error)]

use std::io::Write;

fn llvm_sdk(out_dir: &String) {
    // I could NOT get the cmake crate to work in this docker environment.
    //  Constant 'System is unknown to cmake' with the Platform getting set to 'none' no matter
    //  how many times I set -DCMAKE_SYSTEM_NAME=Linux or -DCMAKE_SYSTEM_INFO_FILE=Platform/Linux

    std::process::Command::new("/home/mos/local/build_sdk.sh")
        .arg(&out_dir)
        .output()
        .expect("Failed to find sdk build script!")
        .status
        .exit_ok()
        .expect("SDK build script failed!");

    println!(
        "cargo:rustc-link-arg={out_dir}/mos-platform/build/install/mos-platform/cx16/lib/libc.a"
    );

    let cx16_header_path =
        format!("{out_dir}/mos-platform/build/install/mos-platform/cx16/include/cx16.h");
    let patched_header = std::fs::read_to_string(&cx16_header_path)
        .expect("Failed to read cx16.h")
        .replace("error This module", "// error This module")
        .replace("<_6522.h>", "\"_6522.h\"");
    std::fs::File::create(&cx16_header_path)
        .expect("Failed to open cx16.h for writing")
        .write_all(patched_header.as_bytes())
        .expect("Failed to write the patched cx16.h");
}

fn generate_bindings(out_dir: &String) {
    let bindings = bindgen::Builder::default()
        .header(format!(
            "{out_dir}/mos-platform/build/install/mos-platform/cx16/include/_6522.h"
        ))
        .header(format!(
            "{out_dir}/mos-platform/build/install/mos-platform/cx16/include/cx16.h"
        ))
        .parse_callbacks(Box::new(bindgen::CargoCallbacks))
        .use_core()
        .generate()
        .expect("Failed to generate cx16 c (kernal) bindings");
    let out_path = std::path::PathBuf::from(std::env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}

fn main() {
    let out_dir =
        std::env::var("OUT_DIR").expect("OUT_DIR env var not found. Are you using Cargo?");

    llvm_sdk(&out_dir);
    generate_bindings(&out_dir);
}
