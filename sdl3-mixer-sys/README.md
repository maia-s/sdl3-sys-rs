# sdl3-mixer-sys: Low level Rust bindings for SDL3_mixer

These are low level Rust bindings for SDL3_mixer, an add-on library for
[SDL 3](https://libsdl.org) for loading and playing audio files.
This version of `sdl3-mixer-sys` has bindings for SDL_image versions `3.2.0` to `3.2.4`, inclusive.

<div class="warning">

Most of the docs are generated directly from the C headers and document how
SDL works in C. Using it from Rust might work differently in some cases.
For example, macros in C are usually translated to constants or constant
functions in Rust. Documentation specific to these Rust bindings are tagged
with `sdl3-sys`.

</div>

[Browse the API at docs.rs!](https://docs.rs/sdl3-mixer-sys)

## Usage

`sdl3-mixer-sys` is compatible with SDL_mixer version `3.2.0` or later.
Some APIs may require later versions; you can check availability in the documentation.
The latest available stable release is recommended.

By default, `sdl3-mixer-sys` will attempt to link to a dynamic/shared library named
`SDL3_mixer` in the default library search path, using the usual platform specific naming
convention for libraries. You can change this behaviour with the following feature flags.

| Feature | Description |
| ------- | ----------- |
| `use-pkg-config` | Use `pkg-config` to find and link the SDL_mixer library. |
| `use-vcpkg` | Use `vcpkg` to find and link the SDL_mixer library. |
| `build-from-source` | Build and link SDL_mixer from source. You have to install any dependencies SDL needs to build for your target first. See below for build related features. |
| `build-from-source-static` | Shortcut for enabling both the `build-from-source` and `link-static` features. This should no longer be necessary. |
| `build-static-vendored` | Build and link SDL_mixer from source, use vendored libraries (see below), and link it all statically. |
| `link-framework` | Link to a framework on Apple targets. This currently requires `SDL3_mixer.xcframework` to be located at `~/Library/Frameworks` or `/Library/Frameworks`. The built executable has to be put in a signed app bundle to be able to run. |
| `link-static` | Link SDL_mixer statically. |
| `no-link` | Don't link anything, and provide linking flags via Cargo metadata so you can do manual linking if desired. |

### Building from source

When building from source with the `build-from-source` feature flag, you can use
features to configure which audio formats to support. Features marked with "(cmake)" below all have
an `sdlmixer-` prefix and correspond to SDL_mixer's cmake variables. They're autodetected if not set.
You can use a `no-` prefix to disable a cmake feature, for example `no-sdlmixer-mp3-drmp3` disables mp3 support via the drmp3 library. Activated features override features disabled with the `no-` prefix.

#### Defaults

Formats are autodetected by default. You can disable them by default instead and enable only the formats you want. 

(These are not cmake features)

| Feature               | Description |
| --------------------- | ----------- |
| `no-default-formats`  | Disable all formats by default |
| `no-default-aiff`     | Disable AIFF by default |
| `no-default-wave`     | Disable WAVE by default |
| `no-default-voc`      | Disable VOC by default |
| `no-default-au`       | Disable AU by default |
| `no-default-flac`     | Disable FLAC by default |
| `no-default-gme`      | Disable GME by default |
| `no-default-mod`      | Disable MOD by default |
| `no-default-mp3`      | Disable MP3 by default |
| `no-default-midi`     | Disable MIDI by default |
| `no-default-opus`     | Disable Opus by default |
| `no-default-vorbis`   | Disable Vorbis by default |
| `no-default-wavpack`  | Disable Wavpack by default |

#### Linking and vendoring

| Feature (cmake) | Description |
| --------------- | ----------- |
| `vendored`      | Build dependencies from source as part of building SDL_mixer |
| `deps-shared`   | Use shared libraries for dependencies. You can also enable this for select libraries only, see below |

#### Formats

| Feature (cmake)     | Description |
| ------------------- | ----------- |
| `aiff`              | Enable AIFF support (built-in) |
| `wave`              | Enable WAVE support (built-in) |
| `voc`               | Enable VOC support (built-in) |
| `au`                | Enable AU support (built-in) |
| `flac-libflac`      | Enable FLAC support via libflac |
| `flac-drflac`       | Enable FLAC support via dr_flac |
| `gme`               | Enable GME support via libgme |
| `mod-xmp`           | Enable MOD support via libxmp |
| `mod-xmp-lite`      | Enable MOD support via libxmp-lite |
| `mp3-drmp3`         | Enable MP3 support via dr_mp3 |
| `mp3-mpg123`        | Enable MP3 support via libmpg123 |
| `midi-fluidsynth`   | Enable MIDI support via libfluidsynth |
| `midi-timidity`     | Enable MIDI support via timidity |
| `opus`              | Enable Opus support via libopus |
| `vorbis-stb`        | Enable Vorbis support via stb |
| `vorbis-vorbisfile` | Enable Vorbis support via vorbisfile |
| `vorbis-tremor`     | Enable Vorbis support via libvorbisidec (tremor) |
| `wavpack`           | Enable Wavpack support |
| `wavpack-dsd`       | Enable Wavpack DSD support |

#### Shared linking

| Feature (cmake)            | Description |
| -------------------------- | ----------- |
| `flac-libflac-shared`      | Link libflac shared. Implies `flac-libflac` |
| `gme-shared`               | Link libgme shared. Imples `gme` |
| `mod-xmp-shared`           | Link libxmp/libxmp-lite shared. Implies `mod-xmp` (enable `mod-xmp-lite` to use lite instead) |
| `mp3-mpg123-shared`        | Link libmpg123 shared. Implies `mp3-mpg123` |
| `midi-fluidsynth-shared`   | Link libfluidsynth shared. Imples `midi-fluidsynth` |
| `opus-shared`              | Link libopus shared. Implies `opus` |
| `vorbis-vorbisfile-shared` | Link vorbisfile shared. Implies `vorbis-vorbisfile` |
| `vorbis-tremor-shared`     | Link libvorbisidec shared. Imples `vorbis-tremor` |
| `wavpack-shared`           | Link wavpack shared. Implies `wavpack` |

## Other features

| Feature | Description |
| ------- | ----------- |
| `debug-impls` | Implement the `Debug` trait for most SDL_mixer types. |
| `metadata`    | Enable metadata. Also see `only-metadata`. |
| `only-metadata` | Shortcut for enabling both the `metadata` and `no-link` features. Recommended when you need metadata but don't need to call into SDL_mixer, e.g. for build scripts. |

## Recent changes

- 0.6.0 WIP:
    - First release

See ChangeLog.md for older changes
