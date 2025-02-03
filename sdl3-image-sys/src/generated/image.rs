//! Header file for SDL_image library
//!
//! A simple library to load images of various formats as SDL surfaces

use sdl3_sys::everything::*;

/// * Printable format: "%d.%d.%d", MAJOR, MINOR, MICRO
pub const SDL_IMAGE_MAJOR_VERSION: ::core::primitive::i32 = 3;

pub const SDL_IMAGE_MINOR_VERSION: ::core::primitive::i32 = 2;

pub const SDL_IMAGE_MICRO_VERSION: ::core::primitive::i32 = 2;

/// * This is the version number macro for the current SDL_image version.
pub const SDL_IMAGE_VERSION: ::core::primitive::i32 = SDL_VERSIONNUM(
    SDL_IMAGE_MAJOR_VERSION,
    SDL_IMAGE_MINOR_VERSION,
    SDL_IMAGE_MICRO_VERSION,
);

/// * This macro will evaluate to true if compiled with SDL_image at least X.Y.Z.
#[inline(always)]
pub const fn SDL_IMAGE_VERSION_ATLEAST(
    X: ::core::primitive::i32,
    Y: ::core::primitive::i32,
    Z: ::core::primitive::i32,
) -> ::core::primitive::bool {
    (((SDL_IMAGE_MAJOR_VERSION >= X)
        && ((SDL_IMAGE_MAJOR_VERSION > X) || (SDL_IMAGE_MINOR_VERSION >= Y)))
        && (((SDL_IMAGE_MAJOR_VERSION > X) || (SDL_IMAGE_MINOR_VERSION > Y))
            || (SDL_IMAGE_MICRO_VERSION >= Z)))
}

extern "C" {
    /// This function gets the version of the dynamically linked SDL_image library.
    ///
    /// ### Return value
    /// Returns SDL_image version.
    ///
    /// ### Availability
    /// This function is available since SDL_image 3.0.0.
    pub fn IMG_Version() -> ::core::ffi::c_int;
}

extern "C" {
    /// Load an image from an SDL data source into a software surface.
    ///
    /// An [`SDL_Surface`] is a buffer of pixels in memory accessible by the CPU. Use
    /// this if you plan to hand the data to something else or manipulate it
    /// further in code.
    ///
    /// There are no guarantees about what format the new [`SDL_Surface`] data will be;
    /// in many cases, SDL_image will attempt to supply a surface that exactly
    /// matches the provided image, but in others it might have to convert (either
    /// because the image is in a format that SDL doesn't directly support or
    /// because it's compressed data that could reasonably uncompress to various
    /// formats and SDL_image had to pick one). You can inspect an [`SDL_Surface`] for
    /// its specifics, and use [`SDL_ConvertSurface`] to then migrate to any supported
    /// format.
    ///
    /// If the image format supports a transparent pixel, SDL will set the colorkey
    /// for the surface. You can enable RLE acceleration on the surface afterwards
    /// by calling: SDL_SetSurfaceColorKey(image, [`SDL_RLEACCEL`],
    /// image->format->colorkey);
    ///
    /// If `closeio` is true, `src` will be closed before returning, whether this
    /// function succeeds or not. SDL_image reads everything it needs from `src`
    /// during this call in any case.
    ///
    /// Even though this function accepts a file type, SDL_image may still try
    /// other decoders that are capable of detecting file type from the contents of
    /// the image data, but may rely on the caller-provided type string for formats
    /// that it cannot autodetect. If `type` is NULL, SDL_image will rely solely on
    /// its ability to guess the format.
    ///
    /// There is a separate function to read files from disk without having to deal
    /// with [`SDL_IOStream`]\: `IMG_Load("filename.jpg")` will call this function and
    /// manage those details for you, determining the file type from the filename's
    /// extension.
    ///
    /// There is also [`IMG_Load_IO()`], which is equivalent to this function except
    /// that it will rely on SDL_image to determine what type of data it is
    /// loading, much like passing a NULL for type.
    ///
    /// If you are using SDL's 2D rendering API, there is an equivalent call to
    /// load images directly into an [`SDL_Texture`] for use by the GPU without using a
    /// software surface: call [`IMG_LoadTextureTyped_IO()`] instead.
    ///
    /// When done with the returned surface, the app should dispose of it with a
    /// call to [`SDL_DestroySurface()`].
    ///
    /// ### Parameters
    /// - `src`: an [`SDL_IOStream`] that data will be read from.
    /// - `closeio`: true to close/free the [`SDL_IOStream`] before returning, false
    ///   to leave it open.
    /// - `type`: a filename extension that represent this data ("BMP", "GIF",
    ///   "PNG", etc).
    ///
    /// ### Return value
    /// Returns a new SDL surface, or NULL on error.
    ///
    /// ### Availability
    /// This function is available since SDL_image 3.0.0.
    ///
    /// ### See also
    /// - [`IMG_Load`]
    /// - [`IMG_Load_IO`]
    /// - [`SDL_DestroySurface`]
    pub fn IMG_LoadTyped_IO(
        src: *mut SDL_IOStream,
        closeio: ::core::primitive::bool,
        r#type: *const ::core::ffi::c_char,
    ) -> *mut SDL_Surface;
}

extern "C" {
    /// Load an image from a filesystem path into a software surface.
    ///
    /// An [`SDL_Surface`] is a buffer of pixels in memory accessible by the CPU. Use
    /// this if you plan to hand the data to something else or manipulate it
    /// further in code.
    ///
    /// There are no guarantees about what format the new [`SDL_Surface`] data will be;
    /// in many cases, SDL_image will attempt to supply a surface that exactly
    /// matches the provided image, but in others it might have to convert (either
    /// because the image is in a format that SDL doesn't directly support or
    /// because it's compressed data that could reasonably uncompress to various
    /// formats and SDL_image had to pick one). You can inspect an [`SDL_Surface`] for
    /// its specifics, and use [`SDL_ConvertSurface`] to then migrate to any supported
    /// format.
    ///
    /// If the image format supports a transparent pixel, SDL will set the colorkey
    /// for the surface. You can enable RLE acceleration on the surface afterwards
    /// by calling: SDL_SetSurfaceColorKey(image, [`SDL_RLEACCEL`],
    /// image->format->colorkey);
    ///
    /// There is a separate function to read files from an [`SDL_IOStream`], if you
    /// need an i/o abstraction to provide data from anywhere instead of a simple
    /// filesystem read; that function is [`IMG_Load_IO()`].
    ///
    /// If you are using SDL's 2D rendering API, there is an equivalent call to
    /// load images directly into an [`SDL_Texture`] for use by the GPU without using a
    /// software surface: call [`IMG_LoadTexture()`] instead.
    ///
    /// When done with the returned surface, the app should dispose of it with a
    /// call to
    /// [SDL_DestroySurface](https://wiki.libsdl.org/SDL3/SDL_DestroySurface)
    /// ().
    ///
    /// ### Parameters
    /// - `file`: a path on the filesystem to load an image from.
    ///
    /// ### Return value
    /// Returns a new SDL surface, or NULL on error.
    ///
    /// ### Availability
    /// This function is available since SDL_image 3.0.0.
    ///
    /// ### See also
    /// - [`IMG_LoadTyped_IO`]
    /// - [`IMG_Load_IO`]
    /// - [`SDL_DestroySurface`]
    pub fn IMG_Load(file: *const ::core::ffi::c_char) -> *mut SDL_Surface;
}

extern "C" {
    /// Load an image from an SDL data source into a software surface.
    ///
    /// An [`SDL_Surface`] is a buffer of pixels in memory accessible by the CPU. Use
    /// this if you plan to hand the data to something else or manipulate it
    /// further in code.
    ///
    /// There are no guarantees about what format the new [`SDL_Surface`] data will be;
    /// in many cases, SDL_image will attempt to supply a surface that exactly
    /// matches the provided image, but in others it might have to convert (either
    /// because the image is in a format that SDL doesn't directly support or
    /// because it's compressed data that could reasonably uncompress to various
    /// formats and SDL_image had to pick one). You can inspect an [`SDL_Surface`] for
    /// its specifics, and use [`SDL_ConvertSurface`] to then migrate to any supported
    /// format.
    ///
    /// If the image format supports a transparent pixel, SDL will set the colorkey
    /// for the surface. You can enable RLE acceleration on the surface afterwards
    /// by calling: SDL_SetSurfaceColorKey(image, [`SDL_RLEACCEL`],
    /// image->format->colorkey);
    ///
    /// If `closeio` is true, `src` will be closed before returning, whether this
    /// function succeeds or not. SDL_image reads everything it needs from `src`
    /// during this call in any case.
    ///
    /// There is a separate function to read files from disk without having to deal
    /// with [`SDL_IOStream`]\: `IMG_Load("filename.jpg")` will call this function and
    /// manage those details for you, determining the file type from the filename's
    /// extension.
    ///
    /// There is also [`IMG_LoadTyped_IO()`], which is equivalent to this function
    /// except a file extension (like "BMP", "JPG", etc) can be specified, in case
    /// SDL_image cannot autodetect the file format.
    ///
    /// If you are using SDL's 2D rendering API, there is an equivalent call to
    /// load images directly into an [`SDL_Texture`] for use by the GPU without using a
    /// software surface: call [`IMG_LoadTexture_IO()`] instead.
    ///
    /// When done with the returned surface, the app should dispose of it with a
    /// call to [`SDL_DestroySurface()`].
    ///
    /// ### Parameters
    /// - `src`: an [`SDL_IOStream`] that data will be read from.
    /// - `closeio`: true to close/free the [`SDL_IOStream`] before returning, false
    ///   to leave it open.
    ///
    /// ### Return value
    /// Returns a new SDL surface, or NULL on error.
    ///
    /// ### Availability
    /// This function is available since SDL_image 3.0.0.
    ///
    /// ### See also
    /// - [`IMG_Load`]
    /// - [`IMG_LoadTyped_IO`]
    /// - [`SDL_DestroySurface`]
    pub fn IMG_Load_IO(
        src: *mut SDL_IOStream,
        closeio: ::core::primitive::bool,
    ) -> *mut SDL_Surface;
}

extern "C" {
    /// Load an image from a filesystem path into a GPU texture.
    ///
    /// An [`SDL_Texture`] represents an image in GPU memory, usable by SDL's 2D Render
    /// API. This can be significantly more efficient than using a CPU-bound
    /// [`SDL_Surface`] if you don't need to manipulate the image directly after
    /// loading it.
    ///
    /// If the loaded image has transparency or a colorkey, a texture with an alpha
    /// channel will be created. Otherwise, SDL_image will attempt to create an
    /// [`SDL_Texture`] in the most format that most reasonably represents the image
    /// data (but in many cases, this will just end up being 32-bit RGB or 32-bit
    /// RGBA).
    ///
    /// There is a separate function to read files from an [`SDL_IOStream`], if you
    /// need an i/o abstraction to provide data from anywhere instead of a simple
    /// filesystem read; that function is [`IMG_LoadTexture_IO()`].
    ///
    /// If you would rather decode an image to an [`SDL_Surface`] (a buffer of pixels
    /// in CPU memory), call [`IMG_Load()`] instead.
    ///
    /// When done with the returned texture, the app should dispose of it with a
    /// call to [`SDL_DestroyTexture()`].
    ///
    /// ### Parameters
    /// - `renderer`: the [`SDL_Renderer`] to use to create the GPU texture.
    /// - `file`: a path on the filesystem to load an image from.
    ///
    /// ### Return value
    /// Returns a new texture, or NULL on error.
    ///
    /// ### Availability
    /// This function is available since SDL_image 3.0.0.
    ///
    /// ### See also
    /// - [`IMG_LoadTextureTyped_IO`]
    /// - [`IMG_LoadTexture_IO`]
    pub fn IMG_LoadTexture(
        renderer: *mut SDL_Renderer,
        file: *const ::core::ffi::c_char,
    ) -> *mut SDL_Texture;
}

