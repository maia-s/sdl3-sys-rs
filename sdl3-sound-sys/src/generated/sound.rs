//! The latest version of SDL_sound can be found at:
//!
//! <https://icculus.org/SDL_sound/>
//!
//! Note that SDL_sound is _not_ an official part of the SDL project, despite
//! its name.
//!
//! The basic gist of SDL_sound is that you use an [`SDL_IOStream`] to get sound
//! data into this library, and SDL_sound will take that data, in one of
//! several popular formats, and decode it into raw waveform data in the format
//! of your choice. This gives you a nice abstraction for getting sound into
//! your game or application; just feed it to SDL_sound, and it will handle
//! decoding and converting, so you can just pass it to your SDL audio callback
//! (or whatever). Since it gets data from an [`SDL_IOStream`], you can get the
//! initial sound data from any number of sources: file, memory buffer, network
//! connection, etc.
//!
//! As the name implies, this library depends on SDL: Simple Directmedia Layer,
//! which is a powerful, free, and cross-platform multimedia library. It can be
//! found at <https://www.libsdl.org/>
//!
//! Support is in place for the following sound formats:
//!
//! - .WAV (Microsoft WAVfile RIFF data, internal.)
//! - .VOC (Creative Labs' Voice format, internal.)
//! - .MP3 (MPEG-1 Layer 3 support, via libmpg123.)
//! - .MID (MIDI music converted to Waveform data, internal.)
//! - .MOD (MOD files, via internal copy of libModPlug.)
//! - .OGG (Ogg files, via internal copy of stb_vorbis.)
//! - .SHN (Shorten files, internal.)
//! - .RAW (Raw sound data in any format, internal.)
//! - .AU (Sun's Audio format, internal.)
//! - .AIFF (Audio Interchange format, internal.)
//! - .FLAC (Lossless audio compression, via libFLAC.)

use sdl3_sys::everything::*;

/// The current major version of the SDL_sound headers.
///
/// If this were SDL_sound version 3.2.1, this value would be 3.
///
/// ## Availability
/// This macro is available since SDL_sound 3.0.0.
pub const SDL_SOUND_MAJOR_VERSION: ::core::primitive::i32 = 3;

/// The current minor version of the SDL_sound headers.
///
/// If this were SDL_sound version 3.2.1, this value would be 2.
///
/// ## Availability
/// This macro is available since SDL_sound 3.0.0.
pub const SDL_SOUND_MINOR_VERSION: ::core::primitive::i32 = 2;

/// The current micro (or patchlevel) version of the SDL_sound headers.
///
/// If this were SDL_sound version 3.2.1, this value would be 1.
///
/// ## Availability
/// This macro is available since SDL_sound 3.0.0.
pub const SDL_SOUND_MICRO_VERSION: ::core::primitive::i32 = 0;

/// This is the version number macro for the current SDL_sound version.
///
/// ## Availability
/// This macro is available since SDL_sound 3.0.0.
///
/// ## See also
/// - [`Sound_Version`]
pub const SDL_SOUND_VERSION: ::core::primitive::i32 = SDL_VERSIONNUM(
    SDL_SOUND_MAJOR_VERSION,
    SDL_SOUND_MINOR_VERSION,
    SDL_SOUND_MICRO_VERSION,
);

/// This macro will evaluate to true if compiled with SDL_sound at least X.Y.Z.
///
/// ## Availability
/// This macro is available since SDL_sound 3.0.0.
#[inline(always)]
pub const fn SDL_SOUND_VERSION_ATLEAST(
    X: ::core::primitive::i32,
    Y: ::core::primitive::i32,
    Z: ::core::primitive::i32,
) -> ::core::primitive::bool {
    (((SDL_SOUND_MAJOR_VERSION >= X)
        && ((SDL_SOUND_MAJOR_VERSION > X) || (SDL_SOUND_MINOR_VERSION >= Y)))
        && (((SDL_SOUND_MAJOR_VERSION > X) || (SDL_SOUND_MINOR_VERSION > Y))
            || (SDL_SOUND_MICRO_VERSION >= Z)))
}

unsafe extern "C" {
    /// This function gets the version of the dynamically linked SDL_sound library.
    ///
    /// ## Return value
    /// Returns SDL_sound version.
    ///
    /// ## Thread safety
    /// It is safe to call this function from any thread.
    ///
    /// ## Availability
    /// This function is available since SDL_sound 3.0.0.
    pub fn Sound_Version() -> ::core::ffi::c_int;
}

