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

main() {
    push=$1
    shift

    git fetch --prune --tags
    git update-index --refresh
    git diff-index --quiet HEAD -- || (git diff-index HEAD --; die "git isn't clean")

    branches="${@:-$(git ls-remote --branches | grep -- -dev | sed -e 's,.*refs/heads/,,')}"
    for branch in $branches; do
        if git checkout "origin/$branch"; then
            branch_base="${branch%%-dev*}"
            branch_ver="${branch##*-dev-}"
            crate="sdl3-$branch_base-sys"
            src_crate="sdl3-$branch_base-src"
            if [[ "$crate" == "sdl3-sdl-sys" ]]; then
                crate=sdl3-sys
                src_crate=sdl3-src
            fi
            src_dir="$src_crate/$(grep lib_dir: $crate/config.txt | sed 's/lib_dir:[[:space:]]*//')"
            if [[ "$branch_ver" == "$branch_base-dev" ]]; then
                src_branch=origin/main
            else
                src_branch="origin/release-$branch_ver.x"
            fi

            git rebase main || rebase_failed "$branch"
            ./update-submodules.sh
            dist="$(git rev-list --count main..HEAD)"
            if [ "$dist" -gt "1" ]; then
                # squash
                message="$(git log -1 --pretty=%B)"
                git reset --soft HEAD~1
                git commit --amend -m "$message"
                ./update-submodules.sh
            fi
            hash="$(git -C "$src_dir" rev-parse HEAD)"
            git -C "$src_dir" fetch --prune --tags
            git -C "$src_dir" checkout "$src_branch"
            new_hash="$(git -C "$src_dir" rev-parse HEAD)"
            if [[ "$hash" != "$new_hash" ]]; then
                rev="$(git -C "$src_dir" describe --tags || git -C "$src_dir" describe --all --long)"
                time="$(git -C "$src_dir" show -s --format=%ci HEAD)"
                ./generate-and-check.sh --crate "$crate" || die "generate $branch/$crate failed"
                git diff --exit-code || git commit -a -m "$(basename $src_dir) $rev @ $time"
            fi
            if $push; then
                git push --force-with-lease origin "HEAD:$branch" || die "push failed"
            fi
        else
            echo "skipping $branch"
        fi
    done

    git checkout main
    ./update-submodules.sh
    exit
}

push=true

args=()
for arg in "$@"; do
    case "$arg" in
        --local) push=false;;
        *) args+=("$arg");;
    esac
done

main $push "$args"