extern "C" {
    /// Load an image from an SDL data source into a GPU texture.
    ///
    /// An [`SDL_Texture`] represents an image in GPU memory, usable by SDL's 2D Render
    /// API. This can be significantly more efficient than using a CPU-bound
    /// [`SDL_Surface`] if you don't need to manipulate the image directly after
    /// loading it.
    ///
    /// If the loaded image has transparency or a colorkey, a texture with an alpha
    /// channel will be created. Otherwise, SDL_image will attempt to create an
    /// [`SDL_Texture`] in the most format that most reasonably represents the image
    /// data (but in many cases, this will just end up being 32-bit RGB or 32-bit
    /// RGBA).
    ///
    /// If `closeio` is true, `src` will be closed before returning, whether this
    /// function succeeds or not. SDL_image reads everything it needs from `src`
    /// during this call in any case.
    ///
    /// There is a separate function to read files from disk without having to deal
    /// with [`SDL_IOStream`]\: `IMG_LoadTexture(renderer, "filename.jpg")` will call
    /// this function and manage those details for you, determining the file type
    /// from the filename's extension.
    ///
    /// There is also [`IMG_LoadTextureTyped_IO()`], which is equivalent to this
    /// function except a file extension (like "BMP", "JPG", etc) can be specified,
    /// in case SDL_image cannot autodetect the file format.
    ///
    /// If you would rather decode an image to an [`SDL_Surface`] (a buffer of pixels
    /// in CPU memory), call [`IMG_Load()`] instead.
    ///
    /// When done with the returned texture, the app should dispose of it with a
    /// call to [`SDL_DestroyTexture()`].
    ///
    /// ### Parameters
    /// - `renderer`: the [`SDL_Renderer`] to use to create the GPU texture.
    /// - `src`: an [`SDL_IOStream`] that data will be read from.
    /// - `closeio`: true to close/free the [`SDL_IOStream`] before returning, false
    ///   to leave it open.
    ///
    /// ### Return value
    /// Returns a new texture, or NULL on error.
    ///
    /// ### Availability
    /// This function is available since SDL_image 3.0.0.
    ///
    /// ### See also
    /// - [`IMG_LoadTexture`]
    /// - [`IMG_LoadTextureTyped_IO`]
    /// - [`SDL_DestroyTexture`]
    pub fn IMG_LoadTexture_IO(
        renderer: *mut SDL_Renderer,
        src: *mut SDL_IOStream,
        closeio: ::core::primitive::bool,
    ) -> *mut SDL_Texture;
}

extern "C" {
    /// Load an image from an SDL data source into a GPU texture.
    ///
    /// An [`SDL_Texture`] represents an image in GPU memory, usable by SDL's 2D Render
    /// API. This can be significantly more efficient than using a CPU-bound
    /// [`SDL_Surface`] if you don't need to manipulate the image directly after
    /// loading it.
    ///
    /// If the loaded image has transparency or a colorkey, a texture with an alpha
    /// channel will be created. Otherwise, SDL_image will attempt to create an
    /// [`SDL_Texture`] in the most format that most reasonably represents the image
    /// data (but in many cases, this will just end up being 32-bit RGB or 32-bit
    /// RGBA).
    ///
    /// If `closeio` is true, `src` will be closed before returning, whether this
    /// function succeeds or not. SDL_image reads everything it needs from `src`
    /// during this call in any case.
    ///
    /// Even though this function accepts a file type, SDL_image may still try
    /// other decoders that are capable of detecting file type from the contents of
    /// the image data, but may rely on the caller-provided type string for formats
    /// that it cannot autodetect. If `type` is NULL, SDL_image will rely solely on
    /// its ability to guess the format.
    ///
    /// There is a separate function to read files from disk without having to deal
    /// with [`SDL_IOStream`]\: `IMG_LoadTexture("filename.jpg")` will call this
    /// function and manage those details for you, determining the file type from
    /// the filename's extension.
    ///
    /// There is also [`IMG_LoadTexture_IO()`], which is equivalent to this function
    /// except that it will rely on SDL_image to determine what type of data it is
    /// loading, much like passing a NULL for type.
    ///
    /// If you would rather decode an image to an [`SDL_Surface`] (a buffer of pixels
    /// in CPU memory), call [`IMG_LoadTyped_IO()`] instead.
    ///
    /// When done with the returned texture, the app should dispose of it with a
    /// call to [`SDL_DestroyTexture()`].
    ///
    /// ### Parameters
    /// - `renderer`: the [`SDL_Renderer`] to use to create the GPU texture.
    /// - `src`: an [`SDL_IOStream`] that data will be read from.
    /// - `closeio`: true to close/free the [`SDL_IOStream`] before returning, false
    ///   to leave it open.
    /// - `type`: a filename extension that represent this data ("BMP", "GIF",
    ///   "PNG", etc).
    ///
    /// ### Return value
    /// Returns a new texture, or NULL on error.
    ///
    /// ### Availability
    /// This function is available since SDL_image 3.0.0.
    ///
    /// ### See also
    /// - [`IMG_LoadTexture`]
    /// - [`IMG_LoadTexture_IO`]
    /// - [`SDL_DestroyTexture`]
    pub fn IMG_LoadTextureTyped_IO(
        renderer: *mut SDL_Renderer,
        src: *mut SDL_IOStream,
        closeio: ::core::primitive::bool,
        r#type: *const ::core::ffi::c_char,
    ) -> *mut SDL_Texture;
}

extern "C" {
    /// Detect AVIF image data on a readable/seekable [`SDL_IOStream`].
    ///
    /// This function attempts to determine if a file is a given filetype, reading
    /// the least amount possible from the [`SDL_IOStream`] (usually a few bytes).
    ///
    /// There is no distinction made between "not the filetype in question" and
    /// basic i/o errors.
    ///
    /// This function will always attempt to seek `src` back to where it started
    /// when this function was called, but it will not report any errors in doing
    /// so, but assuming seeking works, this means you can immediately use this
    /// with a different IMG_isTYPE function, or load the image without further
    /// seeking.
    ///
    /// You do not need to call this function to load data; SDL_image can work to
    /// determine file type in many cases in its standard load functions.
    ///
    /// ### Parameters
    /// - `src`: a seekable/readable [`SDL_IOStream`] to provide image data.
    ///
    /// ### Return value
    /// Returns non-zero if this is AVIF data, zero otherwise.
    ///
    /// ### Availability
    /// This function is available since SDL_image 3.0.0.
    ///
    /// ### See also
    /// - [`IMG_isAVIF`]
    /// - [`IMG_isICO`]
    /// - [`IMG_isCUR`]
    /// - [`IMG_isBMP`]
    /// - [`IMG_isGIF`]
    /// - [`IMG_isJPG`]
    /// - [`IMG_isJXL`]
    /// - [`IMG_isLBM`]
    /// - [`IMG_isPCX`]
    /// - [`IMG_isPNG`]
    /// - [`IMG_isPNM`]
    /// - [`IMG_isSVG`]
    /// - [`IMG_isQOI`]
    /// - [`IMG_isTIF`]
    /// - [`IMG_isXCF`]
    /// - [`IMG_isXPM`]
    /// - [`IMG_isXV`]
    /// - [`IMG_isWEBP`]
    pub fn IMG_isAVIF(src: *mut SDL_IOStream) -> ::core::primitive::bool;
}

extern "C" {
    /// Detect ICO image data on a readable/seekable [`SDL_IOStream`].
    ///
    /// This function attempts to determine if a file is a given filetype, reading
    /// the least amount possible from the [`SDL_IOStream`] (usually a few bytes).
    ///
    /// There is no distinction made between "not the filetype in question" and
    /// basic i/o errors.
    ///
    /// This function will always attempt to seek `src` back to where it started
    /// when this function was called, but it will not report any errors in doing
    /// so, but assuming seeking works, this means you can immediately use this
    /// with a different IMG_isTYPE function, or load the image without further
    /// seeking.
    ///
    /// You do not need to call this function to load data; SDL_image can work to
    /// determine file type in many cases in its standard load functions.
    ///
    /// ### Parameters
    /// - `src`: a seekable/readable [`SDL_IOStream`] to provide image data.
    ///
    /// ### Return value
    /// Returns non-zero if this is ICO data, zero otherwise.
    ///
    /// ### Availability
    /// This function is available since SDL_image 3.0.0.
    ///
    /// ### See also
    /// - [`IMG_isAVIF`]
    /// - [`IMG_isCUR`]
    /// - [`IMG_isBMP`]
    /// - [`IMG_isGIF`]
    /// - [`IMG_isJPG`]
    /// - [`IMG_isJXL`]
    /// - [`IMG_isLBM`]
    /// - [`IMG_isPCX`]
    /// - [`IMG_isPNG`]
    /// - [`IMG_isPNM`]
    /// - [`IMG_isSVG`]
    /// - [`IMG_isQOI`]
    /// - [`IMG_isTIF`]
    /// - [`IMG_isXCF`]
    /// - [`IMG_isXPM`]
    /// - [`IMG_isXV`]
    /// - [`IMG_isWEBP`]
    pub fn IMG_isICO(src: *mut SDL_IOStream) -> ::core::primitive::bool;
}

extern "C" {
    /// Detect CUR image data on a readable/seekable [`SDL_IOStream`].
    ///
    /// This function attempts to determine if a file is a given filetype, reading
    /// the least amount possible from the [`SDL_IOStream`] (usually a few bytes).
    ///
    /// There is no distinction made between "not the filetype in question" and
    /// basic i/o errors.
    ///
    /// This function will always attempt to seek `src` back to where it started
    /// when this function was called, but it will not report any errors in doing
    /// so, but assuming seeking works, this means you can immediately use this
    /// with a different IMG_isTYPE function, or load the image without further
    /// seeking.
    ///
    /// You do not need to call this function to load data; SDL_image can work to
    /// determine file type in many cases in its standard load functions.
    ///
    /// ### Parameters
    /// - `src`: a seekable/readable [`SDL_IOStream`] to provide image data.
    ///
    /// ### Return value
    /// Returns non-zero if this is CUR data, zero otherwise.
    ///
    /// ### Availability
    /// This function is available since SDL_image 3.0.0.
    ///
    /// ### See also
    /// - [`IMG_isAVIF`]
    /// - [`IMG_isICO`]
    /// - [`IMG_isBMP`]
    /// - [`IMG_isGIF`]
    /// - [`IMG_isJPG`]
    /// - [`IMG_isJXL`]
    /// - [`IMG_isLBM`]
    /// - [`IMG_isPCX`]
    /// - [`IMG_isPNG`]
    /// - [`IMG_isPNM`]
    /// - [`IMG_isSVG`]
    /// - [`IMG_isQOI`]
    /// - [`IMG_isTIF`]
    /// - [`IMG_isXCF`]
    /// - [`IMG_isXPM`]
    /// - [`IMG_isXV`]
    /// - [`IMG_isWEBP`]
    pub fn IMG_isCUR(src: *mut SDL_IOStream) -> ::core::primitive::bool;
}

extern "C" {
    /// Detect BMP image data on a readable/seekable [`SDL_IOStream`].
    ///
    /// This function attempts to determine if a file is a given filetype, reading
    /// the least amount possible from the [`SDL_IOStream`] (usually a few bytes).
    ///
    /// There is no distinction made between "not the filetype in question" and
    /// basic i/o errors.
    ///
    /// This function will always attempt to seek `src` back to where it started
    /// when this function was called, but it will not report any errors in doing
    /// so, but assuming seeking works, this means you can immediately use this
    /// with a different IMG_isTYPE function, or load the image without further
    /// seeking.
    ///
    /// You do not need to call this function to load data; SDL_image can work to
    /// determine file type in many cases in its standard load functions.
    ///
    /// ### Parameters
    /// - `src`: a seekable/readable [`SDL_IOStream`] to provide image data.
    ///
    /// ### Return value
    /// Returns non-zero if this is BMP data, zero otherwise.
    ///
    /// ### Availability
    /// This function is available since SDL_image 3.0.0.
    ///
    /// ### See also
    /// - [`IMG_isAVIF`]
    /// - [`IMG_isICO`]
    /// - [`IMG_isCUR`]
    /// - [`IMG_isGIF`]
    /// - [`IMG_isJPG`]
    /// - [`IMG_isJXL`]
    /// - [`IMG_isLBM`]
    /// - [`IMG_isPCX`]
    /// - [`IMG_isPNG`]
    /// - [`IMG_isPNM`]
    /// - [`IMG_isSVG`]
    /// - [`IMG_isQOI`]
    /// - [`IMG_isTIF`]
    /// - [`IMG_isXCF`]
    /// - [`IMG_isXPM`]
    /// - [`IMG_isXV`]
    /// - [`IMG_isWEBP`]
    pub fn IMG_isBMP(src: *mut SDL_IOStream) -> ::core::primitive::bool;
}

