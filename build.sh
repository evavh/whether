#!/usr/bin/env bash

set -e

TARGET=$1

if [ ! -d "cmake-4.0" ]; then
    # Download correct version of cmake to compile grib
    mkdir -p cmake-4.0 && wget -O - "https://cmake.org/files/v4.0/cmake-4.0.0-linux-x86_64.tar.gz" | tar -xz --strip-components=1 -C cmake-4.0
fi
PATH=`pwd`/cmake-4.0/bin:$PATH

if [ -z "$TARGET" ]; then
    cargo build
else
    rustup target add $TARGET

    cd setup_crosscompile
    cargo r -- $TARGET
    cd ..
    cargo b --target $TARGET --release
fi
