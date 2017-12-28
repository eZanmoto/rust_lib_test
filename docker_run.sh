#!/bin/bash

# Copyright 2017 Sean Kelleher. All rights reserved.
# Use of this source code is governed by a BSD
# licence that can be found in the LICENCE file.

set -o errexit

if [ $# -lt 1 ]; then
    echo "usage: $0 <lib-kind>"
    exit 2
fi

docker \
    run \
    --interactive \
    --rm \
    --tty \
    --volume=$(pwd):/rust_lib_test \
    --workdir=/rust_lib_test \
    rust:1.22.1-stretch \
    bash \
    run.sh \
    "$1"