extern "C" {
    /// Detect GIF image data on a readable/seekable [`SDL_IOStream`].
    ///
    /// This function attempts to determine if a file is a given filetype, reading
    /// the least amount possible from the [`SDL_IOStream`] (usually a few bytes).
    ///
    /// There is no distinction made between "not the filetype in question" and
    /// basic i/o errors.
    ///
    /// This function will always attempt to seek `src` back to where it started
    /// when this function was called, but it will not report any errors in doing
    /// so, but assuming seeking works, this means you can immediately use this
    /// with a different IMG_isTYPE function, or load the image without further
    /// seeking.
    ///
    /// You do not need to call this function to load data; SDL_image can work to
    /// determine file type in many cases in its standard load functions.
    ///
    /// ### Parameters
    /// - `src`: a seekable/readable [`SDL_IOStream`] to provide image data.
    ///
    /// ### Return value
    /// Returns non-zero if this is GIF data, zero otherwise.
    ///
    /// ### Availability
    /// This function is available since SDL_image 3.0.0.
    ///
    /// ### See also
    /// - [`IMG_isAVIF`]
    /// - [`IMG_isICO`]
    /// - [`IMG_isCUR`]
    /// - [`IMG_isBMP`]
    /// - [`IMG_isJPG`]
    /// - [`IMG_isJXL`]
    /// - [`IMG_isLBM`]
    /// - [`IMG_isPCX`]
    /// - [`IMG_isPNG`]
    /// - [`IMG_isPNM`]
    /// - [`IMG_isSVG`]
    /// - [`IMG_isQOI`]
    /// - [`IMG_isTIF`]
    /// - [`IMG_isXCF`]
    /// - [`IMG_isXPM`]
    /// - [`IMG_isXV`]
    /// - [`IMG_isWEBP`]
    pub fn IMG_isGIF(src: *mut SDL_IOStream) -> ::core::primitive::bool;
}

extern "C" {
    /// Detect JPG image data on a readable/seekable [`SDL_IOStream`].
    ///
    /// This function attempts to determine if a file is a given filetype, reading
    /// the least amount possible from the [`SDL_IOStream`] (usually a few bytes).
    ///
    /// There is no distinction made between "not the filetype in question" and
    /// basic i/o errors.
    ///
    /// This function will always attempt to seek `src` back to where it started
    /// when this function was called, but it will not report any errors in doing
    /// so, but assuming seeking works, this means you can immediately use this
    /// with a different IMG_isTYPE function, or load the image without further
    /// seeking.
    ///
    /// You do not need to call this function to load data; SDL_image can work to
    /// determine file type in many cases in its standard load functions.
    ///
    /// ### Parameters
    /// - `src`: a seekable/readable [`SDL_IOStream`] to provide image data.
    ///
    /// ### Return value
    /// Returns non-zero if this is JPG data, zero otherwise.
    ///
    /// ### Availability
    /// This function is available since SDL_image 3.0.0.
    ///
    /// ### See also
    /// - [`IMG_isAVIF`]
    /// - [`IMG_isICO`]
    /// - [`IMG_isCUR`]
    /// - [`IMG_isBMP`]
    /// - [`IMG_isGIF`]
    /// - [`IMG_isJXL`]
    /// - [`IMG_isLBM`]
    /// - [`IMG_isPCX`]
    /// - [`IMG_isPNG`]
    /// - [`IMG_isPNM`]
    /// - [`IMG_isSVG`]
    /// - [`IMG_isQOI`]
    /// - [`IMG_isTIF`]
    /// - [`IMG_isXCF`]
    /// - [`IMG_isXPM`]
    /// - [`IMG_isXV`]
    /// - [`IMG_isWEBP`]
    pub fn IMG_isJPG(src: *mut SDL_IOStream) -> ::core::primitive::bool;
}

extern "C" {
    /// Detect JXL image data on a readable/seekable [`SDL_IOStream`].
    ///
    /// This function attempts to determine if a file is a given filetype, reading
    /// the least amount possible from the [`SDL_IOStream`] (usually a few bytes).
    ///
    /// There is no distinction made between "not the filetype in question" and
    /// basic i/o errors.
    ///
    /// This function will always attempt to seek `src` back to where it started
    /// when this function was called, but it will not report any errors in doing
    /// so, but assuming seeking works, this means you can immediately use this
    /// with a different IMG_isTYPE function, or load the image without further
    /// seeking.
    ///
    /// You do not need to call this function to load data; SDL_image can work to
    /// determine file type in many cases in its standard load functions.
    ///
    /// ### Parameters
    /// - `src`: a seekable/readable [`SDL_IOStream`] to provide image data.
    ///
    /// ### Return value
    /// Returns non-zero if this is JXL data, zero otherwise.
    ///
    /// ### Availability
    /// This function is available since SDL_image 3.0.0.
    ///
    /// ### See also
    /// - [`IMG_isAVIF`]
    /// - [`IMG_isICO`]
    /// - [`IMG_isCUR`]
    /// - [`IMG_isBMP`]
    /// - [`IMG_isGIF`]
    /// - [`IMG_isJPG`]
    /// - [`IMG_isLBM`]
    /// - [`IMG_isPCX`]
    /// - [`IMG_isPNG`]
    /// - [`IMG_isPNM`]
    /// - [`IMG_isSVG`]
    /// - [`IMG_isQOI`]
    /// - [`IMG_isTIF`]
    /// - [`IMG_isXCF`]
    /// - [`IMG_isXPM`]
    /// - [`IMG_isXV`]
    /// - [`IMG_isWEBP`]
    pub fn IMG_isJXL(src: *mut SDL_IOStream) -> ::core::primitive::bool;
}

extern "C" {
    /// Detect LBM image data on a readable/seekable [`SDL_IOStream`].
    ///
    /// This function attempts to determine if a file is a given filetype, reading
    /// the least amount possible from the [`SDL_IOStream`] (usually a few bytes).
    ///
    /// There is no distinction made between "not the filetype in question" and
    /// basic i/o errors.
    ///
    /// This function will always attempt to seek `src` back to where it started
    /// when this function was called, but it will not report any errors in doing
    /// so, but assuming seeking works, this means you can immediately use this
    /// with a different IMG_isTYPE function, or load the image without further
    /// seeking.
    ///
    /// You do not need to call this function to load data; SDL_image can work to
    /// determine file type in many cases in its standard load functions.
    ///
    /// ### Parameters
    /// - `src`: a seekable/readable [`SDL_IOStream`] to provide image data.
    ///
    /// ### Return value
    /// Returns non-zero if this is LBM data, zero otherwise.
    ///
    /// ### Availability
    /// This function is available since SDL_image 3.0.0.
    ///
    /// ### See also
    /// - [`IMG_isAVIF`]
    /// - [`IMG_isICO`]
    /// - [`IMG_isCUR`]
    /// - [`IMG_isBMP`]
    /// - [`IMG_isGIF`]
    /// - [`IMG_isJPG`]
    /// - [`IMG_isJXL`]
    /// - [`IMG_isPCX`]
    /// - [`IMG_isPNG`]
    /// - [`IMG_isPNM`]
    /// - [`IMG_isSVG`]
    /// - [`IMG_isQOI`]
    /// - [`IMG_isTIF`]
    /// - [`IMG_isXCF`]
    /// - [`IMG_isXPM`]
    /// - [`IMG_isXV`]
    /// - [`IMG_isWEBP`]
    pub fn IMG_isLBM(src: *mut SDL_IOStream) -> ::core::primitive::bool;
}

extern "C" {
    /// Detect PCX image data on a readable/seekable [`SDL_IOStream`].
    ///
    /// This function attempts to determine if a file is a given filetype, reading
    /// the least amount possible from the [`SDL_IOStream`] (usually a few bytes).
    ///
    /// There is no distinction made between "not the filetype in question" and
    /// basic i/o errors.
    ///
    /// This function will always attempt to seek `src` back to where it started
    /// when this function was called, but it will not report any errors in doing
    /// so, but assuming seeking works, this means you can immediately use this
    /// with a different IMG_isTYPE function, or load the image without further
    /// seeking.
    ///
    /// You do not need to call this function to load data; SDL_image can work to
    /// determine file type in many cases in its standard load functions.
    ///
    /// ### Parameters
    /// - `src`: a seekable/readable [`SDL_IOStream`] to provide image data.
    ///
    /// ### Return value
    /// Returns non-zero if this is PCX data, zero otherwise.
    ///
    /// ### Availability
    /// This function is available since SDL_image 3.0.0.
    ///
    /// ### See also
    /// - [`IMG_isAVIF`]
    /// - [`IMG_isICO`]
    /// - [`IMG_isCUR`]
    /// - [`IMG_isBMP`]
    /// - [`IMG_isGIF`]
    /// - [`IMG_isJPG`]
    /// - [`IMG_isJXL`]
    /// - [`IMG_isLBM`]
    /// - [`IMG_isPNG`]
    /// - [`IMG_isPNM`]
    /// - [`IMG_isSVG`]
    /// - [`IMG_isQOI`]
    /// - [`IMG_isTIF`]
    /// - [`IMG_isXCF`]
    /// - [`IMG_isXPM`]
    /// - [`IMG_isXV`]
    /// - [`IMG_isWEBP`]
    pub fn IMG_isPCX(src: *mut SDL_IOStream) -> ::core::primitive::bool;
}

extern "C" {
    /// Detect PNG image data on a readable/seekable [`SDL_IOStream`].
    ///
    /// This function attempts to determine if a file is a given filetype, reading
    /// the least amount possible from the [`SDL_IOStream`] (usually a few bytes).
    ///
    /// There is no distinction made between "not the filetype in question" and
    /// basic i/o errors.
    ///
    /// This function will always attempt to seek `src` back to where it started
    /// when this function was called, but it will not report any errors in doing
    /// so, but assuming seeking works, this means you can immediately use this
    /// with a different IMG_isTYPE function, or load the image without further
    /// seeking.
    ///
    /// You do not need to call this function to load data; SDL_image can work to
    /// determine file type in many cases in its standard load functions.
    ///
    /// ### Parameters
    /// - `src`: a seekable/readable [`SDL_IOStream`] to provide image data.
    ///
    /// ### Return value
    /// Returns non-zero if this is PNG data, zero otherwise.
    ///
    /// ### Availability
    /// This function is available since SDL_image 3.0.0.
    ///
    /// ### See also
    /// - [`IMG_isAVIF`]
    /// - [`IMG_isICO`]
    /// - [`IMG_isCUR`]
    /// - [`IMG_isBMP`]
    /// - [`IMG_isGIF`]
    /// - [`IMG_isJPG`]
    /// - [`IMG_isJXL`]
    /// - [`IMG_isLBM`]
    /// - [`IMG_isPCX`]
    /// - [`IMG_isPNM`]
    /// - [`IMG_isSVG`]
    /// - [`IMG_isQOI`]
    /// - [`IMG_isTIF`]
    /// - [`IMG_isXCF`]
    /// - [`IMG_isXPM`]
    /// - [`IMG_isXV`]
    /// - [`IMG_isWEBP`]
    pub fn IMG_isPNG(src: *mut SDL_IOStream) -> ::core::primitive::bool;
}

extern "C" {
    /// Detect PNM image data on a readable/seekable [`SDL_IOStream`].
    ///
    /// This function attempts to determine if a file is a given filetype, reading
    /// the least amount possible from the [`SDL_IOStream`] (usually a few bytes).
    ///
    /// There is no distinction made between "not the filetype in question" and
    /// basic i/o errors.
    ///
    /// This function will always attempt to seek `src` back to where it started
    /// when this function was called, but it will not report any errors in doing
    /// so, but assuming seeking works, this means you can immediately use this
    /// with a different IMG_isTYPE function, or load the image without further
    /// seeking.
    ///
    /// You do not need to call this function to load data; SDL_image can work to
    /// determine file type in many cases in its standard load functions.
    ///
    /// ### Parameters
    /// - `src`: a seekable/readable [`SDL_IOStream`] to provide image data.
    ///
    /// ### Return value
    /// Returns non-zero if this is PNM data, zero otherwise.
    ///
    /// ### Availability
    /// This function is available since SDL_image 3.0.0.
    ///
    /// ### See also
    /// - [`IMG_isAVIF`]
    /// - [`IMG_isICO`]
    /// - [`IMG_isCUR`]
    /// - [`IMG_isBMP`]
    /// - [`IMG_isGIF`]
    /// - [`IMG_isJPG`]
    /// - [`IMG_isJXL`]
    /// - [`IMG_isLBM`]
    /// - [`IMG_isPCX`]
    /// - [`IMG_isPNG`]
    /// - [`IMG_isSVG`]
    /// - [`IMG_isQOI`]
    /// - [`IMG_isTIF`]
    /// - [`IMG_isXCF`]
    /// - [`IMG_isXPM`]
    /// - [`IMG_isXV`]
    /// - [`IMG_isWEBP`]
    pub fn IMG_isPNM(src: *mut SDL_IOStream) -> ::core::primitive::bool;
}

