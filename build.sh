#!/usr/bin/env bash

set -e

TARGET=$1

if [ -z "$TARGET" ]; then
    cargo build
else
    rustup target add $TARGET

    cd setup_crosscompile
    cargo r -- $TARGET
    cd ..
    cargo b --target $TARGET --release
fi
