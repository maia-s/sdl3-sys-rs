//! Header file for SDL_image library
//!
//! A simple library to load images of various formats as SDL surfaces

use sdl3_sys::everything::*;

/// * Printable format: "%d.%d.%d", MAJOR, MINOR, MICRO
pub const SDL_IMAGE_MAJOR_VERSION: ::core::primitive::i32 = 3;

pub const SDL_IMAGE_MINOR_VERSION: ::core::primitive::i32 = 4;

pub const SDL_IMAGE_MICRO_VERSION: ::core::primitive::i32 = 0;

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

unsafe extern "C" {
    /// This function gets the version of the dynamically linked SDL_image library.
    ///
    /// ## Return value
    /// Returns SDL_image version.
    ///
    /// ## Availability
    /// This function is available since SDL_image 3.0.0.
    pub safe fn IMG_Version() -> ::core::ffi::c_int;
}

unsafe extern "C" {
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
    /// ## Parameters
    /// - `file`: a path on the filesystem to load an image from.
    ///
    /// ## Return value
    /// Returns a new SDL surface, or NULL on error.
    ///
    /// ## Availability
    /// This function is available since SDL_image 3.0.0.
    ///
    /// ## See also
    /// - [`IMG_LoadTyped_IO`]
    /// - [`IMG_Load_IO`]
    pub fn IMG_Load(file: *const ::core::ffi::c_char) -> *mut SDL_Surface;
}

unsafe extern "C" {
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
    /// ## Parameters
    /// - `src`: an [`SDL_IOStream`] that data will be read from.
    /// - `closeio`: true to close/free the [`SDL_IOStream`] before returning, false
    ///   to leave it open.
    ///
    /// ## Return value
    /// Returns a new SDL surface, or NULL on error.
    ///
    /// ## Availability
    /// This function is available since SDL_image 3.0.0.
    ///
    /// ## See also
    /// - [`IMG_Load`]
    /// - [`IMG_LoadTyped_IO`]
    pub fn IMG_Load_IO(
        src: *mut SDL_IOStream,
        closeio: ::core::primitive::bool,
    ) -> *mut SDL_Surface;
}

unsafe extern "C" {
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
    /// ## Parameters
    /// - `src`: an [`SDL_IOStream`] that data will be read from.
    /// - `closeio`: true to close/free the [`SDL_IOStream`] before returning, false
    ///   to leave it open.
    /// - `type`: a filename extension that represent this data ("BMP", "GIF",
    ///   "PNG", etc).
    ///
    /// ## Return value
    /// Returns a new SDL surface, or NULL on error.
    ///
    /// ## Availability
    /// This function is available since SDL_image 3.0.0.
    ///
    /// ## See also
    /// - [`IMG_Load`]
    /// - [`IMG_Load_IO`]
    pub fn IMG_LoadTyped_IO(
        src: *mut SDL_IOStream,
        closeio: ::core::primitive::bool,
        r#type: *const ::core::ffi::c_char,
    ) -> *mut SDL_Surface;
}

unsafe extern "C" {
    /// Load an image from a filesystem path into a texture.
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
    /// ## Parameters
    /// - `renderer`: the [`SDL_Renderer`] to use to create the texture.
    /// - `file`: a path on the filesystem to load an image from.
    ///
    /// ## Return value
    /// Returns a new texture, or NULL on error.
    ///
    /// ## Availability
    /// This function is available since SDL_image 3.0.0.
    ///
    /// ## See also
    /// - [`IMG_LoadTextureTyped_IO`]
    /// - [`IMG_LoadTexture_IO`]
    pub fn IMG_LoadTexture(
        renderer: *mut SDL_Renderer,
        file: *const ::core::ffi::c_char,
    ) -> *mut SDL_Texture;
}

unsafe extern "C" {
    /// Load an image from an SDL data source into a texture.
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
    /// ## Parameters
    /// - `renderer`: the [`SDL_Renderer`] to use to create the texture.
    /// - `src`: an [`SDL_IOStream`] that data will be read from.
    /// - `closeio`: true to close/free the [`SDL_IOStream`] before returning, false
    ///   to leave it open.
    ///
    /// ## Return value
    /// Returns a new texture, or NULL on error.
    ///
    /// ## Availability
    /// This function is available since SDL_image 3.0.0.
    ///
    /// ## See also
    /// - [`IMG_LoadTexture`]
    /// - [`IMG_LoadTextureTyped_IO`]
    pub fn IMG_LoadTexture_IO(
        renderer: *mut SDL_Renderer,
        src: *mut SDL_IOStream,
        closeio: ::core::primitive::bool,
    ) -> *mut SDL_Texture;
}

unsafe extern "C" {
    /// Load an image from an SDL data source into a texture.
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
    /// ## Parameters
    /// - `renderer`: the [`SDL_Renderer`] to use to create the texture.
    /// - `src`: an [`SDL_IOStream`] that data will be read from.
    /// - `closeio`: true to close/free the [`SDL_IOStream`] before returning, false
    ///   to leave it open.
    /// - `type`: a filename extension that represent this data ("BMP", "GIF",
    ///   "PNG", etc).
    ///
    /// ## Return value
    /// Returns a new texture, or NULL on error.
    ///
    /// ## Availability
    /// This function is available since SDL_image 3.0.0.
    ///
    /// ## See also
    /// - [`IMG_LoadTexture`]
    /// - [`IMG_LoadTexture_IO`]
    pub fn IMG_LoadTextureTyped_IO(
        renderer: *mut SDL_Renderer,
        src: *mut SDL_IOStream,
        closeio: ::core::primitive::bool,
        r#type: *const ::core::ffi::c_char,
    ) -> *mut SDL_Texture;
}

unsafe extern "C" {
    /// Load an image from a filesystem path into a GPU texture.
    ///
    /// An [`SDL_GPUTexture`] represents an image in GPU memory, usable by SDL's GPU
    /// API. Regardless of the source format of the image, this function will
    /// create a GPU texture with the format [`SDL_GPU_TEXTUREFORMAT_R8G8B8A8_UNORM`]
    /// with no mip levels. It can be bound as a sampled texture from a graphics or
    /// compute pipeline and as a a readonly storage texture in a compute pipeline.
    ///
    /// There is a separate function to read files from an [`SDL_IOStream`], if you
    /// need an i/o abstraction to provide data from anywhere instead of a simple
    /// filesystem read; that function is [`IMG_LoadGPUTexture_IO()`].
    ///
    /// When done with the returned texture, the app should dispose of it with a
    /// call to [`SDL_ReleaseGPUTexture()`].
    ///
    /// ## Parameters
    /// - `device`: the [`SDL_GPUDevice`] to use to create the GPU texture.
    /// - `copy_pass`: the [`SDL_GPUCopyPass`] to use to upload the loaded image to
    ///   the GPU texture.
    /// - `file`: a path on the filesystem to load an image from.
    /// - `width`: a pointer filled in with the width of the GPU texture. may be
    ///   NULL.
    /// - `height`: a pointer filled in with the width of the GPU texture. may be
    ///   NULL.
    ///
    /// ## Return value
    /// Returns a new GPU texture, or NULL on error.
    ///
    /// ## Availability
    /// This function is available since SDL_image 3.4.0.
    ///
    /// ## See also
    /// - [`IMG_LoadGPUTextureTyped_IO`]
    /// - [`IMG_LoadGPUTexture_IO`]
    pub fn IMG_LoadGPUTexture(
        device: *mut SDL_GPUDevice,
        copy_pass: *mut SDL_GPUCopyPass,
        file: *const ::core::ffi::c_char,
        width: *mut ::core::ffi::c_int,
        height: *mut ::core::ffi::c_int,
    ) -> *mut SDL_GPUTexture;
}

unsafe extern "C" {
    /// Load an image from an SDL data source into a GPU texture.
    ///
    /// An [`SDL_GPUTexture`] represents an image in GPU memory, usable by SDL's GPU
    /// API. Regardless of the source format of the image, this function will
    /// create a GPU texture with the format [`SDL_GPU_TEXTUREFORMAT_R8G8B8A8_UNORM`]
    /// with no mip levels. It can be bound as a sampled texture from a graphics or
    /// compute pipeline and as a a readonly storage texture in a compute pipeline.
    ///
    /// If `closeio` is true, `src` will be closed before returning, whether this
    /// function succeeds or not. SDL_image reads everything it needs from `src`
    /// during this call in any case.
    ///
    /// There is a separate function to read files from disk without having to deal
    /// with [`SDL_IOStream`]\: `IMG_LoadGPUTexture(device, copy_pass, "filename.jpg",
    /// width, height) will call this function and manage those details for you,
    /// determining the file type from the filename's extension.
    ///
    /// There is also [`IMG_LoadGPUTextureTyped_IO()`], which is equivalent to this
    /// function except a file extension (like "BMP", "JPG", etc) can be specified,
    /// in case SDL_image cannot autodetect the file format.
    ///
    /// When done with the returned texture, the app should dispose of it with a
    /// call to [`SDL_ReleaseGPUTexture()`].
    ///
    /// ## Parameters
    /// - `device`: the [`SDL_GPUDevice`] to use to create the GPU texture.
    /// - `copy_pass`: the [`SDL_GPUCopyPass`] to use to upload the loaded image to
    ///   the GPU texture.
    /// - `src`: an [`SDL_IOStream`] that data will be read from.
    /// - `closeio`: true to close/free the [`SDL_IOStream`] before returning, false
    ///   to leave it open.
    /// - `width`: a pointer filled in with the width of the GPU texture. may be
    ///   NULL.
    /// - `height`: a pointer filled in with the width of the GPU texture. may be
    ///   NULL.
    ///
    /// ## Return value
    /// Returns a new GPU texture, or NULL on error.
    ///
    /// ## Availability
    /// This function is available since SDL_image 3.4.0.
    ///
    /// ## See also
    /// - [`IMG_LoadGPUTexture`]
    /// - [`IMG_LoadGPUTextureTyped_IO`]
    pub fn IMG_LoadGPUTexture_IO(
        device: *mut SDL_GPUDevice,
        copy_pass: *mut SDL_GPUCopyPass,
        src: *mut SDL_IOStream,
        closeio: ::core::primitive::bool,
        width: *mut ::core::ffi::c_int,
        height: *mut ::core::ffi::c_int,
    ) -> *mut SDL_GPUTexture;
}

unsafe extern "C" {
    /// Load an image from an SDL data source into a GPU texture.
    ///
    /// An [`SDL_GPUTexture`] represents an image in GPU memory, usable by SDL's GPU
    /// API. Regardless of the source format of the image, this function will
    /// create a GPU texture with the format [`SDL_GPU_TEXTUREFORMAT_R8G8B8A8_UNORM`]
    /// with no mip levels. It can be bound as a sampled texture from a graphics or
    /// compute pipeline and as a a readonly storage texture in a compute pipeline.
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
    /// with [`SDL_IOStream`]\: `IMG_LoadGPUTexture(device, copy_pass, "filename.jpg",
    /// width, height) will call this function and manage those details for you,
    /// determining the file type from the filename's extension.
    ///
    /// There is also [`IMG_LoadGPUTexture_IO()`], which is equivalent to this function
    /// except that it will rely on SDL_image to determine what type of data it is
    /// loading, much like passing a NULL for type.
    ///
    /// When done with the returned texture, the app should dispose of it with a
    /// call to [`SDL_ReleaseGPUTexture()`].
    ///
    /// ## Parameters
    /// - `device`: the [`SDL_GPUDevice`] to use to create the GPU texture.
    /// - `copy_pass`: the [`SDL_GPUCopyPass`] to use to upload the loaded image to
    ///   the GPU texture.
    /// - `src`: an [`SDL_IOStream`] that data will be read from.
    /// - `closeio`: true to close/free the [`SDL_IOStream`] before returning, false
    ///   to leave it open.
    /// - `type`: a filename extension that represent this data ("BMP", "GIF",
    ///   "PNG", etc).
    /// - `width`: a pointer filled in with the width of the GPU texture. may be
    ///   NULL.
    /// - `height`: a pointer filled in with the width of the GPU texture. may be
    ///   NULL.
    ///
    /// ## Return value
    /// Returns a new GPU texture, or NULL on error.
    ///
    /// ## Availability
    /// This function is available since SDL_image 3.4.0.
    ///
    /// ## See also
    /// - [`IMG_LoadGPUTexture`]
    /// - [`IMG_LoadGPUTexture_IO`]
    pub fn IMG_LoadGPUTextureTyped_IO(
        device: *mut SDL_GPUDevice,
        copy_pass: *mut SDL_GPUCopyPass,
        src: *mut SDL_IOStream,
        closeio: ::core::primitive::bool,
        r#type: *const ::core::ffi::c_char,
        width: *mut ::core::ffi::c_int,
        height: *mut ::core::ffi::c_int,
    ) -> *mut SDL_GPUTexture;
}

unsafe extern "C" {
    /// Get the image currently in the clipboard.
    ///
    /// When done with the returned surface, the app should dispose of it with a
    /// call to [`SDL_DestroySurface()`].
    ///
    /// ## Return value
    /// Returns a new SDL surface, or NULL if no supported image is available.
    ///
    /// ## Availability
    /// This function is available since SDL_image 3.4.0.
    pub fn IMG_GetClipboardImage() -> *mut SDL_Surface;
}

unsafe extern "C" {
    /// Detect ANI animated cursor data on a readable/seekable [`SDL_IOStream`].
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
    /// ## Parameters
    /// - `src`: a seekable/readable [`SDL_IOStream`] to provide image data.
    ///
    /// ## Return value
    /// Returns true if this is ANI animated cursor data, false otherwise.
    ///
    /// ## Availability
    /// This function is available since SDL_image 3.0.0.
    ///
    /// ## See also
    /// - [`IMG_isAVIF`]
    /// - [`IMG_isBMP`]
    /// - [`IMG_isCUR`]
    /// - [`IMG_isGIF`]
    /// - [`IMG_isICO`]
    /// - [`IMG_isJPG`]
    /// - [`IMG_isJXL`]
    /// - [`IMG_isLBM`]
    /// - [`IMG_isPCX`]
    /// - [`IMG_isPNG`]
    /// - [`IMG_isPNM`]
    /// - [`IMG_isQOI`]
    /// - [`IMG_isSVG`]
    /// - [`IMG_isTIF`]
    /// - [`IMG_isWEBP`]
    /// - [`IMG_isXCF`]
    /// - [`IMG_isXPM`]
    /// - [`IMG_isXV`]
    pub fn IMG_isANI(src: *mut SDL_IOStream) -> ::core::primitive::bool;
}

unsafe extern "C" {
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
    /// ## Parameters
    /// - `src`: a seekable/readable [`SDL_IOStream`] to provide image data.
    ///
    /// ## Return value
    /// Returns true if this is AVIF data, false otherwise.
    ///
    /// ## Availability
    /// This function is available since SDL_image 3.0.0.
    ///
    /// ## See also
    /// - [`IMG_isANI`]
    /// - [`IMG_isBMP`]
    /// - [`IMG_isCUR`]
    /// - [`IMG_isGIF`]
    /// - [`IMG_isICO`]
    /// - [`IMG_isJPG`]
    /// - [`IMG_isJXL`]
    /// - [`IMG_isLBM`]
    /// - [`IMG_isPCX`]
    /// - [`IMG_isPNG`]
    /// - [`IMG_isPNM`]
    /// - [`IMG_isQOI`]
    /// - [`IMG_isSVG`]
    /// - [`IMG_isTIF`]
    /// - [`IMG_isWEBP`]
    /// - [`IMG_isXCF`]
    /// - [`IMG_isXPM`]
    /// - [`IMG_isXV`]
    pub fn IMG_isAVIF(src: *mut SDL_IOStream) -> ::core::primitive::bool;
}

unsafe extern "C" {
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
    /// ## Parameters
    /// - `src`: a seekable/readable [`SDL_IOStream`] to provide image data.
    ///
    /// ## Return value
    /// Returns true if this is CUR data, false otherwise.
    ///
    /// ## Availability
    /// This function is available since SDL_image 3.0.0.
    ///
    /// ## See also
    /// - [`IMG_isANI`]
    /// - [`IMG_isAVIF`]
    /// - [`IMG_isBMP`]
    /// - [`IMG_isGIF`]
    /// - [`IMG_isICO`]
    /// - [`IMG_isJPG`]
    /// - [`IMG_isJXL`]
    /// - [`IMG_isLBM`]
    /// - [`IMG_isPCX`]
    /// - [`IMG_isPNG`]
    /// - [`IMG_isPNM`]
    /// - [`IMG_isQOI`]
    /// - [`IMG_isSVG`]
    /// - [`IMG_isTIF`]
    /// - [`IMG_isWEBP`]
    /// - [`IMG_isXCF`]
    /// - [`IMG_isXPM`]
    /// - [`IMG_isXV`]
    pub fn IMG_isCUR(src: *mut SDL_IOStream) -> ::core::primitive::bool;
}

unsafe extern "C" {
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
    /// ## Parameters
    /// - `src`: a seekable/readable [`SDL_IOStream`] to provide image data.
    ///
    /// ## Return value
    /// Returns true if this is BMP data, false otherwise.
    ///
    /// ## Availability
    /// This function is available since SDL_image 3.0.0.
    ///
    /// ## See also
    /// - [`IMG_isANI`]
    /// - [`IMG_isAVIF`]
    /// - [`IMG_isCUR`]
    /// - [`IMG_isGIF`]
    /// - [`IMG_isICO`]
    /// - [`IMG_isJPG`]
    /// - [`IMG_isJXL`]
    /// - [`IMG_isLBM`]
    /// - [`IMG_isPCX`]
    /// - [`IMG_isPNG`]
    /// - [`IMG_isPNM`]
    /// - [`IMG_isQOI`]
    /// - [`IMG_isSVG`]
    /// - [`IMG_isTIF`]
    /// - [`IMG_isWEBP`]
    /// - [`IMG_isXCF`]
    /// - [`IMG_isXPM`]
    /// - [`IMG_isXV`]
    pub fn IMG_isBMP(src: *mut SDL_IOStream) -> ::core::primitive::bool;
}

