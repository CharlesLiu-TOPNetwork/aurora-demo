#![allow(unused)]
use std::env;

use std::process::Command;

#[cfg(feature = "build_as_xtop_lib")]
fn main() {
    let out_dir = env::var("OUT_DIR").unwrap();

    // from: target/release/build/xevm_engine-e40834640f8d4de0/out/
    // to :|-  evm_engine_rs/debug/...
    //     |-  lib/linux/*.a
    Command::new("cp")
        .arg("-f")
        .arg(&format!(
            "{}/../../../../../lib/Linux/libxevm_runtime.a",
            out_dir
        ))
        .arg(&format!("{}", out_dir))
        .status()
        .unwrap();

    println!("cargo:rustc-link-search=native={}", out_dir);
    println!("cargo:rustc-link-lib=static=xevm_runtime");
}

#[cfg(not(feature = "build_as_xtop_lib"))]
fn main() {}