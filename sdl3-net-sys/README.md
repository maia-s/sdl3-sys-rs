# sdl3-net-sys: Low level Rust bindings for SDL3_net

These are low level Rust bindings for SDL3_net, an add-on library for
[SDL 3](https://libsdl.org) for networking.
This version of `sdl3-net-sys` has bindings for SDL_net version `3.2.0`.

`sdl3-net-sys` works out of the box on any target that SDL3_net supports and doesn't require anything
else if the SDL3_net library is installed or otherwise available. On targets that support it you can
use `pkg-config` or `vcpkg` to locate the SDL3_net library if required (see below).

If the SDL3_net library isn't already available, you have the option of building it from source as
part of the build (see below), which requires CMake, a C compiler, and any dependencies that SDL3_net
needs to build on your target.

> ⓘ *Most of the docs are generated directly from the original C headers, which documents how*
> *SDL works in C. Using it from Rust might work differently in some cases.*
> *For example, macros in C are usually translated to constants or constant*
> *functions in Rust, but the docs may still refer to them as macros.*
> *Documentation specific to these Rust bindings are tagged with `sdl3-sys`.*

[Browse the API at docs.rs!](https://docs.rs/sdl3-net-sys)

## Usage

`sdl3-net-sys` is compatible with SDL_net version `3.2.0` or later.
Some APIs may require later versions; you can check availability in the documentation.
The latest available stable release is recommended.

By default, `sdl3-net-sys` will attempt to link to a dynamic/shared library named
`SDL3_net` in the default library search path, using the usual platform specific naming
convention for libraries. You can change this behaviour with the following feature flags.

| Feature | Description |
| ------- | ----------- |
| `use-pkg-config` | Use `pkg-config` to find and link the SDL_net library. |
| `use-vcpkg` | Use `vcpkg` to find and link the SDL_net library. |
| `build-from-source` | Build and link SDL_net from source. You have to install any dependencies SDL needs to build for your target first. See below for build related features. |
| `build-from-source-static` | Shortcut for enabling both the `build-from-source` and `link-static` features. This should no longer be necessary. |
| `build-static-vendored` | Build and link SDL_net from source, use vendored libraries (see below), and link it all statically. |
| `link-framework` | Link to a framework on Apple targets. This currently requires `SDL3_net.xcframework` to be located at `~/Library/Frameworks` or `/Library/Frameworks`. The built executable has to be put in a signed app bundle to be able to run. |
| `link-static` | Link SDL_net statically. |
| `no-link` | Don't link anything, and provide linking flags via Cargo metadata so you can do manual linking if desired. |

### Building from source

## Other features

| Feature | Description |
| ------- | ----------- |
| `debug-impls` | Implement the `Debug` trait for most SDL_net types. |
| `metadata`    | Enable metadata. Also see `only-metadata`. |
| `only-metadata` | Shortcut for enabling both the `metadata` and `no-link` features. Recommended when you need metadata but don't need to call into SDL_net, e.g. for build scripts. |

## Recent changes

- 0.6.0:
    - First release

See ChangeLog.md for older changes
