#!/bin/sh

cd "$(dirname "$0")"

die() {
    echo error >&2
    exit 1
}

run() {
    echo $*
    "$@" || die
}

for target in compiled/*; do
    target="${target#compiled/}"
    for source in source/*; do
        source="${source#source/}"
        source="${source%.hlsl}"
        run shadercross "source/$source.hlsl" -o "compiled/$target/$source.$target"
    done
done
