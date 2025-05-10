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

## Features
| Feature                 | Description |
| ----------------------- | ----------- |
| `alloc`                 | Enable features that require allocation (enabled by default) |
| `std`                   | Enable features that require the standard library (enabled by default) |
| `nightly`               | Enable the `?` operator to convert `Result::Err` and `Option::None` to `AppResult*::Failure` |
| `use-parking-lot-v0-12` | Support parking_lot 0.12 locks in app state accessors |

## Recent changes

- 0.6.0 WIP:
  - Update sdl3-sys to 0.6.0
  - Removed `MainThreadToken::init` (no longer necessary)
  - Pass arguments to main on `std` (`!std` already did)
- 0.5.0:
  - Update sdl3-sys to 0.5.0
  - Added optional parking_lot integration
  - Added `run_{sync,async}_on_main_thread` and `MainThreadData::get[_mut]_on_main_thread`
  - impl `Copy` for `MainThreadToken`
  - impl `FromResidual` for `AppResult*` on nightly
- 0.4.1: Fix potential crash if app_quit takes no arguments and appstate is null
- 0.4.0: Update sdl3-sys to 0.4.0 (first stable SDL release)

