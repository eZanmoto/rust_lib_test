// Copyright 2017-2018 Sean Kelleher. All rights reserved.
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
    //
    // Note that we can't combine the following two statements into one `map`
    // expression because when the value returned by
    // `CString::new(arg).unwrap()` goes out of scope it will collect the value
    // being pointed to by the value returned by `as_ptr()`, and so the values
    // returned by the first `map` expression need to be kept in scope for the
    // lifetime of the values returned by the second `map` expression (see
    // https://doc.rust-lang.org/std/ffi/struct.CString.html#method.as_ptr).
    let c_strs: Vec<_> = ["hello", "world"].iter().map(|&t| { CString::new(t).unwrap() }).collect();
    let ptrs: Vec<_> = c_strs.iter().map(|c_str| c_str.as_ptr()).collect();

    unsafe {
        print_all(
            ptrs.len() as c_int,
            ptrs.as_ptr() as *const *const c_char,
        );
    }
}
