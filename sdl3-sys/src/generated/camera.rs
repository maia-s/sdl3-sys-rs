//! Video capture for the SDL library.
//!
//! This API lets apps read input from video sources, like webcams. Camera
//! devices can be enumerated, queried, and opened. Once opened, it will
//! provide [`SDL_Surface`] objects as new frames of video come in. These surfaces
//! can be uploaded to an [`SDL_Texture`] or processed as pixels in memory.
//!
//! Several platforms will alert the user if an app tries to access a camera,
//! and some will present a UI asking the user if your application should be
//! allowed to obtain images at all, which they can deny. A successfully opened
//! camera will not provide images until permission is granted. Applications,
//! after opening a camera device, can see if they were granted access by
//! either polling with the [`SDL_GetCameraPermissionState()`] function, or waiting
//! for an [`SDL_EVENT_CAMERA_DEVICE_APPROVED`] or [`SDL_EVENT_CAMERA_DEVICE_DENIED`]
//! event. Platforms that don't have any user approval process will report
//! approval immediately.
//!
//! Note that SDL cameras only provide video as individual frames; they will
//! not provide full-motion video encoded in a movie file format, although an
//! app is free to encode the acquired frames into any format it likes. It also
//! does not provide audio from the camera hardware through this API; not only
//! do many webcams not have microphones at all, many people--from streamers to
//! people on Zoom calls--will want to use a separate microphone regardless of
//! the camera. In any case, recorded audio will be available through SDL's
//! audio API no matter what hardware provides the microphone.
//!
//! ## Camera gotchas
//!
//! Consumer-level camera hardware tends to take a little while to warm up,
//! once the device has been opened. Generally most camera apps have some sort
//! of UI to take a picture (a button to snap a pic while a preview is showing,
//! some sort of multi-second countdown for the user to pose, like a photo
//! booth), which puts control in the users' hands, or they are intended to
//! stay on for long times (Pokemon Go, etc).
//!
//! It's not uncommon that a newly-opened camera will provide a couple of
//! completely black frames, maybe followed by some under-exposed images. If
//! taking a single frame automatically, or recording video from a camera's
//! input without the user initiating it from a preview, it could be wise to
//! drop the first several frames (if not the first several _seconds_ worth of
//! frames!) before using images from a camera.

use super::stdinc::*;

use super::error::*;

use super::pixels::*;

use super::properties::*;

use super::surface::*;

/// This is a unique ID for a camera device for the time it is connected to the
/// system, and is never reused for the lifetime of the application.
///
/// If the device is disconnected and reconnected, it will get a new ID.
///
/// The value 0 is an invalid ID.
///
/// ## Availability
/// This datatype is available since SDL 3.2.0.
///
/// ## See also
/// - [`SDL_GetCameras`]
#[repr(transparent)]
#[derive(Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "debug-impls", derive(Debug))]
pub struct SDL_CameraID(pub Uint32);

impl ::core::cmp::PartialEq<Uint32> for SDL_CameraID {
    #[inline(always)]
    fn eq(&self, other: &Uint32) -> bool {
        &self.0 == other
    }
}

impl ::core::cmp::PartialEq<SDL_CameraID> for Uint32 {
    #[inline(always)]
    fn eq(&self, other: &SDL_CameraID) -> bool {
        self == &other.0
    }
}

impl From<SDL_CameraID> for Uint32 {
    #[inline(always)]
    fn from(value: SDL_CameraID) -> Self {
        value.0
    }
}

#[cfg(feature = "display-impls")]
impl ::core::fmt::Display for SDL_CameraID {
    #[inline(always)]
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        <Uint32 as ::core::fmt::Display>::fmt(&self.0, f)
    }
}

impl SDL_CameraID {
    /// Initialize a `SDL_CameraID` from a raw value.
    /// # Safety
    /// The value should be valid for this type
    #[inline(always)]
    pub const unsafe fn from_raw(value: Uint32) -> Self {
        Self(value)
    }

