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
convention for libraries. You can change this behaviour with the following feature flags.

| Feature | Description |
| ------- | ----------- |
| `use-pkg-config` | Use `pkg-config` to find and get link flags for the SDL 3 library. |
| `use-vcpkg` | Use `vcpkg` to find and get link flags for the SDL 3 library. |
| `build-from-source` | Build and link SDL 3 from source. You have to install any dependencies SDL needs to build on your platform first. |
| `build-from-source-static` | Shortcut for enabling both the `build-from-source` and `link-static` features. |
| `link-framework` | Link to a framework on Apple platforms. This currently requires `SDL3.xcframework` to be located at `/Library/Frameworks`. The built executable has to be put in a signed app bundle to be able to run. |
| `link-static` | Link SDL statically. SDL doesn't recommend doing this. On platforms that only support static linking, such as emscripten, you don't have to enable this feature. |

## Optional integrations

`sdl3-sys` can use definitions from other crates for some foreign types that it needs,
e.g. for Vulkan types. By default it'll use opaque structs for these types unless
otherwise specified.

| Feature | Description |
| ------- | ----------- |
| `use-ash-v0-38` | Use Vulkan types from the `ash` crate (v0.38). |
| `use-libc-v0-2` | Use `wchar_t` type from the `libc` crate (v0.2). By default `sdl3-sys` will alias `wchar_t` to `u16` on Windows and `u32` otherwise. |
| `use-x11-v2` | Use X11 types from the `x11` crate (v2). |
| `use-x11-dl-v2` | Use X11 types from the `x11-dl` crate (v2). |

## Assert level

You can set the default assertion level for SDL using the `assert-level-*` features.
This affects the assert macros in the `assert` module and the value of the `SDL_ASSERT_LEVEL`
constant.

If no `assert-level-*` features are enabled, `assert-level-debug` will be enabled by default
for debug builds, and `assert-level-release` otherwise.

These features are mutually exclusive. Features higher in this list override later ones.

| Feature | Description |
| ------- | ----------- |
| `assert-level-disabled` | 0: All SDL assertion macros are disabled. |
| `assert-level-release` | 1: Release settings: `SDL_assert` disabled, `SDL_assert_release` enabled. |
| `assert-level-debug` | 2: Debug settings: `SDL_assert` and `SDL_assert_release` enabled. |
| `assert-level-paranoid` | 3: Paranoid settings: All SDL assertion macros enabled, including `SDL_assert_paranoid`. |

## Other features

| Feature | Description |
| ------- | ----------- |
| `debug-impls` | Implement the `Debug` trait for most SDL types. |
| `nightly` | Enable features that need the nightly compiler. This enables the `VaList` type that is only available in nightly, as well as enabling some intrinsics. |
