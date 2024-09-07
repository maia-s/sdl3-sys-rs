#!/bin/sh
set -e
cargo run -p sdl3-sys-gen --release
cargo check -p sdl3-sys --features debug-impls
