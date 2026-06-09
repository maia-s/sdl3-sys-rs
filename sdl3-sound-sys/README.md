# sdl3-sound-sys: Low level Rust bindings for SDL3_sound

These are low level Rust bindings for [SDL_sound 3](https://github.com/icculus/SDL_sound),
an add-on library for [SDL 3](https://libsdl.org) for decoding several popular sound file formats.
This version of `sdl3-sound-sys` has bindings for SDL_sound version `3.2.0`.

`sdl3-sound-sys` works out of the box on any target that SDL_sound 3 supports and doesn't require anything
else if the SDL_sound 3 library is installed or otherwise available. On targets that support it you can
use `pkg-config` or `vcpkg` to locate the SDL_sound 3 library if required (see below).

If the SDL_sound 3 library isn't already available, you have the option of building it from source as
part of the build (see below), which requires CMake, a C compiler, and SDL (which can also be built
from source via the `sdl3-sys` crate).
Builds for web are supported via Emscripten (`wasm32-unknown-emscripten`).

> ⓘ *Most of the docs are generated directly from the original C headers, which documents how*
> *SDL works in C. Using it from Rust might work differently in some cases.*
> *For example, macros in C are usually translated to constants or constant*
> *functions in Rust, but the docs may still refer to them as macros.*
> *Documentation specific to these Rust bindings are tagged with `sdl3-sys`.*

[Browse the API at docs.rs!](https://docs.rs/sdl3-sound-sys)

## Usage

`sdl3-sound-sys` is compatible with SDL_sound version `3.2.0` or later.
Some APIs may require later versions; you can check availability in the documentation.
The latest available stable release is recommended.

By default, `sdl3-sound-sys` will attempt to link to a dynamic/shared library named
`SDL3_sound` in the default library search path, using the usual platform specific naming
convention for libraries. You can change this behaviour with the following feature flags.

| Feature | Description |
| ------- | ----------- |
| `use-pkg-config` | Use `pkg-config` to find and link the SDL_sound library. |
| `use-vcpkg` | Use `vcpkg` to find and link the SDL_sound library. |
| `build-from-source` | Build and link SDL_sound from source. You have to install any dependencies SDL needs to build for your target first. See below for build related features. |
| `build-from-source-static` | Shortcut for enabling both the `build-from-source` and `link-static` features. This should no longer be necessary. |
| `link-static` | Link SDL_sound statically. |
| `no-link` | Don't link anything, and provide linking flags via Cargo metadata so you can do manual linking if desired. |

### Building from source

When building from source with the `build-from-source` feature flag, you can use
features to configure which audio formats to support. Features marked with "(cmake)" below all have
an `sdlsound-` prefix and correspond to SDL_sound's cmake variables. They're autodetected if not set.
You can use a `no-` prefix to disable a cmake feature, for example `no-sdlsound-mp3` disables mp3 support. Activated features override features disabled with the `no-` prefix.

#### Defaults

Formats are autodetected by default, except for MIDI which is disabled by default because it's LGPL licensed. You can disable all formats by default and enable only the formats you want. 

(This is not a cmake feature)

| Feature               | Description |
| --------------------- | ----------- |
| `no-default-formats`  | Disable all formats by default |

#### Formats

Use `sdlsound-` prefix to enable and `no-sdlsound-` prefix to disable, e.g. `sdlsound-wav` to enable wav.
Enables override disables.

None of these formats require additional dependencies.

| Feature (cmake) | Description |
| --------------- | ----------- |
| `wav`           | Enable WAV support |
| `aiff`          | Enable AIFF support |
| `au`            | Enable AU support |
| `voc`           | Enable VOC support |
| `flac`          | Enable FLAC support |
| `vorbis`        | Enable Vorbis support |
| `raw`           | Enable raw audio support |
| `shn`           | Enable Shorten support |
| `modplug`       | Enable support for various module formats via ModPlug |
| `mp3`           | Enabole MP3, MP2 and MP1 support |
| `midi`          | Enable MIDI support via Timidity. This usues LGPL licensed code. (disabled by default) |
| `coreaudio`     | Enable CoreAudio support on Apple targets |

## Other features

| Feature | Description |
| ------- | ----------- |
| `debug-impls` | Implement the `Debug` trait for most SDL_sound types. |
| `metadata`    | Enable metadata. Also see `only-metadata`. |
| `only-metadata` | Shortcut for enabling both the `metadata` and `no-link` features. Recommended when you need metadata but don't need to call into SDL_sound, e.g. for build scripts. |

## Recent changes

- 0.6.0:
    - First release

See ChangeLog.md for older changes