unsafe extern "C" {
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
    /// ## Parameters
    /// - `src`: a seekable/readable [`SDL_IOStream`] to provide image data.
    ///
    /// ## Return value
    /// Returns true if this is GIF data, false otherwise.
    ///
    /// ## Availability
    /// This function is available since SDL_image 3.0.0.
    ///
    /// ## See also
    /// - [`IMG_isANI`]
    /// - [`IMG_isAVIF`]
    /// - [`IMG_isBMP`]
    /// - [`IMG_isCUR`]
    /// - [`IMG_isICO`]
    /// - [`IMG_isJPG`]
    /// - [`IMG_isJXL`]
    /// - [`IMG_isLBM`]
    /// - [`IMG_isPCX`]
    /// - [`IMG_isPNG`]
    /// - [`IMG_isPNM`]
    /// - [`IMG_isQOI`]
    /// - [`IMG_isSVG`]
    /// - [`IMG_isTIF`]
    /// - [`IMG_isWEBP`]
    /// - [`IMG_isXCF`]
    /// - [`IMG_isXPM`]
    /// - [`IMG_isXV`]
    pub fn IMG_isGIF(src: *mut SDL_IOStream) -> ::core::primitive::bool;
}

unsafe extern "C" {
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
    /// ## Parameters
    /// - `src`: a seekable/readable [`SDL_IOStream`] to provide image data.
    ///
    /// ## Return value
    /// Returns true if this is ICO data, false otherwise.
    ///
    /// ## Availability
    /// This function is available since SDL_image 3.0.0.
    ///
    /// ## See also
    /// - [`IMG_isANI`]
    /// - [`IMG_isAVIF`]
    /// - [`IMG_isBMP`]
    /// - [`IMG_isCUR`]
    /// - [`IMG_isGIF`]
    /// - [`IMG_isJPG`]
    /// - [`IMG_isJXL`]
    /// - [`IMG_isLBM`]
    /// - [`IMG_isPCX`]
    /// - [`IMG_isPNG`]
    /// - [`IMG_isPNM`]
    /// - [`IMG_isQOI`]
    /// - [`IMG_isSVG`]
    /// - [`IMG_isTIF`]
    /// - [`IMG_isWEBP`]
    /// - [`IMG_isXCF`]
    /// - [`IMG_isXPM`]
    /// - [`IMG_isXV`]
    pub fn IMG_isICO(src: *mut SDL_IOStream) -> ::core::primitive::bool;
}

unsafe extern "C" {
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
    /// ## Parameters
    /// - `src`: a seekable/readable [`SDL_IOStream`] to provide image data.
    ///
    /// ## Return value
    /// Returns true if this is JPG data, false otherwise.
    ///
    /// ## Availability
    /// This function is available since SDL_image 3.0.0.
    ///
    /// ## See also
    /// - [`IMG_isANI`]
    /// - [`IMG_isAVIF`]
    /// - [`IMG_isBMP`]
    /// - [`IMG_isCUR`]
    /// - [`IMG_isGIF`]
    /// - [`IMG_isICO`]
    /// - [`IMG_isJXL`]
    /// - [`IMG_isLBM`]
    /// - [`IMG_isPCX`]
    /// - [`IMG_isPNG`]
    /// - [`IMG_isPNM`]
    /// - [`IMG_isQOI`]
    /// - [`IMG_isSVG`]
    /// - [`IMG_isTIF`]
    /// - [`IMG_isWEBP`]
    /// - [`IMG_isXCF`]
    /// - [`IMG_isXPM`]
    /// - [`IMG_isXV`]
    pub fn IMG_isJPG(src: *mut SDL_IOStream) -> ::core::primitive::bool;
}

unsafe extern "C" {
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
    /// ## Parameters
    /// - `src`: a seekable/readable [`SDL_IOStream`] to provide image data.
    ///
    /// ## Return value
    /// Returns true if this is JXL data, false otherwise.
    ///
    /// ## Availability
    /// This function is available since SDL_image 3.0.0.
    ///
    /// ## See also
    /// - [`IMG_isANI`]
    /// - [`IMG_isAVIF`]
    /// - [`IMG_isBMP`]
    /// - [`IMG_isCUR`]
    /// - [`IMG_isGIF`]
    /// - [`IMG_isICO`]
    /// - [`IMG_isJPG`]
    /// - [`IMG_isLBM`]
    /// - [`IMG_isPCX`]
    /// - [`IMG_isPNG`]
    /// - [`IMG_isPNM`]
    /// - [`IMG_isQOI`]
    /// - [`IMG_isSVG`]
    /// - [`IMG_isTIF`]
    /// - [`IMG_isWEBP`]
    /// - [`IMG_isXCF`]
    /// - [`IMG_isXPM`]
    /// - [`IMG_isXV`]
    pub fn IMG_isJXL(src: *mut SDL_IOStream) -> ::core::primitive::bool;
}

unsafe extern "C" {
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
    /// ## Parameters
    /// - `src`: a seekable/readable [`SDL_IOStream`] to provide image data.
    ///
    /// ## Return value
    /// Returns true if this is LBM data, false otherwise.
    ///
    /// ## Availability
    /// This function is available since SDL_image 3.0.0.
    ///
    /// ## See also
    /// - [`IMG_isANI`]
    /// - [`IMG_isAVIF`]
    /// - [`IMG_isBMP`]
    /// - [`IMG_isCUR`]
    /// - [`IMG_isGIF`]
    /// - [`IMG_isICO`]
    /// - [`IMG_isJPG`]
    /// - [`IMG_isJXL`]
    /// - [`IMG_isPCX`]
    /// - [`IMG_isPNG`]
    /// - [`IMG_isPNM`]
    /// - [`IMG_isQOI`]
    /// - [`IMG_isSVG`]
    /// - [`IMG_isTIF`]
    /// - [`IMG_isWEBP`]
    /// - [`IMG_isXCF`]
    /// - [`IMG_isXPM`]
    /// - [`IMG_isXV`]
    pub fn IMG_isLBM(src: *mut SDL_IOStream) -> ::core::primitive::bool;
}

unsafe extern "C" {
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
    /// ## Parameters
    /// - `src`: a seekable/readable [`SDL_IOStream`] to provide image data.
    ///
    /// ## Return value
    /// Returns true if this is PCX data, false otherwise.
    ///
    /// ## Availability
    /// This function is available since SDL_image 3.0.0.
    ///
    /// ## See also
    /// - [`IMG_isANI`]
    /// - [`IMG_isAVIF`]
    /// - [`IMG_isBMP`]
    /// - [`IMG_isCUR`]
    /// - [`IMG_isGIF`]
    /// - [`IMG_isICO`]
    /// - [`IMG_isJPG`]
    /// - [`IMG_isJXL`]
    /// - [`IMG_isLBM`]
    /// - [`IMG_isPNG`]
    /// - [`IMG_isPNM`]
    /// - [`IMG_isQOI`]
    /// - [`IMG_isSVG`]
    /// - [`IMG_isTIF`]
    /// - [`IMG_isWEBP`]
    /// - [`IMG_isXCF`]
    /// - [`IMG_isXPM`]
    /// - [`IMG_isXV`]
    pub fn IMG_isPCX(src: *mut SDL_IOStream) -> ::core::primitive::bool;
}

unsafe extern "C" {
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
    /// ## Parameters
    /// - `src`: a seekable/readable [`SDL_IOStream`] to provide image data.
    ///
    /// ## Return value
    /// Returns true if this is PNG data, false otherwise.
    ///
    /// ## Availability
    /// This function is available since SDL_image 3.0.0.
    ///
    /// ## See also
    /// - [`IMG_isANI`]
    /// - [`IMG_isAVIF`]
    /// - [`IMG_isBMP`]
    /// - [`IMG_isCUR`]
    /// - [`IMG_isGIF`]
    /// - [`IMG_isICO`]
    /// - [`IMG_isJPG`]
    /// - [`IMG_isJXL`]
    /// - [`IMG_isLBM`]
    /// - [`IMG_isPCX`]
    /// - [`IMG_isPNM`]
    /// - [`IMG_isQOI`]
    /// - [`IMG_isSVG`]
    /// - [`IMG_isTIF`]
    /// - [`IMG_isWEBP`]
    /// - [`IMG_isXCF`]
    /// - [`IMG_isXPM`]
    /// - [`IMG_isXV`]
    pub fn IMG_isPNG(src: *mut SDL_IOStream) -> ::core::primitive::bool;
}

unsafe extern "C" {
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
    /// ## Parameters
    /// - `src`: a seekable/readable [`SDL_IOStream`] to provide image data.
    ///
    /// ## Return value
    /// Returns true if this is PNM data, false otherwise.
    ///
    /// ## Availability
    /// This function is available since SDL_image 3.0.0.
    ///
    /// ## See also
    /// - [`IMG_isANI`]
    /// - [`IMG_isAVIF`]
    /// - [`IMG_isBMP`]
    /// - [`IMG_isCUR`]
    /// - [`IMG_isGIF`]
    /// - [`IMG_isICO`]
    /// - [`IMG_isJPG`]
    /// - [`IMG_isJXL`]
    /// - [`IMG_isLBM`]
    /// - [`IMG_isPCX`]
    /// - [`IMG_isPNG`]
    /// - [`IMG_isQOI`]
    /// - [`IMG_isSVG`]
    /// - [`IMG_isTIF`]
    /// - [`IMG_isWEBP`]
    /// - [`IMG_isXCF`]
    /// - [`IMG_isXPM`]
    /// - [`IMG_isXV`]
    pub fn IMG_isPNM(src: *mut SDL_IOStream) -> ::core::primitive::bool;
}

unsafe extern "C" {
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
    /// ## Parameters
    /// - `src`: a seekable/readable [`SDL_IOStream`] to provide image data.
    ///
    /// ## Return value
    /// Returns true if this is QOI data, false otherwise.
    ///
    /// ## Availability
    /// This function is available since SDL_image 3.0.0.
    ///
    /// ## See also
    /// - [`IMG_isANI`]
    /// - [`IMG_isAVIF`]
    /// - [`IMG_isBMP`]
    /// - [`IMG_isCUR`]
    /// - [`IMG_isGIF`]
    /// - [`IMG_isICO`]
    /// - [`IMG_isJPG`]
    /// - [`IMG_isJXL`]
    /// - [`IMG_isLBM`]
    /// - [`IMG_isPCX`]
    /// - [`IMG_isPNG`]
    /// - [`IMG_isPNM`]
    /// - [`IMG_isSVG`]
    /// - [`IMG_isTIF`]
    /// - [`IMG_isWEBP`]
    /// - [`IMG_isXCF`]
    /// - [`IMG_isXPM`]
    /// - [`IMG_isXV`]
    pub fn IMG_isQOI(src: *mut SDL_IOStream) -> ::core::primitive::bool;
}

unsafe extern "C" {
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
    /// ## Parameters
    /// - `src`: a seekable/readable [`SDL_IOStream`] to provide image data.
    ///
    /// ## Return value
    /// Returns true if this is SVG data, false otherwise.
    ///
    /// ## Availability
    /// This function is available since SDL_image 3.0.0.
    ///
    /// ## See also
    /// - [`IMG_isANI`]
    /// - [`IMG_isAVIF`]
    /// - [`IMG_isBMP`]
    /// - [`IMG_isCUR`]
    /// - [`IMG_isGIF`]
    /// - [`IMG_isICO`]
    /// - [`IMG_isJPG`]
    /// - [`IMG_isJXL`]
    /// - [`IMG_isLBM`]
    /// - [`IMG_isPCX`]
    /// - [`IMG_isPNG`]
    /// - [`IMG_isPNM`]
    /// - [`IMG_isQOI`]
    /// - [`IMG_isTIF`]
    /// - [`IMG_isWEBP`]
    /// - [`IMG_isXCF`]
    /// - [`IMG_isXPM`]
    /// - [`IMG_isXV`]
    pub fn IMG_isSVG(src: *mut SDL_IOStream) -> ::core::primitive::bool;
}

unsafe extern "C" {
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
    /// ## Parameters
    /// - `src`: a seekable/readable [`SDL_IOStream`] to provide image data.
    ///
    /// ## Return value
    /// Returns true if this is TIFF data, false otherwise.
    ///
    /// ## Availability
    /// This function is available since SDL_image 3.0.0.
    ///
    /// ## See also
    /// - [`IMG_isANI`]
    /// - [`IMG_isAVIF`]
    /// - [`IMG_isBMP`]
    /// - [`IMG_isCUR`]
    /// - [`IMG_isGIF`]
    /// - [`IMG_isICO`]
    /// - [`IMG_isJPG`]
    /// - [`IMG_isJXL`]
    /// - [`IMG_isLBM`]
    /// - [`IMG_isPCX`]
    /// - [`IMG_isPNG`]
    /// - [`IMG_isPNM`]
    /// - [`IMG_isQOI`]
    /// - [`IMG_isSVG`]
    /// - [`IMG_isWEBP`]
    /// - [`IMG_isXCF`]
    /// - [`IMG_isXPM`]
    /// - [`IMG_isXV`]
    pub fn IMG_isTIF(src: *mut SDL_IOStream) -> ::core::primitive::bool;
}

unsafe extern "C" {
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
    /// ## Parameters
    /// - `src`: a seekable/readable [`SDL_IOStream`] to provide image data.
    ///
    /// ## Return value
    /// Returns true if this is WEBP data, false otherwise.
    ///
    /// ## Availability
    /// This function is available since SDL_image 3.0.0.
    ///
    /// ## See also
    /// - [`IMG_isANI`]
    /// - [`IMG_isAVIF`]
    /// - [`IMG_isBMP`]
    /// - [`IMG_isCUR`]
    /// - [`IMG_isGIF`]
    /// - [`IMG_isICO`]
    /// - [`IMG_isJPG`]
    /// - [`IMG_isJXL`]
    /// - [`IMG_isLBM`]
    /// - [`IMG_isPCX`]
    /// - [`IMG_isPNG`]
    /// - [`IMG_isPNM`]
    /// - [`IMG_isQOI`]
    /// - [`IMG_isSVG`]
    /// - [`IMG_isTIF`]
    /// - [`IMG_isXCF`]
    /// - [`IMG_isXPM`]
    /// - [`IMG_isXV`]
    pub fn IMG_isWEBP(src: *mut SDL_IOStream) -> ::core::primitive::bool;
}

unsafe extern "C" {
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
    /// ## Parameters
    /// - `src`: a seekable/readable [`SDL_IOStream`] to provide image data.
    ///
    /// ## Return value
    /// Returns true if this is XCF data, false otherwise.
    ///
    /// ## Availability
    /// This function is available since SDL_image 3.0.0.
    ///
    /// ## See also
    /// - [`IMG_isANI`]
    /// - [`IMG_isAVIF`]
    /// - [`IMG_isBMP`]
    /// - [`IMG_isCUR`]
    /// - [`IMG_isGIF`]
    /// - [`IMG_isICO`]
    /// - [`IMG_isJPG`]
    /// - [`IMG_isJXL`]
    /// - [`IMG_isLBM`]
    /// - [`IMG_isPCX`]
    /// - [`IMG_isPNG`]
    /// - [`IMG_isPNM`]
    /// - [`IMG_isQOI`]
    /// - [`IMG_isSVG`]
    /// - [`IMG_isTIF`]
    /// - [`IMG_isWEBP`]
    /// - [`IMG_isXPM`]
    /// - [`IMG_isXV`]
    pub fn IMG_isXCF(src: *mut SDL_IOStream) -> ::core::primitive::bool;
}

unsafe extern "C" {
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
    /// ## Parameters
    /// - `src`: a seekable/readable [`SDL_IOStream`] to provide image data.
    ///
    /// ## Return value
    /// Returns true if this is XPM data, false otherwise.
    ///
    /// ## Availability
    /// This function is available since SDL_image 3.0.0.
    ///
    /// ## See also
    /// - [`IMG_isANI`]
    /// - [`IMG_isAVIF`]
    /// - [`IMG_isBMP`]
    /// - [`IMG_isCUR`]
    /// - [`IMG_isGIF`]
    /// - [`IMG_isICO`]
    /// - [`IMG_isJPG`]
    /// - [`IMG_isJXL`]
    /// - [`IMG_isLBM`]
    /// - [`IMG_isPCX`]
    /// - [`IMG_isPNG`]
    /// - [`IMG_isPNM`]
    /// - [`IMG_isQOI`]
    /// - [`IMG_isSVG`]
    /// - [`IMG_isTIF`]
    /// - [`IMG_isWEBP`]
    /// - [`IMG_isXCF`]
    /// - [`IMG_isXV`]
    pub fn IMG_isXPM(src: *mut SDL_IOStream) -> ::core::primitive::bool;
}

unsafe extern "C" {
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
    /// ## Parameters
    /// - `src`: a seekable/readable [`SDL_IOStream`] to provide image data.
    ///
    /// ## Return value
    /// Returns true if this is XV data, false otherwise.
    ///
    /// ## Availability
    /// This function is available since SDL_image 3.0.0.
    ///
    /// ## See also
    /// - [`IMG_isANI`]
    /// - [`IMG_isAVIF`]
    /// - [`IMG_isBMP`]
    /// - [`IMG_isCUR`]
    /// - [`IMG_isGIF`]
    /// - [`IMG_isICO`]
    /// - [`IMG_isJPG`]
    /// - [`IMG_isJXL`]
    /// - [`IMG_isLBM`]
    /// - [`IMG_isPCX`]
    /// - [`IMG_isPNG`]
    /// - [`IMG_isPNM`]
    /// - [`IMG_isQOI`]
    /// - [`IMG_isSVG`]
    /// - [`IMG_isTIF`]
    /// - [`IMG_isWEBP`]
    /// - [`IMG_isXCF`]
    /// - [`IMG_isXPM`]
    pub fn IMG_isXV(src: *mut SDL_IOStream) -> ::core::primitive::bool;
}

