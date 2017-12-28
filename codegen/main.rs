// Copyright 2017 Sean Kelleher. All rights reserved.
// Use of this source code is governed by a BSD
// licence that can be found in the LICENCE file.

use std::ffi::CString;

extern crate libc;

use libc::c_char;
use libc::c_int;

extern {
    fn print_all(argc: i32, argv: *const *const c_char) -> i32;
}

fn main() {
    // https://github.com/rust-lang/rust/issues/9564#issuecomment-95354558
    // NOTE can't allocate in one go - note why
    let c_strs: Vec<_> = ["hello", "world"].iter().map(|&t| { CString::new(t).unwrap() }).collect();
    let ptrs: Vec<_> = c_strs.iter().map(|c_str| c_str.as_ptr())
        .collect();

    unsafe {
        print_all(
            ptrs.len() as c_int,
            ptrs.as_ptr() as *const *const c_char,
        );
    }
}
