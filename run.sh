#!/bin/bash

# Copyright 2017 Sean Kelleher. All rights reserved.
# Use of this source code is governed by a BSD
# licence that can be found in the LICENCE file.

set -o errexit

if [ $# -lt 1 ]; then
    echo "usage: $0 <lib-kind>"
    exit 2
fi

cargo clean

case "$1" in
static)
    LIB_KIND='static' \
        LIB_TARGET='libtest.a' \
        BUILD_LIB='ar -cvq $@ $<' \
        cargo build -vv
    target/debug/test
    ;;

dylib)
    LIB_KIND='dylib' \
        LIB_TARGET='libtest.so' \
        BUILD_LIB='gcc -shared -o $@ $<' \
        cargo build -vv
    LD_LIBRARY_PATH=$(dirname $(find target -name 'libtest.so')) \
        target/debug/test
    ;;
*)
    echo '`lib_kind` must be `static` or `dylib`'
    exit 2
esac