/// Flags that are used in a [`Sound_Sample`] to show various states.
///
/// ## Availability
/// This enum is available since SDL_sound 1.0.0.
///
/// ## See also
/// - [`Sound_SampleNew`]
/// - [`Sound_SampleNewFromFile`]
/// - [`Sound_SampleDecode`]
/// - [`Sound_SampleDecodeAll`]
/// - [`Sound_SampleSeek`]
///
/// ## Known values (`sdl3-sys`)
/// | Associated constant | Global constant | Description |
/// | ------------------- | --------------- | ----------- |
/// | [`NONE`](Sound_SampleFlags::NONE) | [`SOUND_SAMPLEFLAG_NONE`] | No special attributes. |
/// | [`CANSEEK`](Sound_SampleFlags::CANSEEK) | [`SOUND_SAMPLEFLAG_CANSEEK`] | Sample can seek to arbitrary points. |
/// | [`EOF`](Sound_SampleFlags::EOF) | [`SOUND_SAMPLEFLAG_EOF`] | End of input stream. |
/// | [`ERROR`](Sound_SampleFlags::ERROR) | [`SOUND_SAMPLEFLAG_ERROR`] | Unrecoverable error. |
/// | [`EAGAIN`](Sound_SampleFlags::EAGAIN) | [`SOUND_SAMPLEFLAG_EAGAIN`] | Function would block, or temp error. |
#[repr(transparent)]
#[derive(Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Sound_SampleFlags(pub ::core::ffi::c_int);

impl ::core::cmp::PartialEq<::core::ffi::c_int> for Sound_SampleFlags {
    #[inline(always)]
    fn eq(&self, other: &::core::ffi::c_int) -> bool {
        &self.0 == other
    }
}

impl ::core::cmp::PartialEq<Sound_SampleFlags> for ::core::ffi::c_int {
    #[inline(always)]
    fn eq(&self, other: &Sound_SampleFlags) -> bool {
        self == &other.0
    }
}

impl From<Sound_SampleFlags> for ::core::ffi::c_int {
    #[inline(always)]
    fn from(value: Sound_SampleFlags) -> Self {
        value.0
    }
}

#[cfg(feature = "debug-impls")]
impl ::core::fmt::Debug for Sound_SampleFlags {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        #[allow(unreachable_patterns)]
        f.write_str(match *self {
            Self::NONE => "SOUND_SAMPLEFLAG_NONE",
            Self::CANSEEK => "SOUND_SAMPLEFLAG_CANSEEK",
            Self::EOF => "SOUND_SAMPLEFLAG_EOF",
            Self::ERROR => "SOUND_SAMPLEFLAG_ERROR",
            Self::EAGAIN => "SOUND_SAMPLEFLAG_EAGAIN",

            _ => return write!(f, "Sound_SampleFlags({})", self.0),
        })
    }
}

impl Sound_SampleFlags {
    /// No special attributes.
    pub const NONE: Self = Self((0 as ::core::ffi::c_int));
    /// Sample can seek to arbitrary points.
    pub const CANSEEK: Self = Self((1 as ::core::ffi::c_int));
    /// End of input stream.
    pub const EOF: Self = Self((536870912_i32 as ::core::ffi::c_int));
    /// Unrecoverable error.
    pub const ERROR: Self = Self((1073741824_i32 as ::core::ffi::c_int));
    /// Function would block, or temp error.
    pub const EAGAIN: Self = Self((-2147483648_i32 as ::core::ffi::c_int));
}

/// No special attributes.
pub const SOUND_SAMPLEFLAG_NONE: Sound_SampleFlags = Sound_SampleFlags::NONE;
/// Sample can seek to arbitrary points.
pub const SOUND_SAMPLEFLAG_CANSEEK: Sound_SampleFlags = Sound_SampleFlags::CANSEEK;
/// End of input stream.
pub const SOUND_SAMPLEFLAG_EOF: Sound_SampleFlags = Sound_SampleFlags::EOF;
/// Unrecoverable error.
pub const SOUND_SAMPLEFLAG_ERROR: Sound_SampleFlags = Sound_SampleFlags::ERROR;
/// Function would block, or temp error.
pub const SOUND_SAMPLEFLAG_EAGAIN: Sound_SampleFlags = Sound_SampleFlags::EAGAIN;

impl Sound_SampleFlags {
    /// Initialize a `Sound_SampleFlags` from a raw value.
    #[inline(always)]
    pub const fn new(value: ::core::ffi::c_int) -> Self {
        Self(value)
    }
}

impl Sound_SampleFlags {
    /// Get a copy of the inner raw value.
    #[inline(always)]
    pub const fn value(&self) -> ::core::ffi::c_int {
        self.0
    }
}

#[cfg(feature = "metadata")]
impl sdl3_sys::metadata::GroupMetadata for Sound_SampleFlags {
    const GROUP_METADATA: &'static sdl3_sys::metadata::Group =
        &crate::metadata::sound::METADATA_Sound_SampleFlags;
}

