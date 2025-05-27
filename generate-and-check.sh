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
    rm -rf $crate/src/generated
done

cargo run -p sdl3-sys-gen $gen_profile

for crate in sdl3-main; do
    version="$(grep version $crate/Cargo.toml | head -1 | sed -e 's,version = "\([^"]*\)",\1,')"
    cp $crate/README.md $crate/README.md.inc
    sed -e "s,<https://docs.rs/$crate/[^/]*/,<https://docs.rs/$crate/$version/," $crate/README.md.inc >$crate/README.md
    grep -v "]: <https://docs.rs/$crate/" $crate/README.md >$crate/README.md.inc
done

if $require_clean; then
    git diff --quiet || die "sdl3-sys-gen output didn't match committed results"
fi

for crate in *-sys; do
    cargo +1.79.0 check -p $crate --features 'no-link,debug-impls'
    DOCS_RS=1 cargo +nightly check -p $crate --all-features
done
for crate in sdl3-main; do
    cargo +1.79.0 check -p $crate
    cargo +1.79.0 check -p $crate --features 'std'
    cargo +nightly check -p $crate --all-features
done
