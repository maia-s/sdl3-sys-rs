# Changes

- next:
    - Add `from_raw` assoc function and `into_raw` and `value` methods for groups (ids, enums, etc),
      and impl Display for ids under the `display-impls` feature
    - Add metadata for structs and unions

- 0.6.0:
    - Update SDL to 3.4.0
    - Make IDs and flag types newtypes
    - Add metadata
    - Remove deprecated aliases
    - Change return type of `SDL_Vulkan_GetVkGetInstanceProcAddr`
    - Make some functions safe and/or const
    - Add `SDL_Event::event_type()`
    - Add `display-impls` feature
    - MSRV 1.85

- 0.5.12
    - Update SDL to 3.2.30
    - Fix copying the built library when cross compiling

- 0.5.11
    - Update SDL to 3.2.28

- 0.5.10
    - Update SDL to 3.2.26

- 0.5.9
    - Fix copy/paste error in 0.5.8 that prevented building from source from working
      (Thanks to Hudson Regis)

- 0.5.8:
    - Fix compiling for Windows and Android when building from source
      (Thanks to Andrew Minnich and abnormalmaps)
    - Add support for SDL_LEAN_AND_MEAN

- 0.5.7:
    - Fix docs.rs docs

- 0.5.6:
    - Update SDL to 3.2.24
    - Fix an issue where creating symlinks after a build could fail
    - Support windows-sys 0.61 (compatible with 0.59 and 0.60)

- 0.5.5:
    - Update SDL to 3.2.22

- 0.5.4:
    - Update SDL to 3.2.20

- 0.5.3:
    - Update SDL to 3.2.18
    - Copy built library to top level target dir on all targets, not just Windows.

- 0.5.2:
    - Update SDL to 3.2.16

- 0.5.1:
    - Update SDL to 3.2.14
    - Add deprecation warning to padding fields. These shouldn't be accessed directly;
      use `..Default::default()` to init them with struct update syntax.

- 0.5.0:
    - Update SDL to 3.2.12

- 0.4.9:
    - Revert SDL to 3.2.10 to fix semver break in 0.4.8.

- 0.4.8 (yanked):
    - Update SDL to 3.2.12 (reverted in 0.4.9; moved to sdl3-sys 0.5)
    - Add `no-link` feature
    - Add more SDL CMake features for use when building from source
    - On Apple targets, look for frameworks in `~/Library/Frameworks` too

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