/// [`Sound_DecoderInfo`] Information about available sound decoders.
///
/// Each decoder sets up one of these structs, which can be retrieved via the
/// [`Sound_AvailableDecoders()`] function. EVERY FIELD IN THIS IS READ-ONLY.
///
/// The extensions field is a NULL-terminated list of ASCIZ strings. You should
/// read it like this:
///
/// ```c
/// for (const char **ext = info->extensions; *ext != NULL; ext++) {
///     printf("   File extension \"%s\"\n", *ext);
/// }
/// ```
///
/// ## Availability
/// This struct is available since SDL_sound 1.0.0.
///
/// ## See also
/// - [`Sound_AvailableDecoders`]
#[repr(C)]
#[cfg_attr(feature = "debug-impls", derive(Debug))]
pub struct Sound_DecoderInfo {
    /// File extensions, list ends with NULL.
    pub extensions: *mut *const ::core::ffi::c_char,
    /// Human readable description of decoder.
    pub description: *const ::core::ffi::c_char,
    /// "Name Of Author <email@emailhost.dom>"
    pub author: *const ::core::ffi::c_char,
    /// URL specific to this decoder.
    pub url: *const ::core::ffi::c_char,
}

impl ::core::default::Default for Sound_DecoderInfo {
    /// Initialize all fields to zero
    #[inline(always)]
    fn default() -> Self {
        unsafe { ::core::mem::MaybeUninit::<Self>::zeroed().assume_init() }
    }
}

/// Represents sound data in the process of being decoded.
///
/// The [`Sound_Sample`] structure is the heart of SDL_sound. This holds
/// information about a source of sound data as it is being decoded. EVERY
/// FIELD IN THIS IS READ-ONLY. Please use the API functions to change them.
///
/// ## Availability
/// This struct is available since SDL_sound 1.0.0.
#[repr(C)]
#[cfg_attr(feature = "debug-impls", derive(Debug))]
pub struct Sound_Sample {
    /// Internal use only. Don't touch.
    pub opaque: *mut ::core::ffi::c_void,
    /// Decoder used for this sample.
    pub decoder: *const Sound_DecoderInfo,
    /// Desired audio format for conversion.
    pub desired: SDL_AudioSpec,
    /// Actual audio format of sample.
    pub actual: SDL_AudioSpec,
    /// Decoded sound data lands in here.
    pub buffer: *mut ::core::ffi::c_void,
    /// Current size of (buffer), in bytes (Uint8).
    pub buffer_size: Uint32,
    /// Flags relating to this sample.
    pub flags: Sound_SampleFlags,
}

impl ::core::default::Default for Sound_Sample {
    /// Initialize all fields to zero
    #[inline(always)]
    fn default() -> Self {
        unsafe { ::core::mem::MaybeUninit::<Self>::zeroed().assume_init() }
    }
}

unsafe extern "C" {
    /// Initialize SDL_sound.
    ///
    /// This must be called before any other SDL_sound function (except perhaps
    /// [`Sound_Version()`]). You should call [`SDL_Init()`] before calling this.
    /// [`Sound_Init()`] will attempt to call SDL_Init([`SDL_INIT_AUDIO`]), just in case.
    /// This is a safe behaviour, but it may not configure SDL to your liking by
    /// itself.
    ///
    /// ## Return value
    /// Returns nonzero on success, zero on error. Specifics of the error can be
    ///   gleaned from [`Sound_GetError()`].
    ///
    /// ## Thread safety
    /// This call is not thread-safe.
    ///
    /// ## Availability
    /// This function is available since SDL_sound 1.0.0.
    ///
    /// ## See also
    /// - [`Sound_Quit`]
    pub fn Sound_Init() -> ::core::ffi::c_int;
}

unsafe extern "C" {
    /// Shutdown SDL_sound.
    ///
    /// This closes any [`SDL_IOStreams`] that were being used as sound sources, and
    /// frees any resources in use by SDL_sound.
    ///
    /// All [`Sound_Sample`] pointers you had prior to this call are invalidated.
    ///
    /// Once successfully deinitialized, [`Sound_Init()`] can be called again to
    /// restart the subsystem. All default API states are restored at this point.
    ///
    /// You should call this _before_ [`SDL_Quit()`]. This will _not_ call [`SDL_Quit()`]
    /// for you!
    ///
    /// ## Return value
    /// Returns nonzero on success, zero on error. Specifics of the error can be
    ///   gleaned from [`Sound_GetError()`]. If failure, state of SDL_sound is
    ///   undefined, and probably badly screwed up.
    ///
    /// ## Thread safety
    /// This call is not thread-safe.
    ///
    /// ## Availability
    /// This function is available since SDL_sound 1.0.0.
    ///
    /// ## See also
    /// - [`Sound_Init`]
    pub fn Sound_Quit() -> ::core::ffi::c_int;
}

