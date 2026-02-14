# sdl3-image-sys: Low level Rust bindings for SDL3_image

These are low level Rust bindings for SDL3_image, an add-on library for
[SDL 3](https://libsdl.org) for loading images.
This version of `sdl3-image-sys` has bindings for SDL_image versions `3.2.0` to `3.4.0`, inclusive.

<div class="warning">

Most of the docs are generated directly from the C headers and document how
SDL works in C. Using it from Rust might work differently in some cases.
For example, macros in C are usually translated to constants or constant
functions in Rust. Documentation specific to these Rust bindings are tagged
with `sdl3-sys`.

</div>

[Browse the API at docs.rs!](https://docs.rs/sdl3-image-sys)

## Usage

`sdl3-image-sys` is compatible with SDL_image version `3.2.0` or later.
Some APIs may require later versions; you can check availability in the documentation.
The latest available stable release is recommended.

By default, `sdl3-image-sys` will attempt to link to a dynamic/shared library named
`SDL3_image` in the default library search path, using the usual platform specific naming
convention for libraries. You can change this behaviour with the following feature flags.

| Feature | Description |
| ------- | ----------- |
| `use-pkg-config` | Use `pkg-config` to find and link the SDL_image library. |
| `use-vcpkg` | Use `vcpkg` to find and link the SDL_image library. |
| `build-from-source` | Build and link SDL_image from source. You have to install any dependencies SDL needs to build for your target first. See below for build related features. |
| `build-from-source-static` | Shortcut for enabling both the `build-from-source` and `link-static` features. This should no longer be necessary. |
| `build-static-vendored` | Build and link SDL_image from source, use vendored libraries (see below), and link it all statically. |
| `link-framework` | Link to a framework on Apple targets. This currently requires `SDL3_image.xcframework` to be located at `~/Library/Frameworks` or `/Library/Frameworks`. The built executable has to be put in a signed app bundle to be able to run. |
| `link-static` | Link SDL_image statically. |
| `no-link` | Don't link anything, and provide linking flags via Cargo metadata so you can do manual linking if desired. |

### Building from source

When building from source with the `build-from-source` feature flag, you can use
features to configure which backends and image formats to support. Features marked with "(cmake)" below all have
an `sdlimage-` prefix and correspond to SDL_image's cmake variables. They're autodetected if not set. You can use a `no-` prefix to disable a cmake feature,
for example `no-sdlimage-png` disables png support. Activated
features override features disabled with the `no-` prefix.

If an image format is supported by an enabled backend, the backend will handle it and disabling the format's feature has no effect.

#### Defaults

Backends and formats are autodetected by default. You can disable them by default instead and enable only the features you want.

(These are not cmake features)

| Feature               | Description |
| --------------------- | ----------- |
| `no-default`          | Disable all backends and formats by default |
| `no-default-backends` | Disable all backends by default |
| `no-default-formats`  | Disable all formats by default, except for formats supported by enabled backends |

#### Linking and vendoring

| Feature (cmake) | Description |
| --------------- | ----------- |
| `vendored`      | Build dependencies from source as part of building SDL_image. SDL_image can vendor all external libraries, but this crate currently only includes support for vendoring the libraries for png and jpg formats for size reasons. (Also consider using the stb backend if you only need basic png and jpg support.) |
| `deps-shared`   | Use shared libraries for dependencies. You can also enable this for select libraries only, see below |

#### Backends

Features for backends are enabled with a `sdlimage-backend-` prefix and disabled
with `no-sdlimage-backend-`. For example, to enable the STB backend, enable the
`sdlimage-backend-stb` feature. To disable it, enable the `no-sdlimage-backend-stb`
feature. Enabled features override disabled features.

| Feature (cmake, backend) | Description |
| --------------- | ----------- |
| `stb`           | Use [stb_image](https://github.com/nothings/stb) for loading supported formats (all targets). This backend is always vendored |
| `imageio`       | Use ImageIO for loading supported formats on Apple targets |
| `wic`           | Use WIC for loading supported formats on Microsoft targets |

#### Image formats

These are enabled with an `sdlimage-` prefix and disabled with a `no-sdlimage-` prefix.

| Feature (cmake) | Built-in | STB | WIC | ImageIO | Library |
| --------------- | :------: | :-: | :-: | :-----: | ------- |
| `avif`          |          |     |     |         | libavif (BSD-2-Clause) + aom (BSD-2-Clause) + dav1d (BSD-2-Clause) + ... |
| `bmp`           | ✅       |     |     | ✅       |         |
| `gif`           | ✅       |     |     | ✅       |         |
| `jpg`           |          | ✅  | ✅  | ✅       | libjpeg (IJG-short) |
| `jxl`           |          |     |     |         | libjxl (BSD-3-Clause) + ... |
| `lbm`           | ✅       |     |     |         |         |
| `pcx`           | ✅       |     |     |         |         |
| `png`           |          | ✅  | ✅  | ✅       | libpng (Libpng) + libz (Zlib) |
| `pnm`           | ✅       |     |     |         |         |
| `qoi`           | ✅ (MIT) |     |     |         |         |
| `svg`           | ✅       |     |     |         |         |
| `tga`           | ✅       |     |     | ✅      |         |
| `tif`           |          |     | ✅  | ✅      | libtiff (libtiff) |
| `webp`          |          |     |     |         | libwebp (BSD-3-Clause) |
| `xcf`           | ✅       |     |     |         |         |
| `xpm`           | ✅       |     |     |         |         |
| `xv`            | ✅       |     |     |         |         |

##### Save support

| Feature (cmake) | Description |
| --------------- | ----------- |
| `avif-save`     | Support saving images in avif format. Enables the `sdlimage-avif` feature |
| `jpg-save`      | Support saving images in jpg format. Enables the `sdlimage-jpg` feature |
| `png-save`      | Support saving images in png format. Enables the `sdlimage-png` feature |

##### Shared linking

| Feature (cmake) | Description |
| --------------- | ----------- |
| `avif-shared`   | Use shared libraries for avif |
| `jpg-shared`    | Use shared library for jpeg |
| `jxl-shared`    | Use shared libraries for jxl |
| `png-shared`    | Use shared libraries for png |
| `tif-shared`    | Use shared library for tiff |
| `webp-shared`   | Use shared library for webp |

## Other features

| Feature | Description |
| ------- | ----------- |
| `debug-impls` | Implement the `Debug` trait for most SDL_image types. |
| `metadata`    | Enable metadata. Also see `only-metadata`. |
| `only-metadata` | Shortcut for enabling both the `metadata` and `no-link` features. Recommended when you need metadata but don't need to call into SDL_image, e.g. for build scripts. |

## Recent changes

- next
    - Add metadata for structs and unions

- 0.6.0:
    - Update sdl3-sys to 0.6.0
    - Synchronize version number with sdl3-sys
    - Add metadata
    - Make some functions safe and/or const
    - MSRV 1.85

- 0.2.1:
    - Update SDL_image to 3.2.6
    - Fix an issue where creating symlinks after a build could fail
    - Fix copying the built library when cross compiling

- 0.2.0:
    - Update sdl3-sys to 0.5.0
    - Add `no-link` feature
    - On Apple targets, look for frameworks in `~/Library/Frameworks` too

See ChangeLog.md for older changes