    /// Get the inner raw value.
    #[inline(always)]
    pub const fn into_raw(self) -> Uint32 {
        self.0
    }
}

impl SDL_CameraID {
    /// Get a copy of the inner raw value.
    #[inline(always)]
    pub const fn value(&self) -> Uint32 {
        self.0
    }
}

#[cfg(feature = "metadata")]
impl sdl3_sys::metadata::GroupMetadata for SDL_CameraID {
    const GROUP_METADATA: &'static sdl3_sys::metadata::Group =
        &crate::metadata::camera::METADATA_SDL_CameraID;
}

/// The details of an output format for a camera device.
///
/// Cameras often support multiple formats; each one will be encapsulated in
/// this struct.
///
/// ## Availability
/// This struct is available since SDL 3.2.0.
///
/// ## See also
/// - [`SDL_GetCameraSupportedFormats`]
/// - [`SDL_GetCameraFormat`]
#[repr(C)]
#[derive(Clone, Copy, Default, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "debug-impls", derive(Debug))]
pub struct SDL_CameraSpec {
    /// Frame format
    pub format: SDL_PixelFormat,
    /// Frame colorspace
    pub colorspace: SDL_Colorspace,
    /// Frame width
    pub width: ::core::ffi::c_int,
    /// Frame height
    pub height: ::core::ffi::c_int,
    /// Frame rate numerator ((num / denom) == FPS, (denom / num) == duration in seconds)
    pub framerate_numerator: ::core::ffi::c_int,
    /// Frame rate denominator ((num / denom) == FPS, (denom / num) == duration in seconds)
    pub framerate_denominator: ::core::ffi::c_int,
}

/// The position of camera in relation to system device.
///
/// ## Availability
/// This enum is available since SDL 3.2.0.
///
/// ## See also
/// - [`SDL_GetCameraPosition`]
///
/// ## Known values (`sdl3-sys`)
/// | Associated constant | Global constant | Description |
/// | ------------------- | --------------- | ----------- |
/// | [`UNKNOWN`](SDL_CameraPosition::UNKNOWN) | [`SDL_CAMERA_POSITION_UNKNOWN`] | |
/// | [`FRONT_FACING`](SDL_CameraPosition::FRONT_FACING) | [`SDL_CAMERA_POSITION_FRONT_FACING`] | |
/// | [`BACK_FACING`](SDL_CameraPosition::BACK_FACING) | [`SDL_CAMERA_POSITION_BACK_FACING`] | |
#[repr(transparent)]
#[derive(Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct SDL_CameraPosition(pub ::core::ffi::c_int);

impl ::core::cmp::PartialEq<::core::ffi::c_int> for SDL_CameraPosition {
    #[inline(always)]
    fn eq(&self, other: &::core::ffi::c_int) -> bool {
        &self.0 == other
    }
}

impl ::core::cmp::PartialEq<SDL_CameraPosition> for ::core::ffi::c_int {
    #[inline(always)]
    fn eq(&self, other: &SDL_CameraPosition) -> bool {
        self == &other.0
    }
}

impl From<SDL_CameraPosition> for ::core::ffi::c_int {
    #[inline(always)]
    fn from(value: SDL_CameraPosition) -> Self {
        value.0
    }
}

#[cfg(feature = "debug-impls")]
impl ::core::fmt::Debug for SDL_CameraPosition {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        #[allow(unreachable_patterns)]
        f.write_str(match *self {
            Self::UNKNOWN => "SDL_CAMERA_POSITION_UNKNOWN",
            Self::FRONT_FACING => "SDL_CAMERA_POSITION_FRONT_FACING",
            Self::BACK_FACING => "SDL_CAMERA_POSITION_BACK_FACING",

            _ => return write!(f, "SDL_CameraPosition({})", self.0),
        })
    }
}

impl SDL_CameraPosition {
    pub const UNKNOWN: Self = Self((0 as ::core::ffi::c_int));
    pub const FRONT_FACING: Self = Self((1 as ::core::ffi::c_int));
    pub const BACK_FACING: Self = Self((2 as ::core::ffi::c_int));
}