unsafe extern "C" {
    /// Get a list of sound formats supported by this version of SDL_sound.
    ///
    /// This is for informational purposes only. Note that the extension listed is
    /// merely convention: if we list "MP3", you can open an MPEG-1 Layer 3 audio
    /// file with an extension of "XYZ", if you like. The file extensions are
    /// informational, and only required as a hint to choosing the correct decoder,
    /// since the sound data may not be coming from a file at all, thanks to the
    /// abstraction that an [`SDL_IOStream`] provides.
    ///
    /// The returned value is an array of pointers to [`Sound_DecoderInfo`] structures,
    /// with a NULL entry to signify the end of the list:
    ///
    /// ```c
    /// for (Sound_DecoderInfo **i = Sound_AvailableDecoders(); *i != NULL; i++)
    /// {
    ///     printf("Supported sound format: [%s], which is [%s].\n",
    ///              i->extension, i->description);
    ///     // ...and other fields...
    /// }
    /// ```
    ///
    /// The return values are pointers to static internal memory, and should be
    /// considered READ ONLY, and never freed.
    ///
    /// ## Return value
    /// Returns READ ONLY null-terminated array of READ ONLY structures.
    ///
    /// ## Thread safety
    /// It is safe to call this function from any thread.
    ///
    /// ## Availability
    /// This function is available since SDL_sound 1.0.0.
    ///
    /// ## See also
    /// - [`Sound_DecoderInfo`]
    pub fn Sound_AvailableDecoders() -> *mut *const Sound_DecoderInfo;
}

unsafe extern "C" {
    /// Get the last SDL_sound error message as a null-terminated string.
    ///
    /// This will be NULL if there's been no error since the last call to this
    /// function. The pointer returned by this call points to an internal buffer,
    /// and should not be deallocated. Each thread has a unique error state
    /// associated with it, but each time a new error message is set, it will
    /// overwrite the previous one associated with that thread. It is safe to call
    /// this function at anytime, even before [`Sound_Init()`].
    ///
    /// ## Return value
    /// Returns READ ONLY string of last error message.
    ///
    /// ## Thread safety
    /// It is safe to call this function from any thread.
    ///
    /// ## Availability
    /// This function is available since SDL_sound 1.0.0.
    ///
    /// ## See also
    /// - [`Sound_ClearError`]
    pub fn Sound_GetError() -> *const ::core::ffi::c_char;
}

unsafe extern "C" {
    /// Clear the current error message.
    ///
    /// The next call to [`Sound_GetError()`] after [`Sound_ClearError()`] will return
    /// NULL.
    ///
    /// ## Thread safety
    /// It is safe to call this function from any thread.
    ///
    /// ## Availability
    /// This function is available since SDL_sound 1.0.0.
    ///
    /// ## See also
    /// - [`Sound_GetError`]
    pub fn Sound_ClearError();
}

unsafe extern "C" {
    /// Start decoding a new sound sample.
    ///
    /// The data is read via an [`SDL_IOStream`] structure (see SDL_iostream.h in
    /// SDL3's include directory), so it may be coming from memory, disk, network
    /// stream, etc. The (ext) parameter is merely a hint to determining the
    /// correct decoder; if you specify, for example, "mp3" for an extension, and
    /// one of the decoders lists that as a handled extension, then that decoder is
    /// given first shot at trying to claim the data for decoding. If none of the
    /// extensions match (or the extension is NULL), then every decoder examines
    /// the data to determine if it can handle it, until one accepts it. In such a
    /// case your [`SDL_IOStream`] will need to be capable of rewinding to the start of
    /// the stream.
    ///
    /// If no decoders can handle the data, a NULL value is returned, and a human
    /// readable error message can be fetched from [`Sound_GetError()`].
    ///
    /// Optionally, a desired audio format can be specified. If the incoming data
    /// is in a different format, SDL_sound will convert it to the desired format
    /// on the fly. Note that this can be an expensive operation, so it may be wise
    /// to convert data before you need to play it back, if possible, or make sure
    /// your data is initially in the format that you need it in. If you don't want
    /// to convert the data, you can specify NULL for a desired format. The
    /// incoming format of the data, preconversion, can be found in the
    /// [`Sound_Sample`] structure.
    ///
    /// Note that the raw sound data "decoder" needs you to specify both the
    /// extension "RAW" and a "desired" format, or it will refuse to handle the
    /// data. This is to prevent it from catching all formats unsupported by the
    /// other decoders.
    ///
    /// Finally, specify an initial buffer size; this is the number of bytes that
    /// will be allocated to store each read from the sound buffer. The more you
    /// can safely allocate, the more decoding can be done in one block, but the
    /// more resources you have to use up, and the longer each decoding call will
    /// take. Note that different data formats require more or less space to store.
    /// This buffer can be resized later via [`Sound_SetBufferSize()`].
    ///
    /// The buffer size specified must be a multiple of the size of a single sample
    /// frame. So, if you want 16-bit, stereo samples, then your sample frame size
    /// is (2 channels * 16 bits), or 32 bits per frame, which is four bytes. In
    /// such a case, you could specify 128 or 132 bytes for a buffer, but not 129,
    /// 130, or 131 (although in reality, you'll want to specify a MUCH larger
    /// buffer).
    ///
    /// When you are done with this [`Sound_Sample`] pointer, you can dispose of it via
    /// [`Sound_FreeSample()`].
    ///
    /// You do not have to keep a reference to `io` around. If this function
    /// suceeds, it stores `io` internally (and disposes of it during the call to
    /// [`Sound_FreeSample()`]). If this function fails, it will dispose of the
    /// [`SDL_IOStream`] for you.
    ///
    /// ## Parameters
    /// - `io`: an [`SDL_IOStream`] with sound data.
    /// - `ext`: file extension normally associated with a data format. Can
    ///   usually be NULL.
    /// - `desired`: format to convert sound data into. Can usually be NULL, if
    ///   you don't need conversion.
    /// - `bufferSize`: size, in bytes, to allocate for the decoding buffer.
    ///
    /// ## Return value
    /// Returns [`Sound_Sample`] pointer, which is used as a handle to several other
    ///   SDL_sound APIs. NULL on error. If error, use [`Sound_GetError()`] to
    ///   see what went wrong.
    ///
    /// ## Thread safety
    /// It is safe to call this function from any thread.
    ///
    /// ## Availability
    /// This function is available since SDL_sound 1.0.0 (using [`SDL_RWops`]
    /// ```text
    ///    and [`Sound_AudioInfo`]; as of SDL_sound 3.0.0, this expects the SDL3
    ///    equivalents, SDL_IOStream and SDL_AudioSpec, but is otherwise
    ///    identical).
    /// ```
    ///
    /// \sa Sound_NewSampleFromFile
    /// ## See also
    /// - [`Sound_SetBufferSize`]
    /// - [`Sound_Decode`]
    /// - [`Sound_DecodeAll`]
    /// - [`Sound_Seek`]
    /// - [`Sound_Rewind`]
    /// - [`Sound_FreeSample`]
    pub fn Sound_NewSample(
        io: *mut SDL_IOStream,
        ext: *const ::core::ffi::c_char,
        desired: *const SDL_AudioSpec,
        bufferSize: Uint32,
    ) -> *mut Sound_Sample;
}

