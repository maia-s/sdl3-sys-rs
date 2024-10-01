# sdl3-main

This crate provides tools for using SDL 3's main and callback APIs.

## SDL main

To provide your own main but call it through SDL, use the [`main`] attribute macro.
See the documentation for that for more information.

## Callback API

To use the SDL callback API, use the [`app_init`], [`app_iterate`], [`app_event`] and
[`app_quit`] attribute macros. Don't use the `main` attribute in this mode.
See the documentation for more information.
