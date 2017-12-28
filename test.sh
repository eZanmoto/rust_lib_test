#!/bin/bash

# Copyright 2017 Sean Kelleher. All rights reserved.
# Use of this source code is governed by a BSD
# licence that can be found in the LICENCE file.

set -o errexit

for i in `seq 6`; do
    printf "\n\n\n"

    cargo clean

    if [ $i -le 3 ]; then
        lib_kind='static'
        lib_target='libtest.a'
        build_lib='ar -cvq $@ $<'
    else
        lib_kind='dylib'
        lib_target='libtest.so'
        build_lib='gcc -shared -o $@ $<'
    fi

    case $i in
    1)
        echo 'build without linking'
        lib_kind=''
        ;;
    2)
        echo 'static: generate to .so filename'
        lib_target='libtest.so'
        ;;
    3)
        echo 'static: generate dynamic library to .a file'
        build_lib='gcc -shared -o $@ $<'
        ;;
    4)
        echo 'dynamic: generate to .a filename'
        lib_target='libtest.a'
        ;;
    5)
        echo 'dynamic: generate static library to .so file'
        build_lib='ar -cvq $@ $<'
        ;;
    6)
        echo 'dynamic: generate library without -shared'
        build_lib='gcc -o $@ $<'
        ;;
    esac

    (
        LIB_KIND="$lib_kind" \
            LIB_TARGET="$lib_target" \
            BUILD_LIB="$build_lib" \
            CFLAGS="$cflags" \
            cargo build -vv

        if [ "$lib_kind" = "dylib" ]; then
            libs=$(find target -name 'libtest.*')
            echo "$libs"
            LD_LIBRARY_PATH=$(dirname $libs) \
                target/debug/test
        else
            target/debug/test
        fi
    ) \
        2>&1 | sed 's/^/    /'
done