unsafe extern "C" {
    /// Start decoding a new sound sample from a file on disk.
    ///
    /// This is identical to [`Sound_NewSample()`], but it creates an [`SDL_IOStream`] for
    /// you from the `size` bytes of memory referenced by `data`.
    ///
    /// ## Parameters
    /// - `data`: Buffer of data holding contents of an audio file to decode.
    /// - `size`: Size, in bytes, of buffer pointed to by (data).
    /// - `ext`: File extension normally associated with a data format. Can
    ///   usually be NULL.
    /// - `desired`: Format to convert sound data into. Can usually be NULL, if
    ///   you don't need conversion.
    /// - `bufferSize`: size, in bytes, of initial read buffer.
    ///
    /// ## Return value
    /// Returns [`Sound_Sample`] pointer, which is used as a handle to several other
    ///   SDL_sound APIs. NULL on error. If error, use [`Sound_GetError()`] to
    ///   see what went wrong.
    ///
    /// ## Thread safety
    /// It is safe to call this function from any thread.
    ///
    /// ## Availability
    /// This function is available since SDL_sound 1.0.0 (using
    /// ```text
    ///    [`Sound_AudioInfo`]; as of SDL_sound 3.0.0, this expects the SDL3
    ///    equivalent, SDL_AudioSpec, but is otherwise identical).
    /// ```
    ///
    /// \sa Sound_NewSample
    /// ## See also
    /// - [`Sound_SetBufferSize`]
    /// - [`Sound_Decode`]
    /// - [`Sound_DecodeAll`]
    /// - [`Sound_Seek`]
    /// - [`Sound_Rewind`]
    /// - [`Sound_FreeSample`]
    pub fn Sound_NewSampleFromMem(
        data: *const Uint8,
        size: Uint32,
        ext: *const ::core::ffi::c_char,
        desired: *const SDL_AudioSpec,
        bufferSize: Uint32,
    ) -> *mut Sound_Sample;
}

