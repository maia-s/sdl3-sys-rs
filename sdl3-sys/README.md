# sdl3-sys: Low level Rust bindings for SDL 3

These are low level Rust bindings for SDL, the [Simple DirectMedia Layer](https://libsdl.org).
This version of `sdl3-sys` has bindings for SDL versions `3.2.0` to `3.4.10`, inclusive.

`sdl3-sys` works out of the box on any target that SDL 3 supports and doesn't require anything
else if the SDL library is installed or otherwise available. On targets that support it you can
use `pkg-config` or `vcpkg` to locate the SDL library if required (see below).

If the SDL library isn't already available, you have the option of building it from source as
part of the build (see below), which requires CMake, a C compiler, and any dependencies that SDL
needs to build on your target.
Builds for web are supported via Emscripten (`wasm32-unknown-emscripten`).

SDL 3's callback API can be used via the [`sdl3-main`](https://crates.io/crates/sdl3-main) crate.

> ⓘ *Most of the docs are generated directly from the original C headers, which documents how*
> *SDL works in C. Using it from Rust might work differently in some cases.*
> *For example, macros in C are usually translated to constants or constant*
> *functions in Rust, but the docs may still refer to them as macros.*
> *Documentation specific to these Rust bindings are tagged with `sdl3-sys`.*

[Browse the API at docs.rs!](https://docs.rs/sdl3-sys)

## Related crates

### Add-ons and satellite libraries

- [`sdl3-main`](https://crates.io/crates/sdl3-main): Tools for using SDL's main and callback interface
- [`sdl3-image-sys`](https://crates.io/crates/sdl3-image-sys): Bindings for SDL3_image
- [`sdl3-mixer-sys`](https://crates.io/crates/sdl3-mixer-sys): Bindings for SDL3_mixer
- [`sdl3-net-sys`](https://crates.io/crates/sdl3-net-sys): Bindings for SDL3_net
- [`sdl3-ttf-sys`](https://crates.io/crates/sdl3-ttf-sys): Bindings for SDL3_ttf

Other satellite libraries aren't stable yet, and will be released as they're available.

### Higher level bindings using `sdl3-sys`

- [`sdl3`](https://crates.io/crates/sdl3): An evolution of the `sdl2` crate

## Usage

`sdl3-sys` is compatible with SDL version `3.1.3` or later.
Some APIs require later versions; you can check availability in the documentation.
The latest available stable release is recommended.

By default, `sdl3-sys` will attempt to link to a dynamic/shared library named
`SDL3` in the default library search path, using the usual platform specific naming
convention for libraries. You can change this behaviour with the following feature flags.

| Feature | Description |
| ------- | ----------- |
| `use-pkg-config` | Use `pkg-config` to find and link the SDL 3 library (enabled by default). |
| `use-vcpkg` | Use `vcpkg` to find and link the SDL 3 library (enabled by default). |
| `build-from-source` | Build and link SDL 3 from source. You need CMake, a C compiler, and any dependencies SDL needs to build for your target. See below for details. |
| `build-from-source-static` | Shortcut for enabling both the `build-from-source` and `link-static` features. This should no longer be necessary. |
| `link-framework` | Link to a framework on Apple targets. This currently requires `SDL3.xcframework` to be located at `~/Library/Frameworks` or `/Library/Frameworks`. The built executable has to be put in a signed app bundle to be able to run. |
| `link-static` | Link SDL statically. SDL doesn't recommend doing this. <ul><li>On targets that only support static linking, such as emscripten, you don't have to enable this feature.</li><li>On Apple targets, this currently requires frameworks that should be optional.</li></ul> |
| `no-link` | Don't link anything, and provide linking flags via Cargo metadata so you can do manual linking if desired. |

### Building from source

Typically it's better to use a prebuilt SDL library and simply configure `sdl3-sys` to use that if required (see above),
but sometimes building from source can be convenient. Building from source needs CMake, a C compiler, and any
dependencies SDL needs to build for your target.

- For many platforms, including Windows and macOS, this is just the standard OS SDK.
- For builds for the web, you need the [Emscripten SDK](https://emscripten.org/). Make sure that is activated
  and build for the `wasm32-unknown-emscripten` target. Emscripten builds of SDL are always statically linked.
- Building for Linux needs [many libraries](https://github.com/libsdl-org/SDL/blob/main/docs/README-linux.md)
  for full support, but you can leave out anything you don't need.

When building from source with the `build-from-source` feature flag, you can enable the features
below to configure the build. They have no effect when not building from source.
Most of them correspond to SDL CMake variables. You can prefix them with `no-` to disable them,
e.g. `no-sdl-libc` to not link with the system C library. If you both enable and disable a
feature, enable takes precedence.

Most of these features only work on some targets; don't enable them unless you need them.

| Feature                  | Description |
| ------------------------ | ----------- |
| `sdl-asan`               | Compile SDL with Address Sanitizer. |
| `sdl-ccache`             | Compile SDL with Ccache. |
| `sdl-lean-and-mean`      | Define SDL_LEAN_AND_MEAN when building SDL (disabled by default). |
| `sdl-libc`               | Link SDL with system C library (default). Use `no-sdl-libc` to disable. |
| `sdl-rpath`              | Set RPATH when linking SDL (default on some targets). Use `no-sdl-rpath` to disable. |
| `sdl-unix-console-build` | Allow building SDL without X11 or Wayland support on Linux and other targets that usually use X11/Wayland. By default, SDL requires either X11 or Wayland on these targets as a sanity check. |

#### Subsystems

These features are only used when building from source.

By default, subsystems are autodetected at build time and all available subsystems are enabled. If you
want more control over which subsystems are enabled, you can use these features. Prefix `no-` to a
feature name to disable the subsytem associated with that feature.

As before, positive features override negative features, so e.g. enabling both the `no-sdl-audio` and
`sdl-audio` features will enable the audio subsystem. You can use the `no-default-subsystems` feature
to disable all subsystems by default, so for example, if you enable both of the `no-default-subsystems`
and `sdl-audio` features, only the audio subsystem will be enabled, and all other subsystems disabled.

APIs for disabled subsystems are still available, but will fail if initialized/used.

If you want a really slimmed down (but less capable) SDL, also see the `sdl-lean-and-mean` feature.

| Feature        | Description |
| -------------- | ----------- |
| `no-default-subsystems` | Disable all subsystems by default. Combine this with the features for the subsystems you want below to enable only those subsystems. |
| `sdl-audio`    | Enable the Audio subsystem |
| `sdl-video`    | Enable the Video subsystem |
| `sdl-gpu`      | Enable the GPU subsystem (implies `sdl-video`) |
| `sdl-render`   | Enable the Render subsystem (implies `sdl-video`) |
| `sdl-camera`   | Enable the Camera subsystem (implies `sdl-video`) |
| `sdl-joystick` | Enable the Joystick subsystem |
| `sdl-haptic`   | Enable the Haptic subsystem |
| `sdl-hidapi`   | Enable the HIDAPI subsystem |
| `sdl-power`    | Enable the Power subsystem |
| `sdl-sensor`   | Enable the Sensor subsystem |
| `sdl-dialog`   | Enable the Dialog subsystem |
| `sdl-tray`     | Enable the Tray subsystem |

### Target specific features

These features are always available, but only make sense for some targets.

| Feature | Description |
| ------- | ----------- |
| `target-gdk` | Enable APIs that require Microsoft's Game Development Kit (GDK). (This is not related to Gnome's GDK.) |

### Optional integrations

These features are always available, but some of them only make sense for some targets.

`sdl3-sys` can use definitions from other crates for some foreign types that it needs,
e.g. for Vulkan types. By default it'll use opaque structs or pointers to opaque structs
for these types unless otherwise specified.

| Feature | Description |
| ------- | ----------- |
| `use-ash-v0-38` | Use Vulkan types from the `ash` crate (v0.38). |
| `use-libc-v0-2` | Use `wchar_t` type from the `libc` crate (v0.2). By default `sdl3-sys` will alias `wchar_t` to `u16` on Windows and `u32` otherwise. |
| `use-windows-sys-*` | Use Windows types from the `windows-sys` crate. <ul><li>`use-windows-sys-v0-59`: v0.59 or compatible (currently 0.59..=0.61)</li></ul> |
| `use-x11-v2` | Use X11 types from the `x11` crate (v2). |
| `use-x11-dl-v2` | Use X11 types from the `x11-dl` crate (v2). |

### Assert level

These features are always available.

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

### Other features

These features are always available.

| Feature | Description |
| ------- | ----------- |
| `debug-impls` | Implement the `Debug` trait for most SDL types. |
| `display-impls` | Implement the `Display` trait for applicable SDL types. |
| `metadata`    | Enable metadata. Also see `only-metadata`. |
| `only-metadata` | Shortcut for enabling both the `metadata` and `no-link` features. Recommended when you need metadata but don't need to call into SDL, e.g. for build scripts. |
| `nightly` | Enable features that need the nightly compiler. This enables the `VaList` type, as well as enabling some intrinsics. |

## Recent changes

    - Enable `use-pkg-config` and `use-vcpkg` features by default
    - Replace use of deprecated `cdecl` and `stdcall` ABIs with equivalent non-deprecated ones

- 0.6.5:
    - Update SDL to 3.4.8 (hotfix)

- 0.6.4:
    - Update SDL to 3.4.6

- 0.6.3:
    - Add features for selecting which subsystems to include when building from source

- 0.6.2:
    - Update SDL to 3.4.4
    - Require exact version match for source crate when building from source (fix for Android)

- 0.6.1:
    - Update SDL to 3.4.2
    - Add `new` and `value` assoc function/method for groups (ids, enums, etc)
    - impl Display for ids under the `display-impls` feature
    - Add metadata for structs and unions

- 0.6.0:
    - Update SDL to 3.4.0
    - Make IDs and flag types newtypes
    - Add metadata
    - Remove deprecated aliases
    - Change return type of `SDL_Vulkan_GetVkGetInstanceProcAddr`
    - Make some functions safe and/or const
    - Add `SDL_Event::event_type()`
    - Add `display-impls` feature
    - MSRV 1.85

See ChangeLog.md for older changes
