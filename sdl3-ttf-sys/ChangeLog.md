# Changes

- 0.6.0 WIP:
    - Update sdl3-sys to 0.6.0
    - Synchronize version number with sdl3-sys
    - Add metadata
    - Remove deprecated aliases
    - Make some functions safe and/or const

- next:
    - Expose `TTF_TextData`
    - Fix an issue where creating symlinks after a build could fail

- 0.2.0:
    - Update sdl3-sys to 0.5.0
    - Add `no-link` feature
    - On Apple targets, look for frameworks in `~/Library/Frameworks` too

- 0.1.3:
    - Add `link-framework` feature (doesn't work with `build-from-source` yet)

- 0.1.2:
    - Fix vendored build of Freetype with CMake 4

- 0.1.1:
    - Update SDL_ttf to 3.2.2

- 0.1.0:
    - First release
