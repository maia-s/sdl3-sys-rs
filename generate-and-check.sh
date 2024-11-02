#!/bin/sh
set -e
cargo run -p sdl3-sys-gen --release
DOCS_RS=1 cargo check -p sdl3-sys --all-features