unsafe extern "C" {
    /// Load a AVIF image directly.
    ///
    /// If you know you definitely have a AVIF image, you can call this function,
    /// which will skip SDL_image's file format detection routines. Generally it's
    /// better to use the abstract interfaces; also, there is only an [`SDL_IOStream`]
    /// interface available here.
    ///
    /// ## Parameters
    /// - `src`: an [`SDL_IOStream`] to load image data from.
    ///
    /// ## Return value
    /// Returns SDL surface, or NULL on error.
    ///
    /// ## Availability
    /// This function is available since SDL_image 3.0.0.
    ///
    /// ## See also
    /// - [`IMG_LoadBMP_IO`]
    /// - [`IMG_LoadCUR_IO`]
    /// - [`IMG_LoadGIF_IO`]
    /// - [`IMG_LoadICO_IO`]
    /// - [`IMG_LoadJPG_IO`]
    /// - [`IMG_LoadJXL_IO`]
    /// - [`IMG_LoadLBM_IO`]
    /// - [`IMG_LoadPCX_IO`]
    /// - [`IMG_LoadPNG_IO`]
    /// - [`IMG_LoadPNM_IO`]
    /// - [`IMG_LoadQOI_IO`]
    /// - [`IMG_LoadSVG_IO`]
    /// - [`IMG_LoadTGA_IO`]
    /// - [`IMG_LoadTIF_IO`]
    /// - [`IMG_LoadWEBP_IO`]
    /// - [`IMG_LoadXCF_IO`]
    /// - [`IMG_LoadXPM_IO`]
    /// - [`IMG_LoadXV_IO`]
    pub fn IMG_LoadAVIF_IO(src: *mut SDL_IOStream) -> *mut SDL_Surface;
}

unsafe extern "C" {
    /// Load a BMP image directly.
    ///
    /// If you know you definitely have a BMP image, you can call this function,
    /// which will skip SDL_image's file format detection routines. Generally it's
    /// better to use the abstract interfaces; also, there is only an [`SDL_IOStream`]
    /// interface available here.
    ///
    /// ## Parameters
    /// - `src`: an [`SDL_IOStream`] to load image data from.
    ///
    /// ## Return value
    /// Returns SDL surface, or NULL on error.
    ///
    /// ## Availability
    /// This function is available since SDL_image 3.0.0.
    ///
    /// ## See also
    /// - [`IMG_LoadAVIF_IO`]
    /// - [`IMG_LoadCUR_IO`]
    /// - [`IMG_LoadGIF_IO`]
    /// - [`IMG_LoadICO_IO`]
    /// - [`IMG_LoadJPG_IO`]
    /// - [`IMG_LoadJXL_IO`]
    /// - [`IMG_LoadLBM_IO`]
    /// - [`IMG_LoadPCX_IO`]
    /// - [`IMG_LoadPNG_IO`]
    /// - [`IMG_LoadPNM_IO`]
    /// - [`IMG_LoadQOI_IO`]
    /// - [`IMG_LoadSVG_IO`]
    /// - [`IMG_LoadTGA_IO`]
    /// - [`IMG_LoadTIF_IO`]
    /// - [`IMG_LoadWEBP_IO`]
    /// - [`IMG_LoadXCF_IO`]
    /// - [`IMG_LoadXPM_IO`]
    /// - [`IMG_LoadXV_IO`]
    pub fn IMG_LoadBMP_IO(src: *mut SDL_IOStream) -> *mut SDL_Surface;
}

unsafe extern "C" {
    /// Load a CUR image directly.
    ///
    /// If you know you definitely have a CUR image, you can call this function,
    /// which will skip SDL_image's file format detection routines. Generally it's
    /// better to use the abstract interfaces; also, there is only an [`SDL_IOStream`]
    /// interface available here.
    ///
    /// ## Parameters
    /// - `src`: an [`SDL_IOStream`] to load image data from.
    ///
    /// ## Return value
    /// Returns SDL surface, or NULL on error.
    ///
    /// ## Availability
    /// This function is available since SDL_image 3.0.0.
    ///
    /// ## See also
    /// - [`IMG_LoadAVIF_IO`]
    /// - [`IMG_LoadBMP_IO`]
    /// - [`IMG_LoadGIF_IO`]
    /// - [`IMG_LoadICO_IO`]
    /// - [`IMG_LoadJPG_IO`]
    /// - [`IMG_LoadJXL_IO`]
    /// - [`IMG_LoadLBM_IO`]
    /// - [`IMG_LoadPCX_IO`]
    /// - [`IMG_LoadPNG_IO`]
    /// - [`IMG_LoadPNM_IO`]
    /// - [`IMG_LoadQOI_IO`]
    /// - [`IMG_LoadSVG_IO`]
    /// - [`IMG_LoadTGA_IO`]
    /// - [`IMG_LoadTIF_IO`]
    /// - [`IMG_LoadWEBP_IO`]
    /// - [`IMG_LoadXCF_IO`]
    /// - [`IMG_LoadXPM_IO`]
    /// - [`IMG_LoadXV_IO`]
    pub fn IMG_LoadCUR_IO(src: *mut SDL_IOStream) -> *mut SDL_Surface;
}

unsafe extern "C" {
    /// Load a GIF image directly.
    ///
    /// If you know you definitely have a GIF image, you can call this function,
    /// which will skip SDL_image's file format detection routines. Generally it's
    /// better to use the abstract interfaces; also, there is only an [`SDL_IOStream`]
    /// interface available here.
    ///
    /// ## Parameters
    /// - `src`: an [`SDL_IOStream`] to load image data from.
    ///
    /// ## Return value
    /// Returns SDL surface, or NULL on error.
    ///
    /// ## Availability
    /// This function is available since SDL_image 3.0.0.
    ///
    /// ## See also
    /// - [`IMG_LoadAVIF_IO`]
    /// - [`IMG_LoadBMP_IO`]
    /// - [`IMG_LoadCUR_IO`]
    /// - [`IMG_LoadICO_IO`]
    /// - [`IMG_LoadJPG_IO`]
    /// - [`IMG_LoadJXL_IO`]
    /// - [`IMG_LoadLBM_IO`]
    /// - [`IMG_LoadPCX_IO`]
    /// - [`IMG_LoadPNG_IO`]
    /// - [`IMG_LoadPNM_IO`]
    /// - [`IMG_LoadQOI_IO`]
    /// - [`IMG_LoadSVG_IO`]
    /// - [`IMG_LoadTGA_IO`]
    /// - [`IMG_LoadTIF_IO`]
    /// - [`IMG_LoadWEBP_IO`]
    /// - [`IMG_LoadXCF_IO`]
    /// - [`IMG_LoadXPM_IO`]
    /// - [`IMG_LoadXV_IO`]
    pub fn IMG_LoadGIF_IO(src: *mut SDL_IOStream) -> *mut SDL_Surface;
}

unsafe extern "C" {
    /// Load a ICO image directly.
    ///
    /// If you know you definitely have a ICO image, you can call this function,
    /// which will skip SDL_image's file format detection routines. Generally it's
    /// better to use the abstract interfaces; also, there is only an [`SDL_IOStream`]
    /// interface available here.
    ///
    /// ## Parameters
    /// - `src`: an [`SDL_IOStream`] to load image data from.
    ///
    /// ## Return value
    /// Returns SDL surface, or NULL on error.
    ///
    /// ## Availability
    /// This function is available since SDL_image 3.0.0.
    ///
    /// ## See also
    /// - [`IMG_LoadAVIF_IO`]
    /// - [`IMG_LoadBMP_IO`]
    /// - [`IMG_LoadCUR_IO`]
    /// - [`IMG_LoadGIF_IO`]
    /// - [`IMG_LoadJPG_IO`]
    /// - [`IMG_LoadJXL_IO`]
    /// - [`IMG_LoadLBM_IO`]
    /// - [`IMG_LoadPCX_IO`]
    /// - [`IMG_LoadPNG_IO`]
    /// - [`IMG_LoadPNM_IO`]
    /// - [`IMG_LoadQOI_IO`]
    /// - [`IMG_LoadSVG_IO`]
    /// - [`IMG_LoadTGA_IO`]
    /// - [`IMG_LoadTIF_IO`]
    /// - [`IMG_LoadWEBP_IO`]
    /// - [`IMG_LoadXCF_IO`]
    /// - [`IMG_LoadXPM_IO`]
    /// - [`IMG_LoadXV_IO`]
    pub fn IMG_LoadICO_IO(src: *mut SDL_IOStream) -> *mut SDL_Surface;
}

unsafe extern "C" {
    /// Load a JPG image directly.
    ///
    /// If you know you definitely have a JPG image, you can call this function,
    /// which will skip SDL_image's file format detection routines. Generally it's
    /// better to use the abstract interfaces; also, there is only an [`SDL_IOStream`]
    /// interface available here.
    ///
    /// ## Parameters
    /// - `src`: an [`SDL_IOStream`] to load image data from.
    ///
    /// ## Return value
    /// Returns SDL surface, or NULL on error.
    ///
    /// ## Availability
    /// This function is available since SDL_image 3.0.0.
    ///
    /// ## See also
    /// - [`IMG_LoadAVIF_IO`]
    /// - [`IMG_LoadBMP_IO`]
    /// - [`IMG_LoadCUR_IO`]
    /// - [`IMG_LoadGIF_IO`]
    /// - [`IMG_LoadICO_IO`]
    /// - [`IMG_LoadJXL_IO`]
    /// - [`IMG_LoadLBM_IO`]
    /// - [`IMG_LoadPCX_IO`]
    /// - [`IMG_LoadPNG_IO`]
    /// - [`IMG_LoadPNM_IO`]
    /// - [`IMG_LoadQOI_IO`]
    /// - [`IMG_LoadSVG_IO`]
    /// - [`IMG_LoadTGA_IO`]
    /// - [`IMG_LoadTIF_IO`]
    /// - [`IMG_LoadWEBP_IO`]
    /// - [`IMG_LoadXCF_IO`]
    /// - [`IMG_LoadXPM_IO`]
    /// - [`IMG_LoadXV_IO`]
    pub fn IMG_LoadJPG_IO(src: *mut SDL_IOStream) -> *mut SDL_Surface;
}

unsafe extern "C" {
    /// Load a JXL image directly.
    ///
    /// If you know you definitely have a JXL image, you can call this function,
    /// which will skip SDL_image's file format detection routines. Generally it's
    /// better to use the abstract interfaces; also, there is only an [`SDL_IOStream`]
    /// interface available here.
    ///
    /// ## Parameters
    /// - `src`: an [`SDL_IOStream`] to load image data from.
    ///
    /// ## Return value
    /// Returns SDL surface, or NULL on error.
    ///
    /// ## Availability
    /// This function is available since SDL_image 3.0.0.
    ///
    /// ## See also
    /// - [`IMG_LoadAVIF_IO`]
    /// - [`IMG_LoadBMP_IO`]
    /// - [`IMG_LoadCUR_IO`]
    /// - [`IMG_LoadGIF_IO`]
    /// - [`IMG_LoadICO_IO`]
    /// - [`IMG_LoadJPG_IO`]
    /// - [`IMG_LoadLBM_IO`]
    /// - [`IMG_LoadPCX_IO`]
    /// - [`IMG_LoadPNG_IO`]
    /// - [`IMG_LoadPNM_IO`]
    /// - [`IMG_LoadQOI_IO`]
    /// - [`IMG_LoadSVG_IO`]
    /// - [`IMG_LoadTGA_IO`]
    /// - [`IMG_LoadTIF_IO`]
    /// - [`IMG_LoadWEBP_IO`]
    /// - [`IMG_LoadXCF_IO`]
    /// - [`IMG_LoadXPM_IO`]
    /// - [`IMG_LoadXV_IO`]
    pub fn IMG_LoadJXL_IO(src: *mut SDL_IOStream) -> *mut SDL_Surface;
}

unsafe extern "C" {
    /// Load a LBM image directly.
    ///
    /// If you know you definitely have a LBM image, you can call this function,
    /// which will skip SDL_image's file format detection routines. Generally it's
    /// better to use the abstract interfaces; also, there is only an [`SDL_IOStream`]
    /// interface available here.
    ///
    /// ## Parameters
    /// - `src`: an [`SDL_IOStream`] to load image data from.
    ///
    /// ## Return value
    /// Returns SDL surface, or NULL on error.
    ///
    /// ## Availability
    /// This function is available since SDL_image 3.0.0.
    ///
    /// ## See also
    /// - [`IMG_LoadAVIF_IO`]
    /// - [`IMG_LoadBMP_IO`]
    /// - [`IMG_LoadCUR_IO`]
    /// - [`IMG_LoadGIF_IO`]
    /// - [`IMG_LoadICO_IO`]
    /// - [`IMG_LoadJPG_IO`]
    /// - [`IMG_LoadJXL_IO`]
    /// - [`IMG_LoadPCX_IO`]
    /// - [`IMG_LoadPNG_IO`]
    /// - [`IMG_LoadPNM_IO`]
    /// - [`IMG_LoadQOI_IO`]
    /// - [`IMG_LoadSVG_IO`]
    /// - [`IMG_LoadTGA_IO`]
    /// - [`IMG_LoadTIF_IO`]
    /// - [`IMG_LoadWEBP_IO`]
    /// - [`IMG_LoadXCF_IO`]
    /// - [`IMG_LoadXPM_IO`]
    /// - [`IMG_LoadXV_IO`]
    pub fn IMG_LoadLBM_IO(src: *mut SDL_IOStream) -> *mut SDL_Surface;
}

unsafe extern "C" {
    /// Load a PCX image directly.
    ///
    /// If you know you definitely have a PCX image, you can call this function,
    /// which will skip SDL_image's file format detection routines. Generally it's
    /// better to use the abstract interfaces; also, there is only an [`SDL_IOStream`]
    /// interface available here.
    ///
    /// ## Parameters
    /// - `src`: an [`SDL_IOStream`] to load image data from.
    ///
    /// ## Return value
    /// Returns SDL surface, or NULL on error.
    ///
    /// ## Availability
    /// This function is available since SDL_image 3.0.0.
    ///
    /// ## See also
    /// - [`IMG_LoadAVIF_IO`]
    /// - [`IMG_LoadBMP_IO`]
    /// - [`IMG_LoadCUR_IO`]
    /// - [`IMG_LoadGIF_IO`]
    /// - [`IMG_LoadICO_IO`]
    /// - [`IMG_LoadJPG_IO`]
    /// - [`IMG_LoadJXL_IO`]
    /// - [`IMG_LoadLBM_IO`]
    /// - [`IMG_LoadPNG_IO`]
    /// - [`IMG_LoadPNM_IO`]
    /// - [`IMG_LoadQOI_IO`]
    /// - [`IMG_LoadSVG_IO`]
    /// - [`IMG_LoadTGA_IO`]
    /// - [`IMG_LoadTIF_IO`]
    /// - [`IMG_LoadWEBP_IO`]
    /// - [`IMG_LoadXCF_IO`]
    /// - [`IMG_LoadXPM_IO`]
    /// - [`IMG_LoadXV_IO`]
    pub fn IMG_LoadPCX_IO(src: *mut SDL_IOStream) -> *mut SDL_Surface;
}

unsafe extern "C" {
    /// Load a PNG image directly.
    ///
    /// If you know you definitely have a PNG image, you can call this function,
    /// which will skip SDL_image's file format detection routines. Generally it's
    /// better to use the abstract interfaces; also, there is only an [`SDL_IOStream`]
    /// interface available here.
    ///
    /// ## Parameters
    /// - `src`: an [`SDL_IOStream`] to load image data from.
    ///
    /// ## Return value
    /// Returns SDL surface, or NULL on error.
    ///
    /// ## Availability
    /// This function is available since SDL_image 3.0.0.
    ///
    /// ## See also
    /// - [`IMG_LoadAVIF_IO`]
    /// - [`IMG_LoadBMP_IO`]
    /// - [`IMG_LoadCUR_IO`]
    /// - [`IMG_LoadGIF_IO`]
    /// - [`IMG_LoadICO_IO`]
    /// - [`IMG_LoadJPG_IO`]
    /// - [`IMG_LoadJXL_IO`]
    /// - [`IMG_LoadLBM_IO`]
    /// - [`IMG_LoadPCX_IO`]
    /// - [`IMG_LoadPNM_IO`]
    /// - [`IMG_LoadQOI_IO`]
    /// - [`IMG_LoadSVG_IO`]
    /// - [`IMG_LoadTGA_IO`]
    /// - [`IMG_LoadTIF_IO`]
    /// - [`IMG_LoadWEBP_IO`]
    /// - [`IMG_LoadXCF_IO`]
    /// - [`IMG_LoadXPM_IO`]
    /// - [`IMG_LoadXV_IO`]
    pub fn IMG_LoadPNG_IO(src: *mut SDL_IOStream) -> *mut SDL_Surface;
}

unsafe extern "C" {
    /// Load a PNM image directly.
    ///
    /// If you know you definitely have a PNM image, you can call this function,
    /// which will skip SDL_image's file format detection routines. Generally it's
    /// better to use the abstract interfaces; also, there is only an [`SDL_IOStream`]
    /// interface available here.
    ///
    /// ## Parameters
    /// - `src`: an [`SDL_IOStream`] to load image data from.
    ///
    /// ## Return value
    /// Returns SDL surface, or NULL on error.
    ///
    /// ## Availability
    /// This function is available since SDL_image 3.0.0.
    ///
    /// ## See also
    /// - [`IMG_LoadAVIF_IO`]
    /// - [`IMG_LoadBMP_IO`]
    /// - [`IMG_LoadCUR_IO`]
    /// - [`IMG_LoadGIF_IO`]
    /// - [`IMG_LoadICO_IO`]
    /// - [`IMG_LoadJPG_IO`]
    /// - [`IMG_LoadJXL_IO`]
    /// - [`IMG_LoadLBM_IO`]
    /// - [`IMG_LoadPCX_IO`]
    /// - [`IMG_LoadPNG_IO`]
    /// - [`IMG_LoadQOI_IO`]
    /// - [`IMG_LoadSVG_IO`]
    /// - [`IMG_LoadTGA_IO`]
    /// - [`IMG_LoadTIF_IO`]
    /// - [`IMG_LoadWEBP_IO`]
    /// - [`IMG_LoadXCF_IO`]
    /// - [`IMG_LoadXPM_IO`]
    /// - [`IMG_LoadXV_IO`]
    pub fn IMG_LoadPNM_IO(src: *mut SDL_IOStream) -> *mut SDL_Surface;
}

