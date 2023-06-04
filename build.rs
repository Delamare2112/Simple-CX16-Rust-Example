fn main() {
    println!("cargo:rustc-link-arg=/home/mos/local/extern/libc.a");
    println!("cargo:rerun-if-changed=/home/mos/local/extern/cx16.h");
    let bindings = bindgen::Builder::default()
        .header("extern/cx16.h")
        .parse_callbacks(Box::new(bindgen::CargoCallbacks))
        .use_core()
        .generate()
        .expect("Failed to generate cx16 c (kernal) bindings");
    let out_path = std::path::PathBuf::from(std::env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}