unsafe extern "C" {
    /// Start decoding a new sound sample from a file on disk.
    ///
    /// This is identical to [`Sound_NewSample()`], but it creates an [`SDL_IOStream`] for
    /// you from the file located in `filename`. Note that `filename` is specified
    /// in platform-dependent notation. ("C:\\music\\mysong.mp3" on Windows, and
    /// "/home/icculus/music/mysong.mp3" or whatever on Unix, etc.)
    ///
    /// [`Sound_NewSample()`]'s "ext" parameter is gleaned from the contents of
    /// `filename`.
    ///
    /// ## Parameters
    /// - `filename`: file containing sound data.
    /// - `desired`: Format to convert sound data into. Can usually be NULL, if
    ///   you don't need conversion.
    /// - `bufferSize`: size, in bytes, of initial read buffer.
    ///
    /// ## Return value
    /// Returns [`Sound_Sample`] pointer, which is used as a handle to several other
    ///   SDL_sound APIs. NULL on error. If error, use [`Sound_GetError()`] to
    ///   see what went wrong.
    ///
    /// ## Thread safety
    /// It is safe to call this function from any thread.
    ///
    /// ## Availability
    /// This function is available since SDL_sound 1.0.0 (using
    /// ```text
    ///    [`Sound_AudioInfo`]; as of SDL_sound 3.0.0, this expects the SDL3
    ///    equivalent, SDL_AudioSpec, but is otherwise identical).
    /// ```
    ///
    /// \sa Sound_NewSample
    /// ## See also
    /// - [`Sound_SetBufferSize`]
    /// - [`Sound_Decode`]
    /// - [`Sound_DecodeAll`]
    /// - [`Sound_Seek`]
    /// - [`Sound_Rewind`]
    /// - [`Sound_FreeSample`]
    pub fn Sound_NewSampleFromFile(
        filename: *const ::core::ffi::c_char,
        desired: *const SDL_AudioSpec,
        bufferSize: Uint32,
    ) -> *mut Sound_Sample;
}

unsafe extern "C" {
    /// Dispose of a [`Sound_Sample`].
    ///
    /// This will also close/dispose of the [`SDL_IOStream`] that was used at creation
    /// time, so there's no need to keep a reference to that around. The
    /// [`Sound_Sample`] pointer is invalid after this call, and will almos certainly
    /// result in a crash if you attempt to keep using it.
    ///
    /// ## Parameters
    /// - `sample`: The [`Sound_Sample`] to delete.
    ///
    /// ## Thread safety
    /// It is safe to call this function from any thread.
    ///
    /// ## Availability
    /// This function is available since SDL_sound 1.0.0 (using
    /// ```text
    ///    [`Sound_AudioInfo`]; as of SDL_sound 3.0.0, this expects the SDL3
    ///    equivalent, SDL_AudioSpec, but is otherwise identical).
    /// ```
    ///
    /// \sa Sound_NewSample
    /// ## See also
    /// - [`Sound_NewSampleFromFile`]
    pub fn Sound_FreeSample(sample: *mut Sound_Sample);
}

unsafe extern "C" {
    /// Retrieve total play time of sample, in milliseconds.
    ///
    /// Report total time length of sample, in milliseconds. This is a fast call.
    /// Duration is calculated during Sound_NewSample*, so this is just an accessor
    /// into otherwise opaque data.
    ///
    /// Please note that not all formats can determine a total time, some can't be
    /// exact without fully decoding the data, and thus will estimate the duration.
    /// Many decoders will require the ability to seek in the data stream to
    /// calculate this, so even if we can tell you how long an .ogg file will be,
    /// the same data set may fail if it's, say, streamed over an HTTP connection.
    /// Plan accordingly.
    ///
    /// Most people won't need this function to just decode and playback, but it
    /// can be useful for informational purposes in, say, a music player's UI.
    ///
    /// ## Parameters
    /// - `sample`: [`Sound_Sample`] from which to retrieve duration information.
    ///
    /// ## Return value
    /// Returns Sample length in milliseconds, or -1 if duration can't be
    ///   determined for any reason.
    ///
    /// ## Thread safety
    /// It is safe to call this function from any thread.
    ///
    /// ## Availability
    /// This function is available since SDL_sound 1.0.0.
    pub fn Sound_GetDuration(sample: *mut Sound_Sample) -> Sint32;
}

unsafe extern "C" {
    /// Change the current buffer size for a sample.
    ///
    /// If the buffer size could be changed, then the `sample->buffer` and
    /// `sample->buffer_size` fields will reflect that. If they could not be
    /// changed, then your original sample state is preserved. If the buffer is
    /// shrinking, the data at the end of buffer is truncated. If the buffer is
    /// growing, the contents of the new space at the end is undefined until you
    /// decode more into it or initialize it yourself.
    ///
    /// The buffer size specified must be a multiple of the size of a single sample
    /// frame. So, if you want 16-bit, stereo samples, then your sample frame size
    /// is (2 channels * 16 bits), or 32 bits per sample, which is four bytes. In
    /// such a case, you could specify 128 or 132 bytes for a buffer, but not 129,
    /// 130, or 131 (although in reality, you'll want to specify a MUCH larger
    /// buffer).
    ///
    /// ## Parameters
    /// - `sample`: The [`Sound_Sample`] whose buffer to modify.
    /// - `new_size`: The desired size, in bytes, of the new buffer.
    ///
    /// ## Return value
    /// Returns non-zero if buffer size changed, zero on failure.
    ///
    /// ## Thread safety
    /// It is safe to call this function from any thread, but a
    ///   single [`Sound_Sample`] should not be accessed from two threads
    ///   at the same time.
    ///
    /// ## Availability
    /// This function is available since SDL_sound 1.0.0.
    ///
    /// ## See also
    /// - [`Sound_Decode`]
    /// - [`Sound_DecodeAll`]
    pub fn Sound_SetBufferSize(sample: *mut Sound_Sample, new_size: Uint32) -> ::core::ffi::c_int;
}

