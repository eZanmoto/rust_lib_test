// Copyright 2017 Sean Kelleher. All rights reserved.
// Use of this source code is governed by a BSD
// licence that can be found in the LICENCE file.

use std::env;
use std::path::Path;
use std::process::Command;

fn main() {
    let out_dir = env::var("OUT_DIR").unwrap();

    assert_eq!(
        true,
        Command::new("cp")
            .args(&[
                "--recursive",
                "-v",
                "codegen",
                &out_dir,
            ])
            .status()
            .unwrap()
            .success()
    );

    let libtest_out_dir = format!("{}/codegen/libtest", &out_dir);
    assert_eq!(
        true,
        Command::new("sed")
            .args(&[
                "-i",
                "-e", &format!("s/TARGET/{}/", env::var("LIB_TARGET").unwrap()),
                "-e", &format!("s/BUILD_LIB/{}/", env::var("BUILD_LIB").unwrap()),
                &format!("{}/Makefile", libtest_out_dir),
            ])
            .status()
            .unwrap()
            .success()
    );

    assert_eq!(
        true,
        Command::new("make")
            .current_dir(&Path::new(&libtest_out_dir))
            .status()
            .unwrap()
            .success()
    );

    println!("cargo:rustc-link-search=native={}", &libtest_out_dir);

    let lib_kind = env::var("LIB_KIND").unwrap();
    if lib_kind.len() != 0 {
        // We use a cargo directive in `build.rs` instead of a link directive in
        // `main.rs` as recommended in
        // http://doc.crates.io/build-script.html#case-study-building-some-native-code.
        // This allows us to avoid hardcoding the linking in the source file.
        println!("cargo:rustc-link-lib={}=test", lib_kind);
    }
}
