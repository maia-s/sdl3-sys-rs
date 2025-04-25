#!/bin/sh
set -e

die() {
    echo $* >&2
    exit 1
}

require_clean=false
gen_profile=--release

for arg in "$@"; do
    case "$arg" in
        --debug)
            gen_profile=
            ;;
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
    rm -rf $crate/src/generated $crate/src/metadata/generated
done

cargo run -p sdl3-sys-gen $gen_profile

if $require_clean; then
    git diff --quiet || die "sdl3-sys-gen output didn't match committed results"
fi

for crate in *-sys; do
    cargo +1.79.0 check -p $crate --features 'no-link'
    cargo +1.79.0 check -p $crate --features 'no-link,debug-impls,metadata'
    cargo +nightly check -p $crate --all-features
done
