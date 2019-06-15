extern crate bindgen;

use std::env;
use std::path::PathBuf;

fn main() {

    let nix_cflags = env::var("NIX_CFLAGS_COMPILE").unwrap();
    println!("cargo:rustc-link-search=/nix/store/32krsb2xn6pss05syvivqxp67pv1ma7c-hydraHarpLib-3.0.0.2/lib");
    println!("cargo:rustc-link-lib=hh");

    let bindings = bindgen::Builder::default()
        .header("wrapper.h")
        .clang_args(nix_cflags.split(" "))
        .generate()
        .expect("Unable to generate bindings");

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings.write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}
