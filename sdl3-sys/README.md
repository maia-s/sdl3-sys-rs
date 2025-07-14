# sdl3-sys: Low level Rust bindings for SDL 3

These are low level Rust bindings for SDL, the [Simple DirectMedia Layer](https://libsdl.org).
This version of `sdl3-sys` has bindings for SDL versions `3.2.0` to `3.2.18`, inclusive.

Many types can be initialized to all zero with the `Default` trait for convenience.
However, many of these aren't valid when passed to SDL without further modification.
They're intended to be used with `..Default::default()` in initializers.
The `Default` impl of interface types also sets the version field to the correct value.

Add-on crates:
- [`sdl3-main`](https://crates.io/crates/sdl3-main): Tools for using SDL's main and callback interface
- [`sdl3-image-sys`](https://crates.io/crates/sdl3-image-sys): Bindings for SDL3_image
- [`sdl3-ttf-sys`](https://crates.io/crates/sdl3-ttf-sys): Bindings for SDL3_ttf

Other satellite libraries aren't stable yet, but will be released as they're available.

<div class="warning">

Most of the docs are generated directly from the C headers and document how
SDL works in C. Using it from Rust might work differently in some cases.
For example, macros in C are usually translated to constants or constant
functions in Rust. Documentation specific to these Rust bindings are tagged
with `sdl3-sys`.

</div>

[Browse the API at docs.rs!](https://docs.rs/sdl3-sys)

If you're looking for more idiomatic or higher level bindings, check out the
[sdl3](https://crates.io/crates/sdl3) crate.

## Usage

`sdl3-sys` is compatible with SDL version `3.1.3` or later.
Some APIs require later versions; you can check availability in the documentation.
The latest available stable release is recommended.

By default, `sdl3-sys` will attempt to link to a dynamic/shared library named
`SDL3` in the default library search path, using the usual platform specific naming
convention for libraries. You can change this behaviour with the following feature flags.

| Feature | Description |
| ------- | ----------- |
| `use-pkg-config` | Use `pkg-config` to find and link the SDL 3 library. |
| `use-vcpkg` | Use `vcpkg` to find and link the SDL 3 library. |
| `build-from-source` | Build and link SDL 3 from source. You have to install any dependencies SDL needs to build for your target first. See below for build related features. |
| `build-from-source-static` | Shortcut for enabling both the `build-from-source` and `link-static` features. This should no longer be necessary. |
| `link-framework` | Link to a framework on Apple targets. This currently requires `SDL3.xcframework` to be located at `~/Library/Frameworks` or `/Library/Frameworks`. The built executable has to be put in a signed app bundle to be able to run. |
| `link-static` | Link SDL statically. SDL doesn't recommend doing this. <ul><li>On targets that only support static linking, such as emscripten, you don't have to enable this feature.</li><li>On Apple targets, this currently requires frameworks that should be optional.</li></ul> |
| `no-link` | Don't link anything, and provide linking flags via Cargo metadata so you can do manual linking if desired. |

### Building from source

When building from source with the `build-from-source` feature flag, you can enable these
additional features to configure the build. They have no effect when not building from source.
They correspond to SDL CMake variables, and you can prefix them with `no-` to disable them,
e.g. `no-sdl-libc` to not link with the system C library. If you both enable and disable a
feature, enable takes precedence.

Most of these features only work on some targets; don't enable them unless you need them.

| Feature                  | Description |
| ------------------------ | ----------- |
| `sdl-asan`               | Compile SDL with Address Sanitizer. |
| `sdl-ccache`             | Compile SDL with Ccache. |
| `sdl-gpu-dxvk`           | Build SDL GPU with DXVK support. |
| `sdl-libc`               | Link SDL with system C library (default). Use `no-sdl-libc` to disable. |
| `sdl-rpath`              | Set RPATH when linking SDL (default on some targets). Use `no-sdl-rpath` to disable. |
| `sdl-unix-console-build` | Allow building SDL without X11 or Wayland support on Linux and other targets that usually use X11/Wayland. By default, SDL requires either X11 or Wayland on these targets as a sanity check. |

## Target specific features
| Feature | Description |
| ------- | ----------- |
| `target-gdk` | Enable APIs that require Microsoft's Game Development Kit (GDK). (This is not related to Gnome's GDK.) |

## Optional integrations

`sdl3-sys` can use definitions from other crates for some foreign types that it needs,
e.g. for Vulkan types. By default it'll use opaque structs or pointers to opaque structs
for these types unless otherwise specified.

| Feature | Description |
| ------- | ----------- |
| `use-ash-v0-38` | Use Vulkan types from the `ash` crate (v0.38). |
| `use-libc-v0-2` | Use `wchar_t` type from the `libc` crate (v0.2). By default `sdl3-sys` will alias `wchar_t` to `u16` on Windows and `u32` otherwise. |
| `use-windows-sys-*` | Use Windows types from the `windows-sys` crate. <ul><li>`use-windows-sys-v0-59`: v0.59</li><li>`use-windows-sys-v0-60`: v0.60</li></ul> |
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
| `metadata`    | Enable metadata. |
| `only-metadata` | Shortcut for enabling both the `metadata` and `no-link` features. |
| `nightly` | Enable features that need the nightly compiler. This enables the `VaList` type, as well as enabling some intrinsics. |

## Recent changes

- 0.6.0 WIP:
    - Make IDs and flag types newtypes
    - Add metadata
    - Remove deprecated aliases
    - Change return type of `SDL_Vulkan_GetVkGetInstanceProcAddr`
    - MSRV 1.81

- 0.5.3:
    - Update SDL to 3.2.18
    - Copy built library to top level target dir on all targets, not just Windows.

- 0.5.2:
    - Update SDL to 3.2.16

- 0.5.1:
    - Update SDL to 3.2.14
    - Add deprecation warning to padding fields. These shouldn't be accessed directly;
      use `..Default::default()` to init them with struct update syntax.

- 0.5.0:
    - Update SDL to 3.2.12

See ChangeLog.md for older changes
