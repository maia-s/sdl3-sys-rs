#!/bin/sh
set -e
cargo run -p sdl3-sys-gen --release
DOCS_RS=1 cargo +1.79.0 check -p sdl3-sys
DOCS_RS=1 cargo +nightly check -p sdl3-sys --all-features
