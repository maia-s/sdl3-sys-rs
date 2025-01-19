#!/bin/sh
set -e

die() {
    echo $* >&2
    exit 1
}

require_clean=false

for arg in "$@"; do
    case "$arg" in
        --require-clean) 
            require_clean=true
            ;;
        *)
            die "unknown argument: $arg"
            ;;
    esac
done

if $require_clean; then
    git diff --quiet || die "uncommitted changes"
fi

for crate in *-sys; do
    cp -f build-common.rs $crate
done

cargo run -p sdl3-sys-gen --release

if $require_clean; then
    git diff --quiet || die "sdl3-sys-gen output didn't match committed results"
fi

for crate in *-sys; do
    DOCS_RS=1 cargo +1.79.0 check -p $crate
    DOCS_RS=1 cargo +nightly check -p $crate --all-features
done