unsafe extern "C" {
    /// Change the desired output format for a sample.
    ///
    /// Future calls to [`Sound_Decode()`] or [`Sound_DecodeAll()`] will produce audio in
    /// the newly-requested format; SDL_sound will convert on-the-fly while
    /// decoding, if necessary.
    ///
    /// The sample's buffer size, must be a multiple of the size of a single sample
    /// frame. If this is no longer the case after the format change, the buffer
    /// size will be reduced by a few bytes to accommodate this, but no actual
    /// reallocation of the buffer is made. [`Sound_SetBufferSize()`] can take more
    /// explicit control over the buffer after a format change.
    ///
    /// If `desired` is NULL, the sample's actual format is used, disabling audio
    /// conversion.
    ///
    /// If this function fails (out of memory setting up a new internal audio
    /// stream, etc), the sample remains usable with its current, unchanged format.
    ///
    /// ## Parameters
    /// - `sample`: The [`Sound_Sample`] whose format should be modified.
    /// - `desired`: The new desired format.
    ///
    /// ## Return value
    /// Returns non-zero if format changed, zero on failure.
    ///
    /// ## Thread safety
    /// It is safe to call this function from any thread, but a
    ///   single [`Sound_Sample`] should not be accessed from two threads
    ///   at the same time.
    ///
    /// ## Availability
    /// This function is available since SDL_sound 3.0.0.
    ///
    /// ## See also
    /// - [`Sound_Decode`]
    /// - [`Sound_DecodeAll`]
    pub fn Sound_SetDesiredFormat(
        sample: *mut Sound_Sample,
        desired: *const SDL_AudioSpec,
    ) -> ::core::ffi::c_int;
}

unsafe extern "C" {
    /// Decode more of the sound data in a [`Sound_Sample`].
    ///
    /// It will decode at most `sample->buffer_size` bytes into `sample->buffer` in
    /// the desired format, and return the number of decoded bytes.
    ///
    /// If `sample->buffer_size` bytes could not be decoded, then please refer to
    /// `sample->flags` to determine if this was an end-of-stream or error
    /// condition.
    ///
    /// ## Parameters
    /// - `sample`: Do more decoding to this [`Sound_Sample`].
    ///
    /// ## Return value
    /// Returns number of bytes decoded into sample->buffer.
    ///
    /// ## Thread safety
    /// It is safe to call this function from any thread, but a
    ///   single [`Sound_Sample`] should not be accessed from two threads
    ///   at the same time.
    ///
    /// ## Availability
    /// This function is available since SDL_sound 1.0.0.
    ///
    /// ## See also
    /// - [`Sound_DecodeAll`]
    /// - [`Sound_SetBufferSize`]
    /// - [`Sound_Seek`]
    /// - [`Sound_Rewind`]
    pub fn Sound_Decode(sample: *mut Sound_Sample) -> Uint32;
}

unsafe extern "C" {
    /// Decode the remainder of the sound data in a [`Sound_Sample`].
    ///
    /// This will dynamically allocate memory for the ENTIRE remaining sample.
    /// `sample->buffer_size` and `sample->buffer` will be updated to reflect the
    /// new buffer. Please refer to sample->flags to determine if the decoding
    /// finished due to an end-of-stream or error condition.
    ///
    /// Be aware that sound data can take a large amount of memory, and that this
    /// function may block for quite awhile while processing. Also note that a
    /// streaming source (for example, from a [`SDL_IOStream`] that is getting fed from
    /// an Internet radio feed that doesn't end) may fill all available memory
    /// before giving up...be sure to use this on finite sound sources only!
    ///
    /// When decoding the sample in its entirety, the work is done one buffer at a
    /// time. That is, sound is decoded in `sample->buffer_size` blocks, and
    /// appended to a continually-growing buffer until the decoding completes. That
    /// means that this function will need enough RAM to hold approximately
    /// `sample->buffer_size` bytes plus the complete decoded sample at most. The
    /// larger your buffer size, the less overhead this function needs, but beware
    /// the possibility of paging to disk. Best to make this user-configurable if
    /// the sample isn't specific and small.
    ///
    /// ## Parameters
    /// - `sample`: do all decoding for this [`Sound_Sample`].
    ///
    /// ## Return value
    /// Returns number of bytes decoded into sample->buffer. You should check
    ///   sample->flags to see what the current state of the sample is (EOF,
    ///   error, read again).
    ///
    /// ## Thread safety
    /// It is safe to call this function from any thread, but a
    ///   single [`Sound_Sample`] should not be accessed from two threads
    ///   at the same time.
    ///
    /// ## Availability
    /// This function is available since SDL_sound 1.0.0.
    ///
    /// ## See also
    /// - [`Sound_Decode`]
    /// - [`Sound_SetBufferSize`]
    pub fn Sound_DecodeAll(sample: *mut Sound_Sample) -> Uint32;
}

