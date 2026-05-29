#!/bin/sh
set -e

cd "$(dirname "$(readlink -f "$0")")"

die() {
    echo $* >&2
    exit 1
}

pre_gen() {
    for crate in "$@"; do
        case "$crate" in
            *-sys)
                cp -f build-common.rs $crate
                rm -rf $crate/src/generated $crate/src/metadata/generated
                ;;
        esac
    done
}

post_gen() {
    for crate in "$@"; do
        case "$crate" in
            sdl3-main)
                version="$(grep version $crate/Cargo.toml | head -1 | sed -e 's,version = "\([^"]*\)",\1,')"
                cp $crate/README.md $crate/README.md.inc
                sed -e "s,<https://docs.rs/$crate/[^/]*/,<https://docs.rs/$crate/$version/," $crate/README.md.inc >$crate/README.md
                grep -v "]: <https://docs.rs/$crate/" $crate/README.md >$crate/README.md.inc
                ;;
        esac
    done
}

check() {
    for crate in "$@"; do
        case "$crate" in
            *-sys)
                cargo +1.85.0 check -p $crate --features 'no-link'
                cargo +1.85.0 check -p $crate --features 'no-link,debug-impls,metadata'
                cargo +nightly check -p $crate --all-features
                ;;
            sdl3-main)
                cargo +1.85.0 check -p $crate
                cargo +1.85.0 check -p $crate --features 'std'
                cargo +nightly check -p $crate --all-features
                ;;
        esac
    done
}

main() {
    require_clean=false
    gen_profile=--release
    crates=
    set_arg=none

    for arg in "$@"; do
        case "$arg" in
            --crate)
                set_arg="$arg"
                ;;
            --debug)
                set_arg=none
                gen_profile=
                ;;
            --require-clean)
                set_arg=none
                require_clean=true
                ;;
            *)
                case "$set_arg" in
                    --crate)
                        crates="$crates $arg"
                        ;;
                    none)
                        die "unknown argument: $arg"
                        ;;
                esac
                set_arg=none
                ;;
        esac
    done

    if [ -z "$crates" ]; then
        crates=*-sys
        crates="$crates sdl3-main"
    fi

    if $require_clean; then
        git diff --quiet || die "uncommitted changes"
    fi

    pre_gen $crates

    cargo run -p sdl3-sys-gen $gen_profile

    post_gen $crates

    if $require_clean; then
        git diff --quiet || die "sdl3-sys-gen output didn't match committed results"
    fi

    check $crates
}

main "$@"
