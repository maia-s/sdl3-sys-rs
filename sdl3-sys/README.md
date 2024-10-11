# sdl3-sys (preview release)

This is a preview release of the `sdl3-sys` SDL 3 bindings for Rust!

SDL 3 is now ABI stable as of the 3.1.3 preview release, but `sdl3-sys` itself
is still considered unstable. The generator that parses and generates these
bindings is new and may have bugs. In particular, functions and constants
generated from C defines with inferred types may need to have their types
changed.

Please submit an issue at github if you have any issues or comments!

Known issues:
- Satellite libraries (mixer, image, ttf) aren't available yet
- There are no tests yet, except for static asserts translated from the
  original headers
- Some less common targets are missing detection or features to enable
  corresponding SDL features
- Some external types are generated as opaque structs; these could use a
  feature to use a definition from another crate (e.g. vulkan types)
- Documentation could be formatted better