unsafe extern "C" {
    /// Rewind a sample to the start.
    ///
    /// Restart a sample at the start of its waveform data, as if newly created
    /// with [`Sound_NewSample()`]. If successful, the next call to [`Sound_Decode()`] or
    /// [`Sound_DecodeAll()`] will give audio data from the earliest point in the
    /// stream.
    ///
    /// Beware that this function will fail if the [`SDL_IOStream`] that feeds the
    /// decoder can not be rewound via [`SDL_SeekIO()`], but this can theoretically be
    /// avoided by wrapping it in some sort of buffering [`SDL_IOStream`].
    ///
    /// This function should ONLY fail if the [`SDL_IOStream`] is not seekable, or
    /// SDL_sound is not initialized. Both can be controlled by the application,
    /// and thus, it is up to the developer's paranoia to dictate whether this
    /// function's return value need be checked at all.
    ///
    /// If this function fails, the state of the sample is undefined, but it is
    /// still safe to call [`Sound_FreeSample()`] to dispose of it.
    ///
    /// On success, SOUND_SAMPLEFLAG_ERROR, SOUND_SAMPLEFLAG_EOF, and
    /// SOUND_SAMPLEFLAG_EAGAIN are cleared from `sample->flags`. The
    /// SOUND_SAMPLEFLAG_ERROR flag will be set on error.
    ///
    /// ## Parameters
    /// - `sample`: the [`Sound_Sample`] to rewind.
    ///
    /// ## Return value
    /// Returns nonzero on success, zero on error. Specifics of the error can be
    ///   gleaned from [`Sound_GetError()`].
    ///
    /// ## Thread safety
    /// It is safe to call this function from any thread, but a
    ///   single [`Sound_Sample`] should not be accessed from two threads
    ///   at the same time.
    ///
    /// ## Availability
    /// This function is available since SDL_sound 1.0.0.
    ///
    /// ## See also
    /// - [`Sound_Seek`]
    pub fn Sound_Rewind(sample: *mut Sound_Sample) -> ::core::ffi::c_int;
}

unsafe extern "C" {
    /// Seek to a different point in a sample.
    ///
    /// Reposition a sample's stream. If successful, the next call to
    /// [`Sound_Decode()`] or [`Sound_DecodeAll()`] will give audio data from the offset
    /// you specified.
    ///
    /// The offset is specified in milliseconds from the start of the sample.
    ///
    /// Beware that this function can fail for several reasons. If the [`SDL_IOStream`]
    /// that feeds the decoder can not seek, this call will almost certainly fail,
    /// but this can theoretically be avoided by wrapping it in some sort of
    /// buffering [`SDL_IOStream`]. Some decoders can never seek, others can only seek
    /// with certain files. The decoders will set a flag in the sample at creation
    /// time to help you determine this.
    ///
    /// You should check `sample->flags & SOUND_SAMPLEFLAG_CANSEEK` before
    /// attempting. [`Sound_Seek()`] reports failure immediately if this flag isn't
    /// set. This function can still fail for other reasons if the flag is set.
    ///
    /// This function can be emulated in the application with [`Sound_Rewind()`] and
    /// predecoding a specific amount of the sample, but this can be extremely
    /// inefficient. [`Sound_Seek()`] accelerates the seek with decoder-specific code.
    ///
    /// If this function fails, the sample should continue to function as if this
    /// call was never made. If there was an unrecoverable error, `sample->flags &
    /// SOUND_SAMPLEFLAG_ERROR` will be set, which your regular decoding loop can
    /// pick up.
    ///
    /// On success, SOUND_SAMPLEFLAG_ERROR, SOUND_SAMPLEFLAG_EOF, and
    /// SOUND_SAMPLEFLAG_EAGAIN are cleared from `sample->flags`.
    ///
    /// ## Parameters
    /// - `sample`: the [`Sound_Sample`] to seek.
    /// - `ms`: the new position, in milliseconds from start of sample.
    ///
    /// ## Return value
    /// Returns nonzero on success, zero on error. Specifics of the error can be
    ///   gleaned from [`Sound_GetError()`].
    ///
    /// ## Thread safety
    /// It is safe to call this function from any thread, but a
    ///   single [`Sound_Sample`] should not be accessed from two threads
    ///   at the same time.
    ///
    /// ## Availability
    /// This function is available since SDL_sound 1.0.0.
    ///
    /// ## See also
    /// - [`Sound_Rewind`]
    pub fn Sound_Seek(sample: *mut Sound_Sample, ms: Uint32) -> ::core::ffi::c_int;
}

#[cfg(doc)]
use crate::everything::*;
