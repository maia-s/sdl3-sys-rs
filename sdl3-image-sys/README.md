# sdl3-image-sys: Low level Rust bindings for SDL_image 3

These are low level Rust bindings for SDL_image, an add-on library for [SDL](https://libsdl.org)
for loading images.
This version of `sdl3-image-sys` has bindings for SDL_image version `3.2.0` and earlier.

<div class="warning">

Most of the docs are generated directly from the C headers and document how
SDL works in C. Using it from Rust might work differently in some cases.
For example, macros in C are usually translated to constants or constant
functions in Rust. Documentation specific to these Rust bindings are tagged
with `sdl3-sys`.

</div>

[Browse the API at docs.rs!](https://docs.rs/sdl3-image-sys)

## Usage

`sdl3-image-sys` requires SDL_image version `3.2.0` or later.
Some APIs may require a later version. You can check availability in the documentation.

By default, `sdl3-image-sys` will attempt to link to a dynamic/shared library named
`SDL3_image` in the default library search path, using the usual platform specific naming
convention for libraries. You can change this behaviour with the following feature flags.

| Feature | Description |
| ------- | ----------- |
| `use-pkg-config` | Use `pkg-config` to find and link the SDL 3 library. |
| `use-vcpkg` | Use `vcpkg` to find and link the SDL 3 library. |
| `build-from-source` | Build and link SDL 3 from source. You have to install any dependencies SDL needs to build for your target first. See below for build related features. |
| `build-from-source-static` | Shortcut for enabling both the `build-from-source` and `link-static` features. This should no longer be necessary. |
| `build-static-vendored` | Build and link SDL_image from source, use vendored libraries (see below), and link it all statically. |
| `link-static` | Link SDL_image statically. |

### Building from source

When building from source with the `build-from-source` feature flag, you can enable these
features to configure which image formats and backends to use. These correspond to SDL_image's cmake variables. These are all autodetected by default. You can use a `no-` prefix to disable a feature,
for example `no-sdlimage-png` disables PNG support (unless activated by a backend). Activated
features override features disabled with the `no-` prefix.

You can disable all backends and/or formats by default with the following features.

| Feature             | Description |
| ------------------- | ----------- |
| no-default          | Disable all backends and formats by default |
| no-default-backends | Disable all backends by default |
| no-default-formats  | Disable all formats by default, except for formats supported by enabled backends |

#### Backends

Features for backends are enabled with a `sdlimage-backend-` prefix and disabled with `no-sdlimage-backend-`.
For example, to enable the STB backend, enable the `sdlimage-backend-stb` feature. To disable it,
enable the `no-sdlimage-backend-stb` feature. Enabled features override disabled features.

These features only have an effect when building SDL_image from source.

| Backend | Description |
| ------- | ----------- |
| stb     | Use STB_image for loading supported formats (all targets) |
| imageio | Use ImageIO for loading supported formats on Apple targets |
| wic     | Use WIC for loading supported formats on Microsoft targets |

#### Image formats

Features for formats are enabled with a `sdlimage-` prefix and disabled with `no-sdlimage-`.
For example, to enable PNG support, enable the `sdlimage-png` feature. To disable it,
enable the `no-sdlimage-png` feature. Enabled features override disabled features, and
disabled formats will still be availabe if supported by an enabled backend.

These features only have an effect when building SDL_image from source.

| Format | Built-in | STB | WIC | ImageIO | Library |
| ------ | :------: | :-: | :-: | :-----: | ------- |
| avif   |          |     |     |         | libavif (BSD-2-Clause) + aom (BSD-2-Clause) + dav1d (BSD-2-Clause) + ... |
| bmp    | ✅       |     |     | ✅       |         |
| gif    | ✅       |     |     | ✅       |         |
| jpg    |          | ✅  | ✅  | ✅       | libjpeg (IJG-short) |
| jxl    |          |     |     |         | libjxl (BSD-3-Clause) + ... |
| lbm    | ✅       |     |     |         |         |
| pcx    | ✅       |     |     |         |         |
| png    |          | ✅  | ✅  | ✅       | libpng (Libpng) + libz (Zlib) |
| pnm    | ✅       |     |     |         |         |
| qoi    | ✅ (MIT) |     |     |         |         |
| svg    | ✅       |     |     |         |         |
| tga    | ✅       |     |     | ✅      |         |
| tif    |          |     | ✅  | ✅      | libtiff (libtiff) |
| webp   |          |     |     |         | libwebp (BSD-3-Clause) |
| xcf    | ✅       |     |     |         |         |
| xpm    | ✅       |     |     |         |         |
| xv     | ✅       |     |     |         |         |

Save support

| Format    | Description |
| --------- | ----------- |
| avif-save | Support saving images in AVIF format. Enables the `sdlimage-avif` feature |
| jpg-save  | Support saving images in JPG format. Enables the `sdlimage-jpg` feature |
| png-save  | Support saving images in PNG format. Enables the `sdlimage-png` feature |

## Other features

| Feature | Description |
| ------- | ----------- |
| `debug-impls` | Implement the `Debug` trait for most SDL_image types. |

## Version history

TBD
