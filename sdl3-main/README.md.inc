# sdl3-main

This crate provides tools for using SDL 3's main and callback APIs, and
for interfacing with the main thread of the process.

## SDL main

To provide your own main but call it through SDL, use the [`main`] attribute macro.
See the documentation for that for more information.

This macro must be called at the root of `main.rs` in your crate.

## Callback API

To use the SDL callback API, you can use either the [`app_impl`] attribute macro,
or all four of the [`app_init`], [`app_iterate`], [`app_event`] and [`app_quit`]
attribute macros. Don't use the `main` attribute macro in this mode.
See the documentation for more information.

These macros must be called at the root of `main.rs` in your crate. In the case of
`app_impl`, the impl block for that must be at the root of `main.rs`, but the rest of
the app struct and other impl blocks can be defined elsewhere.

## Main thread helpers

Some SDL functions have to be called on the main thread of the process. This crate
provides some helper types and functions to alleviate this.

- [`MainThreadToken`]\: Zero-sized token that can only exist on the main thread.
- [`MainThreadData`]\: Wrapper for data that can move between threads but that should
  only be accessed on the main thread.
- [`run_sync_on_main_thread()`]\: Run a callback on the main thread, synchronously.
- [`run_async_on_main_thread()`]\: Run a callback on the main thread, asynchronously.

## Features
| Feature                 | Description |
| ----------------------- | ----------- |
| `alloc`                 | Enable features that require allocation (enabled by default) |
| `std`                   | Enable features that require the standard library (enabled by default) |
| `nightly`               | Enable the `?` operator to convert `Result::Err` and `Option::None` to `AppResult*::Failure` |
| `use-parking-lot-v0-12` | Support parking_lot 0.12 locks in app state accessors |

## Recent changes

- 0.6.2:
    - Fix using `app_*` macros with app states that have generic arguments
      (all generics have to be known)

- 0.6.1:
    - Make `MainThreadToken::init()` safe
    - Fix undefined reference to `main_with_result` when using the `main` macro in
      some cases

- 0.6.0:
    - Update sdl3-sys to 0.6.0
    - `MainThreadToken::init()` is no longer necessary in most cases
    - Pass arguments to main on `std` (`!std` already did)
    - Add `MainThreadData::assert_new/get/get_mut`
    - Log error when converting to `AppResult*`
    - MSRV 1.85

See ChangeLog.md for older changes