unsafe extern "C" {
    /// Load a SVG image directly.
    ///
    /// If you know you definitely have a SVG image, you can call this function,
    /// which will skip SDL_image's file format detection routines. Generally it's
    /// better to use the abstract interfaces; also, there is only an [`SDL_IOStream`]
    /// interface available here.
    ///
    /// ## Parameters
    /// - `src`: an [`SDL_IOStream`] to load image data from.
    ///
    /// ## Return value
    /// Returns SDL surface, or NULL on error.
    ///
    /// ## Availability
    /// This function is available since SDL_image 3.0.0.
    ///
    /// ## See also
    /// - [`IMG_LoadAVIF_IO`]
    /// - [`IMG_LoadBMP_IO`]
    /// - [`IMG_LoadCUR_IO`]
    /// - [`IMG_LoadGIF_IO`]
    /// - [`IMG_LoadICO_IO`]
    /// - [`IMG_LoadJPG_IO`]
    /// - [`IMG_LoadJXL_IO`]
    /// - [`IMG_LoadLBM_IO`]
    /// - [`IMG_LoadPCX_IO`]
    /// - [`IMG_LoadPNG_IO`]
    /// - [`IMG_LoadPNM_IO`]
    /// - [`IMG_LoadQOI_IO`]
    /// - [`IMG_LoadSizedSVG_IO`]
    /// - [`IMG_LoadTGA_IO`]
    /// - [`IMG_LoadTIF_IO`]
    /// - [`IMG_LoadWEBP_IO`]
    /// - [`IMG_LoadXCF_IO`]
    /// - [`IMG_LoadXPM_IO`]
    /// - [`IMG_LoadXV_IO`]
    pub fn IMG_LoadSVG_IO(src: *mut SDL_IOStream) -> *mut SDL_Surface;
}

unsafe extern "C" {
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
    /// ## Parameters
    /// - `src`: an [`SDL_IOStream`] to load SVG data from.
    /// - `width`: desired width of the generated surface, in pixels.
    /// - `height`: desired height of the generated surface, in pixels.
    ///
    /// ## Return value
    /// Returns a new SDL surface, or NULL on error.
    ///
    /// ## Availability
    /// This function is available since SDL_image 3.0.0.
    ///
    /// ## See also
    /// - [`IMG_LoadSVG_IO`]
    pub fn IMG_LoadSizedSVG_IO(
        src: *mut SDL_IOStream,
        width: ::core::ffi::c_int,
        height: ::core::ffi::c_int,
    ) -> *mut SDL_Surface;
}

unsafe extern "C" {
    /// Load a QOI image directly.
    ///
    /// If you know you definitely have a QOI image, you can call this function,
    /// which will skip SDL_image's file format detection routines. Generally it's
    /// better to use the abstract interfaces; also, there is only an [`SDL_IOStream`]
    /// interface available here.
    ///
    /// ## Parameters
    /// - `src`: an [`SDL_IOStream`] to load image data from.
    ///
    /// ## Return value
    /// Returns SDL surface, or NULL on error.
    ///
    /// ## Availability
    /// This function is available since SDL_image 3.0.0.
    ///
    /// ## See also
    /// - [`IMG_LoadAVIF_IO`]
    /// - [`IMG_LoadBMP_IO`]
    /// - [`IMG_LoadCUR_IO`]
    /// - [`IMG_LoadGIF_IO`]
    /// - [`IMG_LoadICO_IO`]
    /// - [`IMG_LoadJPG_IO`]
    /// - [`IMG_LoadJXL_IO`]
    /// - [`IMG_LoadLBM_IO`]
    /// - [`IMG_LoadPCX_IO`]
    /// - [`IMG_LoadPNG_IO`]
    /// - [`IMG_LoadPNM_IO`]
    /// - [`IMG_LoadSVG_IO`]
    /// - [`IMG_LoadTGA_IO`]
    /// - [`IMG_LoadTIF_IO`]
    /// - [`IMG_LoadWEBP_IO`]
    /// - [`IMG_LoadXCF_IO`]
    /// - [`IMG_LoadXPM_IO`]
    /// - [`IMG_LoadXV_IO`]
    pub fn IMG_LoadQOI_IO(src: *mut SDL_IOStream) -> *mut SDL_Surface;
}

unsafe extern "C" {
    /// Load a TGA image directly.
    ///
    /// If you know you definitely have a TGA image, you can call this function,
    /// which will skip SDL_image's file format detection routines. Generally it's
    /// better to use the abstract interfaces; also, there is only an [`SDL_IOStream`]
    /// interface available here.
    ///
    /// ## Parameters
    /// - `src`: an [`SDL_IOStream`] to load image data from.
    ///
    /// ## Return value
    /// Returns SDL surface, or NULL on error.
    ///
    /// ## Availability
    /// This function is available since SDL_image 3.0.0.
    ///
    /// ## See also
    /// - [`IMG_LoadAVIF_IO`]
    /// - [`IMG_LoadBMP_IO`]
    /// - [`IMG_LoadCUR_IO`]
    /// - [`IMG_LoadGIF_IO`]
    /// - [`IMG_LoadICO_IO`]
    /// - [`IMG_LoadJPG_IO`]
    /// - [`IMG_LoadJXL_IO`]
    /// - [`IMG_LoadLBM_IO`]
    /// - [`IMG_LoadPCX_IO`]
    /// - [`IMG_LoadPNG_IO`]
    /// - [`IMG_LoadPNM_IO`]
    /// - [`IMG_LoadQOI_IO`]
    /// - [`IMG_LoadSVG_IO`]
    /// - [`IMG_LoadTIF_IO`]
    /// - [`IMG_LoadWEBP_IO`]
    /// - [`IMG_LoadXCF_IO`]
    /// - [`IMG_LoadXPM_IO`]
    /// - [`IMG_LoadXV_IO`]
    pub fn IMG_LoadTGA_IO(src: *mut SDL_IOStream) -> *mut SDL_Surface;
}

unsafe extern "C" {
    /// Load a TIFF image directly.
    ///
    /// If you know you definitely have a TIFF image, you can call this function,
    /// which will skip SDL_image's file format detection routines. Generally it's
    /// better to use the abstract interfaces; also, there is only an [`SDL_IOStream`]
    /// interface available here.
    ///
    /// ## Parameters
    /// - `src`: an [`SDL_IOStream`] to load image data from.
    ///
    /// ## Return value
    /// Returns SDL surface, or NULL on error.
    ///
    /// ## Availability
    /// This function is available since SDL_image 3.0.0.
    ///
    /// ## See also
    /// - [`IMG_LoadAVIF_IO`]
    /// - [`IMG_LoadBMP_IO`]
    /// - [`IMG_LoadCUR_IO`]
    /// - [`IMG_LoadGIF_IO`]
    /// - [`IMG_LoadICO_IO`]
    /// - [`IMG_LoadJPG_IO`]
    /// - [`IMG_LoadJXL_IO`]
    /// - [`IMG_LoadLBM_IO`]
    /// - [`IMG_LoadPCX_IO`]
    /// - [`IMG_LoadPNG_IO`]
    /// - [`IMG_LoadPNM_IO`]
    /// - [`IMG_LoadQOI_IO`]
    /// - [`IMG_LoadSVG_IO`]
    /// - [`IMG_LoadTGA_IO`]
    /// - [`IMG_LoadWEBP_IO`]
    /// - [`IMG_LoadXCF_IO`]
    /// - [`IMG_LoadXPM_IO`]
    /// - [`IMG_LoadXV_IO`]
    pub fn IMG_LoadTIF_IO(src: *mut SDL_IOStream) -> *mut SDL_Surface;
}

unsafe extern "C" {
    /// Load a WEBP image directly.
    ///
    /// If you know you definitely have a WEBP image, you can call this function,
    /// which will skip SDL_image's file format detection routines. Generally it's
    /// better to use the abstract interfaces; also, there is only an [`SDL_IOStream`]
    /// interface available here.
    ///
    /// ## Parameters
    /// - `src`: an [`SDL_IOStream`] to load image data from.
    ///
    /// ## Return value
    /// Returns SDL surface, or NULL on error.
    ///
    /// ## Availability
    /// This function is available since SDL_image 3.0.0.
    ///
    /// ## See also
    /// - [`IMG_LoadAVIF_IO`]
    /// - [`IMG_LoadBMP_IO`]
    /// - [`IMG_LoadCUR_IO`]
    /// - [`IMG_LoadGIF_IO`]
    /// - [`IMG_LoadICO_IO`]
    /// - [`IMG_LoadJPG_IO`]
    /// - [`IMG_LoadJXL_IO`]
    /// - [`IMG_LoadLBM_IO`]
    /// - [`IMG_LoadPCX_IO`]
    /// - [`IMG_LoadPNG_IO`]
    /// - [`IMG_LoadPNM_IO`]
    /// - [`IMG_LoadQOI_IO`]
    /// - [`IMG_LoadSVG_IO`]
    /// - [`IMG_LoadTGA_IO`]
    /// - [`IMG_LoadTIF_IO`]
    /// - [`IMG_LoadXCF_IO`]
    /// - [`IMG_LoadXPM_IO`]
    /// - [`IMG_LoadXV_IO`]
    pub fn IMG_LoadWEBP_IO(src: *mut SDL_IOStream) -> *mut SDL_Surface;
}

unsafe extern "C" {
    /// Load a XCF image directly.
    ///
    /// If you know you definitely have a XCF image, you can call this function,
    /// which will skip SDL_image's file format detection routines. Generally it's
    /// better to use the abstract interfaces; also, there is only an [`SDL_IOStream`]
    /// interface available here.
    ///
    /// ## Parameters
    /// - `src`: an [`SDL_IOStream`] to load image data from.
    ///
    /// ## Return value
    /// Returns SDL surface, or NULL on error.
    ///
    /// ## Availability
    /// This function is available since SDL_image 3.0.0.
    ///
    /// ## See also
    /// - [`IMG_LoadAVIF_IO`]
    /// - [`IMG_LoadBMP_IO`]
    /// - [`IMG_LoadCUR_IO`]
    /// - [`IMG_LoadGIF_IO`]
    /// - [`IMG_LoadICO_IO`]
    /// - [`IMG_LoadJPG_IO`]
    /// - [`IMG_LoadJXL_IO`]
    /// - [`IMG_LoadLBM_IO`]
    /// - [`IMG_LoadPCX_IO`]
    /// - [`IMG_LoadPNG_IO`]
    /// - [`IMG_LoadPNM_IO`]
    /// - [`IMG_LoadQOI_IO`]
    /// - [`IMG_LoadSVG_IO`]
    /// - [`IMG_LoadTGA_IO`]
    /// - [`IMG_LoadTIF_IO`]
    /// - [`IMG_LoadWEBP_IO`]
    /// - [`IMG_LoadXPM_IO`]
    /// - [`IMG_LoadXV_IO`]
    pub fn IMG_LoadXCF_IO(src: *mut SDL_IOStream) -> *mut SDL_Surface;
}

unsafe extern "C" {
    /// Load a XPM image directly.
    ///
    /// If you know you definitely have a XPM image, you can call this function,
    /// which will skip SDL_image's file format detection routines. Generally it's
    /// better to use the abstract interfaces; also, there is only an [`SDL_IOStream`]
    /// interface available here.
    ///
    /// ## Parameters
    /// - `src`: an [`SDL_IOStream`] to load image data from.
    ///
    /// ## Return value
    /// Returns SDL surface, or NULL on error.
    ///
    /// ## Availability
    /// This function is available since SDL_image 3.0.0.
    ///
    /// ## See also
    /// - [`IMG_LoadAVIF_IO`]
    /// - [`IMG_LoadBMP_IO`]
    /// - [`IMG_LoadCUR_IO`]
    /// - [`IMG_LoadGIF_IO`]
    /// - [`IMG_LoadICO_IO`]
    /// - [`IMG_LoadJPG_IO`]
    /// - [`IMG_LoadJXL_IO`]
    /// - [`IMG_LoadLBM_IO`]
    /// - [`IMG_LoadPCX_IO`]
    /// - [`IMG_LoadPNG_IO`]
    /// - [`IMG_LoadPNM_IO`]
    /// - [`IMG_LoadQOI_IO`]
    /// - [`IMG_LoadSVG_IO`]
    /// - [`IMG_LoadTGA_IO`]
    /// - [`IMG_LoadTIF_IO`]
    /// - [`IMG_LoadWEBP_IO`]
    /// - [`IMG_LoadXCF_IO`]
    /// - [`IMG_LoadXV_IO`]
    pub fn IMG_LoadXPM_IO(src: *mut SDL_IOStream) -> *mut SDL_Surface;
}

unsafe extern "C" {
    /// Load a XV image directly.
    ///
    /// If you know you definitely have a XV image, you can call this function,
    /// which will skip SDL_image's file format detection routines. Generally it's
    /// better to use the abstract interfaces; also, there is only an [`SDL_IOStream`]
    /// interface available here.
    ///
    /// ## Parameters
    /// - `src`: an [`SDL_IOStream`] to load image data from.
    ///
    /// ## Return value
    /// Returns SDL surface, or NULL on error.
    ///
    /// ## Availability
    /// This function is available since SDL_image 3.0.0.
    ///
    /// ## See also
    /// - [`IMG_LoadAVIF_IO`]
    /// - [`IMG_LoadBMP_IO`]
    /// - [`IMG_LoadCUR_IO`]
    /// - [`IMG_LoadGIF_IO`]
    /// - [`IMG_LoadICO_IO`]
    /// - [`IMG_LoadJPG_IO`]
    /// - [`IMG_LoadJXL_IO`]
    /// - [`IMG_LoadLBM_IO`]
    /// - [`IMG_LoadPCX_IO`]
    /// - [`IMG_LoadPNG_IO`]
    /// - [`IMG_LoadPNM_IO`]
    /// - [`IMG_LoadQOI_IO`]
    /// - [`IMG_LoadSVG_IO`]
    /// - [`IMG_LoadTGA_IO`]
    /// - [`IMG_LoadTIF_IO`]
    /// - [`IMG_LoadWEBP_IO`]
    /// - [`IMG_LoadXCF_IO`]
    /// - [`IMG_LoadXPM_IO`]
    pub fn IMG_LoadXV_IO(src: *mut SDL_IOStream) -> *mut SDL_Surface;
}

unsafe extern "C" {
    /// Load an XPM image from a memory array.
    ///
    /// The returned surface will be an 8bpp indexed surface, if possible,
    /// otherwise it will be 32bpp. If you always want 32-bit data, use
    /// [`IMG_ReadXPMFromArrayToRGB888()`] instead.
    ///
    /// When done with the returned surface, the app should dispose of it with a
    /// call to [`SDL_DestroySurface()`].
    ///
    /// ## Parameters
    /// - `xpm`: a null-terminated array of strings that comprise XPM data.
    ///
    /// ## Return value
    /// Returns a new SDL surface, or NULL on error.
    ///
    /// ## Availability
    /// This function is available since SDL_image 3.0.0.
    ///
    /// ## See also
    /// - [`IMG_ReadXPMFromArrayToRGB888`]
    pub fn IMG_ReadXPMFromArray(xpm: *mut *mut ::core::ffi::c_char) -> *mut SDL_Surface;
}

unsafe extern "C" {
    /// Load an XPM image from a memory array.
    ///
    /// The returned surface will always be a 32-bit RGB surface. If you want 8-bit
    /// indexed colors (and the XPM data allows it), use [`IMG_ReadXPMFromArray()`]
    /// instead.
    ///
    /// When done with the returned surface, the app should dispose of it with a
    /// call to [`SDL_DestroySurface()`].
    ///
    /// ## Parameters
    /// - `xpm`: a null-terminated array of strings that comprise XPM data.
    ///
    /// ## Return value
    /// Returns a new SDL surface, or NULL on error.
    ///
    /// ## Availability
    /// This function is available since SDL_image 3.0.0.
    ///
    /// ## See also
    /// - [`IMG_ReadXPMFromArray`]
    pub fn IMG_ReadXPMFromArrayToRGB888(xpm: *mut *mut ::core::ffi::c_char) -> *mut SDL_Surface;
}

unsafe extern "C" {
    /// Save an [`SDL_Surface`] into an image file.
    ///
    /// If the file already exists, it will be overwritten.
    ///
    /// For formats that accept a quality, a default quality of 90 will be used.
    ///
    /// ## Parameters
    /// - `surface`: the SDL surface to save.
    /// - `file`: path on the filesystem to write new file to.
    ///
    /// ## Return value
    /// Returns true on success or false on failure; call [`SDL_GetError()`] for more
    ///   information.
    ///
    /// ## Availability
    /// This function is available since SDL_image 3.4.0.
    ///
    /// ## See also
    /// - [`IMG_SaveTyped_IO`]
    /// - [`IMG_SaveAVIF`]
    /// - [`IMG_SaveBMP`]
    /// - [`IMG_SaveCUR`]
    /// - [`IMG_SaveGIF`]
    /// - [`IMG_SaveICO`]
    /// - [`IMG_SaveJPG`]
    /// - [`IMG_SavePNG`]
    /// - [`IMG_SaveTGA`]
    /// - [`IMG_SaveWEBP`]
    pub fn IMG_Save(
        surface: *mut SDL_Surface,
        file: *const ::core::ffi::c_char,
    ) -> ::core::primitive::bool;
}

