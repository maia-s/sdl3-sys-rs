# Changes

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

- 0.1.3:
    - Add `link-framework` feature (doesn't work with `build-from-source` yet)

- 0.1.2:
    - Update SDL_image to 3.2.4

- 0.1.1:
    - Update SDL_image to 3.2.2
    - Disable vendoring by default on all targets
    - When vendoring, disable libraries that aren't included in the crate by default

- 0.1.0:
    - First release