extern "C" {
    /// Detect SVG image data on a readable/seekable [`SDL_IOStream`].
    ///
    /// This function attempts to determine if a file is a given filetype, reading
    /// the least amount possible from the [`SDL_IOStream`] (usually a few bytes).
    ///
    /// There is no distinction made between "not the filetype in question" and
    /// basic i/o errors.
    ///
    /// This function will always attempt to seek `src` back to where it started
    /// when this function was called, but it will not report any errors in doing
    /// so, but assuming seeking works, this means you can immediately use this
    /// with a different IMG_isTYPE function, or load the image without further
    /// seeking.
    ///
    /// You do not need to call this function to load data; SDL_image can work to
    /// determine file type in many cases in its standard load functions.
    ///
    /// ### Parameters
    /// - `src`: a seekable/readable [`SDL_IOStream`] to provide image data.
    ///
    /// ### Return value
    /// Returns non-zero if this is SVG data, zero otherwise.
    ///
    /// ### Availability
    /// This function is available since SDL_image 3.0.0.
    ///
    /// ### See also
    /// - [`IMG_isAVIF`]
    /// - [`IMG_isICO`]
    /// - [`IMG_isCUR`]
    /// - [`IMG_isBMP`]
    /// - [`IMG_isGIF`]
    /// - [`IMG_isJPG`]
    /// - [`IMG_isJXL`]
    /// - [`IMG_isLBM`]
    /// - [`IMG_isPCX`]
    /// - [`IMG_isPNG`]
    /// - [`IMG_isPNM`]
    /// - [`IMG_isQOI`]
    /// - [`IMG_isTIF`]
    /// - [`IMG_isXCF`]
    /// - [`IMG_isXPM`]
    /// - [`IMG_isXV`]
    /// - [`IMG_isWEBP`]
    pub fn IMG_isSVG(src: *mut SDL_IOStream) -> ::core::primitive::bool;
}

extern "C" {
    /// Detect QOI image data on a readable/seekable [`SDL_IOStream`].
    ///
    /// This function attempts to determine if a file is a given filetype, reading
    /// the least amount possible from the [`SDL_IOStream`] (usually a few bytes).
    ///
    /// There is no distinction made between "not the filetype in question" and
    /// basic i/o errors.
    ///
    /// This function will always attempt to seek `src` back to where it started
    /// when this function was called, but it will not report any errors in doing
    /// so, but assuming seeking works, this means you can immediately use this
    /// with a different IMG_isTYPE function, or load the image without further
    /// seeking.
    ///
    /// You do not need to call this function to load data; SDL_image can work to
    /// determine file type in many cases in its standard load functions.
    ///
    /// ### Parameters
    /// - `src`: a seekable/readable [`SDL_IOStream`] to provide image data.
    ///
    /// ### Return value
    /// Returns non-zero if this is QOI data, zero otherwise.
    ///
    /// ### Availability
    /// This function is available since SDL_image 3.0.0.
    ///
    /// ### See also
    /// - [`IMG_isAVIF`]
    /// - [`IMG_isICO`]
    /// - [`IMG_isCUR`]
    /// - [`IMG_isBMP`]
    /// - [`IMG_isGIF`]
    /// - [`IMG_isJPG`]
    /// - [`IMG_isJXL`]
    /// - [`IMG_isLBM`]
    /// - [`IMG_isPCX`]
    /// - [`IMG_isPNG`]
    /// - [`IMG_isPNM`]
    /// - [`IMG_isSVG`]
    /// - [`IMG_isTIF`]
    /// - [`IMG_isXCF`]
    /// - [`IMG_isXPM`]
    /// - [`IMG_isXV`]
    /// - [`IMG_isWEBP`]
    pub fn IMG_isQOI(src: *mut SDL_IOStream) -> ::core::primitive::bool;
}

extern "C" {
    /// Detect TIFF image data on a readable/seekable [`SDL_IOStream`].
    ///
    /// This function attempts to determine if a file is a given filetype, reading
    /// the least amount possible from the [`SDL_IOStream`] (usually a few bytes).
    ///
    /// There is no distinction made between "not the filetype in question" and
    /// basic i/o errors.
    ///
    /// This function will always attempt to seek `src` back to where it started
    /// when this function was called, but it will not report any errors in doing
    /// so, but assuming seeking works, this means you can immediately use this
    /// with a different IMG_isTYPE function, or load the image without further
    /// seeking.
    ///
    /// You do not need to call this function to load data; SDL_image can work to
    /// determine file type in many cases in its standard load functions.
    ///
    /// ### Parameters
    /// - `src`: a seekable/readable [`SDL_IOStream`] to provide image data.
    ///
    /// ### Return value
    /// Returns non-zero if this is TIFF data, zero otherwise.
    ///
    /// ### Availability
    /// This function is available since SDL_image 3.0.0.
    ///
    /// ### See also
    /// - [`IMG_isAVIF`]
    /// - [`IMG_isICO`]
    /// - [`IMG_isCUR`]
    /// - [`IMG_isBMP`]
    /// - [`IMG_isGIF`]
    /// - [`IMG_isJPG`]
    /// - [`IMG_isJXL`]
    /// - [`IMG_isLBM`]
    /// - [`IMG_isPCX`]
    /// - [`IMG_isPNG`]
    /// - [`IMG_isPNM`]
    /// - [`IMG_isSVG`]
    /// - [`IMG_isQOI`]
    /// - [`IMG_isXCF`]
    /// - [`IMG_isXPM`]
    /// - [`IMG_isXV`]
    /// - [`IMG_isWEBP`]
    pub fn IMG_isTIF(src: *mut SDL_IOStream) -> ::core::primitive::bool;
}

extern "C" {
    /// Detect XCF image data on a readable/seekable [`SDL_IOStream`].
    ///
    /// This function attempts to determine if a file is a given filetype, reading
    /// the least amount possible from the [`SDL_IOStream`] (usually a few bytes).
    ///
    /// There is no distinction made between "not the filetype in question" and
    /// basic i/o errors.
    ///
    /// This function will always attempt to seek `src` back to where it started
    /// when this function was called, but it will not report any errors in doing
    /// so, but assuming seeking works, this means you can immediately use this
    /// with a different IMG_isTYPE function, or load the image without further
    /// seeking.
    ///
    /// You do not need to call this function to load data; SDL_image can work to
    /// determine file type in many cases in its standard load functions.
    ///
    /// ### Parameters
    /// - `src`: a seekable/readable [`SDL_IOStream`] to provide image data.
    ///
    /// ### Return value
    /// Returns non-zero if this is XCF data, zero otherwise.
    ///
    /// ### Availability
    /// This function is available since SDL_image 3.0.0.
    ///
    /// ### See also
    /// - [`IMG_isAVIF`]
    /// - [`IMG_isICO`]
    /// - [`IMG_isCUR`]
    /// - [`IMG_isBMP`]
    /// - [`IMG_isGIF`]
    /// - [`IMG_isJPG`]
    /// - [`IMG_isJXL`]
    /// - [`IMG_isLBM`]
    /// - [`IMG_isPCX`]
    /// - [`IMG_isPNG`]
    /// - [`IMG_isPNM`]
    /// - [`IMG_isSVG`]
    /// - [`IMG_isQOI`]
    /// - [`IMG_isTIF`]
    /// - [`IMG_isXPM`]
    /// - [`IMG_isXV`]
    /// - [`IMG_isWEBP`]
    pub fn IMG_isXCF(src: *mut SDL_IOStream) -> ::core::primitive::bool;
}

extern "C" {
    /// Detect XPM image data on a readable/seekable [`SDL_IOStream`].
    ///
    /// This function attempts to determine if a file is a given filetype, reading
    /// the least amount possible from the [`SDL_IOStream`] (usually a few bytes).
    ///
    /// There is no distinction made between "not the filetype in question" and
    /// basic i/o errors.
    ///
    /// This function will always attempt to seek `src` back to where it started
    /// when this function was called, but it will not report any errors in doing
    /// so, but assuming seeking works, this means you can immediately use this
    /// with a different IMG_isTYPE function, or load the image without further
    /// seeking.
    ///
    /// You do not need to call this function to load data; SDL_image can work to
    /// determine file type in many cases in its standard load functions.
    ///
    /// ### Parameters
    /// - `src`: a seekable/readable [`SDL_IOStream`] to provide image data.
    ///
    /// ### Return value
    /// Returns non-zero if this is XPM data, zero otherwise.
    ///
    /// ### Availability
    /// This function is available since SDL_image 3.0.0.
    ///
    /// ### See also
    /// - [`IMG_isAVIF`]
    /// - [`IMG_isICO`]
    /// - [`IMG_isCUR`]
    /// - [`IMG_isBMP`]
    /// - [`IMG_isGIF`]
    /// - [`IMG_isJPG`]
    /// - [`IMG_isJXL`]
    /// - [`IMG_isLBM`]
    /// - [`IMG_isPCX`]
    /// - [`IMG_isPNG`]
    /// - [`IMG_isPNM`]
    /// - [`IMG_isSVG`]
    /// - [`IMG_isQOI`]
    /// - [`IMG_isTIF`]
    /// - [`IMG_isXCF`]
    /// - [`IMG_isXV`]
    /// - [`IMG_isWEBP`]
    pub fn IMG_isXPM(src: *mut SDL_IOStream) -> ::core::primitive::bool;
}

extern "C" {
    /// Detect XV image data on a readable/seekable [`SDL_IOStream`].
    ///
    /// This function attempts to determine if a file is a given filetype, reading
    /// the least amount possible from the [`SDL_IOStream`] (usually a few bytes).
    ///
    /// There is no distinction made between "not the filetype in question" and
    /// basic i/o errors.
    ///
    /// This function will always attempt to seek `src` back to where it started
    /// when this function was called, but it will not report any errors in doing
    /// so, but assuming seeking works, this means you can immediately use this
    /// with a different IMG_isTYPE function, or load the image without further
    /// seeking.
    ///
    /// You do not need to call this function to load data; SDL_image can work to
    /// determine file type in many cases in its standard load functions.
    ///
    /// ### Parameters
    /// - `src`: a seekable/readable [`SDL_IOStream`] to provide image data.
    ///
    /// ### Return value
    /// Returns non-zero if this is XV data, zero otherwise.
    ///
    /// ### Availability
    /// This function is available since SDL_image 3.0.0.
    ///
    /// ### See also
    /// - [`IMG_isAVIF`]
    /// - [`IMG_isICO`]
    /// - [`IMG_isCUR`]
    /// - [`IMG_isBMP`]
    /// - [`IMG_isGIF`]
    /// - [`IMG_isJPG`]
    /// - [`IMG_isJXL`]
    /// - [`IMG_isLBM`]
    /// - [`IMG_isPCX`]
    /// - [`IMG_isPNG`]
    /// - [`IMG_isPNM`]
    /// - [`IMG_isSVG`]
    /// - [`IMG_isQOI`]
    /// - [`IMG_isTIF`]
    /// - [`IMG_isXCF`]
    /// - [`IMG_isXPM`]
    /// - [`IMG_isWEBP`]
    pub fn IMG_isXV(src: *mut SDL_IOStream) -> ::core::primitive::bool;
}