unsafe extern "C" {
    /// Save an [`SDL_Surface`] into formatted image data, via an [`SDL_IOStream`].
    ///
    /// If you just want to save to a filename, you can use [`IMG_Save()`] instead.
    ///
    /// If `closeio` is true, `dst` will be closed before returning, whether this
    /// function succeeds or not.
    ///
    /// For formats that accept a quality, a default quality of 90 will be used.
    ///
    /// ## Parameters
    /// - `surface`: the SDL surface to save.
    /// - `dst`: the [`SDL_IOStream`] to save the image data to.
    /// - `closeio`: true to close/free the [`SDL_IOStream`] before returning, false
    ///   to leave it open.
    /// - `type`: a filename extension that represent this data ("BMP", "GIF",
    ///   "PNG", etc).
    ///
    /// ## Return value
    /// Returns true on success or false on failure; call [`SDL_GetError()`] for more
    ///   information.
    ///
    /// ## Availability
    /// This function is available since SDL_image 3.4.0.
    ///
    /// ## See also
    /// - [`IMG_Save`]
    /// - [`IMG_SaveAVIF_IO`]
    /// - [`IMG_SaveBMP_IO`]
    /// - [`IMG_SaveCUR_IO`]
    /// - [`IMG_SaveGIF_IO`]
    /// - [`IMG_SaveICO_IO`]
    /// - [`IMG_SaveJPG_IO`]
    /// - [`IMG_SavePNG_IO`]
    /// - [`IMG_SaveTGA_IO`]
    /// - [`IMG_SaveWEBP_IO`]
    pub fn IMG_SaveTyped_IO(
        surface: *mut SDL_Surface,
        dst: *mut SDL_IOStream,
        closeio: ::core::primitive::bool,
        r#type: *const ::core::ffi::c_char,
    ) -> ::core::primitive::bool;
}

unsafe extern "C" {
    /// Save an [`SDL_Surface`] into a AVIF image file.
    ///
    /// If the file already exists, it will be overwritten.
    ///
    /// ## Parameters
    /// - `surface`: the SDL surface to save.
    /// - `file`: path on the filesystem to write new file to.
    /// - `quality`: the desired quality, ranging between 0 (lowest) and 100
    ///   (highest).
    ///
    /// ## Return value
    /// Returns true on success or false on failure; call [`SDL_GetError()`] for more
    ///   information.
    ///
    /// ## Availability
    /// This function is available since SDL_image 3.0.0.
    ///
    /// ## See also
    /// - [`IMG_SaveAVIF_IO`]
    pub fn IMG_SaveAVIF(
        surface: *mut SDL_Surface,
        file: *const ::core::ffi::c_char,
        quality: ::core::ffi::c_int,
    ) -> ::core::primitive::bool;
}

unsafe extern "C" {
    /// Save an [`SDL_Surface`] into AVIF image data, via an [`SDL_IOStream`].
    ///
    /// If you just want to save to a filename, you can use [`IMG_SaveAVIF()`] instead.
    ///
    /// If `closeio` is true, `dst` will be closed before returning, whether this
    /// function succeeds or not.
    ///
    /// ## Parameters
    /// - `surface`: the SDL surface to save.
    /// - `dst`: the [`SDL_IOStream`] to save the image data to.
    /// - `closeio`: true to close/free the [`SDL_IOStream`] before returning, false
    ///   to leave it open.
    /// - `quality`: the desired quality, ranging between 0 (lowest) and 100
    ///   (highest).
    ///
    /// ## Return value
    /// Returns true on success or false on failure; call [`SDL_GetError()`] for more
    ///   information.
    ///
    /// ## Availability
    /// This function is available since SDL_image 3.0.0.
    ///
    /// ## See also
    /// - [`IMG_SaveAVIF`]
    pub fn IMG_SaveAVIF_IO(
        surface: *mut SDL_Surface,
        dst: *mut SDL_IOStream,
        closeio: ::core::primitive::bool,
        quality: ::core::ffi::c_int,
    ) -> ::core::primitive::bool;
}

unsafe extern "C" {
    /// Save an [`SDL_Surface`] into a BMP image file.
    ///
    /// If the file already exists, it will be overwritten.
    ///
    /// ## Parameters
    /// - `surface`: the SDL surface to save.
    /// - `file`: path on the filesystem to write new file to.
    ///
    /// ## Return value
    /// Returns true on success or false on failure; call [`SDL_GetError()`] for more
    ///   information.
    ///
    /// ## Availability
    /// This function is available since SDL_image 3.4.0.
    ///
    /// ## See also
    /// - [`IMG_SaveBMP_IO`]
    pub fn IMG_SaveBMP(
        surface: *mut SDL_Surface,
        file: *const ::core::ffi::c_char,
    ) -> ::core::primitive::bool;
}

unsafe extern "C" {
    /// Save an [`SDL_Surface`] into BMP image data, via an [`SDL_IOStream`].
    ///
    /// If you just want to save to a filename, you can use [`IMG_SaveBMP()`] instead.
    ///
    /// If `closeio` is true, `dst` will be closed before returning, whether this
    /// function succeeds or not.
    ///
    /// ## Parameters
    /// - `surface`: the SDL surface to save.
    /// - `dst`: the [`SDL_IOStream`] to save the image data to.
    /// - `closeio`: true to close/free the [`SDL_IOStream`] before returning, false
    ///   to leave it open.
    ///
    /// ## Return value
    /// Returns true on success or false on failure; call [`SDL_GetError()`] for more
    ///   information.
    ///
    /// ## Availability
    /// This function is available since SDL_image 3.4.0.
    ///
    /// ## See also
    /// - [`IMG_SaveBMP`]
    pub fn IMG_SaveBMP_IO(
        surface: *mut SDL_Surface,
        dst: *mut SDL_IOStream,
        closeio: ::core::primitive::bool,
    ) -> ::core::primitive::bool;
}

unsafe extern "C" {
    /// Save an [`SDL_Surface`] into a CUR image file.
    ///
    /// If the file already exists, it will be overwritten.
    ///
    /// ## Parameters
    /// - `surface`: the SDL surface to save.
    /// - `file`: path on the filesystem to write new file to.
    ///
    /// ## Return value
    /// Returns true on success or false on failure; call [`SDL_GetError()`] for more
    ///   information.
    ///
    /// ## Availability
    /// This function is available since SDL_image 3.4.0.
    ///
    /// ## See also
    /// - [`IMG_SaveCUR_IO`]
    pub fn IMG_SaveCUR(
        surface: *mut SDL_Surface,
        file: *const ::core::ffi::c_char,
    ) -> ::core::primitive::bool;
}

unsafe extern "C" {
    /// Save an [`SDL_Surface`] into CUR image data, via an [`SDL_IOStream`].
    ///
    /// If you just want to save to a filename, you can use [`IMG_SaveCUR()`] instead.
    ///
    /// If `closeio` is true, `dst` will be closed before returning, whether this
    /// function succeeds or not.
    ///
    /// ## Parameters
    /// - `surface`: the SDL surface to save.
    /// - `dst`: the [`SDL_IOStream`] to save the image data to.
    /// - `closeio`: true to close/free the [`SDL_IOStream`] before returning, false
    ///   to leave it open.
    ///
    /// ## Return value
    /// Returns true on success or false on failure; call [`SDL_GetError()`] for more
    ///   information.
    ///
    /// ## Availability
    /// This function is available since SDL_image 3.4.0.
    ///
    /// ## See also
    /// - [`IMG_SaveCUR`]
    pub fn IMG_SaveCUR_IO(
        surface: *mut SDL_Surface,
        dst: *mut SDL_IOStream,
        closeio: ::core::primitive::bool,
    ) -> ::core::primitive::bool;
}

unsafe extern "C" {
    /// Save an [`SDL_Surface`] into a GIF image file.
    ///
    /// If the file already exists, it will be overwritten.
    ///
    /// ## Parameters
    /// - `surface`: the SDL surface to save.
    /// - `file`: path on the filesystem to write new file to.
    ///
    /// ## Return value
    /// Returns true on success or false on failure; call [`SDL_GetError()`] for more
    ///   information.
    ///
    /// ## Availability
    /// This function is available since SDL_image 3.4.0.
    ///
    /// ## See also
    /// - [`IMG_SaveGIF_IO`]
    pub fn IMG_SaveGIF(
        surface: *mut SDL_Surface,
        file: *const ::core::ffi::c_char,
    ) -> ::core::primitive::bool;
}

unsafe extern "C" {
    /// Save an [`SDL_Surface`] into GIF image data, via an [`SDL_IOStream`].
    ///
    /// If you just want to save to a filename, you can use [`IMG_SaveGIF()`] instead.
    ///
    /// If `closeio` is true, `dst` will be closed before returning, whether this
    /// function succeeds or not.
    ///
    /// ## Parameters
    /// - `surface`: the SDL surface to save.
    /// - `dst`: the [`SDL_IOStream`] to save the image data to.
    /// - `closeio`: true to close/free the [`SDL_IOStream`] before returning, false
    ///   to leave it open.
    ///
    /// ## Return value
    /// Returns true on success or false on failure; call [`SDL_GetError()`] for more
    ///   information.
    ///
    /// ## Availability
    /// This function is available since SDL_image 3.4.0.
    ///
    /// ## See also
    /// - [`IMG_SaveGIF`]
    pub fn IMG_SaveGIF_IO(
        surface: *mut SDL_Surface,
        dst: *mut SDL_IOStream,
        closeio: ::core::primitive::bool,
    ) -> ::core::primitive::bool;
}

unsafe extern "C" {
    /// Save an [`SDL_Surface`] into a ICO image file.
    ///
    /// If the file already exists, it will be overwritten.
    ///
    /// ## Parameters
    /// - `surface`: the SDL surface to save.
    /// - `file`: path on the filesystem to write new file to.
    ///
    /// ## Return value
    /// Returns true on success or false on failure; call [`SDL_GetError()`] for more
    ///   information.
    ///
    /// ## Availability
    /// This function is available since SDL_image 3.4.0.
    ///
    /// ## See also
    /// - [`IMG_SaveICO_IO`]
    pub fn IMG_SaveICO(
        surface: *mut SDL_Surface,
        file: *const ::core::ffi::c_char,
    ) -> ::core::primitive::bool;
}

unsafe extern "C" {
    /// Save an [`SDL_Surface`] into ICO image data, via an [`SDL_IOStream`].
    ///
    /// If you just want to save to a filename, you can use [`IMG_SaveICO()`] instead.
    ///
    /// If `closeio` is true, `dst` will be closed before returning, whether this
    /// function succeeds or not.
    ///
    /// ## Parameters
    /// - `surface`: the SDL surface to save.
    /// - `dst`: the [`SDL_IOStream`] to save the image data to.
    /// - `closeio`: true to close/free the [`SDL_IOStream`] before returning, false
    ///   to leave it open.
    ///
    /// ## Return value
    /// Returns true on success or false on failure; call [`SDL_GetError()`] for more
    ///   information.
    ///
    /// ## Availability
    /// This function is available since SDL_image 3.4.0.
    ///
    /// ## See also
    /// - [`IMG_SaveICO`]
    pub fn IMG_SaveICO_IO(
        surface: *mut SDL_Surface,
        dst: *mut SDL_IOStream,
        closeio: ::core::primitive::bool,
    ) -> ::core::primitive::bool;
}

unsafe extern "C" {
    /// Save an [`SDL_Surface`] into a JPEG image file.
    ///
    /// If the file already exists, it will be overwritten.
    ///
    /// ## Parameters
    /// - `surface`: the SDL surface to save.
    /// - `file`: path on the filesystem to write new file to.
    /// - `quality`: \[0; 33\] is Lowest quality, \[34; 66\] is Middle quality, [67;
    ///   100\] is Highest quality.
    ///
    /// ## Return value
    /// Returns true on success or false on failure; call [`SDL_GetError()`] for more
    ///   information.
    ///
    /// ## Availability
    /// This function is available since SDL_image 3.0.0.
    ///
    /// ## See also
    /// - [`IMG_SaveJPG_IO`]
    pub fn IMG_SaveJPG(
        surface: *mut SDL_Surface,
        file: *const ::core::ffi::c_char,
        quality: ::core::ffi::c_int,
    ) -> ::core::primitive::bool;
}

unsafe extern "C" {
    /// Save an [`SDL_Surface`] into JPEG image data, via an [`SDL_IOStream`].
    ///
    /// If you just want to save to a filename, you can use [`IMG_SaveJPG()`] instead.
    ///
    /// If `closeio` is true, `dst` will be closed before returning, whether this
    /// function succeeds or not.
    ///
    /// ## Parameters
    /// - `surface`: the SDL surface to save.
    /// - `dst`: the [`SDL_IOStream`] to save the image data to.
    /// - `closeio`: true to close/free the [`SDL_IOStream`] before returning, false
    ///   to leave it open.
    /// - `quality`: \[0; 33\] is Lowest quality, \[34; 66\] is Middle quality, [67;
    ///   100\] is Highest quality.
    ///
    /// ## Return value
    /// Returns true on success or false on failure; call [`SDL_GetError()`] for more
    ///   information.
    ///
    /// ## Availability
    /// This function is available since SDL_image 3.0.0.
    ///
    /// ## See also
    /// - [`IMG_SaveJPG`]
    pub fn IMG_SaveJPG_IO(
        surface: *mut SDL_Surface,
        dst: *mut SDL_IOStream,
        closeio: ::core::primitive::bool,
        quality: ::core::ffi::c_int,
    ) -> ::core::primitive::bool;
}

unsafe extern "C" {
    /// Save an [`SDL_Surface`] into a PNG image file.
    ///
    /// If the file already exists, it will be overwritten.
    ///
    /// ## Parameters
    /// - `surface`: the SDL surface to save.
    /// - `file`: path on the filesystem to write new file to.
    ///
    /// ## Return value
    /// Returns true on success or false on failure; call [`SDL_GetError()`] for more
    ///   information.
    ///
    /// ## Availability
    /// This function is available since SDL_image 3.0.0.
    ///
    /// ## See also
    /// - [`IMG_SavePNG_IO`]
    pub fn IMG_SavePNG(
        surface: *mut SDL_Surface,
        file: *const ::core::ffi::c_char,
    ) -> ::core::primitive::bool;
}

unsafe extern "C" {
    /// Save an [`SDL_Surface`] into PNG image data, via an [`SDL_IOStream`].
    ///
    /// If you just want to save to a filename, you can use [`IMG_SavePNG()`] instead.
    ///
    /// If `closeio` is true, `dst` will be closed before returning, whether this
    /// function succeeds or not.
    ///
    /// ## Parameters
    /// - `surface`: the SDL surface to save.
    /// - `dst`: the [`SDL_IOStream`] to save the image data to.
    /// - `closeio`: true to close/free the [`SDL_IOStream`] before returning, false
    ///   to leave it open.
    ///
    /// ## Return value
    /// Returns true on success or false on failure; call [`SDL_GetError()`] for more
    ///   information.
    ///
    /// ## Availability
    /// This function is available since SDL_image 3.0.0.
    ///
    /// ## See also
    /// - [`IMG_SavePNG`]
    pub fn IMG_SavePNG_IO(
        surface: *mut SDL_Surface,
        dst: *mut SDL_IOStream,
        closeio: ::core::primitive::bool,
    ) -> ::core::primitive::bool;
}

unsafe extern "C" {
    /// Save an [`SDL_Surface`] into a TGA image file.
    ///
    /// If the file already exists, it will be overwritten.
    ///
    /// ## Parameters
    /// - `surface`: the SDL surface to save.
    /// - `file`: path on the filesystem to write new file to.
    ///
    /// ## Return value
    /// Returns true on success or false on failure; call [`SDL_GetError()`] for more
    ///   information.
    ///
    /// ## Availability
    /// This function is available since SDL_image 3.4.0.
    ///
    /// ## See also
    /// - [`IMG_SaveTGA_IO`]
    pub fn IMG_SaveTGA(
        surface: *mut SDL_Surface,
        file: *const ::core::ffi::c_char,
    ) -> ::core::primitive::bool;
}

unsafe extern "C" {
    /// Save an [`SDL_Surface`] into TGA image data, via an [`SDL_IOStream`].
    ///
    /// If you just want to save to a filename, you can use [`IMG_SaveTGA()`] instead.
    ///
    /// If `closeio` is true, `dst` will be closed before returning, whether this
    /// function succeeds or not.
    ///
    /// ## Parameters
    /// - `surface`: the SDL surface to save.
    /// - `dst`: the [`SDL_IOStream`] to save the image data to.
    /// - `closeio`: true to close/free the [`SDL_IOStream`] before returning, false
    ///   to leave it open.
    ///
    /// ## Return value
    /// Returns true on success or false on failure; call [`SDL_GetError()`] for more
    ///   information.
    ///
    /// ## Availability
    /// This function is available since SDL_image 3.4.0.
    ///
    /// ## See also
    /// - [`IMG_SaveTGA`]
    pub fn IMG_SaveTGA_IO(
        surface: *mut SDL_Surface,
        dst: *mut SDL_IOStream,
        closeio: ::core::primitive::bool,
    ) -> ::core::primitive::bool;
}

unsafe extern "C" {
    /// Save an [`SDL_Surface`] into a WEBP image file.
    ///
    /// If the file already exists, it will be overwritten.
    ///
    /// ## Parameters
    /// - `surface`: the SDL surface to save.
    /// - `file`: path on the filesystem to write the new file to.
    /// - `quality`: between 0 and 100. For lossy, 0 gives the smallest size and
    ///   100 the largest. For lossless, this parameter is the amount
    ///   of effort put into the compression: 0 is the fastest but
    ///   gives larger files compared to the slowest, but best, 100.
    ///
    /// ## Return value
    /// Returns true on success or false on failure; call [`SDL_GetError()`] for more
    ///   information.
    ///
    /// ## Availability
    /// This function is available since SDL_image 3.4.0.
    ///
    /// ## See also
    /// - [`IMG_SaveWEBP_IO`]
    pub fn IMG_SaveWEBP(
        surface: *mut SDL_Surface,
        file: *const ::core::ffi::c_char,
        quality: ::core::ffi::c_float,
    ) -> ::core::primitive::bool;
}

