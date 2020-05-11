extern crate bindgen;
extern crate cc;

use std::env;
use std::path::PathBuf;

fn main() {

    // Compile the c source code
    cc::Build::new()
        .file("c-src/nrlmsise-00.c")
        .file("c-src/nrlmsise-00_data.c")
        .compile("nrlmsise00");

    println!("cargo:rerun-if-changed=wrapper.h");

    // Create bindings
    let bindings = bindgen::Builder::default()
        .header("wrapper.h")
        .parse_callbacks(Box::new(bindgen::CargoCallbacks))
        .generate()
        .expect("Unable to generate bindings");

        // Write out bindings
        let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
        bindings
            .write_to_file(out_path.join("bindings.rs"))
            .expect("Couldn't write bindings!");
}