pub const SDL_CAMERA_POSITION_UNKNOWN: SDL_CameraPosition = SDL_CameraPosition::UNKNOWN;
pub const SDL_CAMERA_POSITION_FRONT_FACING: SDL_CameraPosition = SDL_CameraPosition::FRONT_FACING;
pub const SDL_CAMERA_POSITION_BACK_FACING: SDL_CameraPosition = SDL_CameraPosition::BACK_FACING;

impl SDL_CameraPosition {
    /// Initialize a `SDL_CameraPosition` from a raw value.
    /// # Safety
    /// The value should be valid for this type
    #[inline(always)]
    pub const unsafe fn from_raw(value: ::core::ffi::c_int) -> Self {
        Self(value)
    }

    /// Get the inner raw value.
    #[inline(always)]
    pub const fn into_raw(self) -> ::core::ffi::c_int {
        self.0
    }
}

impl SDL_CameraPosition {
    /// Get a copy of the inner raw value.
    #[inline(always)]
    pub const fn value(&self) -> ::core::ffi::c_int {
        self.0
    }
}

#[cfg(feature = "metadata")]
impl sdl3_sys::metadata::GroupMetadata for SDL_CameraPosition {
    const GROUP_METADATA: &'static sdl3_sys::metadata::Group =
        &crate::metadata::camera::METADATA_SDL_CameraPosition;
}

/// The current state of a request for camera access.
///
/// ## Availability
/// This enum is available since SDL 3.4.0.
///
/// ## See also
/// - [`SDL_GetCameraPermissionState`]
///
/// ## Known values (`sdl3-sys`)
/// | Associated constant | Global constant | Description |
/// | ------------------- | --------------- | ----------- |
/// | [`DENIED`](SDL_CameraPermissionState::DENIED) | [`SDL_CAMERA_PERMISSION_STATE_DENIED`] | |
/// | [`PENDING`](SDL_CameraPermissionState::PENDING) | [`SDL_CAMERA_PERMISSION_STATE_PENDING`] | |
/// | [`APPROVED`](SDL_CameraPermissionState::APPROVED) | [`SDL_CAMERA_PERMISSION_STATE_APPROVED`] | |
#[repr(transparent)]
#[derive(Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct SDL_CameraPermissionState(pub ::core::ffi::c_int);

impl ::core::cmp::PartialEq<::core::ffi::c_int> for SDL_CameraPermissionState {
    #[inline(always)]
    fn eq(&self, other: &::core::ffi::c_int) -> bool {
        &self.0 == other
    }
}

impl ::core::cmp::PartialEq<SDL_CameraPermissionState> for ::core::ffi::c_int {
    #[inline(always)]
    fn eq(&self, other: &SDL_CameraPermissionState) -> bool {
        self == &other.0
    }
}

impl From<SDL_CameraPermissionState> for ::core::ffi::c_int {
    #[inline(always)]
    fn from(value: SDL_CameraPermissionState) -> Self {
        value.0
    }
}

#[cfg(feature = "debug-impls")]
impl ::core::fmt::Debug for SDL_CameraPermissionState {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        #[allow(unreachable_patterns)]
        f.write_str(match *self {
            Self::DENIED => "SDL_CAMERA_PERMISSION_STATE_DENIED",
            Self::PENDING => "SDL_CAMERA_PERMISSION_STATE_PENDING",
            Self::APPROVED => "SDL_CAMERA_PERMISSION_STATE_APPROVED",

            _ => return write!(f, "SDL_CameraPermissionState({})", self.0),
        })
    }
}

impl SDL_CameraPermissionState {
    pub const DENIED: Self = Self((-1_i32 as ::core::ffi::c_int));
    pub const PENDING: Self = Self((0_i32 as ::core::ffi::c_int));
    pub const APPROVED: Self = Self((1_i32 as ::core::ffi::c_int));
}

