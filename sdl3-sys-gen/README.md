To generate bindings for your own custom versions of SDL and satellite libraries,
first check out this repository with submodules:

```shell
git clone --recurse-submodules https://codeberg.org/maia/sdl3-sys-rs.git
```

Each of the src crates contain the original source repository as a submodule.
For example, SDL itself is located in `sdl3-src/SDL`. You can make your changes there,
or point it to another repository with your custom version.

To generate the bindings, run `generate-and-check.sh` script in the repo root:

```shell
./generate-and-check.sh
```

This will update all sys crates with the newly generated bindings.
