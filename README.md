# sdl3-sys

(I'm currently on vacation until the end of August)

This is a work in progress of the "new" sdl3-sys for Rust! The bindings themselves will be in the
sdl3-sys crate in the sdl3-sys dir, but currently that's mostly empty as I'm working on finishing the
generator, in sdl3-sys-gen.

You can look at `stdinc.rs` in sdl3-sys/src/generated for an idea of what the bindings will be like.
(Note this currently doesn't output anything for #define'd values. They're parsed and available
to the generator, they're just not emitted yet.)

### Why not use bindgen?

Because we can do better, and the strict syntax of SDL's headers means we don't have to support all of
C to do so. `sdl3-sys-gen` makes platform independent bindings with full documentation from the original
headers and collects the output into modules. The generator is standalone.
