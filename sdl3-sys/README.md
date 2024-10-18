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

<div class="warning">

Most of the docs are generated directly from the C headers and document how
SDL works in C. Using it from Rust might work differently in some cases.
For example, macros in C are usually translated to constants or constant
functions in Rust. Documentation specific to these Rust bindings are tagged
with `sdl3-sys`.

</div>

[Browse the API at docs.rs!](https://docs.rs/sdl3-sys)

## Usage

`sdl3-sys` requires SDL version `3.1.3-preview` or later. `3.1.3-preview` is
the first ABI stable version of SDL 3.

By default, `sdl3-sys` will attempt to link to a dynamic/shared library named
`SDL3` in the default library search path, using the usual platform specific naming
convention for libraries. If your environment is set up so that this will succeed,
you don't have to do anything more than including `sdl3-sys` as a dependency of
your project.

If the SDL 3 library can be found using either `pkg-config` or `vcpkg`, you can
enable the corresponding feature to use these tools to find and get link flags
for SDL. Enable the `use-pkg-config` feature to use `pkg-config`, or the
`use-vcpkg` feature to use `vcpkg`. (Currently the `sdl3` vcpkg package hasn't
been published yet.)

You can also choose to build the SDL 3 library from source by enabling the
`build-from-source` feature. This brings in the SDL source code crate as a
dependency of `sdl3-sys` and builds it from source. You'll have to install
any dependencies SDL needs to build on your platform first for this to succeed.
Currently this may fail to run when building a shared library on some platforms
because Rust can't find the library at runtime. This will be fixed in a future
version, but in the meantime you can do a static build instead by enabling the
`link-static` feature along with `build-from-source`. The `build-from-source-static`
feature is a shortcut to enable both.

On Apple platforms, you can enable the `link-framework` feature to link to the
SDL 3 framework instead of a dylib. For the `SDL3.xcframework` that SDL provides,
`sdl3-sys` currently requires this to be located in `/Library/Frameworks`.
This will link, but you have to put your executable along with the frameworks it
needs in a signed app bundle for it to be able to run.

You can enable static linking with the `link-static` feature, but SDL doesn't
recommend doing this except on platforms where it's required. On platforms that
only support static libraries, such as emscripten, `link-static` will effectively
be enabled by default, so you don't have to explicitly enable it for those.