extern "C" {
    /// Detect WEBP image data on a readable/seekable [`SDL_IOStream`].
    ///
    /// This function attempts to determine if a file is a given filetype, reading
    /// the least amount possible from the [`SDL_IOStream`] (usually a few bytes).
    ///
    /// There is no distinction made between "not the filetype in question" and
    /// basic i/o errors.
    ///
    /// This function will always attempt to seek `src` back to where it started
    /// when this function was called, but it will not report any errors in doing
    /// so, but assuming seeking works, this means you can immediately use this
    /// with a different IMG_isTYPE function, or load the image without further
    /// seeking.
    ///
    /// You do not need to call this function to load data; SDL_image can work to
    /// determine file type in many cases in its standard load functions.
    ///
    /// ### Parameters
    /// - `src`: a seekable/readable [`SDL_IOStream`] to provide image data.
    ///
    /// ### Return value
    /// Returns non-zero if this is WEBP data, zero otherwise.
    ///
    /// ### Availability
    /// This function is available since SDL_image 3.0.0.
    ///
    /// ### See also
    /// - [`IMG_isAVIF`]
    /// - [`IMG_isICO`]
    /// - [`IMG_isCUR`]
    /// - [`IMG_isBMP`]
    /// - [`IMG_isGIF`]
    /// - [`IMG_isJPG`]
    /// - [`IMG_isJXL`]
    /// - [`IMG_isLBM`]
    /// - [`IMG_isPCX`]
    /// - [`IMG_isPNG`]
    /// - [`IMG_isPNM`]
    /// - [`IMG_isSVG`]
    /// - [`IMG_isQOI`]
    /// - [`IMG_isTIF`]
    /// - [`IMG_isXCF`]
    /// - [`IMG_isXPM`]
    /// - [`IMG_isXV`]
    pub fn IMG_isWEBP(src: *mut SDL_IOStream) -> ::core::primitive::bool;
}

extern "C" {
    /// Load a AVIF image directly.
    ///
    /// If you know you definitely have a AVIF image, you can call this function,
    /// which will skip SDL_image's file format detection routines. Generally it's
    /// better to use the abstract interfaces; also, there is only an [`SDL_IOStream`]
    /// interface available here.
    ///
    /// ### Parameters
    /// - `src`: an [`SDL_IOStream`] to load image data from.
    ///
    /// ### Return value
    /// Returns SDL surface, or NULL on error.
    ///
    /// ### Availability
    /// This function is available since SDL_image 3.0.0.
    ///
    /// ### See also
    /// - [`IMG_LoadICO_IO`]
    /// - [`IMG_LoadCUR_IO`]
    /// - [`IMG_LoadBMP_IO`]
    /// - [`IMG_LoadGIF_IO`]
    /// - [`IMG_LoadJPG_IO`]
    /// - [`IMG_LoadJXL_IO`]
    /// - [`IMG_LoadLBM_IO`]
    /// - [`IMG_LoadPCX_IO`]
    /// - [`IMG_LoadPNG_IO`]
    /// - [`IMG_LoadPNM_IO`]
    /// - [`IMG_LoadSVG_IO`]
    /// - [`IMG_LoadQOI_IO`]
    /// - [`IMG_LoadTGA_IO`]
    /// - [`IMG_LoadTIF_IO`]
    /// - [`IMG_LoadXCF_IO`]
    /// - [`IMG_LoadXPM_IO`]
    /// - [`IMG_LoadXV_IO`]
    /// - [`IMG_LoadWEBP_IO`]
    pub fn IMG_LoadAVIF_IO(src: *mut SDL_IOStream) -> *mut SDL_Surface;
}

extern "C" {
    /// Load a ICO image directly.
    ///
    /// If you know you definitely have a ICO image, you can call this function,
    /// which will skip SDL_image's file format detection routines. Generally it's
    /// better to use the abstract interfaces; also, there is only an [`SDL_IOStream`]
    /// interface available here.
    ///
    /// ### Parameters
    /// - `src`: an [`SDL_IOStream`] to load image data from.
    ///
    /// ### Return value
    /// Returns SDL surface, or NULL on error.
    ///
    /// ### Availability
    /// This function is available since SDL_image 3.0.0.
    ///
    /// ### See also
    /// - [`IMG_LoadAVIF_IO`]
    /// - [`IMG_LoadCUR_IO`]
    /// - [`IMG_LoadBMP_IO`]
    /// - [`IMG_LoadGIF_IO`]
    /// - [`IMG_LoadJPG_IO`]
    /// - [`IMG_LoadJXL_IO`]
    /// - [`IMG_LoadLBM_IO`]
    /// - [`IMG_LoadPCX_IO`]
    /// - [`IMG_LoadPNG_IO`]
    /// - [`IMG_LoadPNM_IO`]
    /// - [`IMG_LoadSVG_IO`]
    /// - [`IMG_LoadQOI_IO`]
    /// - [`IMG_LoadTGA_IO`]
    /// - [`IMG_LoadTIF_IO`]
    /// - [`IMG_LoadXCF_IO`]
    /// - [`IMG_LoadXPM_IO`]
    /// - [`IMG_LoadXV_IO`]
    /// - [`IMG_LoadWEBP_IO`]
    pub fn IMG_LoadICO_IO(src: *mut SDL_IOStream) -> *mut SDL_Surface;
}

extern "C" {
    /// Load a CUR image directly.
    ///
    /// If you know you definitely have a CUR image, you can call this function,
    /// which will skip SDL_image's file format detection routines. Generally it's
    /// better to use the abstract interfaces; also, there is only an [`SDL_IOStream`]
    /// interface available here.
    ///
    /// ### Parameters
    /// - `src`: an [`SDL_IOStream`] to load image data from.
    ///
    /// ### Return value
    /// Returns SDL surface, or NULL on error.
    ///
    /// ### Availability
    /// This function is available since SDL_image 3.0.0.
    ///
    /// ### See also
    /// - [`IMG_LoadAVIF_IO`]
    /// - [`IMG_LoadICO_IO`]
    /// - [`IMG_LoadBMP_IO`]
    /// - [`IMG_LoadGIF_IO`]
    /// - [`IMG_LoadJPG_IO`]
    /// - [`IMG_LoadJXL_IO`]
    /// - [`IMG_LoadLBM_IO`]
    /// - [`IMG_LoadPCX_IO`]
    /// - [`IMG_LoadPNG_IO`]
    /// - [`IMG_LoadPNM_IO`]
    /// - [`IMG_LoadSVG_IO`]
    /// - [`IMG_LoadQOI_IO`]
    /// - [`IMG_LoadTGA_IO`]
    /// - [`IMG_LoadTIF_IO`]
    /// - [`IMG_LoadXCF_IO`]
    /// - [`IMG_LoadXPM_IO`]
    /// - [`IMG_LoadXV_IO`]
    /// - [`IMG_LoadWEBP_IO`]
    pub fn IMG_LoadCUR_IO(src: *mut SDL_IOStream) -> *mut SDL_Surface;
}

extern "C" {
    /// Load a BMP image directly.
    ///
    /// If you know you definitely have a BMP image, you can call this function,
    /// which will skip SDL_image's file format detection routines. Generally it's
    /// better to use the abstract interfaces; also, there is only an [`SDL_IOStream`]
    /// interface available here.
    ///
    /// ### Parameters
    /// - `src`: an [`SDL_IOStream`] to load image data from.
    ///
    /// ### Return value
    /// Returns SDL surface, or NULL on error.
    ///
    /// ### Availability
    /// This function is available since SDL_image 3.0.0.
    ///
    /// ### See also
    /// - [`IMG_LoadAVIF_IO`]
    /// - [`IMG_LoadICO_IO`]
    /// - [`IMG_LoadCUR_IO`]
    /// - [`IMG_LoadGIF_IO`]
    /// - [`IMG_LoadJPG_IO`]
    /// - [`IMG_LoadJXL_IO`]
    /// - [`IMG_LoadLBM_IO`]
    /// - [`IMG_LoadPCX_IO`]
    /// - [`IMG_LoadPNG_IO`]
    /// - [`IMG_LoadPNM_IO`]
    /// - [`IMG_LoadSVG_IO`]
    /// - [`IMG_LoadQOI_IO`]
    /// - [`IMG_LoadTGA_IO`]
    /// - [`IMG_LoadTIF_IO`]
    /// - [`IMG_LoadXCF_IO`]
    /// - [`IMG_LoadXPM_IO`]
    /// - [`IMG_LoadXV_IO`]
    /// - [`IMG_LoadWEBP_IO`]
    pub fn IMG_LoadBMP_IO(src: *mut SDL_IOStream) -> *mut SDL_Surface;
}

extern "C" {
    /// Load a GIF image directly.
    ///
    /// If you know you definitely have a GIF image, you can call this function,
    /// which will skip SDL_image's file format detection routines. Generally it's
    /// better to use the abstract interfaces; also, there is only an [`SDL_IOStream`]
    /// interface available here.
    ///
    /// ### Parameters
    /// - `src`: an [`SDL_IOStream`] to load image data from.
    ///
    /// ### Return value
    /// Returns SDL surface, or NULL on error.
    ///
    /// ### Availability
    /// This function is available since SDL_image 3.0.0.
    ///
    /// ### See also
    /// - [`IMG_LoadAVIF_IO`]
    /// - [`IMG_LoadICO_IO`]
    /// - [`IMG_LoadCUR_IO`]
    /// - [`IMG_LoadBMP_IO`]
    /// - [`IMG_LoadJPG_IO`]
    /// - [`IMG_LoadJXL_IO`]
    /// - [`IMG_LoadLBM_IO`]
    /// - [`IMG_LoadPCX_IO`]
    /// - [`IMG_LoadPNG_IO`]
    /// - [`IMG_LoadPNM_IO`]
    /// - [`IMG_LoadSVG_IO`]
    /// - [`IMG_LoadQOI_IO`]
    /// - [`IMG_LoadTGA_IO`]
    /// - [`IMG_LoadTIF_IO`]
    /// - [`IMG_LoadXCF_IO`]
    /// - [`IMG_LoadXPM_IO`]
    /// - [`IMG_LoadXV_IO`]
    /// - [`IMG_LoadWEBP_IO`]
    pub fn IMG_LoadGIF_IO(src: *mut SDL_IOStream) -> *mut SDL_Surface;
}

extern "C" {
    /// Load a JPG image directly.
    ///
    /// If you know you definitely have a JPG image, you can call this function,
    /// which will skip SDL_image's file format detection routines. Generally it's
    /// better to use the abstract interfaces; also, there is only an [`SDL_IOStream`]
    /// interface available here.
    ///
    /// ### Parameters
    /// - `src`: an [`SDL_IOStream`] to load image data from.
    ///
    /// ### Return value
    /// Returns SDL surface, or NULL on error.
    ///
    /// ### Availability
    /// This function is available since SDL_image 3.0.0.
    ///
    /// ### See also
    /// - [`IMG_LoadAVIF_IO`]
    /// - [`IMG_LoadICO_IO`]
    /// - [`IMG_LoadCUR_IO`]
    /// - [`IMG_LoadBMP_IO`]
    /// - [`IMG_LoadGIF_IO`]
    /// - [`IMG_LoadJXL_IO`]
    /// - [`IMG_LoadLBM_IO`]
    /// - [`IMG_LoadPCX_IO`]
    /// - [`IMG_LoadPNG_IO`]
    /// - [`IMG_LoadPNM_IO`]
    /// - [`IMG_LoadSVG_IO`]
    /// - [`IMG_LoadQOI_IO`]
    /// - [`IMG_LoadTGA_IO`]
    /// - [`IMG_LoadTIF_IO`]
    /// - [`IMG_LoadXCF_IO`]
    /// - [`IMG_LoadXPM_IO`]
    /// - [`IMG_LoadXV_IO`]
    /// - [`IMG_LoadWEBP_IO`]
    pub fn IMG_LoadJPG_IO(src: *mut SDL_IOStream) -> *mut SDL_Surface;
}

