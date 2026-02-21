#!/bin/bash -e

cd "$(dirname "$(readlink -f "$0")")"

die() {
    echo $* >&2
    exit 1
}

rebase_failed() {
    git rebase --abort
    die "rebase failed for $*"
}

branches="${@:-sdl-dev-3.4 sdl-dev-3.6 image-dev-3.4 image-dev-3.6 ttf-dev mixer-dev net-dev shadercross-dev}"
for branch in $branches; do
    case $branch in
        sdl-dev-3.4) src_dir=sdl3-src/SDL; src_branch=origin/release-3.4.x;;
        sdl-dev-3.6) src_dir=sdl3-src/SDL; src_branch=origin/main;;
        image-dev-3.4) src_dir=sdl3-image-src/SDL_image; src_branch=origin/release-3.4.x;;
        image-dev-3.6) src_dir=sdl3-image-src/SDL_image; src_branch=origin/main;;
        ttf-dev) src_dir=sdl3-ttf-src/SDL_ttf; src_branch=origin/main;;
        mixer-dev) src_dir=sdl3-mixer-src/SDL_mixer; src_branch=origin/main;;
        net-dev) src_dir=sdl3-net-src/SDL_net; src_branch=origin/main;;
        shadercross-dev) src_dir=sdl3-shadercross-src/SDL_shadercross; src_branch=origin/main;;
        *) die "unknown branch $branch"
    esac

    git checkout "$branch"
    git rebase main || rebase_failed "$branch"
    git checkout "$branch"
    git -C "$src_dir" fetch
    git -C "$src_dir" checkout "$src_branch"
    rev="$(git -C "$src_dir" describe --tags || git -C "$src_dir" describe --all --long)"
    ./generate-and-check.sh || die "generate $branch failed" 
    git commit -a --amend -m "update $(basename $src_dir) ($rev)"
done
git checkout main
