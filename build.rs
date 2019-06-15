extern crate bindgen;

use std::env;
use std::path::PathBuf;

fn main() {

    println!("cargo:rustc-link-lib=hhlib");
    
    let nix_cflags = env::var("NIX_CFLAGS_COMPILE").unwrap();

    let bindings = bindgen::Builder::default()
        .header("wrapper.h")
        .clang_args(nix_cflags.split(" "))
        .generate()
        .expect("Unable to generate bindings");

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings.write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}
