#!/usr/bin/env bash

set -e

TARGET="armv7-unknown-linux-musleabihf"

rustup target add $TARGET

cd setup_crosscompile
cargo r -- $TARGET
cd ..
cargo b --target $TARGET --release
