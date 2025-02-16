# sdl3-main

This crate provides tools for using SDL 3's main and callback APIs.

## SDL main

To provide your own main but call it through SDL, use the [`main`] attribute macro.
See the documentation for that for more information.

## Callback API

To use the SDL callback API, you can use either the [`app_impl`] attribute macro,
or all four of the [`app_init`], [`app_iterate`], [`app_event`] and [`app_quit`]
attribute macros. Don't use the `main` attribute macro in this mode.
See the documentation for more information.

## Version history

- 0.4.1: Fix potential crash if app_quit takes no arguments and appstate is null
- 0.4.0: Update sdl3-sys to 0.4.0 (first stable SDL release)
