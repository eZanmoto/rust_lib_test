#!/bin/bash

# Copyright 2017 Sean Kelleher. All rights reserved.
# Use of this source code is governed by a BSD
# licence that can be found in the LICENCE file.

docker \
    run \
    --interactive \
    --rm \
    --tty \
    --volume=$(pwd):/rust_lib_test \
    --workdir=/rust_lib_test \
    rust:1.22.1-stretch \
    bash \
    test.sh