pub const SDL_CAMERA_PERMISSION_STATE_DENIED: SDL_CameraPermissionState =
    SDL_CameraPermissionState::DENIED;
pub const SDL_CAMERA_PERMISSION_STATE_PENDING: SDL_CameraPermissionState =
    SDL_CameraPermissionState::PENDING;
pub const SDL_CAMERA_PERMISSION_STATE_APPROVED: SDL_CameraPermissionState =
    SDL_CameraPermissionState::APPROVED;

impl SDL_CameraPermissionState {
    /// Initialize a `SDL_CameraPermissionState` from a raw value.
    /// # Safety
    /// The value should be valid for this type
    #[inline(always)]
    pub const unsafe fn from_raw(value: ::core::ffi::c_int) -> Self {
        Self(value)
    }

    /// Get the inner raw value.
    #[inline(always)]
    pub const fn into_raw(self) -> ::core::ffi::c_int {
        self.0
    }
}

impl SDL_CameraPermissionState {
    /// Get a copy of the inner raw value.
    #[inline(always)]
    pub const fn value(&self) -> ::core::ffi::c_int {
        self.0
    }
}

#[cfg(feature = "metadata")]
impl sdl3_sys::metadata::GroupMetadata for SDL_CameraPermissionState {
    const GROUP_METADATA: &'static sdl3_sys::metadata::Group =
        &crate::metadata::camera::METADATA_SDL_CameraPermissionState;
}

unsafe extern "C" {
    /// Use this function to get the number of built-in camera drivers.
    ///
    /// This function returns a hardcoded number. This never returns a negative
    /// value; if there are no drivers compiled into this build of SDL, this
    /// function returns zero. The presence of a driver in this list does not mean
    /// it will function, it just means SDL is capable of interacting with that
    /// interface. For example, a build of SDL might have v4l2 support, but if
    /// there's no kernel support available, SDL's v4l2 driver would fail if used.
    ///
    /// By default, SDL tries all drivers, in its preferred order, until one is
    /// found to be usable.
    ///
    /// ## Return value
    /// Returns the number of built-in camera drivers.
    ///
    /// ## Thread safety
    /// It is safe to call this function from any thread.
    ///
    /// ## Availability
    /// This function is available since SDL 3.2.0.
    ///
    /// ## See also
    /// - [`SDL_GetCameraDriver`]
    pub fn SDL_GetNumCameraDrivers() -> ::core::ffi::c_int;
}

unsafe extern "C" {
    /// Use this function to get the name of a built in camera driver.
    ///
    /// The list of camera drivers is given in the order that they are normally
    /// initialized by default; the drivers that seem more reasonable to choose
    /// first (as far as the SDL developers believe) are earlier in the list.
    ///
    /// The names of drivers are all simple, low-ASCII identifiers, like "v4l2",
    /// "coremedia" or "android". These never have Unicode characters, and are not
    /// meant to be proper names.
    ///
    /// ## Parameters
    /// - `index`: the index of the camera driver; the value ranges from 0 to
    ///   [`SDL_GetNumCameraDrivers()`] - 1.
    ///
    /// ## Return value
    /// Returns the name of the camera driver at the requested index, or NULL if
    ///   an invalid index was specified.
    ///
    /// ## Thread safety
    /// It is safe to call this function from any thread.
    ///
    /// ## Availability
    /// This function is available since SDL 3.2.0.
    ///
    /// ## See also
    /// - [`SDL_GetNumCameraDrivers`]
    pub fn SDL_GetCameraDriver(index: ::core::ffi::c_int) -> *const ::core::ffi::c_char;
}

unsafe extern "C" {
    /// Get the name of the current camera driver.
    ///
    /// The names of drivers are all simple, low-ASCII identifiers, like "v4l2",
    /// "coremedia" or "android". These never have Unicode characters, and are not
    /// meant to be proper names.
    ///
    /// ## Return value
    /// Returns the name of the current camera driver or NULL if no driver has
    ///   been initialized.
    ///
    /// ## Thread safety
    /// It is safe to call this function from any thread.
    ///
    /// ## Availability
    /// This function is available since SDL 3.2.0.
    pub fn SDL_GetCurrentCameraDriver() -> *const ::core::ffi::c_char;
}