extern "C" {
    /// Load a JXL image directly.
    ///
    /// If you know you definitely have a JXL image, you can call this function,
    /// which will skip SDL_image's file format detection routines. Generally it's
    /// better to use the abstract interfaces; also, there is only an [`SDL_IOStream`]
    /// interface available here.
    ///
    /// ### Parameters
    /// - `src`: an [`SDL_IOStream`] to load image data from.
    ///
    /// ### Return value
    /// Returns SDL surface, or NULL on error.
    ///
    /// ### Availability
    /// This function is available since SDL_image 3.0.0.
    ///
    /// ### See also
    /// - [`IMG_LoadAVIF_IO`]
    /// - [`IMG_LoadICO_IO`]
    /// - [`IMG_LoadCUR_IO`]
    /// - [`IMG_LoadBMP_IO`]
    /// - [`IMG_LoadGIF_IO`]
    /// - [`IMG_LoadJPG_IO`]
    /// - [`IMG_LoadLBM_IO`]
    /// - [`IMG_LoadPCX_IO`]
    /// - [`IMG_LoadPNG_IO`]
    /// - [`IMG_LoadPNM_IO`]
    /// - [`IMG_LoadSVG_IO`]
    /// - [`IMG_LoadQOI_IO`]
    /// - [`IMG_LoadTGA_IO`]
    /// - [`IMG_LoadTIF_IO`]
    /// - [`IMG_LoadXCF_IO`]
    /// - [`IMG_LoadXPM_IO`]
    /// - [`IMG_LoadXV_IO`]
    /// - [`IMG_LoadWEBP_IO`]
    pub fn IMG_LoadJXL_IO(src: *mut SDL_IOStream) -> *mut SDL_Surface;
}

extern "C" {
    /// Load a LBM image directly.
    ///
    /// If you know you definitely have a LBM image, you can call this function,
    /// which will skip SDL_image's file format detection routines. Generally it's
    /// better to use the abstract interfaces; also, there is only an [`SDL_IOStream`]
    /// interface available here.
    ///
    /// ### Parameters
    /// - `src`: an [`SDL_IOStream`] to load image data from.
    ///
    /// ### Return value
    /// Returns SDL surface, or NULL on error.
    ///
    /// ### Availability
    /// This function is available since SDL_image 3.0.0.
    ///
    /// ### See also
    /// - [`IMG_LoadAVIF_IO`]
    /// - [`IMG_LoadICO_IO`]
    /// - [`IMG_LoadCUR_IO`]
    /// - [`IMG_LoadBMP_IO`]
    /// - [`IMG_LoadGIF_IO`]
    /// - [`IMG_LoadJPG_IO`]
    /// - [`IMG_LoadJXL_IO`]
    /// - [`IMG_LoadPCX_IO`]
    /// - [`IMG_LoadPNG_IO`]
    /// - [`IMG_LoadPNM_IO`]
    /// - [`IMG_LoadSVG_IO`]
    /// - [`IMG_LoadQOI_IO`]
    /// - [`IMG_LoadTGA_IO`]
    /// - [`IMG_LoadTIF_IO`]
    /// - [`IMG_LoadXCF_IO`]
    /// - [`IMG_LoadXPM_IO`]
    /// - [`IMG_LoadXV_IO`]
    /// - [`IMG_LoadWEBP_IO`]
    pub fn IMG_LoadLBM_IO(src: *mut SDL_IOStream) -> *mut SDL_Surface;
}

extern "C" {
    /// Load a PCX image directly.
    ///
    /// If you know you definitely have a PCX image, you can call this function,
    /// which will skip SDL_image's file format detection routines. Generally it's
    /// better to use the abstract interfaces; also, there is only an [`SDL_IOStream`]
    /// interface available here.
    ///
    /// ### Parameters
    /// - `src`: an [`SDL_IOStream`] to load image data from.
    ///
    /// ### Return value
    /// Returns SDL surface, or NULL on error.
    ///
    /// ### Availability
    /// This function is available since SDL_image 3.0.0.
    ///
    /// ### See also
    /// - [`IMG_LoadAVIF_IO`]
    /// - [`IMG_LoadICO_IO`]
    /// - [`IMG_LoadCUR_IO`]
    /// - [`IMG_LoadBMP_IO`]
    /// - [`IMG_LoadGIF_IO`]
    /// - [`IMG_LoadJPG_IO`]
    /// - [`IMG_LoadJXL_IO`]
    /// - [`IMG_LoadLBM_IO`]
    /// - [`IMG_LoadPNG_IO`]
    /// - [`IMG_LoadPNM_IO`]
    /// - [`IMG_LoadSVG_IO`]
    /// - [`IMG_LoadQOI_IO`]
    /// - [`IMG_LoadTGA_IO`]
    /// - [`IMG_LoadTIF_IO`]
    /// - [`IMG_LoadXCF_IO`]
    /// - [`IMG_LoadXPM_IO`]
    /// - [`IMG_LoadXV_IO`]
    /// - [`IMG_LoadWEBP_IO`]
    pub fn IMG_LoadPCX_IO(src: *mut SDL_IOStream) -> *mut SDL_Surface;
}

extern "C" {
    /// Load a PNG image directly.
    ///
    /// If you know you definitely have a PNG image, you can call this function,
    /// which will skip SDL_image's file format detection routines. Generally it's
    /// better to use the abstract interfaces; also, there is only an [`SDL_IOStream`]
    /// interface available here.
    ///
    /// ### Parameters
    /// - `src`: an [`SDL_IOStream`] to load image data from.
    ///
    /// ### Return value
    /// Returns SDL surface, or NULL on error.
    ///
    /// ### Availability
    /// This function is available since SDL_image 3.0.0.
    ///
    /// ### See also
    /// - [`IMG_LoadAVIF_IO`]
    /// - [`IMG_LoadICO_IO`]
    /// - [`IMG_LoadCUR_IO`]
    /// - [`IMG_LoadBMP_IO`]
    /// - [`IMG_LoadGIF_IO`]
    /// - [`IMG_LoadJPG_IO`]
    /// - [`IMG_LoadJXL_IO`]
    /// - [`IMG_LoadLBM_IO`]
    /// - [`IMG_LoadPCX_IO`]
    /// - [`IMG_LoadPNM_IO`]
    /// - [`IMG_LoadSVG_IO`]
    /// - [`IMG_LoadQOI_IO`]
    /// - [`IMG_LoadTGA_IO`]
    /// - [`IMG_LoadTIF_IO`]
    /// - [`IMG_LoadXCF_IO`]
    /// - [`IMG_LoadXPM_IO`]
    /// - [`IMG_LoadXV_IO`]
    /// - [`IMG_LoadWEBP_IO`]
    pub fn IMG_LoadPNG_IO(src: *mut SDL_IOStream) -> *mut SDL_Surface;
}

extern "C" {
    /// Load a PNM image directly.
    ///
    /// If you know you definitely have a PNM image, you can call this function,
    /// which will skip SDL_image's file format detection routines. Generally it's
    /// better to use the abstract interfaces; also, there is only an [`SDL_IOStream`]
    /// interface available here.
    ///
    /// ### Parameters
    /// - `src`: an [`SDL_IOStream`] to load image data from.
    ///
    /// ### Return value
    /// Returns SDL surface, or NULL on error.
    ///
    /// ### Availability
    /// This function is available since SDL_image 3.0.0.
    ///
    /// ### See also
    /// - [`IMG_LoadAVIF_IO`]
    /// - [`IMG_LoadICO_IO`]
    /// - [`IMG_LoadCUR_IO`]
    /// - [`IMG_LoadBMP_IO`]
    /// - [`IMG_LoadGIF_IO`]
    /// - [`IMG_LoadJPG_IO`]
    /// - [`IMG_LoadJXL_IO`]
    /// - [`IMG_LoadLBM_IO`]
    /// - [`IMG_LoadPCX_IO`]
    /// - [`IMG_LoadPNG_IO`]
    /// - [`IMG_LoadSVG_IO`]
    /// - [`IMG_LoadQOI_IO`]
    /// - [`IMG_LoadTGA_IO`]
    /// - [`IMG_LoadTIF_IO`]
    /// - [`IMG_LoadXCF_IO`]
    /// - [`IMG_LoadXPM_IO`]
    /// - [`IMG_LoadXV_IO`]
    /// - [`IMG_LoadWEBP_IO`]
    pub fn IMG_LoadPNM_IO(src: *mut SDL_IOStream) -> *mut SDL_Surface;
}

extern "C" {
    /// Load a SVG image directly.
    ///
    /// If you know you definitely have a SVG image, you can call this function,
    /// which will skip SDL_image's file format detection routines. Generally it's
    /// better to use the abstract interfaces; also, there is only an [`SDL_IOStream`]
    /// interface available here.
    ///
    /// ### Parameters
    /// - `src`: an [`SDL_IOStream`] to load image data from.
    ///
    /// ### Return value
    /// Returns SDL surface, or NULL on error.
    ///
    /// ### Availability
    /// This function is available since SDL_image 3.0.0.
    ///
    /// ### See also
    /// - [`IMG_LoadAVIF_IO`]
    /// - [`IMG_LoadICO_IO`]
    /// - [`IMG_LoadCUR_IO`]
    /// - [`IMG_LoadBMP_IO`]
    /// - [`IMG_LoadGIF_IO`]
    /// - [`IMG_LoadJPG_IO`]
    /// - [`IMG_LoadJXL_IO`]
    /// - [`IMG_LoadLBM_IO`]
    /// - [`IMG_LoadPCX_IO`]
    /// - [`IMG_LoadPNG_IO`]
    /// - [`IMG_LoadPNM_IO`]
    /// - [`IMG_LoadQOI_IO`]
    /// - [`IMG_LoadTGA_IO`]
    /// - [`IMG_LoadTIF_IO`]
    /// - [`IMG_LoadXCF_IO`]
    /// - [`IMG_LoadXPM_IO`]
    /// - [`IMG_LoadXV_IO`]
    /// - [`IMG_LoadWEBP_IO`]
    pub fn IMG_LoadSVG_IO(src: *mut SDL_IOStream) -> *mut SDL_Surface;
}

extern "C" {
    /// Load a QOI image directly.
    ///
    /// If you know you definitely have a QOI image, you can call this function,
    /// which will skip SDL_image's file format detection routines. Generally it's
    /// better to use the abstract interfaces; also, there is only an [`SDL_IOStream`]
    /// interface available here.
    ///
    /// ### Parameters
    /// - `src`: an [`SDL_IOStream`] to load image data from.
    ///
    /// ### Return value
    /// Returns SDL surface, or NULL on error.
    ///
    /// ### Availability
    /// This function is available since SDL_image 3.0.0.
    ///
    /// ### See also
    /// - [`IMG_LoadAVIF_IO`]
    /// - [`IMG_LoadICO_IO`]
    /// - [`IMG_LoadCUR_IO`]
    /// - [`IMG_LoadBMP_IO`]
    /// - [`IMG_LoadGIF_IO`]
    /// - [`IMG_LoadJPG_IO`]
    /// - [`IMG_LoadJXL_IO`]
    /// - [`IMG_LoadLBM_IO`]
    /// - [`IMG_LoadPCX_IO`]
    /// - [`IMG_LoadPNG_IO`]
    /// - [`IMG_LoadPNM_IO`]
    /// - [`IMG_LoadSVG_IO`]
    /// - [`IMG_LoadTGA_IO`]
    /// - [`IMG_LoadTIF_IO`]
    /// - [`IMG_LoadXCF_IO`]
    /// - [`IMG_LoadXPM_IO`]
    /// - [`IMG_LoadXV_IO`]
    /// - [`IMG_LoadWEBP_IO`]
    pub fn IMG_LoadQOI_IO(src: *mut SDL_IOStream) -> *mut SDL_Surface;
}

extern "C" {
    /// Load a TGA image directly.
    ///
    /// If you know you definitely have a TGA image, you can call this function,
    /// which will skip SDL_image's file format detection routines. Generally it's
    /// better to use the abstract interfaces; also, there is only an [`SDL_IOStream`]
    /// interface available here.
    ///
    /// ### Parameters
    /// - `src`: an [`SDL_IOStream`] to load image data from.
    ///
    /// ### Return value
    /// Returns SDL surface, or NULL on error.
    ///
    /// ### Availability
    /// This function is available since SDL_image 3.0.0.
    ///
    /// ### See also
    /// - [`IMG_LoadAVIF_IO`]
    /// - [`IMG_LoadICO_IO`]
    /// - [`IMG_LoadCUR_IO`]
    /// - [`IMG_LoadBMP_IO`]
    /// - [`IMG_LoadGIF_IO`]
    /// - [`IMG_LoadJPG_IO`]
    /// - [`IMG_LoadJXL_IO`]
    /// - [`IMG_LoadLBM_IO`]
    /// - [`IMG_LoadPCX_IO`]
    /// - [`IMG_LoadPNG_IO`]
    /// - [`IMG_LoadPNM_IO`]
    /// - [`IMG_LoadSVG_IO`]
    /// - [`IMG_LoadQOI_IO`]
    /// - [`IMG_LoadTIF_IO`]
    /// - [`IMG_LoadXCF_IO`]
    /// - [`IMG_LoadXPM_IO`]
    /// - [`IMG_LoadXV_IO`]
    /// - [`IMG_LoadWEBP_IO`]
    pub fn IMG_LoadTGA_IO(src: *mut SDL_IOStream) -> *mut SDL_Surface;
}

