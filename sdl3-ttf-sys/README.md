# sdl3-ttf-sys: Low level Rust bindings for SDL3_ttf

These are low level Rust bindings for SDL3_ttf, an add-on library for
[SDL 3](https://libsdl.org) for rendering text.
This version of `sdl3-ttf-sys` has bindings for SDL_ttf versions `3.2.0` to `3.2.2`, inclusive.

<div class="warning">

Most of the docs are generated directly from the C headers and document how
SDL works in C. Using it from Rust might work differently in some cases.
For example, macros in C are usually translated to constants or constant
functions in Rust. Documentation specific to these Rust bindings are tagged
with `sdl3-sys`.

</div>

[Browse the API at docs.rs!](https://docs.rs/sdl3-ttf-sys)

## Usage

`sdl3-ttf-sys` is compatible with SDL_ttf version `3.2.0` or later.
Some APIs require later versions; you can check availability in the documentation.
The latest available stable release is recommended.

By default, `sdl3-ttf-sys` will attempt to link to a dynamic/shared library named
`SDL3_ttf` in the default library search path, using the usual platform specific naming
convention for libraries. You can change this behaviour with the following feature flags.

| Feature | Description |
| ------- | ----------- |
| `use-pkg-config` | Use `pkg-config` to find and link the SDL_ttf library. |
| `use-vcpkg` | Use `vcpkg` to find and link the SDL_ttf library. |
| `build-from-source` | Build and link SDL_ttf from source. You have to install any dependencies SDL needs to build for your target first. See below for build related features. |
| `build-from-source-static` | Shortcut for enabling both the `build-from-source` and `link-static` features. This should no longer be necessary. |
| `build-static-vendored` | Build and link SDL_ttf from source, use vendored libraries (see below), and link it all statically. |
| `link-framework` | Link to a framework on Apple targets. This currently requires `SDL3_ttf.xcframework` to be located at `~/Library/Frameworks` or `/Library/Frameworks`. The built executable has to be put in a signed app bundle to be able to run. |
| `link-static` | Link SDL_ttf statically. |
| `no-link` | Don't link anything, and provide linking flags via Cargo metadata so you can do manual linking if desired. |

### Building from source

When building from source with the `build-from-source` feature flag, you can use features to
configure which backends and image formats to support. Features marked with "(cmake)" below all
have an `sdlttf-` prefix and correspond to SDL_ttf's cmake variables. They're autodetected if
not set. You can use a `no-` prefix to disable a cmake feature, for example `no-sdlttf-harfbuzz`
disables harfbuzz support. Activated features override features disabled with the `no-` prefix.

#### Linking and vendoring

| Feature (cmake) | Description |
| --------------- | ----------- |
| `vendored`      | Build dependencies from source as part of building SDL_ttf |

#### Rendering

| Feature (cmake) | Description |
| --------------- | ----------- |
| `harfbuzz`      | Use harfbuzz to improve text shaping |
| `plutosvg`      | Use plutosvg for color emoji support |

## Other features

| Feature | Description |
| ------- | ----------- |
| `debug-impls` | Implement the `Debug` trait for most SDL_ttf types. |
| `metadata`    | Enable metadata. |
| `only-metadata` | Shortcut for enabling both the `metadata` and `no-link` features. |

## Recent changes

- 0.6.0 WIP:
    - Update sdl3-sys to 0.6.0
    - Synchronize version number with sdl3-sys
    - Add metadata
    - Remove deprecated aliases
    - Make some functions safe and/or const

- next:
    - Expose `TTF_TextData`
    - Fix an issue where creating symlinks after a build could fail
    - Fix copying the built library when cross compiling

- 0.2.0:
    - Update sdl3-sys to 0.5.0
    - Add `no-link` feature
    - On Apple targets, look for frameworks in `~/Library/Frameworks` too

See ChangeLog.md for older changes
