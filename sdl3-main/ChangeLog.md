# Changes

- 0.6.0:
    - Update sdl3-sys to 0.6.0
    - `MainThreadToken::init()` is no longer necessary in most cases
    - Pass arguments to main on `std` (`!std` already did)
    - Add `MainThreadData::assert_new/get/get_mut`
    - Log error when converting to `AppResult*`
    - MSRV 1.85

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

- 0.4.1:
    - Fix potential crash if `app_quit` takes no arguments and appstate is null

- 0.4.0:
    - Update sdl3-sys to 0.4.0 (first stable SDL 3 release)

- 0.3.0:
    - Update sdl3-sys to 0.3.0

- 0.2.0:
    - Update sdl3-sys to 0.2.0
    - Add `app_impl` attribute macro
    - Parser improvements

- 0.1.0:
    - First release