unsafe extern "C" {
    /// Save an [`SDL_Surface`] into WEBP image data, via an [`SDL_IOStream`].
    ///
    /// If you just want to save to a filename, you can use [`IMG_SaveWEBP()`] instead.
    ///
    /// If `closeio` is true, `dst` will be closed before returning, whether this
    /// function succeeds or not.
    ///
    /// ## Parameters
    /// - `surface`: the SDL surface to save.
    /// - `dst`: the [`SDL_IOStream`] to save the image data to.
    /// - `closeio`: true to close/free the [`SDL_IOStream`] before returning, false
    ///   to leave it open.
    /// - `quality`: between 0 and 100. For lossy, 0 gives the smallest size and
    ///   100 the largest. For lossless, this parameter is the amount
    ///   of effort put into the compression: 0 is the fastest but
    ///   gives larger files compared to the slowest, but best, 100.
    ///
    /// ## Return value
    /// Returns true on success or false on failure; call [`SDL_GetError()`] for more
    ///   information.
    ///
    /// ## Availability
    /// This function is available since SDL_image 3.4.0.
    ///
    /// ## See also
    /// - [`IMG_SaveWEBP`]
    pub fn IMG_SaveWEBP_IO(
        surface: *mut SDL_Surface,
        dst: *mut SDL_IOStream,
        closeio: ::core::primitive::bool,
        quality: ::core::ffi::c_float,
    ) -> ::core::primitive::bool;
}

/// * Animated image support
#[repr(C)]
#[cfg_attr(feature = "debug-impls", derive(Debug))]
pub struct IMG_Animation {
    /// The width of the frames
    pub w: ::core::ffi::c_int,
    /// The height of the frames
    pub h: ::core::ffi::c_int,
    /// The number of frames
    pub count: ::core::ffi::c_int,
    /// An array of frames
    pub frames: *mut *mut SDL_Surface,
    /// An array of frame delays, in milliseconds
    pub delays: *mut ::core::ffi::c_int,
}

impl ::core::default::Default for IMG_Animation {
    /// Initialize all fields to zero
    #[inline(always)]
    fn default() -> Self {
        unsafe { ::core::mem::MaybeUninit::<Self>::zeroed().assume_init() }
    }
}

unsafe extern "C" {
    /// Load an animation from a file.
    ///
    /// When done with the returned animation, the app should dispose of it with a
    /// call to [`IMG_FreeAnimation()`].
    ///
    /// ## Parameters
    /// - `file`: path on the filesystem containing an animated image.
    ///
    /// ## Return value
    /// Returns a new [`IMG_Animation`], or NULL on error.
    ///
    /// ## Availability
    /// This function is available since SDL_image 3.0.0.
    ///
    /// ## See also
    /// - [`IMG_CreateAnimatedCursor`]
    /// - [`IMG_LoadAnimation_IO`]
    /// - [`IMG_LoadAnimationTyped_IO`]
    /// - [`IMG_LoadANIAnimation_IO`]
    /// - [`IMG_LoadAPNGAnimation_IO`]
    /// - [`IMG_LoadAVIFAnimation_IO`]
    /// - [`IMG_LoadGIFAnimation_IO`]
    /// - [`IMG_LoadWEBPAnimation_IO`]
    /// - [`IMG_FreeAnimation`]
    pub fn IMG_LoadAnimation(file: *const ::core::ffi::c_char) -> *mut IMG_Animation;
}

unsafe extern "C" {
    /// Load an animation from an [`SDL_IOStream`].
    ///
    /// If `closeio` is true, `src` will be closed before returning, whether this
    /// function succeeds or not. SDL_image reads everything it needs from `src`
    /// during this call in any case.
    ///
    /// When done with the returned animation, the app should dispose of it with a
    /// call to [`IMG_FreeAnimation()`].
    ///
    /// ## Parameters
    /// - `src`: an [`SDL_IOStream`] that data will be read from.
    /// - `closeio`: true to close/free the [`SDL_IOStream`] before returning, false
    ///   to leave it open.
    ///
    /// ## Return value
    /// Returns a new [`IMG_Animation`], or NULL on error.
    ///
    /// ## Availability
    /// This function is available since SDL_image 3.0.0.
    ///
    /// ## See also
    /// - [`IMG_CreateAnimatedCursor`]
    /// - [`IMG_LoadAnimation`]
    /// - [`IMG_LoadAnimationTyped_IO`]
    /// - [`IMG_LoadANIAnimation_IO`]
    /// - [`IMG_LoadAPNGAnimation_IO`]
    /// - [`IMG_LoadAVIFAnimation_IO`]
    /// - [`IMG_LoadGIFAnimation_IO`]
    /// - [`IMG_LoadWEBPAnimation_IO`]
    /// - [`IMG_FreeAnimation`]
    pub fn IMG_LoadAnimation_IO(
        src: *mut SDL_IOStream,
        closeio: ::core::primitive::bool,
    ) -> *mut IMG_Animation;
}

unsafe extern "C" {
    /// Load an animation from an [`SDL_IOStream`].
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
    /// ## Parameters
    /// - `src`: an [`SDL_IOStream`] that data will be read from.
    /// - `closeio`: true to close/free the [`SDL_IOStream`] before returning, false
    ///   to leave it open.
    /// - `type`: a filename extension that represent this data ("GIF", etc).
    ///
    /// ## Return value
    /// Returns a new [`IMG_Animation`], or NULL on error.
    ///
    /// ## Availability
    /// This function is available since SDL_image 3.0.0.
    ///
    /// ## See also
    /// - [`IMG_CreateAnimatedCursor`]
    /// - [`IMG_LoadAnimation`]
    /// - [`IMG_LoadAnimation_IO`]
    /// - [`IMG_LoadANIAnimation_IO`]
    /// - [`IMG_LoadAPNGAnimation_IO`]
    /// - [`IMG_LoadAVIFAnimation_IO`]
    /// - [`IMG_LoadGIFAnimation_IO`]
    /// - [`IMG_LoadWEBPAnimation_IO`]
    /// - [`IMG_FreeAnimation`]
    pub fn IMG_LoadAnimationTyped_IO(
        src: *mut SDL_IOStream,
        closeio: ::core::primitive::bool,
        r#type: *const ::core::ffi::c_char,
    ) -> *mut IMG_Animation;
}

unsafe extern "C" {
    /// Load an ANI animation directly from an [`SDL_IOStream`].
    ///
    /// If you know you definitely have an ANI image, you can call this function,
    /// which will skip SDL_image's file format detection routines. Generally, it's
    /// better to use the abstract interfaces; also, there is only an [`SDL_IOStream`]
    /// interface available here.
    ///
    /// When done with the returned animation, the app should dispose of it with a
    /// call to [`IMG_FreeAnimation()`].
    ///
    /// ## Parameters
    /// - `src`: an [`SDL_IOStream`] from which data will be read.
    ///
    /// ## Return value
    /// Returns a new [`IMG_Animation`], or NULL on error.
    ///
    /// ## Availability
    /// This function is available since SDL_image 3.4.0.
    ///
    /// ## See also
    /// - [`IMG_isANI`]
    /// - [`IMG_LoadAnimation`]
    /// - [`IMG_LoadAnimation_IO`]
    /// - [`IMG_LoadAnimationTyped_IO`]
    /// - [`IMG_LoadAPNGAnimation_IO`]
    /// - [`IMG_LoadAVIFAnimation_IO`]
    /// - [`IMG_LoadGIFAnimation_IO`]
    /// - [`IMG_LoadWEBPAnimation_IO`]
    /// - [`IMG_FreeAnimation`]
    pub fn IMG_LoadANIAnimation_IO(src: *mut SDL_IOStream) -> *mut IMG_Animation;
}

unsafe extern "C" {
    /// Load an APNG animation directly from an [`SDL_IOStream`].
    ///
    /// If you know you definitely have an APNG image, you can call this function,
    /// which will skip SDL_image's file format detection routines. Generally, it's
    /// better to use the abstract interfaces; also, there is only an [`SDL_IOStream`]
    /// interface available here.
    ///
    /// When done with the returned animation, the app should dispose of it with a
    /// call to [`IMG_FreeAnimation()`].
    ///
    /// ## Parameters
    /// - `src`: an [`SDL_IOStream`] from which data will be read.
    ///
    /// ## Return value
    /// Returns a new [`IMG_Animation`], or NULL on error.
    ///
    /// ## Availability
    /// This function is available since SDL_image 3.4.0.
    ///
    /// ## See also
    /// - [`IMG_isPNG`]
    /// - [`IMG_LoadAnimation`]
    /// - [`IMG_LoadAnimation_IO`]
    /// - [`IMG_LoadAnimationTyped_IO`]
    /// - [`IMG_LoadANIAnimation_IO`]
    /// - [`IMG_LoadAVIFAnimation_IO`]
    /// - [`IMG_LoadGIFAnimation_IO`]
    /// - [`IMG_LoadWEBPAnimation_IO`]
    /// - [`IMG_FreeAnimation`]
    pub fn IMG_LoadAPNGAnimation_IO(src: *mut SDL_IOStream) -> *mut IMG_Animation;
}

unsafe extern "C" {
    /// Load an AVIF animation directly from an [`SDL_IOStream`].
    ///
    /// If you know you definitely have an AVIF animation, you can call this
    /// function, which will skip SDL_image's file format detection routines.
    /// Generally it's better to use the abstract interfaces; also, there is only
    /// an [`SDL_IOStream`] interface available here.
    ///
    /// When done with the returned animation, the app should dispose of it with a
    /// call to [`IMG_FreeAnimation()`].
    ///
    /// ## Parameters
    /// - `src`: an [`SDL_IOStream`] that data will be read from.
    ///
    /// ## Return value
    /// Returns a new [`IMG_Animation`], or NULL on error.
    ///
    /// ## Availability
    /// This function is available since SDL_image 3.4.0.
    ///
    /// ## See also
    /// - [`IMG_isAVIF`]
    /// - [`IMG_LoadAnimation`]
    /// - [`IMG_LoadAnimation_IO`]
    /// - [`IMG_LoadAnimationTyped_IO`]
    /// - [`IMG_LoadANIAnimation_IO`]
    /// - [`IMG_LoadAPNGAnimation_IO`]
    /// - [`IMG_LoadGIFAnimation_IO`]
    /// - [`IMG_LoadWEBPAnimation_IO`]
    /// - [`IMG_FreeAnimation`]
    pub fn IMG_LoadAVIFAnimation_IO(src: *mut SDL_IOStream) -> *mut IMG_Animation;
}

unsafe extern "C" {
    /// Load a GIF animation directly.
    ///
    /// If you know you definitely have a GIF image, you can call this function,
    /// which will skip SDL_image's file format detection routines. Generally it's
    /// better to use the abstract interfaces; also, there is only an [`SDL_IOStream`]
    /// interface available here.
    ///
    /// ## Parameters
    /// - `src`: an [`SDL_IOStream`] that data will be read from.
    ///
    /// ## Return value
    /// Returns a new [`IMG_Animation`], or NULL on error.
    ///
    /// ## Availability
    /// This function is available since SDL_image 3.0.0.
    ///
    /// ## See also
    /// - [`IMG_isGIF`]
    /// - [`IMG_LoadAnimation`]
    /// - [`IMG_LoadAnimation_IO`]
    /// - [`IMG_LoadAnimationTyped_IO`]
    /// - [`IMG_LoadANIAnimation_IO`]
    /// - [`IMG_LoadAPNGAnimation_IO`]
    /// - [`IMG_LoadAVIFAnimation_IO`]
    /// - [`IMG_LoadWEBPAnimation_IO`]
    /// - [`IMG_FreeAnimation`]
    pub fn IMG_LoadGIFAnimation_IO(src: *mut SDL_IOStream) -> *mut IMG_Animation;
}

unsafe extern "C" {
    /// Load a WEBP animation directly.
    ///
    /// If you know you definitely have a WEBP image, you can call this function,
    /// which will skip SDL_image's file format detection routines. Generally it's
    /// better to use the abstract interfaces; also, there is only an [`SDL_IOStream`]
    /// interface available here.
    ///
    /// ## Parameters
    /// - `src`: an [`SDL_IOStream`] that data will be read from.
    ///
    /// ## Return value
    /// Returns a new [`IMG_Animation`], or NULL on error.
    ///
    /// ## Availability
    /// This function is available since SDL_image 3.0.0.
    ///
    /// ## See also
    /// - [`IMG_isWEBP`]
    /// - [`IMG_LoadAnimation`]
    /// - [`IMG_LoadAnimation_IO`]
    /// - [`IMG_LoadAnimationTyped_IO`]
    /// - [`IMG_LoadANIAnimation_IO`]
    /// - [`IMG_LoadAPNGAnimation_IO`]
    /// - [`IMG_LoadAVIFAnimation_IO`]
    /// - [`IMG_LoadGIFAnimation_IO`]
    /// - [`IMG_FreeAnimation`]
    pub fn IMG_LoadWEBPAnimation_IO(src: *mut SDL_IOStream) -> *mut IMG_Animation;
}

unsafe extern "C" {
    /// Save an animation to a file.
    ///
    /// For formats that accept a quality, a default quality of 90 will be used.
    ///
    /// ## Parameters
    /// - `anim`: the animation to save.
    /// - `file`: path on the filesystem containing an animated image.
    ///
    /// ## Return value
    /// Returns true on success or false on failure; call [`SDL_GetError()`] for more
    ///   information.
    ///
    /// ## Availability
    /// This function is available since SDL_image 3.4.0.
    ///
    /// ## See also
    /// - [`IMG_SaveAnimationTyped_IO`]
    /// - [`IMG_SaveANIAnimation_IO`]
    /// - [`IMG_SaveAPNGAnimation_IO`]
    /// - [`IMG_SaveAVIFAnimation_IO`]
    /// - [`IMG_SaveGIFAnimation_IO`]
    /// - [`IMG_SaveWEBPAnimation_IO`]
    pub fn IMG_SaveAnimation(
        anim: *mut IMG_Animation,
        file: *const ::core::ffi::c_char,
    ) -> ::core::primitive::bool;
}

unsafe extern "C" {
    /// Save an animation to an [`SDL_IOStream`].
    ///
    /// If you just want to save to a filename, you can use [`IMG_SaveAnimation()`]
    /// instead.
    ///
    /// If `closeio` is true, `dst` will be closed before returning, whether this
    /// function succeeds or not.
    ///
    /// For formats that accept a quality, a default quality of 90 will be used.
    ///
    /// ## Parameters
    /// - `anim`: the animation to save.
    /// - `dst`: an [`SDL_IOStream`] that data will be written to.
    /// - `closeio`: true to close/free the [`SDL_IOStream`] before returning, false
    ///   to leave it open.
    /// - `type`: a filename extension that represent this data ("GIF", etc).
    ///
    /// ## Return value
    /// Returns true on success or false on failure; call [`SDL_GetError()`] for more
    ///   information.
    ///
    /// ## Availability
    /// This function is available since SDL_image 3.4.0.
    ///
    /// ## See also
    /// - [`IMG_SaveAnimation`]
    /// - [`IMG_SaveANIAnimation_IO`]
    /// - [`IMG_SaveAPNGAnimation_IO`]
    /// - [`IMG_SaveAVIFAnimation_IO`]
    /// - [`IMG_SaveGIFAnimation_IO`]
    /// - [`IMG_SaveWEBPAnimation_IO`]
    pub fn IMG_SaveAnimationTyped_IO(
        anim: *mut IMG_Animation,
        dst: *mut SDL_IOStream,
        closeio: ::core::primitive::bool,
        r#type: *const ::core::ffi::c_char,
    ) -> ::core::primitive::bool;
}

unsafe extern "C" {
    /// Save an animation in ANI format to an [`SDL_IOStream`].
    ///
    /// If `closeio` is true, `dst` will be closed before returning, whether this
    /// function succeeds or not.
    ///
    /// ## Parameters
    /// - `anim`: the animation to save.
    /// - `dst`: an [`SDL_IOStream`] from which data will be written to.
    /// - `closeio`: true to close/free the [`SDL_IOStream`] before returning, false
    ///   to leave it open.
    ///
    /// ## Return value
    /// Returns true on success or false on failure; call [`SDL_GetError()`] for more
    ///   information.
    ///
    /// ## Availability
    /// This function is available since SDL_image 3.4.0.
    ///
    /// ## See also
    /// - [`IMG_SaveAnimation`]
    /// - [`IMG_SaveAnimationTyped_IO`]
    /// - [`IMG_SaveAPNGAnimation_IO`]
    /// - [`IMG_SaveAVIFAnimation_IO`]
    /// - [`IMG_SaveGIFAnimation_IO`]
    /// - [`IMG_SaveWEBPAnimation_IO`]
    pub fn IMG_SaveANIAnimation_IO(
        anim: *mut IMG_Animation,
        dst: *mut SDL_IOStream,
        closeio: ::core::primitive::bool,
    ) -> ::core::primitive::bool;
}

unsafe extern "C" {
    /// Save an animation in APNG format to an [`SDL_IOStream`].
    ///
    /// If `closeio` is true, `dst` will be closed before returning, whether this
    /// function succeeds or not.
    ///
    /// ## Parameters
    /// - `anim`: the animation to save.
    /// - `dst`: an [`SDL_IOStream`] from which data will be written to.
    /// - `closeio`: true to close/free the [`SDL_IOStream`] before returning, false
    ///   to leave it open.
    ///
    /// ## Return value
    /// Returns true on success or false on failure; call [`SDL_GetError()`] for more
    ///   information.
    ///
    /// ## Availability
    /// This function is available since SDL_image 3.4.0.
    ///
    /// ## See also
    /// - [`IMG_SaveAnimation`]
    /// - [`IMG_SaveAnimationTyped_IO`]
    /// - [`IMG_SaveANIAnimation_IO`]
    /// - [`IMG_SaveAVIFAnimation_IO`]
    /// - [`IMG_SaveGIFAnimation_IO`]
    /// - [`IMG_SaveWEBPAnimation_IO`]
    pub fn IMG_SaveAPNGAnimation_IO(
        anim: *mut IMG_Animation,
        dst: *mut SDL_IOStream,
        closeio: ::core::primitive::bool,
    ) -> ::core::primitive::bool;
}

