# sdl3-main

<div class="warning">

This version is a prerelease. There may be breaking changes before the final 0.6.0 release.

</div>

This crate provides tools for using SDL 3's main and callback APIs, and
for interfacing with the main thread of the process.

## SDL main

To provide your own main but call it through SDL, use the [`main`] attribute macro.
See the documentation for that for more information.

## Callback API

To use the SDL callback API, you can use either the [`app_impl`] attribute macro,
or all four of the [`app_init`], [`app_iterate`], [`app_event`] and [`app_quit`]
attribute macros. Don't use the `main` attribute macro in this mode.
See the documentation for more information.

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

- 0.6.0-pre-2:
    - Update sdl3-sys to 0.6.0-pre-2

- 0.6.0-pre-1:
    - Update sdl3-sys to 0.6.0-pre-1
    - Fix issue from 0.6.0-pre-0 that made it possible to get a `MainThreadToken`
      on multiple threads

- 0.6.0-pre-0:
    - Update sdl3-sys to 0.6.0-pre-0
    - `MainThreadToken::init()` is no longer necessary in most cases
    - Pass arguments to main on `std` (`!std` already did)
    - Add `MainThreadData::assert_new/get/get_mut`
    - Log error when converting to `AppResult*`

- 0.5.4:
    - Fix accidental dependency on Rust 1.84

- 0.5.3:
    - Only use the alloc-less optimization of `run_async_on_main_thread` if the size of
      the callback is zero, because that's the only way to guarantee we won't copy
      uninit bytes, which would be unsound

- 0.5.2:
    - Fix more unsoundness in `run_async_on_main_thread`

- 0.5.1:
    - Fix unsoundness in `run_async_on_main_thread`
    - Don't allocate in `run_async_on_main_thread` unless it's necessary

- 0.5.0:
    - Update sdl3-sys to 0.5.0
    - Add optional parking_lot integration
    - Add `run_{sync,async}_on_main_thread` and `MainThreadData::get[_mut]_on_main_thread`
    - impl `Copy` for `MainThreadToken`
    - impl `FromResidual` for `AppResult*` on nightly

See ChangeLog.md for older changes

