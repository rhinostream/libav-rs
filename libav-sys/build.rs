extern crate bindgen;

use std::env;
use std::path::PathBuf;

use bindgen::builder;

fn main() {
    let dir = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap());

    println!("cargo:rerun-if-changed=wrapper_headers/avcodec.h");
    println!("cargo:rustc-link-search=native={}", dir.join("build/lib").display());
    println!("cargo:rustc-link-lib=static=avcodec");
    println!("cargo:rustc-link-lib=static=avdevice");
    println!("cargo:rustc-link-lib=static=avfilter");
    println!("cargo:rustc-link-lib=static=avformat");
    println!("cargo:rustc-link-lib=static=avutil");
    println!("cargo:rustc-link-lib=static=swresample");
    println!("cargo:rustc-link-lib=static=swscale");
    println!("cargo:rustc-link-lib=dylib=Bcrypt");
    println!("cargo:rustc-link-lib=dylib=User32");

    let bindings = builder()
        .header("wrapper_headers/avcodec.h")
        .clang_arg("-Ibuild/include")
        .parse_callbacks(Box::new(bindgen::CargoCallbacks))
        .generate().expect("unable to generate bindings");


    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("avcodec.rs"))
        .expect("Couldn't write bindings!");
}
