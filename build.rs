extern crate bindgen;

use std::env;
use std::path::PathBuf;

fn main() {
    // Tell cargo to tell rustc to link the system bzip2
    // shared library.
    // println!("cargo:rustc-link-lib=bz2");

    // Tell cargo to invalidate the built crate whenever the wrapper changes
    cc::Build::new()
        .cpp(true)
        .file("./cpp/ui.cpp")
        .file("./cpp/testui.cpp")
        .file("./cpp/testtab.cpp")
        .include("/usr/include/x86_64-linux-gnu/qt5/")
        .include("/home/oem/github/ninja-analysis-framework/binja-rs/binaryninjacore-sys/binaryninja-api")
        .compile("libinterface.a");

    println!("cargo:rustc-link-lib=static=interface");

    println!("cargo:rerun-if-changed=wrapper.h");
    println!("cargo:rerun-if-changed=./cpp/ui.cpp");
    println!("cargo:rerun-if-changed=./cpp/ui.h");

    // The bindgen::Builder is the main entry point
    // to bindgen, and lets you build up options for
    // the resulting bindings.
    let bindings = bindgen::Builder::default()
        // The input header we would like to generate
        // bindings for.
        .header("wrapper.h")
        // Tell cargo to invalidate the built crate whenever any of the
        // included header files changed.
        .clang_args(&["-x", "c++", "-std=c++14"])
        .parse_callbacks(Box::new(bindgen::CargoCallbacks))
        // Finish the builder and generate the bindings.
        .generate()
        // Unwrap the Result and panic on failure.
        .expect("Unable to generate bindings");

    // Write the bindings to the $OUT_DIR/bindings.rs file.
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}