extern "C" {
    /// Load a TIFF image directly.
    ///
    /// If you know you definitely have a TIFF image, you can call this function,
    /// which will skip SDL_image's file format detection routines. Generally it's
    /// better to use the abstract interfaces; also, there is only an [`SDL_IOStream`]
    /// interface available here.
    ///
    /// ### Parameters
    /// - `src`: an [`SDL_IOStream`] to load image data from.
    ///
    /// ### Return value
    /// Returns SDL surface, or NULL on error.
    ///
    /// ### Availability
    /// This function is available since SDL_image 3.0.0.
    ///
    /// ### See also
    /// - [`IMG_LoadAVIF_IO`]
    /// - [`IMG_LoadICO_IO`]
    /// - [`IMG_LoadCUR_IO`]
    /// - [`IMG_LoadBMP_IO`]
    /// - [`IMG_LoadGIF_IO`]
    /// - [`IMG_LoadJPG_IO`]
    /// - [`IMG_LoadJXL_IO`]
    /// - [`IMG_LoadLBM_IO`]
    /// - [`IMG_LoadPCX_IO`]
    /// - [`IMG_LoadPNG_IO`]
    /// - [`IMG_LoadPNM_IO`]
    /// - [`IMG_LoadSVG_IO`]
    /// - [`IMG_LoadQOI_IO`]
    /// - [`IMG_LoadTGA_IO`]
    /// - [`IMG_LoadXCF_IO`]
    /// - [`IMG_LoadXPM_IO`]
    /// - [`IMG_LoadXV_IO`]
    /// - [`IMG_LoadWEBP_IO`]
    pub fn IMG_LoadTIF_IO(src: *mut SDL_IOStream) -> *mut SDL_Surface;
}

extern "C" {
    /// Load a XCF image directly.
    ///
    /// If you know you definitely have a XCF image, you can call this function,
    /// which will skip SDL_image's file format detection routines. Generally it's
    /// better to use the abstract interfaces; also, there is only an [`SDL_IOStream`]
    /// interface available here.
    ///
    /// ### Parameters
    /// - `src`: an [`SDL_IOStream`] to load image data from.
    ///
    /// ### Return value
    /// Returns SDL surface, or NULL on error.
    ///
    /// ### Availability
    /// This function is available since SDL_image 3.0.0.
    ///
    /// ### See also
    /// - [`IMG_LoadAVIF_IO`]
    /// - [`IMG_LoadICO_IO`]
    /// - [`IMG_LoadCUR_IO`]
    /// - [`IMG_LoadBMP_IO`]
    /// - [`IMG_LoadGIF_IO`]
    /// - [`IMG_LoadJPG_IO`]
    /// - [`IMG_LoadJXL_IO`]
    /// - [`IMG_LoadLBM_IO`]
    /// - [`IMG_LoadPCX_IO`]
    /// - [`IMG_LoadPNG_IO`]
    /// - [`IMG_LoadPNM_IO`]
    /// - [`IMG_LoadSVG_IO`]
    /// - [`IMG_LoadQOI_IO`]
    /// - [`IMG_LoadTGA_IO`]
    /// - [`IMG_LoadTIF_IO`]
    /// - [`IMG_LoadXPM_IO`]
    /// - [`IMG_LoadXV_IO`]
    /// - [`IMG_LoadWEBP_IO`]
    pub fn IMG_LoadXCF_IO(src: *mut SDL_IOStream) -> *mut SDL_Surface;
}

extern "C" {
    /// Load a XPM image directly.
    ///
    /// If you know you definitely have a XPM image, you can call this function,
    /// which will skip SDL_image's file format detection routines. Generally it's
    /// better to use the abstract interfaces; also, there is only an [`SDL_IOStream`]
    /// interface available here.
    ///
    /// ### Parameters
    /// - `src`: an [`SDL_IOStream`] to load image data from.
    ///
    /// ### Return value
    /// Returns SDL surface, or NULL on error.
    ///
    /// ### Availability
    /// This function is available since SDL_image 3.0.0.
    ///
    /// ### See also
    /// - [`IMG_LoadAVIF_IO`]
    /// - [`IMG_LoadICO_IO`]
    /// - [`IMG_LoadCUR_IO`]
    /// - [`IMG_LoadBMP_IO`]
    /// - [`IMG_LoadGIF_IO`]
    /// - [`IMG_LoadJPG_IO`]
    /// - [`IMG_LoadJXL_IO`]
    /// - [`IMG_LoadLBM_IO`]
    /// - [`IMG_LoadPCX_IO`]
    /// - [`IMG_LoadPNG_IO`]
    /// - [`IMG_LoadPNM_IO`]
    /// - [`IMG_LoadSVG_IO`]
    /// - [`IMG_LoadQOI_IO`]
    /// - [`IMG_LoadTGA_IO`]
    /// - [`IMG_LoadTIF_IO`]
    /// - [`IMG_LoadXCF_IO`]
    /// - [`IMG_LoadXV_IO`]
    /// - [`IMG_LoadWEBP_IO`]
    pub fn IMG_LoadXPM_IO(src: *mut SDL_IOStream) -> *mut SDL_Surface;
}

extern "C" {
    /// Load a XV image directly.
    ///
    /// If you know you definitely have a XV image, you can call this function,
    /// which will skip SDL_image's file format detection routines. Generally it's
    /// better to use the abstract interfaces; also, there is only an [`SDL_IOStream`]
    /// interface available here.
    ///
    /// ### Parameters
    /// - `src`: an [`SDL_IOStream`] to load image data from.
    ///
    /// ### Return value
    /// Returns SDL surface, or NULL on error.
    ///
    /// ### Availability
    /// This function is available since SDL_image 3.0.0.
    ///
    /// ### See also
    /// - [`IMG_LoadAVIF_IO`]
    /// - [`IMG_LoadICO_IO`]
    /// - [`IMG_LoadCUR_IO`]
    /// - [`IMG_LoadBMP_IO`]
    /// - [`IMG_LoadGIF_IO`]
    /// - [`IMG_LoadJPG_IO`]
    /// - [`IMG_LoadJXL_IO`]
    /// - [`IMG_LoadLBM_IO`]
    /// - [`IMG_LoadPCX_IO`]
    /// - [`IMG_LoadPNG_IO`]
    /// - [`IMG_LoadPNM_IO`]
    /// - [`IMG_LoadSVG_IO`]
    /// - [`IMG_LoadQOI_IO`]
    /// - [`IMG_LoadTGA_IO`]
    /// - [`IMG_LoadTIF_IO`]
    /// - [`IMG_LoadXCF_IO`]
    /// - [`IMG_LoadXPM_IO`]
    /// - [`IMG_LoadWEBP_IO`]
    pub fn IMG_LoadXV_IO(src: *mut SDL_IOStream) -> *mut SDL_Surface;
}

extern "C" {
    /// Load a WEBP image directly.
    ///
    /// If you know you definitely have a WEBP image, you can call this function,
    /// which will skip SDL_image's file format detection routines. Generally it's
    /// better to use the abstract interfaces; also, there is only an [`SDL_IOStream`]
    /// interface available here.
    ///
    /// ### Parameters
    /// - `src`: an [`SDL_IOStream`] to load image data from.
    ///
    /// ### Return value
    /// Returns SDL surface, or NULL on error.
    ///
    /// ### Availability
    /// This function is available since SDL_image 3.0.0.
    ///
    /// ### See also
    /// - [`IMG_LoadAVIF_IO`]
    /// - [`IMG_LoadICO_IO`]
    /// - [`IMG_LoadCUR_IO`]
    /// - [`IMG_LoadBMP_IO`]
    /// - [`IMG_LoadGIF_IO`]
    /// - [`IMG_LoadJPG_IO`]
    /// - [`IMG_LoadJXL_IO`]
    /// - [`IMG_LoadLBM_IO`]
    /// - [`IMG_LoadPCX_IO`]
    /// - [`IMG_LoadPNG_IO`]
    /// - [`IMG_LoadPNM_IO`]
    /// - [`IMG_LoadSVG_IO`]
    /// - [`IMG_LoadQOI_IO`]
    /// - [`IMG_LoadTGA_IO`]
    /// - [`IMG_LoadTIF_IO`]
    /// - [`IMG_LoadXCF_IO`]
    /// - [`IMG_LoadXPM_IO`]
    /// - [`IMG_LoadXV_IO`]
    pub fn IMG_LoadWEBP_IO(src: *mut SDL_IOStream) -> *mut SDL_Surface;
}

extern "C" {
    /// Load an SVG image, scaled to a specific size.
    ///
    /// Since SVG files are resolution-independent, you specify the size you would
    /// like the output image to be and it will be generated at those dimensions.
    ///
    /// Either width or height may be 0 and the image will be auto-sized to
    /// preserve aspect ratio.
    ///
    /// When done with the returned surface, the app should dispose of it with a
    /// call to [`SDL_DestroySurface()`].
    ///
    /// ### Parameters
    /// - `src`: an [`SDL_IOStream`] to load SVG data from.
    /// - `width`: desired width of the generated surface, in pixels.
    /// - `height`: desired height of the generated surface, in pixels.
    ///
    /// ### Return value
    /// Returns a new SDL surface, or NULL on error.
    ///
    /// ### Availability
    /// This function is available since SDL_image 3.0.0.
    pub fn IMG_LoadSizedSVG_IO(
        src: *mut SDL_IOStream,
        width: ::core::ffi::c_int,
        height: ::core::ffi::c_int,
    ) -> *mut SDL_Surface;
}

extern "C" {
    /// Load an XPM image from a memory array.
    ///
    /// The returned surface will be an 8bpp indexed surface, if possible,
    /// otherwise it will be 32bpp. If you always want 32-bit data, use
    /// [`IMG_ReadXPMFromArrayToRGB888()`] instead.
    ///
    /// When done with the returned surface, the app should dispose of it with a
    /// call to [`SDL_DestroySurface()`].
    ///
    /// ### Parameters
    /// - `xpm`: a null-terminated array of strings that comprise XPM data.
    ///
    /// ### Return value
    /// Returns a new SDL surface, or NULL on error.
    ///
    /// ### Availability
    /// This function is available since SDL_image 3.0.0.
    ///
    /// ### See also
    /// - [`IMG_ReadXPMFromArrayToRGB888`]
    pub fn IMG_ReadXPMFromArray(xpm: *mut *mut ::core::ffi::c_char) -> *mut SDL_Surface;
}

extern "C" {
    /// Load an XPM image from a memory array.
    ///
    /// The returned surface will always be a 32-bit RGB surface. If you want 8-bit
    /// indexed colors (and the XPM data allows it), use [`IMG_ReadXPMFromArray()`]
    /// instead.
    ///
    /// When done with the returned surface, the app should dispose of it with a
    /// call to [`SDL_DestroySurface()`].
    ///
    /// ### Parameters
    /// - `xpm`: a null-terminated array of strings that comprise XPM data.
    ///
    /// ### Return value
    /// Returns a new SDL surface, or NULL on error.
    ///
    /// ### Availability
    /// This function is available since SDL_image 3.0.0.
    ///
    /// ### See also
    /// - [`IMG_ReadXPMFromArray`]
    pub fn IMG_ReadXPMFromArrayToRGB888(xpm: *mut *mut ::core::ffi::c_char) -> *mut SDL_Surface;
}

