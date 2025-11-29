#!/usr/bin/env bash

set -e

cargo build --release
scp target/release/whether eva@asgard:/home/eva/bin/whether
scp ~/.config/whether.toml eva@asgard:/home/eva/.config/whether.toml