unsafe extern "C" {
    /// Get a list of currently connected camera devices.
    ///
    /// ## Parameters
    /// - `count`: a pointer filled in with the number of cameras returned, may
    ///   be NULL.
    ///
    /// ## Return value
    /// Returns a 0 terminated array of camera instance IDs or NULL on failure;
    ///   call [`SDL_GetError()`] for more information. This should be freed
    ///   with [`SDL_free()`] when it is no longer needed.
    ///
    /// ## Thread safety
    /// It is safe to call this function from any thread.
    ///
    /// ## Availability
    /// This function is available since SDL 3.2.0.
    ///
    /// ## See also
    /// - [`SDL_OpenCamera`]
    pub fn SDL_GetCameras(count: *mut ::core::ffi::c_int) -> *mut SDL_CameraID;
}

unsafe extern "C" {
    /// Get the list of native formats/sizes a camera supports.
    ///
    /// This returns a list of all formats and frame sizes that a specific camera
    /// can offer. This is useful if your app can accept a variety of image formats
    /// and sizes and so want to find the optimal spec that doesn't require
    /// conversion.
    ///
    /// This function isn't strictly required; if you call [`SDL_OpenCamera`] with a
    /// NULL spec, SDL will choose a native format for you, and if you instead
    /// specify a desired format, it will transparently convert to the requested
    /// format on your behalf.
    ///
    /// If `count` is not NULL, it will be filled with the number of elements in
    /// the returned array.
    ///
    /// Note that it's legal for a camera to supply an empty list. This is what
    /// will happen on Emscripten builds, since that platform won't tell _anything_
    /// about available cameras until you've opened one, and won't even tell if
    /// there _is_ a camera until the user has given you permission to check
    /// through a scary warning popup.
    ///
    /// ## Parameters
    /// - `instance_id`: the camera device instance ID.
    /// - `count`: a pointer filled in with the number of elements in the list,
    ///   may be NULL.
    ///
    /// ## Return value
    /// Returns a NULL terminated array of pointers to [`SDL_CameraSpec`] or NULL on
    ///   failure; call [`SDL_GetError()`] for more information. This is a
    ///   single allocation that should be freed with [`SDL_free()`] when it is
    ///   no longer needed.
    ///
    /// ## Thread safety
    /// It is safe to call this function from any thread.
    ///
    /// ## Availability
    /// This function is available since SDL 3.2.0.
    ///
    /// ## See also
    /// - [`SDL_GetCameras`]
    /// - [`SDL_OpenCamera`]
    pub fn SDL_GetCameraSupportedFormats(
        instance_id: SDL_CameraID,
        count: *mut ::core::ffi::c_int,
    ) -> *mut *mut SDL_CameraSpec;
}

unsafe extern "C" {
    /// Get the human-readable device name for a camera.
    ///
    /// ## Parameters
    /// - `instance_id`: the camera device instance ID.
    ///
    /// ## Return value
    /// Returns a human-readable device name or NULL on failure; call
    ///   [`SDL_GetError()`] for more information.
    ///
    /// ## Thread safety
    /// It is safe to call this function from any thread.
    ///
    /// ## Availability
    /// This function is available since SDL 3.2.0.
    ///
    /// ## See also
    /// - [`SDL_GetCameras`]
    pub fn SDL_GetCameraName(instance_id: SDL_CameraID) -> *const ::core::ffi::c_char;
}