unsafe extern "C" {
    /// Save an animation in AVIF format to an [`SDL_IOStream`].
    ///
    /// If `closeio` is true, `dst` will be closed before returning, whether this
    /// function succeeds or not.
    ///
    /// ## Parameters
    /// - `anim`: the animation to save.
    /// - `dst`: an [`SDL_IOStream`] from which data will be written to.
    /// - `closeio`: true to close/free the [`SDL_IOStream`] before returning, false
    ///   to leave it open.
    /// - `quality`: the desired quality, ranging between 0 (lowest) and 100
    ///   (highest).
    ///
    /// ## Return value
    /// Returns true on success or false on failure; call [`SDL_GetError()`] for more
    ///   information.
    ///
    /// ## Availability
    /// This function is available since SDL_image 3.4.0.
    ///
    /// ## See also
    /// - [`IMG_SaveAnimation`]
    /// - [`IMG_SaveAnimationTyped_IO`]
    /// - [`IMG_SaveANIAnimation_IO`]
    /// - [`IMG_SaveAPNGAnimation_IO`]
    /// - [`IMG_SaveGIFAnimation_IO`]
    /// - [`IMG_SaveWEBPAnimation_IO`]
    pub fn IMG_SaveAVIFAnimation_IO(
        anim: *mut IMG_Animation,
        dst: *mut SDL_IOStream,
        closeio: ::core::primitive::bool,
        quality: ::core::ffi::c_int,
    ) -> ::core::primitive::bool;
}

unsafe extern "C" {
    /// Save an animation in GIF format to an [`SDL_IOStream`].
    ///
    /// If `closeio` is true, `dst` will be closed before returning, whether this
    /// function succeeds or not.
    ///
    /// ## Parameters
    /// - `anim`: the animation to save.
    /// - `dst`: an [`SDL_IOStream`] from which data will be written to.
    /// - `closeio`: true to close/free the [`SDL_IOStream`] before returning, false
    ///   to leave it open.
    ///
    /// ## Return value
    /// Returns true on success or false on failure; call [`SDL_GetError()`] for more
    ///   information.
    ///
    /// ## Availability
    /// This function is available since SDL_image 3.4.0.
    ///
    /// ## See also
    /// - [`IMG_SaveAnimation`]
    /// - [`IMG_SaveAnimationTyped_IO`]
    /// - [`IMG_SaveANIAnimation_IO`]
    /// - [`IMG_SaveAPNGAnimation_IO`]
    /// - [`IMG_SaveAVIFAnimation_IO`]
    /// - [`IMG_SaveWEBPAnimation_IO`]
    pub fn IMG_SaveGIFAnimation_IO(
        anim: *mut IMG_Animation,
        dst: *mut SDL_IOStream,
        closeio: ::core::primitive::bool,
    ) -> ::core::primitive::bool;
}

unsafe extern "C" {
    /// Save an animation in WEBP format to an [`SDL_IOStream`].
    ///
    /// If `closeio` is true, `dst` will be closed before returning, whether this
    /// function succeeds or not.
    ///
    /// ## Parameters
    /// - `anim`: the animation to save.
    /// - `dst`: an [`SDL_IOStream`] from which data will be written to.
    /// - `closeio`: true to close/free the [`SDL_IOStream`] before returning, false
    ///   to leave it open.
    /// - `quality`: between 0 and 100. For lossy, 0 gives the smallest size and
    ///   100 the largest. For lossless, this parameter is the amount
    ///   of effort put into the compression: 0 is the fastest but
    ///   gives larger files compared to the slowest, but best, 100.
    ///
    /// ## Return value
    /// Returns true on success or false on failure; call [`SDL_GetError()`] for more
    ///   information.
    ///
    /// ## Availability
    /// This function is available since SDL_image 3.4.0.
    ///
    /// ## See also
    /// - [`IMG_SaveAnimation`]
    /// - [`IMG_SaveAnimationTyped_IO`]
    /// - [`IMG_SaveANIAnimation_IO`]
    /// - [`IMG_SaveAPNGAnimation_IO`]
    /// - [`IMG_SaveAVIFAnimation_IO`]
    /// - [`IMG_SaveGIFAnimation_IO`]
    pub fn IMG_SaveWEBPAnimation_IO(
        anim: *mut IMG_Animation,
        dst: *mut SDL_IOStream,
        closeio: ::core::primitive::bool,
        quality: ::core::ffi::c_int,
    ) -> ::core::primitive::bool;
}

unsafe extern "C" {
    /// Create an animated cursor from an animation.
    ///
    /// ## Parameters
    /// - `anim`: an animation to use to create an animated cursor.
    /// - `hot_x`: the x position of the cursor hot spot.
    /// - `hot_y`: the y position of the cursor hot spot.
    ///
    /// ## Return value
    /// Returns the new cursor on success or NULL on failure; call [`SDL_GetError()`]
    ///   for more information.
    ///
    /// ## Availability
    /// This function is available since SDL_image 3.4.0.
    ///
    /// ## See also
    /// - [`IMG_LoadAnimation`]
    /// - [`IMG_LoadAnimation_IO`]
    /// - [`IMG_LoadAnimationTyped_IO`]
    pub fn IMG_CreateAnimatedCursor(
        anim: *mut IMG_Animation,
        hot_x: ::core::ffi::c_int,
        hot_y: ::core::ffi::c_int,
    ) -> *mut SDL_Cursor;
}

unsafe extern "C" {
    /// Dispose of an [`IMG_Animation`] and free its resources.
    ///
    /// The provided `anim` pointer is not valid once this call returns.
    ///
    /// ## Parameters
    /// - `anim`: [`IMG_Animation`] to dispose of.
    ///
    /// ## Availability
    /// This function is available since SDL_image 3.0.0.
    ///
    /// ## See also
    /// - [`IMG_LoadAnimation`]
    /// - [`IMG_LoadAnimation_IO`]
    /// - [`IMG_LoadAnimationTyped_IO`]
    /// - [`IMG_LoadANIAnimation_IO`]
    /// - [`IMG_LoadAPNGAnimation_IO`]
    /// - [`IMG_LoadAVIFAnimation_IO`]
    /// - [`IMG_LoadGIFAnimation_IO`]
    /// - [`IMG_LoadWEBPAnimation_IO`]
    pub fn IMG_FreeAnimation(anim: *mut IMG_Animation);
}

unsafe extern "C" {
    /// Create an encoder to save a series of images to a file.
    ///
    /// These animation types are currently supported:
    ///
    /// - ANI
    /// - APNG
    /// - AVIFS
    /// - GIF
    /// - WEBP
    ///
    /// The file type is determined from the file extension, e.g. "file.webp" will
    /// be encoded using WEBP.
    ///
    /// ## Parameters
    /// - `file`: the file where the animation will be saved.
    ///
    /// ## Return value
    /// Returns a new [`IMG_AnimationEncoder`], or NULL on failure; call
    ///   [`SDL_GetError()`] for more information.
    ///
    /// ## Availability
    /// This function is available since SDL_image 3.4.0.
    ///
    /// ## See also
    /// - [`IMG_CreateAnimationEncoder_IO`]
    /// - [`IMG_CreateAnimationEncoderWithProperties`]
    /// - [`IMG_AddAnimationEncoderFrame`]
    /// - [`IMG_CloseAnimationEncoder`]
    pub fn IMG_CreateAnimationEncoder(
        file: *const ::core::ffi::c_char,
    ) -> *mut IMG_AnimationEncoder;
}

unsafe extern "C" {
    /// Create an encoder to save a series of images to an IOStream.
    ///
    /// These animation types are currently supported:
    ///
    /// - ANI
    /// - APNG
    /// - AVIFS
    /// - GIF
    /// - WEBP
    ///
    /// If `closeio` is true, `dst` will be closed before returning if this
    /// function fails, or when the animation encoder is closed if this function
    /// succeeds.
    ///
    /// ## Parameters
    /// - `dst`: an [`SDL_IOStream`] that will be used to save the stream.
    /// - `closeio`: true to close the [`SDL_IOStream`] when done, false to leave it
    ///   open.
    /// - `type`: a filename extension that represent this data ("WEBP", etc).
    ///
    /// ## Return value
    /// Returns a new [`IMG_AnimationEncoder`], or NULL on failure; call
    ///   [`SDL_GetError()`] for more information.
    ///
    /// ## Availability
    /// This function is available since SDL_image 3.4.0.
    ///
    /// ## See also
    /// - [`IMG_CreateAnimationEncoder`]
    /// - [`IMG_CreateAnimationEncoderWithProperties`]
    /// - [`IMG_AddAnimationEncoderFrame`]
    /// - [`IMG_CloseAnimationEncoder`]
    pub fn IMG_CreateAnimationEncoder_IO(
        dst: *mut SDL_IOStream,
        closeio: ::core::primitive::bool,
        r#type: *const ::core::ffi::c_char,
    ) -> *mut IMG_AnimationEncoder;
}

unsafe extern "C" {
    /// Create an animation encoder with the specified properties.
    ///
    /// These animation types are currently supported:
    ///
    /// - ANI
    /// - APNG
    /// - AVIFS
    /// - GIF
    /// - WEBP
    ///
    /// These are the supported properties:
    ///
    /// - [`IMG_PROP_ANIMATION_ENCODER_CREATE_FILENAME_STRING`]\: the file to save, if
    ///   an [`SDL_IOStream`] isn't being used. This is required if
    ///   [`IMG_PROP_ANIMATION_ENCODER_CREATE_IOSTREAM_POINTER`] isn't set.
    /// - [`IMG_PROP_ANIMATION_ENCODER_CREATE_IOSTREAM_POINTER`]\: an [`SDL_IOStream`]
    ///   that will be used to save the stream. This should not be closed until the
    ///   animation encoder is closed. This is required if
    ///   [`IMG_PROP_ANIMATION_ENCODER_CREATE_FILENAME_STRING`] isn't set.
    /// - [`IMG_PROP_ANIMATION_ENCODER_CREATE_IOSTREAM_AUTOCLOSE_BOOLEAN`]\: true if
    ///   closing the animation encoder should also close the associated
    ///   [`SDL_IOStream`].
    /// - [`IMG_PROP_ANIMATION_ENCODER_CREATE_TYPE_STRING`]\: the output file type,
    ///   e.g. "webp", defaults to the file extension if
    ///   [`IMG_PROP_ANIMATION_ENCODER_CREATE_FILENAME_STRING`] is set.
    /// - [`IMG_PROP_ANIMATION_ENCODER_CREATE_QUALITY_NUMBER`]\: the compression
    ///   quality, in the range of 0 to 100. The higher the number, the higher the
    ///   quality and file size. This defaults to a balanced value for compression
    ///   and quality.
    /// - [`IMG_PROP_ANIMATION_ENCODER_CREATE_TIMEBASE_NUMERATOR_NUMBER`]\: the
    ///   numerator of the fraction used to multiply the pts to convert it to
    ///   seconds. This defaults to 1.
    /// - [`IMG_PROP_ANIMATION_ENCODER_CREATE_TIMEBASE_DENOMINATOR_NUMBER`]\: the
    ///   denominator of the fraction used to multiply the pts to convert it to
    ///   seconds. This defaults to 1000.
    ///
    /// ## Parameters
    /// - `props`: the properties of the animation encoder.
    ///
    /// ## Return value
    /// Returns a new [`IMG_AnimationEncoder`], or NULL on failure; call
    ///   [`SDL_GetError()`] for more information.
    ///
    /// ## Availability
    /// This function is available since SDL_image 3.4.0.
    ///
    /// ## See also
    /// - [`IMG_CreateAnimationEncoder`]
    /// - [`IMG_CreateAnimationEncoder_IO`]
    /// - [`IMG_AddAnimationEncoderFrame`]
    /// - [`IMG_CloseAnimationEncoder`]
    pub fn IMG_CreateAnimationEncoderWithProperties(
        props: SDL_PropertiesID,
    ) -> *mut IMG_AnimationEncoder;
}

pub const IMG_PROP_ANIMATION_ENCODER_CREATE_FILENAME_STRING: *const ::core::ffi::c_char =
    c"SDL_image.animation_encoder.create.filename".as_ptr();

pub const IMG_PROP_ANIMATION_ENCODER_CREATE_IOSTREAM_POINTER: *const ::core::ffi::c_char =
    c"SDL_image.animation_encoder.create.iostream".as_ptr();

pub const IMG_PROP_ANIMATION_ENCODER_CREATE_IOSTREAM_AUTOCLOSE_BOOLEAN: *const ::core::ffi::c_char =
    c"SDL_image.animation_encoder.create.iostream.autoclose".as_ptr();

pub const IMG_PROP_ANIMATION_ENCODER_CREATE_TYPE_STRING: *const ::core::ffi::c_char =
    c"SDL_image.animation_encoder.create.type".as_ptr();

pub const IMG_PROP_ANIMATION_ENCODER_CREATE_QUALITY_NUMBER: *const ::core::ffi::c_char =
    c"SDL_image.animation_encoder.create.quality".as_ptr();

pub const IMG_PROP_ANIMATION_ENCODER_CREATE_TIMEBASE_NUMERATOR_NUMBER: *const ::core::ffi::c_char =
    c"SDL_image.animation_encoder.create.timebase.numerator".as_ptr();

pub const IMG_PROP_ANIMATION_ENCODER_CREATE_TIMEBASE_DENOMINATOR_NUMBER:
    *const ::core::ffi::c_char =
    c"SDL_image.animation_encoder.create.timebase.denominator".as_ptr();

pub const IMG_PROP_ANIMATION_ENCODER_CREATE_AVIF_MAX_THREADS_NUMBER: *const ::core::ffi::c_char =
    c"SDL_image.animation_encoder.create.avif.max_threads".as_ptr();

pub const IMG_PROP_ANIMATION_ENCODER_CREATE_AVIF_KEYFRAME_INTERVAL_NUMBER:
    *const ::core::ffi::c_char =
    c"SDL_image.animation_encoder.create.avif.keyframe_interval".as_ptr();

pub const IMG_PROP_ANIMATION_ENCODER_CREATE_GIF_USE_LUT_BOOLEAN: *const ::core::ffi::c_char =
    c"SDL_image.animation_encoder.create.gif.use_lut".as_ptr();

unsafe extern "C" {
    /// Add a frame to an animation encoder.
    ///
    /// ## Parameters
    /// - `encoder`: the receiving images.
    /// - `surface`: the surface to add as the next frame in the animation.
    /// - `duration`: the duration of the frame, usually in milliseconds but can
    ///   be other units if the
    ///   [`IMG_PROP_ANIMATION_ENCODER_CREATE_TIMEBASE_DENOMINATOR_NUMBER`]
    ///   property is set when creating the encoder.
    ///
    /// ## Return value
    /// Returns true on success or false on failure; call [`SDL_GetError()`] for more
    ///   information.
    ///
    /// ## Availability
    /// This function is available since SDL_image 3.4.0.
    ///
    /// ## See also
    /// - [`IMG_CreateAnimationEncoder`]
    /// - [`IMG_CreateAnimationEncoder_IO`]
    /// - [`IMG_CreateAnimationEncoderWithProperties`]
    /// - [`IMG_CloseAnimationEncoder`]
    pub fn IMG_AddAnimationEncoderFrame(
        encoder: *mut IMG_AnimationEncoder,
        surface: *mut SDL_Surface,
        duration: Uint64,
    ) -> ::core::primitive::bool;
}

unsafe extern "C" {
    /// Close an animation encoder, finishing any encoding.
    ///
    /// Calling this function frees the animation encoder, and returns the final
    /// status of the encoding process.
    ///
    /// ## Parameters
    /// - `encoder`: the encoder to close.
    ///
    /// ## Return value
    /// Returns true on success or false on failure; call [`SDL_GetError()`] for more
    ///   information.
    ///
    /// ## Availability
    /// This function is available since SDL_image 3.4.0.
    ///
    /// ## See also
    /// - [`IMG_CreateAnimationEncoder`]
    /// - [`IMG_CreateAnimationEncoder_IO`]
    /// - [`IMG_CreateAnimationEncoderWithProperties`]
    pub fn IMG_CloseAnimationEncoder(encoder: *mut IMG_AnimationEncoder)
    -> ::core::primitive::bool;
}

/// An enum representing the status of an animation decoder.
///
/// ## Availability
/// This enum is available since SDL_image 3.4.0.
///
/// ## Known values (`sdl3-sys`)
/// | Associated constant | Global constant | Description |
/// | ------------------- | --------------- | ----------- |
/// | [`INVALID`](IMG_AnimationDecoderStatus::INVALID) | [`IMG_DECODER_STATUS_INVALID`] | The decoder is invalid |
/// | [`OK`](IMG_AnimationDecoderStatus::OK) | [`IMG_DECODER_STATUS_OK`] | The decoder is ready to decode the next frame |
/// | [`FAILED`](IMG_AnimationDecoderStatus::FAILED) | [`IMG_DECODER_STATUS_FAILED`] | The decoder failed to decode a frame, call [`SDL_GetError()`] for more information. |
/// | [`COMPLETE`](IMG_AnimationDecoderStatus::COMPLETE) | [`IMG_DECODER_STATUS_COMPLETE`] | No more frames available |
#[repr(transparent)]
#[derive(Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct IMG_AnimationDecoderStatus(pub ::core::ffi::c_int);

impl ::core::cmp::PartialEq<::core::ffi::c_int> for IMG_AnimationDecoderStatus {
    #[inline(always)]
    fn eq(&self, other: &::core::ffi::c_int) -> bool {
        &self.0 == other
    }
}

