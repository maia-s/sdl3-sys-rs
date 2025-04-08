This crate contains the source code of the SDL3_ttf library. It's used by the
`sdl3-ttf-sys` crate when building from source.

Version 3.2.2 of this crate didn't include updates to submodules for that version
of SDL_ttf. Version 3.2.3 is the SDL_ttf 3.2.2 source with updated submodules.
This fixes the vendored freetype not building with Cmake 4.
