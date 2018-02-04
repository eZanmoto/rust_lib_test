README
======

About
-----

This repository explores the two methods for building a Rust executable against
a C library: statically and dynamically.

`codegen` would be used as the `src` directory in a regular Rust project, i.e.
the files in `codegen` would be in `src` instead. We use code generation in this
project to show the different errors that can occur when building Rust code
against C libraries.

Use `bash run.sh static` to generate, build, and run the code for a rust project
that uses a statically linked library. Use `bash run.sh dylib` to do the same
for a dynamically linked library.

`bash test.sh` runs a set of tests that generates, builds and runs the Rust
project with incorrect configurations. The errors are presented with their
reasoning in the "Tests" section. A reverse mapping of errors to their causes
and solutions is detailed in the next section, "Solutions".

Note that Rust doesn't require C header files in order to link with C-based
libraries, which can be demonstrated by running the examples after removing
`print.h`, and removing the include statement for it in `print.c`.

Solutions
---------

This section outlines solutions, based on errors that may be encountered when
building Rust against C code.

* Error: `undefined reference to ``main'`
* Cause: A library is being compiled as an executable.
* Solutions:
  1. If a static library is desired, replace the `gcc -o` call for building the
     dynamic library with a call to `ar -cvq` to build a static library. For
     example, if building a library using `gcc -o libhello.a hello.o`, then
     replace this call with `ar -cvq libhello.a hello.o`.
  2. If a dynamic library is desired, add a `-shared` flag to the `gcc` call for
     building the dynamic library. For example, if building a library using `gcc
     -o libhello.so hello.o`, then add `-shared` to get `gcc -shared -o
     libhello.so hello.o`.

* Error: `attempted static link of dynamic object ``${OUT_DIR}/codegen/libtest/libtest.a'`
* Cause: The static library `libtest.a` was built as a dynamic library.
* Solution: Replace the `gcc -shared -o` call that builds the dynamic library
  with a call to `ar -cvq`. For Example, if building a library using
  `gcc -shared -o libhello.a hello.o`, then replace this call with
  `ar -cvq libhello.a hello.o`.

* Error: `cannot find -lx`
* Cause: None of the linked search directories contain a file called `libx.a`.
* Solution: Ensure that your static library, `libx.a` for example, is being
  generated, and that the directory that it is in, `dir` for example, is being
  output by `build.rs`, in the form `cargo:rustc-link-search=native=dir`.

* Error: `undefined reference to ``x'`
* Cause: None of the linked libraries (if any) contained a definition for `x`.
* Solution: Ensure that `build.rs` prints either `cargo:rustc-link-lib=static=x`
  or `cargo:rustc-link-lib=dylib=x` (replace `x` with the name of the library
  that contains the function `x`). For example, if `libhello.a` was created and
  contains the definition for `x`, then it can be linked by having `build.rs`
  output `cargo:rustc-link-lib=static=hello`.

Note that statically building Rust code by either (1) generating a dynamic
library to a static library name, such as `libhello.a`, or (2) generating a
static library to a dynamic library name, such as `libhello.so`, will build an
executable successfully, but may result in confusion during later maintenance
and debugging.

Tests
-----

### Build without linking

In this test, `build.rs` does not output a `cargo:rustc-link-lib` line (and
`main.rs` doesn't contain a `link` directive).

`build.rs` outputs:

    cargo:rustc-link-search=native=${OUT_DIR}/codegen/libtest

Building `main.rs` produces the following error:

    /rust_lib_test/target/debug/deps/test-8959f10d29a724b7.test13.rust-cgu.o: In function `test::main':
    ${OUT_DIR}/codegen/main.rs:24: undefined reference to `print_all'

This error occurs because a referenced function is not defined in the final
executable.

### Static Library

These tests attempt to build `main.rs` against a statically-linked library.

#### Generate to `libtest.so`

In this test, the static library is generated with a filename of `libtest.so`.

The commands to build the library are:

    gcc -c -I. -o print.o print.c
    ar -cvq libtest.so print.o

The `build.rs` `cargo` commands to link the library are:

    cargo:rustc-link-search=native=${OUT_DIR}/codegen/libtest
    cargo:rustc-link-lib=static=test

Building `main.rs` produces the following error:

    /usr/bin/ld: cannot find -ltest

This error occurs because a file of the form `lib*.a` cannot be found in any of
the link search directories. Note that this is the same error that would be
output in the case that `build.rs` didn't output the `cargo:rustc-link-search`
line.

#### Generate dynamic library

In this test a, dynamic library is generated with a filename of `libtest.a`,
which the linker will attempt to link as a static library.

The commands to build the library are:

    gcc -c -I. -o print.o print.c
    gcc -shared -o libtest.a print.o

The `build.rs` `cargo` commands to link the library are:

    cargo:rustc-link-search=native=${OUT_DIR}/codegen/libtest
    cargo:rustc-link-lib=static=test

Building `main.rs` produces the following error:

    /usr/bin/ld: attempted static link of dynamic object `${OUT_DIR}/codegen/libtest/libtest.a'

This error occurs because the linker attempted to link a dynamic library
statically.

### Dynamic Library

These tests attempt to build `main.rs` against a dynamically-linked library.

#### Generate to `libtest.a`

In this test, the dynamic library is generated with a filename of `libtest.a`.

The commands to build the library are:

    gcc -c -I. -o print.o print.c
    gcc -shared -o libtest.a print.o

The `build.rs` `cargo` commands to link the library are:

    cargo:rustc-link-search=native=${OUT_DIR}/codegen/libtest
    cargo:rustc-link-lib=dylib=test

This test actually builds and runs the executable successfully. The dynamic
library is actually built against statically, as can be confirmed by running the
built executable `target/debug/test` without supplying `LD_LIBRARY_PATH`, which
runs successfully.

#### Generate static library to `libtest.so`

The commands to build the library are:

    gcc -c -I. -o print.o print.c
    ar -cvq libtest.so print.o

The `build.rs` `cargo` commands to link the library are:

    cargo:rustc-link-search=native=${OUT_DIR}/codegen/libtest
    cargo:rustc-link-lib=dylib=test

As above, the test builds the executable statically and runs it successfully.
See the previous section for details.

#### Generate without `-shared`

The commands to build the library are:

    gcc -c -I. -o print.o print.c
    gcc -o libtest.so print.o

Building the C library produces the following error:

    /usr/lib/gcc/x86_64-linux-gnu/6/../../../x86_64-linux-gnu/Scrt1.o: In function `_start':
    (.text+0x20): undefined reference to `main'

This error occurs before `cargo` attempts to build `main.rs`. It occurs because,
without the `-shared` flag, `gcc` attempts to build `libtest.so` as an
executable instead of a dynamic library. But building an executable requires a
`main` function to be defined, which isn't defined in `print.o`/`print.c`, and
so the build fails.