impl ::core::cmp::PartialEq<IMG_AnimationDecoderStatus> for ::core::ffi::c_int {
    #[inline(always)]
    fn eq(&self, other: &IMG_AnimationDecoderStatus) -> bool {
        self == &other.0
    }
}

impl From<IMG_AnimationDecoderStatus> for ::core::ffi::c_int {
    #[inline(always)]
    fn from(value: IMG_AnimationDecoderStatus) -> Self {
        value.0
    }
}

#[cfg(feature = "debug-impls")]
impl ::core::fmt::Debug for IMG_AnimationDecoderStatus {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        #[allow(unreachable_patterns)]
        f.write_str(match *self {
            Self::INVALID => "IMG_DECODER_STATUS_INVALID",
            Self::OK => "IMG_DECODER_STATUS_OK",
            Self::FAILED => "IMG_DECODER_STATUS_FAILED",
            Self::COMPLETE => "IMG_DECODER_STATUS_COMPLETE",

            _ => return write!(f, "IMG_AnimationDecoderStatus({})", self.0),
        })
    }
}

impl IMG_AnimationDecoderStatus {
    /// The decoder is invalid
    pub const INVALID: Self = Self((-1_i32 as ::core::ffi::c_int));
    /// The decoder is ready to decode the next frame
    pub const OK: Self = Self((0_i32 as ::core::ffi::c_int));
    /// The decoder failed to decode a frame, call [`SDL_GetError()`] for more information.
    pub const FAILED: Self = Self((1_i32 as ::core::ffi::c_int));
    /// No more frames available
    pub const COMPLETE: Self = Self((2_i32 as ::core::ffi::c_int));
}

/// The decoder is invalid
pub const IMG_DECODER_STATUS_INVALID: IMG_AnimationDecoderStatus =
    IMG_AnimationDecoderStatus::INVALID;
/// The decoder is ready to decode the next frame
pub const IMG_DECODER_STATUS_OK: IMG_AnimationDecoderStatus = IMG_AnimationDecoderStatus::OK;
/// The decoder failed to decode a frame, call [`SDL_GetError()`] for more information.
pub const IMG_DECODER_STATUS_FAILED: IMG_AnimationDecoderStatus =
    IMG_AnimationDecoderStatus::FAILED;
/// No more frames available
pub const IMG_DECODER_STATUS_COMPLETE: IMG_AnimationDecoderStatus =
    IMG_AnimationDecoderStatus::COMPLETE;

impl IMG_AnimationDecoderStatus {
    /// Initialize a `IMG_AnimationDecoderStatus` from a raw value.
    #[inline(always)]
    pub const fn new(value: ::core::ffi::c_int) -> Self {
        Self(value)
    }
}

impl IMG_AnimationDecoderStatus {
    /// Get a copy of the inner raw value.
    #[inline(always)]
    pub const fn value(&self) -> ::core::ffi::c_int {
        self.0
    }
}

#[cfg(feature = "metadata")]
impl sdl3_sys::metadata::GroupMetadata for IMG_AnimationDecoderStatus {
    const GROUP_METADATA: &'static sdl3_sys::metadata::Group =
        &crate::metadata::image::METADATA_IMG_AnimationDecoderStatus;
}

unsafe extern "C" {
    /// Create a decoder to read a series of images from a file.
    ///
    /// These animation types are currently supported:
    ///
    /// - ANI
    /// - APNG
    /// - AVIFS
    /// - GIF
    /// - WEBP
    ///
    /// The file type is determined from the file extension, e.g. "file.webp" will
    /// be decoded using WEBP.
    ///
    /// ## Parameters
    /// - `file`: the file containing a series of images.
    ///
    /// ## Return value
    /// Returns a new [`IMG_AnimationDecoder`], or NULL on failure; call
    ///   [`SDL_GetError()`] for more information.
    ///
    /// ## Availability
    /// This function is available since SDL_image 3.4.0.
    ///
    /// ## See also
    /// - [`IMG_CreateAnimationDecoder_IO`]
    /// - [`IMG_CreateAnimationDecoderWithProperties`]
    /// - [`IMG_GetAnimationDecoderFrame`]
    /// - [`IMG_ResetAnimationDecoder`]
    /// - [`IMG_CloseAnimationDecoder`]
    pub fn IMG_CreateAnimationDecoder(
        file: *const ::core::ffi::c_char,
    ) -> *mut IMG_AnimationDecoder;
}

unsafe extern "C" {
    /// Create a decoder to read a series of images from an IOStream.
    ///
    /// These animation types are currently supported:
    ///
    /// - ANI
    /// - APNG
    /// - AVIFS
    /// - GIF
    /// - WEBP
    ///
    /// If `closeio` is true, `src` will be closed before returning if this
    /// function fails, or when the animation decoder is closed if this function
    /// succeeds.
    ///
    /// ## Parameters
    /// - `src`: an [`SDL_IOStream`] containing a series of images.
    /// - `closeio`: true to close the [`SDL_IOStream`] when done, false to leave it
    ///   open.
    /// - `type`: a filename extension that represent this data ("WEBP", etc).
    ///
    /// ## Return value
    /// Returns a new [`IMG_AnimationDecoder`], or NULL on failure; call
    ///   [`SDL_GetError()`] for more information.
    ///
    /// ## Availability
    /// This function is available since SDL_image 3.4.0.
    ///
    /// ## See also
    /// - [`IMG_CreateAnimationDecoder`]
    /// - [`IMG_CreateAnimationDecoderWithProperties`]
    /// - [`IMG_GetAnimationDecoderFrame`]
    /// - [`IMG_ResetAnimationDecoder`]
    /// - [`IMG_CloseAnimationDecoder`]
    pub fn IMG_CreateAnimationDecoder_IO(
        src: *mut SDL_IOStream,
        closeio: ::core::primitive::bool,
        r#type: *const ::core::ffi::c_char,
    ) -> *mut IMG_AnimationDecoder;
}

unsafe extern "C" {
    /// Create an animation decoder with the specified properties.
    ///
    /// These animation types are currently supported:
    ///
    /// - ANI
    /// - APNG
    /// - AVIFS
    /// - GIF
    /// - WEBP
    ///
    /// These are the supported properties:
    ///
    /// - [`IMG_PROP_ANIMATION_DECODER_CREATE_FILENAME_STRING`]\: the file to load, if
    ///   an [`SDL_IOStream`] isn't being used. This is required if
    ///   [`IMG_PROP_ANIMATION_DECODER_CREATE_IOSTREAM_POINTER`] isn't set.
    /// - [`IMG_PROP_ANIMATION_DECODER_CREATE_IOSTREAM_POINTER`]\: an [`SDL_IOStream`]
    ///   containing a series of images. This should not be closed until the
    ///   animation decoder is closed. This is required if
    ///   [`IMG_PROP_ANIMATION_DECODER_CREATE_FILENAME_STRING`] isn't set.
    /// - [`IMG_PROP_ANIMATION_DECODER_CREATE_IOSTREAM_AUTOCLOSE_BOOLEAN`]\: true if
    ///   closing the animation decoder should also close the associated
    ///   [`SDL_IOStream`].
    /// - [`IMG_PROP_ANIMATION_DECODER_CREATE_TYPE_STRING`]\: the input file type,
    ///   e.g. "webp", defaults to the file extension if
    ///   [`IMG_PROP_ANIMATION_DECODER_CREATE_FILENAME_STRING`] is set.
    ///
    /// ## Parameters
    /// - `props`: the properties of the animation decoder.
    ///
    /// ## Return value
    /// Returns a new [`IMG_AnimationDecoder`], or NULL on failure; call
    ///   [`SDL_GetError()`] for more information.
    ///
    /// ## Availability
    /// This function is available since SDL_image 3.4.0.
    ///
    /// ## See also
    /// - [`IMG_CreateAnimationDecoder`]
    /// - [`IMG_CreateAnimationDecoder_IO`]
    /// - [`IMG_GetAnimationDecoderFrame`]
    /// - [`IMG_ResetAnimationDecoder`]
    /// - [`IMG_CloseAnimationDecoder`]
    pub fn IMG_CreateAnimationDecoderWithProperties(
        props: SDL_PropertiesID,
    ) -> *mut IMG_AnimationDecoder;
}

pub const IMG_PROP_ANIMATION_DECODER_CREATE_FILENAME_STRING: *const ::core::ffi::c_char =
    c"SDL_image.animation_decoder.create.filename".as_ptr();

pub const IMG_PROP_ANIMATION_DECODER_CREATE_IOSTREAM_POINTER: *const ::core::ffi::c_char =
    c"SDL_image.animation_decoder.create.iostream".as_ptr();

pub const IMG_PROP_ANIMATION_DECODER_CREATE_IOSTREAM_AUTOCLOSE_BOOLEAN: *const ::core::ffi::c_char =
    c"SDL_image.animation_decoder.create.iostream.autoclose".as_ptr();

pub const IMG_PROP_ANIMATION_DECODER_CREATE_TYPE_STRING: *const ::core::ffi::c_char =
    c"SDL_image.animation_decoder.create.type".as_ptr();

pub const IMG_PROP_ANIMATION_DECODER_CREATE_TIMEBASE_NUMERATOR_NUMBER: *const ::core::ffi::c_char =
    c"SDL_image.animation_decoder.create.timebase.numerator".as_ptr();

pub const IMG_PROP_ANIMATION_DECODER_CREATE_TIMEBASE_DENOMINATOR_NUMBER:
    *const ::core::ffi::c_char =
    c"SDL_image.animation_decoder.create.timebase.denominator".as_ptr();

pub const IMG_PROP_ANIMATION_DECODER_CREATE_AVIF_MAX_THREADS_NUMBER: *const ::core::ffi::c_char =
    c"SDL_image.animation_decoder.create.avif.max_threads".as_ptr();

pub const IMG_PROP_ANIMATION_DECODER_CREATE_AVIF_ALLOW_INCREMENTAL_BOOLEAN:
    *const ::core::ffi::c_char =
    c"SDL_image.animation_decoder.create.avif.allow_incremental".as_ptr();

pub const IMG_PROP_ANIMATION_DECODER_CREATE_AVIF_ALLOW_PROGRESSIVE_BOOLEAN:
    *const ::core::ffi::c_char =
    c"SDL_image.animation_decoder.create.avif.allow_progressive".as_ptr();

pub const IMG_PROP_ANIMATION_DECODER_CREATE_GIF_TRANSPARENT_COLOR_INDEX_NUMBER:
    *const ::core::ffi::c_char =
    c"SDL_image.animation_encoder.create.gif.transparent_color_index".as_ptr();

pub const IMG_PROP_ANIMATION_DECODER_CREATE_GIF_NUM_COLORS_NUMBER: *const ::core::ffi::c_char =
    c"SDL_image.animation_encoder.create.gif.num_colors".as_ptr();

unsafe extern "C" {
    /// Get the properties of an animation decoder.
    ///
    /// This function returns the properties of the animation decoder, which holds
    /// information about the underlying image such as description, copyright text
    /// and loop count.
    ///
    /// ## Parameters
    /// - `decoder`: the animation decoder.
    ///
    /// ## Return value
    /// Returns the properties ID of the animation decoder, or 0 if there are no
    ///   properties; call [`SDL_GetError()`] for more information.
    ///
    /// ## Availability
    /// This function is available since SDL_image 3.4.0.
    ///
    /// ## See also
    /// - [`IMG_CreateAnimationDecoder`]
    /// - [`IMG_CreateAnimationDecoder_IO`]
    /// - [`IMG_CreateAnimationDecoderWithProperties`]
    pub fn IMG_GetAnimationDecoderProperties(
        decoder: *mut IMG_AnimationDecoder,
    ) -> SDL_PropertiesID;
}

pub const IMG_PROP_METADATA_IGNORE_PROPS_BOOLEAN: *const ::core::ffi::c_char =
    c"SDL_image.metadata.ignore_props".as_ptr();

pub const IMG_PROP_METADATA_DESCRIPTION_STRING: *const ::core::ffi::c_char =
    c"SDL_image.metadata.description".as_ptr();

pub const IMG_PROP_METADATA_COPYRIGHT_STRING: *const ::core::ffi::c_char =
    c"SDL_image.metadata.copyright".as_ptr();

pub const IMG_PROP_METADATA_TITLE_STRING: *const ::core::ffi::c_char =
    c"SDL_image.metadata.title".as_ptr();

pub const IMG_PROP_METADATA_AUTHOR_STRING: *const ::core::ffi::c_char =
    c"SDL_image.metadata.author".as_ptr();

pub const IMG_PROP_METADATA_CREATION_TIME_STRING: *const ::core::ffi::c_char =
    c"SDL_image.metadata.creation_time".as_ptr();

pub const IMG_PROP_METADATA_FRAME_COUNT_NUMBER: *const ::core::ffi::c_char =
    c"SDL_image.metadata.frame_count".as_ptr();

pub const IMG_PROP_METADATA_LOOP_COUNT_NUMBER: *const ::core::ffi::c_char =
    c"SDL_image.metadata.loop_count".as_ptr();

unsafe extern "C" {
    /// Get the next frame in an animation decoder.
    ///
    /// This function decodes the next frame in the animation decoder, returning it
    /// as an [`SDL_Surface`]. The returned surface should be freed with
    /// [`SDL_FreeSurface()`] when no longer needed.
    ///
    /// If the animation decoder has no more frames or an error occurred while
    /// decoding the frame, this function returns false. In that case, please call
    /// [`SDL_GetError()`] for more information. If [`SDL_GetError()`] returns an empty
    /// string, that means there are no more available frames. If [`SDL_GetError()`]
    /// returns a valid string, that means the decoding failed.
    ///
    /// ## Parameters
    /// - `decoder`: the animation decoder.
    /// - `frame`: a pointer filled in with the [`SDL_Surface`] for the next frame in
    ///   the animation.
    /// - `duration`: the duration of the frame, usually in milliseconds but can
    ///   be other units if the
    ///   [`IMG_PROP_ANIMATION_DECODER_CREATE_TIMEBASE_DENOMINATOR_NUMBER`]
    ///   property is set when creating the decoder.
    ///
    /// ## Return value
    /// Returns true on success or false on failure and when no more frames are
    ///   available; call [`IMG_GetAnimationDecoderStatus()`] or [`SDL_GetError()`]
    ///   for more information.
    ///
    /// ## Availability
    /// This function is available since SDL_image 3.4.0.
    ///
    /// ## See also
    /// - [`IMG_CreateAnimationDecoder`]
    /// - [`IMG_CreateAnimationDecoder_IO`]
    /// - [`IMG_CreateAnimationDecoderWithProperties`]
    /// - [`IMG_GetAnimationDecoderStatus`]
    /// - [`IMG_ResetAnimationDecoder`]
    /// - [`IMG_CloseAnimationDecoder`]
    pub fn IMG_GetAnimationDecoderFrame(
        decoder: *mut IMG_AnimationDecoder,
        frame: *mut *mut SDL_Surface,
        duration: *mut Uint64,
    ) -> ::core::primitive::bool;
}

unsafe extern "C" {
    /// Get the decoder status indicating the current state of the decoder.
    ///
    /// ## Parameters
    /// - `decoder`: the decoder to get the status of.
    ///
    /// ## Return value
    /// Returns the status of the underlying decoder, or
    ///   [`IMG_DECODER_STATUS_INVALID`] if the given decoder is invalid.
    ///
    /// ## Availability
    /// This function is available since SDL_image 3.4.0.
    ///
    /// ## See also
    /// - [`IMG_GetAnimationDecoderFrame`]
    pub fn IMG_GetAnimationDecoderStatus(
        decoder: *mut IMG_AnimationDecoder,
    ) -> IMG_AnimationDecoderStatus;
}

unsafe extern "C" {
    /// Reset an animation decoder.
    ///
    /// Calling this function resets the animation decoder, allowing it to start
    /// from the beginning again. This is useful if you want to decode the frame
    /// sequence again without creating a new decoder.
    ///
    /// ## Parameters
    /// - `decoder`: the decoder to reset.
    ///
    /// ## Return value
    /// Returns true on success or false on failure; call [`SDL_GetError()`] for more
    ///   information.
    ///
    /// ## Availability
    /// This function is available since SDL_image 3.4.0.
    ///
    /// ## See also
    /// - [`IMG_CreateAnimationDecoder`]
    /// - [`IMG_CreateAnimationDecoder_IO`]
    /// - [`IMG_CreateAnimationDecoderWithProperties`]
    /// - [`IMG_GetAnimationDecoderFrame`]
    /// - [`IMG_CloseAnimationDecoder`]
    pub fn IMG_ResetAnimationDecoder(decoder: *mut IMG_AnimationDecoder)
    -> ::core::primitive::bool;
}

unsafe extern "C" {
    /// Close an animation decoder, finishing any decoding.
    ///
    /// Calling this function frees the animation decoder, and returns the final
    /// status of the decoding process.
    ///
    /// ## Parameters
    /// - `decoder`: the decoder to close.
    ///
    /// ## Return value
    /// Returns true on success or false on failure; call [`SDL_GetError()`] for more
    ///   information.
    ///
    /// ## Availability
    /// This function is available since SDL_image 3.4.0.
    ///
    /// ## See also
    /// - [`IMG_CreateAnimationDecoder`]
    /// - [`IMG_CreateAnimationDecoder_IO`]
    /// - [`IMG_CreateAnimationDecoderWithProperties`]
    pub fn IMG_CloseAnimationDecoder(decoder: *mut IMG_AnimationDecoder)
    -> ::core::primitive::bool;
}

/// * An object representing animation decoder.
#[repr(C)]
pub struct IMG_AnimationDecoder {
    _opaque: [::core::primitive::u8; 0],
}

/// * An object representing the encoder context.
#[repr(C)]
pub struct IMG_AnimationEncoder {
    _opaque: [::core::primitive::u8; 0],
}

#[cfg(doc)]
use crate::everything::*;
