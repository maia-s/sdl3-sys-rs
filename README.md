# sdl3-sys

This is a work in progress of the "new" sdl3-sys for Rust! The bindings themselves are in sdl3-sys,
and the generator that generates them is in sdl3-sys-gen.

### Why not use bindgen?

Because we can do better, and the strict syntax of SDL's headers means we don't have to support all of
C to do so. `sdl3-sys-gen` makes platform independent bindings with full documentation from the original
headers and collects the output into modules. The generator is standalone.
