#![allow(unused)]
use std::env;

use std::process::Command;

#[cfg(feature = "build_as_xtop_lib")]
fn main() {
    let out_dir = env::var("OUT_DIR").unwrap();

    // Command::new("cp")
    //     .arg(&format!("/tmp/test.txt"))
    //     .arg(&format!("{}", out_dir))
    //     .status()
    //     .unwrap();

    // from: target/release/build/xevm_engine-e40834640f8d4de0/out/
    // to :|-  evm_engine_rs/debug/...
    //     |-  lib/linux/*.a
    Command::new("cp")
        .arg("-f")
        .arg(&format!(
            "{}/../../../../../lib/Linux/libxtop_mock_api.a",
            out_dir
        ))
        .arg(&format!("{}", out_dir))
        .status()
        .unwrap();

    println!("cargo:rustc-link-search=native={}", out_dir);
    println!("cargo:rustc-link-lib=static=xtop_mock_api");
}

#[cfg(not(feature = "build_as_xtop_lib"))]
fn main() {}
