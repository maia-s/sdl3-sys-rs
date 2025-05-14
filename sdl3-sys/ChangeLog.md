# Changes

- 0.6.0 WIP:
    - Make IDs and flag types newtypes
    - Add metadata
    - MSRV 1.81

- 0.5.1:
    - Update SDL to 3.2.14

- 0.5.0:
    - Update SDL to 3.2.12
    - Add `no-link` feature
    - Add more SDL CMake features for use when building from source
    - On Apple targets, look for frameworks in `~/Library/Frameworks` too

- 0.4.9:
    - Revert semver break in 0.4.8 (moved to sdl3-sys 0.5).

- 0.4.7:
    - Update SDL to 3.2.10
    - Derive `PartialEq`/`Eq`/`Hash` traits for applicable types

- 0.4.6:
    - Update SDL to 3.2.8

- 0.4.5:
    - Update SDL to 3.2.6

- 0.4.4:
    - Update SDL to 3.2.4

- 0.4.3:
    - Update SDL to 3.2.2

- 0.4.2:
    - Add homebrew to library search path on macos (fix for Apple Silicon)

- 0.4.1:
    - Fix linking on Fedora and other Linux distros that use lib64

- 0.4.0:
    - First stable release of SDL 3
