// extern crate protoc_rust as protoc;

// use std::env::consts;

// const OSX: &'static str = "macos";

fn main() -> () {
    println!("cargo:rerun-if-changed=build.rs");

    /*
    println!("cargo:rerun-if-changed=protos/sample.proto");

    if consts::OS == OSX {
        println!("cargo:rustc-link-search=/usr/local/Cellar/gcc/7.3.0/lib/gcc/7/");
    }

    protoc::run(protoc::Args {
        out_dir: "src/protos",
        input: &["protos/sample.proto"],
        includes: &["protos"],
    }).expect("protoc");
    */
}