unsafe extern "C" {
    /// Get the position of the camera in relation to the system.
    ///
    /// Most platforms will report UNKNOWN, but mobile devices, like phones, can
    /// often make a distinction between cameras on the front of the device (that
    /// points towards the user, for taking "selfies") and cameras on the back (for
    /// filming in the direction the user is facing).
    ///
    /// ## Parameters
    /// - `instance_id`: the camera device instance ID.
    ///
    /// ## Return value
    /// Returns the position of the camera on the system hardware.
    ///
    /// ## Thread safety
    /// It is safe to call this function from any thread.
    ///
    /// ## Availability
    /// This function is available since SDL 3.2.0.
    ///
    /// ## See also
    /// - [`SDL_GetCameras`]
    pub fn SDL_GetCameraPosition(instance_id: SDL_CameraID) -> SDL_CameraPosition;
}

unsafe extern "C" {
    /// Open a video recording device (a "camera").
    ///
    /// You can open the device with any reasonable spec, and if the hardware can't
    /// directly support it, it will convert data seamlessly to the requested
    /// format. This might incur overhead, including scaling of image data.
    ///
    /// If you would rather accept whatever format the device offers, you can pass
    /// a NULL spec here and it will choose one for you (and you can use
    /// SDL_Surface's conversion/scaling functions directly if necessary).
    ///
    /// You can call [`SDL_GetCameraFormat()`] to get the actual data format if passing
    /// a NULL spec here. You can see the exact specs a device can support without
    /// conversion with [`SDL_GetCameraSupportedFormats()`].
    ///
    /// SDL will not attempt to emulate framerate; it will try to set the hardware
    /// to the rate closest to the requested speed, but it won't attempt to limit
    /// or duplicate frames artificially; call [`SDL_GetCameraFormat()`] to see the
    /// actual framerate of the opened the device, and check your timestamps if
    /// this is crucial to your app!
    ///
    /// Note that the camera is not usable until the user approves its use! On some
    /// platforms, the operating system will prompt the user to permit access to
    /// the camera, and they can choose Yes or No at that point. Until they do, the
    /// camera will not be usable. The app should either wait for an
    /// [`SDL_EVENT_CAMERA_DEVICE_APPROVED`] (or [`SDL_EVENT_CAMERA_DEVICE_DENIED`]) event,
    /// or poll [`SDL_GetCameraPermissionState()`] occasionally until it returns
    /// non-zero. On platforms that don't require explicit user approval (and
    /// perhaps in places where the user previously permitted access), the approval
    /// event might come immediately, but it might come seconds, minutes, or hours
    /// later!
    ///
    /// ## Parameters
    /// - `instance_id`: the camera device instance ID.
    /// - `spec`: the desired format for data the device will provide. Can be
    ///   NULL.
    ///
    /// ## Return value
    /// Returns an [`SDL_Camera`] object or NULL on failure; call [`SDL_GetError()`] for
    ///   more information.
    ///
    /// ## Thread safety
    /// It is safe to call this function from any thread.
    ///
    /// ## Availability
    /// This function is available since SDL 3.2.0.
    ///
    /// ## See also
    /// - [`SDL_GetCameras`]
    /// - [`SDL_GetCameraFormat`]
    pub fn SDL_OpenCamera(
        instance_id: SDL_CameraID,
        spec: *const SDL_CameraSpec,
    ) -> *mut SDL_Camera;
}

