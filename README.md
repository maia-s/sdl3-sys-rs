# sdl3-sys

See the [`sdl3-sys`](sdl3-sys) dir for the SDL 3 Rust bindings.

- [`sdl3-sys`](sdl3-sys): Rust bindings for SDL 3
- [`sdl3-src`](sdl3-src): Source code crate for SDL 3, used by `sdl3-sys` when building from source.
  This contains the official SDL repository as a git submodule.
- [`sdl3-sys-gen`](sdl3-sys-gen): Parser and generator that generates `sdl3-sys` from the official SDL 3 headers.
- [`sdl3-main`](sdl3-main): Tools for using SDL's main and callback APIs.
- [`sdl3-main-macros`](sdl3-main-macros): Proc-macros for `sdl3-main`.

### Why not use bindgen?

Because we can do better. `sdl3-sys-gen` makes platform independent bindings with full documentation
from the original headers and collects the output into modules. The generator is standalone.
