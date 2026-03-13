#!/bin/sh
set -e

cd "$(dirname "$(readlink -f "$0")")"

git submodule update --init --recursive