unsafe extern "C" {
    /// Query if camera access has been approved by the user.
    ///
    /// Cameras will not function between when the device is opened by the app and
    /// when the user permits access to the hardware. On some platforms, this
    /// presents as a popup dialog where the user has to explicitly approve access;
    /// on others the approval might be implicit and not alert the user at all.
    ///
    /// This function can be used to check the status of that approval. It will
    /// return [`SDL_CAMERA_PERMISSION_STATE_PENDING`] if waiting for user response,
    /// [`SDL_CAMERA_PERMISSION_STATE_APPROVED`] if the camera is approved for use, and
    /// [`SDL_CAMERA_PERMISSION_STATE_DENIED`] if the user denied access.
    ///
    /// Instead of polling with this function, you can wait for a
    /// [`SDL_EVENT_CAMERA_DEVICE_APPROVED`] (or [`SDL_EVENT_CAMERA_DEVICE_DENIED`]) event
    /// in the standard SDL event loop, which is guaranteed to be sent once when
    /// permission to use the camera is decided.
    ///
    /// If a camera is declined, there's nothing to be done but call
    /// [`SDL_CloseCamera()`] to dispose of it.
    ///
    /// ## Parameters
    /// - `camera`: the opened camera device to query.
    ///
    /// ## Return value
    /// Returns an [`SDL_CameraPermissionState`] value indicating if access is
    ///   granted, or [`SDL_CAMERA_PERMISSION_STATE_PENDING`] if the decision
    ///   is still pending.
    ///
    /// ## Thread safety
    /// It is safe to call this function from any thread.
    ///
    /// ## Availability
    /// This function is available since SDL 3.2.0.
    ///
    /// ## See also
    /// - [`SDL_OpenCamera`]
    /// - [`SDL_CloseCamera`]
    pub fn SDL_GetCameraPermissionState(camera: *mut SDL_Camera) -> SDL_CameraPermissionState;
}

unsafe extern "C" {
    /// Get the instance ID of an opened camera.
    ///
    /// ## Parameters
    /// - `camera`: an [`SDL_Camera`] to query.
    ///
    /// ## Return value
    /// Returns the instance ID of the specified camera on success or 0 on
    ///   failure; call [`SDL_GetError()`] for more information.
    ///
    /// ## Thread safety
    /// It is safe to call this function from any thread.
    ///
    /// ## Availability
    /// This function is available since SDL 3.2.0.
    ///
    /// ## See also
    /// - [`SDL_OpenCamera`]
    pub fn SDL_GetCameraID(camera: *mut SDL_Camera) -> SDL_CameraID;
}

unsafe extern "C" {
    /// Get the properties associated with an opened camera.
    ///
    /// ## Parameters
    /// - `camera`: the [`SDL_Camera`] obtained from [`SDL_OpenCamera()`].
    ///
    /// ## Return value
    /// Returns a valid property ID on success or 0 on failure; call
    ///   [`SDL_GetError()`] for more information.
    ///
    /// ## Thread safety
    /// It is safe to call this function from any thread.
    ///
    /// ## Availability
    /// This function is available since SDL 3.2.0.
    pub fn SDL_GetCameraProperties(camera: *mut SDL_Camera) -> SDL_PropertiesID;
}

unsafe extern "C" {
    /// Get the spec that a camera is using when generating images.
    ///
    /// Note that this might not be the native format of the hardware, as SDL might
    /// be converting to this format behind the scenes.
    ///
    /// If the system is waiting for the user to approve access to the camera, as
    /// some platforms require, this will return false, but this isn't necessarily
    /// a fatal error; you should either wait for an
    /// [`SDL_EVENT_CAMERA_DEVICE_APPROVED`] (or [`SDL_EVENT_CAMERA_DEVICE_DENIED`]) event,
    /// or poll [`SDL_GetCameraPermissionState()`] occasionally until it returns
    /// non-zero.
    ///
    /// ## Parameters
    /// - `camera`: opened camera device.
    /// - `spec`: the [`SDL_CameraSpec`] to be initialized by this function.
    ///
    /// ## Return value
    /// Returns true on success or false on failure; call [`SDL_GetError()`] for more
    ///   information.
    ///
    /// ## Thread safety
    /// It is safe to call this function from any thread.
    ///
    /// ## Availability
    /// This function is available since SDL 3.2.0.
    ///
    /// ## See also
    /// - [`SDL_OpenCamera`]
    pub fn SDL_GetCameraFormat(
        camera: *mut SDL_Camera,
        spec: *mut SDL_CameraSpec,
    ) -> ::core::primitive::bool;
}

