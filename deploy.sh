#!/usr/bin/env bash

set -e

TARGET="armv7-unknown-linux-musleabihf"

./build.sh $TARGET
scp target/armv7-unknown-linux-musleabihf/release/whether eva@pi:/home/eva/bin/whether