extern "C" {
    /// Save an [`SDL_Surface`] into a AVIF image file.
    ///
    /// If the file already exists, it will be overwritten.
    ///
    /// ### Parameters
    /// - `surface`: the SDL surface to save.
    /// - `file`: path on the filesystem to write new file to.
    /// - `quality`: the desired quality, ranging between 0 (lowest) and 100
    ///   (highest).
    ///
    /// ### Return value
    /// Returns true on success or false on failure; call [`SDL_GetError()`] for more
    ///   information.
    ///
    /// ### Availability
    /// This function is available since SDL_image 3.0.0.
    ///
    /// ### See also
    /// - [`IMG_SaveAVIF_IO`]
    pub fn IMG_SaveAVIF(
        surface: *mut SDL_Surface,
        file: *const ::core::ffi::c_char,
        quality: ::core::ffi::c_int,
    ) -> ::core::primitive::bool;
}

extern "C" {
    /// Save an [`SDL_Surface`] into AVIF image data, via an [`SDL_IOStream`].
    ///
    /// If you just want to save to a filename, you can use [`IMG_SaveAVIF()`] instead.
    ///
    /// If `closeio` is true, `dst` will be closed before returning, whether this
    /// function succeeds or not.
    ///
    /// ### Parameters
    /// - `surface`: the SDL surface to save.
    /// - `dst`: the [`SDL_IOStream`] to save the image data to.
    /// - `closeio`: true to close/free the [`SDL_IOStream`] before returning, false
    ///   to leave it open.
    /// - `quality`: the desired quality, ranging between 0 (lowest) and 100
    ///   (highest).
    ///
    /// ### Return value
    /// Returns true on success or false on failure; call [`SDL_GetError()`] for more
    ///   information.
    ///
    /// ### Availability
    /// This function is available since SDL_image 3.0.0.
    ///
    /// ### See also
    /// - [`IMG_SaveAVIF`]
    pub fn IMG_SaveAVIF_IO(
        surface: *mut SDL_Surface,
        dst: *mut SDL_IOStream,
        closeio: ::core::primitive::bool,
        quality: ::core::ffi::c_int,
    ) -> ::core::primitive::bool;
}

extern "C" {
    /// Save an [`SDL_Surface`] into a PNG image file.
    ///
    /// If the file already exists, it will be overwritten.
    ///
    /// ### Parameters
    /// - `surface`: the SDL surface to save.
    /// - `file`: path on the filesystem to write new file to.
    ///
    /// ### Return value
    /// Returns true on success or false on failure; call [`SDL_GetError()`] for more
    ///   information.
    ///
    /// ### Availability
    /// This function is available since SDL_image 3.0.0.
    ///
    /// ### See also
    /// - [`IMG_SavePNG_IO`]
    pub fn IMG_SavePNG(
        surface: *mut SDL_Surface,
        file: *const ::core::ffi::c_char,
    ) -> ::core::primitive::bool;
}

extern "C" {
    /// Save an [`SDL_Surface`] into PNG image data, via an [`SDL_IOStream`].
    ///
    /// If you just want to save to a filename, you can use [`IMG_SavePNG()`] instead.
    ///
    /// If `closeio` is true, `dst` will be closed before returning, whether this
    /// function succeeds or not.
    ///
    /// ### Parameters
    /// - `surface`: the SDL surface to save.
    /// - `dst`: the [`SDL_IOStream`] to save the image data to.
    /// - `closeio`: true to close/free the [`SDL_IOStream`] before returning, false
    ///   to leave it open.
    ///
    /// ### Return value
    /// Returns true on success or false on failure; call [`SDL_GetError()`] for more
    ///   information.
    ///
    /// ### Availability
    /// This function is available since SDL_image 3.0.0.
    ///
    /// ### See also
    /// - [`IMG_SavePNG`]
    pub fn IMG_SavePNG_IO(
        surface: *mut SDL_Surface,
        dst: *mut SDL_IOStream,
        closeio: ::core::primitive::bool,
    ) -> ::core::primitive::bool;
}

extern "C" {
    /// Save an [`SDL_Surface`] into a JPEG image file.
    ///
    /// If the file already exists, it will be overwritten.
    ///
    /// ### Parameters
    /// - `surface`: the SDL surface to save.
    /// - `file`: path on the filesystem to write new file to.
    /// - `quality`: \[0; 33\] is Lowest quality, \[34; 66\] is Middle quality, [67;
    ///   100\] is Highest quality.
    ///
    /// ### Return value
    /// Returns true on success or false on failure; call [`SDL_GetError()`] for more
    ///   information.
    ///
    /// ### Availability
    /// This function is available since SDL_image 3.0.0.
    ///
    /// ### See also
    /// - [`IMG_SaveJPG_IO`]
    pub fn IMG_SaveJPG(
        surface: *mut SDL_Surface,
        file: *const ::core::ffi::c_char,
        quality: ::core::ffi::c_int,
    ) -> ::core::primitive::bool;
}

extern "C" {
    /// Save an [`SDL_Surface`] into JPEG image data, via an [`SDL_IOStream`].
    ///
    /// If you just want to save to a filename, you can use [`IMG_SaveJPG()`] instead.
    ///
    /// If `closeio` is true, `dst` will be closed before returning, whether this
    /// function succeeds or not.
    ///
    /// ### Parameters
    /// - `surface`: the SDL surface to save.
    /// - `dst`: the [`SDL_IOStream`] to save the image data to.
    /// - `closeio`: true to close/free the [`SDL_IOStream`] before returning, false
    ///   to leave it open.
    /// - `quality`: \[0; 33\] is Lowest quality, \[34; 66\] is Middle quality, [67;
    ///   100\] is Highest quality.
    ///
    /// ### Return value
    /// Returns true on success or false on failure; call [`SDL_GetError()`] for more
    ///   information.
    ///
    /// ### Availability
    /// This function is available since SDL_image 3.0.0.
    ///
    /// ### See also
    /// - [`IMG_SaveJPG`]
    pub fn IMG_SaveJPG_IO(
        surface: *mut SDL_Surface,
        dst: *mut SDL_IOStream,
        closeio: ::core::primitive::bool,
        quality: ::core::ffi::c_int,
    ) -> ::core::primitive::bool;
}

/// Animated image support
///
/// Currently only animated GIFs and WEBP images are supported.
#[repr(C)]
#[cfg_attr(feature = "debug-impls", derive(Debug))]
pub struct IMG_Animation {
    pub w: ::core::ffi::c_int,
    pub h: ::core::ffi::c_int,
    pub count: ::core::ffi::c_int,
    pub frames: *mut *mut SDL_Surface,
    pub delays: *mut ::core::ffi::c_int,
}

impl ::core::default::Default for IMG_Animation {
    /// Initialize all fields to zero
    #[inline(always)]
    fn default() -> Self {
        unsafe { ::core::mem::MaybeUninit::<Self>::zeroed().assume_init() }
    }
}

extern "C" {
    /// Load an animation from a file.
    ///
    /// When done with the returned animation, the app should dispose of it with a
    /// call to [`IMG_FreeAnimation()`].
    ///
    /// ### Parameters
    /// - `file`: path on the filesystem containing an animated image.
    ///
    /// ### Return value
    /// Returns a new [`IMG_Animation`], or NULL on error.
    ///
    /// ### Availability
    /// This function is available since SDL_image 3.0.0.
    ///
    /// ### See also
    /// - [`IMG_FreeAnimation`]
    pub fn IMG_LoadAnimation(file: *const ::core::ffi::c_char) -> *mut IMG_Animation;
}

extern "C" {
    /// Load an animation from an [`SDL_IOStream`].
    ///
    /// If `closeio` is true, `src` will be closed before returning, whether this
    /// function succeeds or not. SDL_image reads everything it needs from `src`
    /// during this call in any case.
    ///
    /// When done with the returned animation, the app should dispose of it with a
    /// call to [`IMG_FreeAnimation()`].
    ///
    /// ### Parameters
    /// - `src`: an [`SDL_IOStream`] that data will be read from.
    /// - `closeio`: true to close/free the [`SDL_IOStream`] before returning, false
    ///   to leave it open.
    ///
    /// ### Return value
    /// Returns a new [`IMG_Animation`], or NULL on error.
    ///
    /// ### Availability
    /// This function is available since SDL_image 3.0.0.
    ///
    /// ### See also
    /// - [`IMG_FreeAnimation`]
    pub fn IMG_LoadAnimation_IO(
        src: *mut SDL_IOStream,
        closeio: ::core::primitive::bool,
    ) -> *mut IMG_Animation;
}

extern "C" {
    /// Load an animation from an SDL datasource
    ///
    /// Even though this function accepts a file type, SDL_image may still try
    /// other decoders that are capable of detecting file type from the contents of
    /// the image data, but may rely on the caller-provided type string for formats
    /// that it cannot autodetect. If `type` is NULL, SDL_image will rely solely on
    /// its ability to guess the format.
    ///
    /// If `closeio` is true, `src` will be closed before returning, whether this
    /// function succeeds or not. SDL_image reads everything it needs from `src`
    /// during this call in any case.
    ///
    /// When done with the returned animation, the app should dispose of it with a
    /// call to [`IMG_FreeAnimation()`].
    ///
    /// ### Parameters
    /// - `src`: an [`SDL_IOStream`] that data will be read from.
    /// - `closeio`: true to close/free the [`SDL_IOStream`] before returning, false
    ///   to leave it open.
    /// - `type`: a filename extension that represent this data ("GIF", etc).
    ///
    /// ### Return value
    /// Returns a new [`IMG_Animation`], or NULL on error.
    ///
    /// ### Availability
    /// This function is available since SDL_image 3.0.0.
    ///
    /// ### See also
    /// - [`IMG_LoadAnimation`]
    /// - [`IMG_LoadAnimation_IO`]
    /// - [`IMG_FreeAnimation`]
    pub fn IMG_LoadAnimationTyped_IO(
        src: *mut SDL_IOStream,
        closeio: ::core::primitive::bool,
        r#type: *const ::core::ffi::c_char,
    ) -> *mut IMG_Animation;
}

extern "C" {
    /// Dispose of an [`IMG_Animation`] and free its resources.
    ///
    /// The provided `anim` pointer is not valid once this call returns.
    ///
    /// ### Parameters
    /// - `anim`: [`IMG_Animation`] to dispose of.
    ///
    /// ### Availability
    /// This function is available since SDL_image 3.0.0.
    ///
    /// ### See also
    /// - [`IMG_LoadAnimation`]
    /// - [`IMG_LoadAnimation_IO`]
    /// - [`IMG_LoadAnimationTyped_IO`]
    pub fn IMG_FreeAnimation(anim: *mut IMG_Animation);
}

extern "C" {
    /// Load a GIF animation directly.
    ///
    /// If you know you definitely have a GIF image, you can call this function,
    /// which will skip SDL_image's file format detection routines. Generally it's
    /// better to use the abstract interfaces; also, there is only an [`SDL_IOStream`]
    /// interface available here.
    ///
    /// ### Parameters
    /// - `src`: an [`SDL_IOStream`] that data will be read from.
    ///
    /// ### Return value
    /// Returns a new [`IMG_Animation`], or NULL on error.
    ///
    /// ### Availability
    /// This function is available since SDL_image 3.0.0.
    ///
    /// ### See also
    /// - [`IMG_LoadAnimation`]
    /// - [`IMG_LoadAnimation_IO`]
    /// - [`IMG_LoadAnimationTyped_IO`]
    /// - [`IMG_FreeAnimation`]
    pub fn IMG_LoadGIFAnimation_IO(src: *mut SDL_IOStream) -> *mut IMG_Animation;
}

extern "C" {
    /// Load a WEBP animation directly.
    ///
    /// If you know you definitely have a WEBP image, you can call this function,
    /// which will skip SDL_image's file format detection routines. Generally it's
    /// better to use the abstract interfaces; also, there is only an [`SDL_IOStream`]
    /// interface available here.
    ///
    /// ### Parameters
    /// - `src`: an [`SDL_IOStream`] that data will be read from.
    ///
    /// ### Return value
    /// Returns a new [`IMG_Animation`], or NULL on error.
    ///
    /// ### Availability
    /// This function is available since SDL_image 3.0.0.
    ///
    /// ### See also
    /// - [`IMG_LoadAnimation`]
    /// - [`IMG_LoadAnimation_IO`]
    /// - [`IMG_LoadAnimationTyped_IO`]
    /// - [`IMG_FreeAnimation`]
    pub fn IMG_LoadWEBPAnimation_IO(src: *mut SDL_IOStream) -> *mut IMG_Animation;
}

#[cfg(doc)]
use crate::everything::*;