unsafe extern "C" {
    /// Acquire a frame.
    ///
    /// The frame is a memory pointer to the image data, whose size and format are
    /// given by the spec requested when opening the device.
    ///
    /// This is a non blocking API. If there is a frame available, a non-NULL
    /// surface is returned, and timestampNS will be filled with a non-zero value.
    ///
    /// Note that an error case can also return NULL, but a NULL by itself is
    /// normal and just signifies that a new frame is not yet available. Note that
    /// even if a camera device fails outright (a USB camera is unplugged while in
    /// use, etc), SDL will send an event separately to notify the app, but
    /// continue to provide blank frames at ongoing intervals until
    /// [`SDL_CloseCamera()`] is called, so real failure here is almost always an out
    /// of memory condition.
    ///
    /// After use, the frame should be released with [`SDL_ReleaseCameraFrame()`]. If
    /// you don't do this, the system may stop providing more video!
    ///
    /// Do not call [`SDL_DestroySurface()`] on the returned surface! It must be given
    /// back to the camera subsystem with SDL_ReleaseCameraFrame!
    ///
    /// If the system is waiting for the user to approve access to the camera, as
    /// some platforms require, this will return NULL (no frames available); you
    /// should either wait for an [`SDL_EVENT_CAMERA_DEVICE_APPROVED`] (or
    /// [`SDL_EVENT_CAMERA_DEVICE_DENIED`]) event, or poll
    /// [`SDL_GetCameraPermissionState()`] occasionally until it returns non-zero.
    ///
    /// ## Parameters
    /// - `camera`: opened camera device.
    /// - `timestampNS`: a pointer filled in with the frame's timestamp, or 0 on
    ///   error. Can be NULL.
    ///
    /// ## Return value
    /// Returns a new frame of video on success, NULL if none is currently
    ///   available.
    ///
    /// ## Thread safety
    /// It is safe to call this function from any thread.
    ///
    /// ## Availability
    /// This function is available since SDL 3.2.0.
    ///
    /// ## See also
    /// - [`SDL_ReleaseCameraFrame`]
    pub fn SDL_AcquireCameraFrame(
        camera: *mut SDL_Camera,
        timestampNS: *mut Uint64,
    ) -> *mut SDL_Surface;
}

unsafe extern "C" {
    /// Release a frame of video acquired from a camera.
    ///
    /// Let the back-end re-use the internal buffer for camera.
    ///
    /// This function _must_ be called only on surface objects returned by
    /// [`SDL_AcquireCameraFrame()`]. This function should be called as quickly as
    /// possible after acquisition, as SDL keeps a small FIFO queue of surfaces for
    /// video frames; if surfaces aren't released in a timely manner, SDL may drop
    /// upcoming video frames from the camera.
    ///
    /// If the app needs to keep the surface for a significant time, they should
    /// make a copy of it and release the original.
    ///
    /// The app should not use the surface again after calling this function;
    /// assume the surface is freed and the pointer is invalid.
    ///
    /// ## Parameters
    /// - `camera`: opened camera device.
    /// - `frame`: the video frame surface to release.
    ///
    /// ## Thread safety
    /// It is safe to call this function from any thread.
    ///
    /// ## Availability
    /// This function is available since SDL 3.2.0.
    ///
    /// ## See also
    /// - [`SDL_AcquireCameraFrame`]
    pub fn SDL_ReleaseCameraFrame(camera: *mut SDL_Camera, frame: *mut SDL_Surface);
}

unsafe extern "C" {
    /// Use this function to shut down camera processing and close the camera
    /// device.
    ///
    /// ## Parameters
    /// - `camera`: opened camera device.
    ///
    /// ## Thread safety
    /// It is safe to call this function from any thread, but no
    ///   thread may reference `device` once this function is called.
    ///
    /// ## Availability
    /// This function is available since SDL 3.2.0.
    ///
    /// ## See also
    /// - [`SDL_OpenCamera`]
    pub fn SDL_CloseCamera(camera: *mut SDL_Camera);
}

/// The opaque structure used to identify an opened SDL camera.
///
/// ## Availability
/// This struct is available since SDL 3.2.0.
#[repr(C)]
pub struct SDL_Camera {
    _opaque: [::core::primitive::u8; 0],
}

#[cfg(doc)]
use crate::everything::*;
