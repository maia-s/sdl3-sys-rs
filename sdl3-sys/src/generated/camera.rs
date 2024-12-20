//! Video capture for the SDL library.
//!
//! This API lets apps read input from video sources, like webcams. Camera
//! devices can be enumerated, queried, and opened. Once opened, it will
//! provide [`SDL_Surface`] objects as new frames of video come in. These surfaces
//! can be uploaded to an [`SDL_Texture`] or processed as pixels in memory.

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
/// ### Availability
/// This datatype is available since SDL 3.1.3.
///
/// ### See also
/// - [`SDL_GetCameras`]
pub type SDL_CameraID = Uint32;

/// The details of an output format for a camera device.
///
/// Cameras often support multiple formats; each one will be encapsulated in
/// this struct.
///
/// ### Availability
/// This struct is available since SDL 3.1.3.
///
/// ### See also
/// - [`SDL_GetCameraSupportedFormats`]
/// - [`SDL_GetCameraFormat`]
#[repr(C)]
#[derive(Clone, Copy)]
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
    /// Frame rate demoninator ((num / denom) == FPS, (denom / num) == duration in seconds)
    pub framerate_denominator: ::core::ffi::c_int,
}

/// The position of camera in relation to system device.
///
/// ### Availability
/// This enum is available since SDL 3.1.3.
///
/// ### See also
/// - [`SDL_GetCameraPosition`]
///
/// ### Known values (`sdl3-sys`)
/// | Associated constant | Global constant | Description |
/// | ------------------- | --------------- | ----------- |
/// | [`UNKNOWN`](SDL_CameraPosition::UNKNOWN) | [`SDL_CAMERA_POSITION_UNKNOWN`] | |
/// | [`FRONT_FACING`](SDL_CameraPosition::FRONT_FACING) | [`SDL_CAMERA_POSITION_FRONT_FACING`] | |
/// | [`BACK_FACING`](SDL_CameraPosition::BACK_FACING) | [`SDL_CAMERA_POSITION_BACK_FACING`] | |
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct SDL_CameraPosition(pub ::core::ffi::c_int);

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
    pub const UNKNOWN: Self = Self(0);
    pub const FRONT_FACING: Self = Self(1);
    pub const BACK_FACING: Self = Self(2);
}

pub const SDL_CAMERA_POSITION_UNKNOWN: SDL_CameraPosition = SDL_CameraPosition::UNKNOWN;
pub const SDL_CAMERA_POSITION_FRONT_FACING: SDL_CameraPosition = SDL_CameraPosition::FRONT_FACING;
pub const SDL_CAMERA_POSITION_BACK_FACING: SDL_CameraPosition = SDL_CameraPosition::BACK_FACING;

extern "C" {
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
    /// ### Return value
    /// Returns the number of built-in camera drivers.
    ///
    /// ### Thread safety
    /// It is safe to call this function from any thread.
    ///
    /// ### Availability
    /// This function is available since SDL 3.1.3.
    ///
    /// ### See also
    /// - [`SDL_GetCameraDriver`]
    pub fn SDL_GetNumCameraDrivers() -> ::core::ffi::c_int;
}

extern "C" {
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
    /// ### Parameters
    /// - `index`: the index of the camera driver; the value ranges from 0 to
    ///   [`SDL_GetNumCameraDrivers()`] - 1.
    ///
    /// ### Return value
    /// Returns the name of the camera driver at the requested index, or NULL if
    ///   an invalid index was specified.
    ///
    /// ### Thread safety
    /// It is safe to call this function from any thread.
    ///
    /// ### Availability
    /// This function is available since SDL 3.1.3.
    ///
    /// ### See also
    /// - [`SDL_GetNumCameraDrivers`]
    pub fn SDL_GetCameraDriver(index: ::core::ffi::c_int) -> *const ::core::ffi::c_char;
}

extern "C" {
    /// Get the name of the current camera driver.
    ///
    /// The names of drivers are all simple, low-ASCII identifiers, like "v4l2",
    /// "coremedia" or "android". These never have Unicode characters, and are not
    /// meant to be proper names.
    ///
    /// ### Return value
    /// Returns the name of the current camera driver or NULL if no driver has
    ///   been initialized.
    ///
    /// ### Thread safety
    /// It is safe to call this function from any thread.
    ///
    /// ### Availability
    /// This function is available since SDL 3.1.3.
    pub fn SDL_GetCurrentCameraDriver() -> *const ::core::ffi::c_char;
}

extern "C" {
    /// Get a list of currently connected camera devices.
    ///
    /// ### Parameters
    /// - `count`: a pointer filled in with the number of cameras returned, may
    ///   be NULL.
    ///
    /// ### Return value
    /// Returns a 0 terminated array of camera instance IDs or NULL on failure;
    ///   call [`SDL_GetError()`] for more information. This should be freed
    ///   with [`SDL_free()`] when it is no longer needed.
    ///
    /// ### Thread safety
    /// It is safe to call this function from any thread.
    ///
    /// ### Availability
    /// This function is available since SDL 3.1.3.
    ///
    /// ### See also
    /// - [`SDL_OpenCamera`]
    pub fn SDL_GetCameras(count: *mut ::core::ffi::c_int) -> *mut SDL_CameraID;
}

extern "C" {
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
    /// ### Parameters
    /// - `devid`: the camera device instance ID to query.
    /// - `count`: a pointer filled in with the number of elements in the list,
    ///   may be NULL.
    ///
    /// ### Return value
    /// Returns a NULL terminated array of pointers to [`SDL_CameraSpec`] or NULL on
    ///   failure; call [`SDL_GetError()`] for more information. This is a
    ///   single allocation that should be freed with [`SDL_free()`] when it is
    ///   no longer needed.
    ///
    /// ### Thread safety
    /// It is safe to call this function from any thread.
    ///
    /// ### Availability
    /// This function is available since SDL 3.1.3.
    ///
    /// ### See also
    /// - [`SDL_GetCameras`]
    /// - [`SDL_OpenCamera`]
    pub fn SDL_GetCameraSupportedFormats(
        devid: SDL_CameraID,
        count: *mut ::core::ffi::c_int,
    ) -> *mut *mut SDL_CameraSpec;
}

extern "C" {
    /// Get the human-readable device name for a camera.
    ///
    /// ### Parameters
    /// - `instance_id`: the camera device instance ID.
    ///
    /// ### Return value
    /// Returns a human-readable device name or NULL on failure; call
    ///   [`SDL_GetError()`] for more information.
    ///
    /// ### Thread safety
    /// It is safe to call this function from any thread.
    ///
    /// ### Availability
    /// This function is available since SDL 3.1.3.
    ///
    /// ### See also
    /// - [`SDL_GetCameras`]
    pub fn SDL_GetCameraName(instance_id: SDL_CameraID) -> *const ::core::ffi::c_char;
}

extern "C" {
    /// Get the position of the camera in relation to the system.
    ///
    /// Most platforms will report UNKNOWN, but mobile devices, like phones, can
    /// often make a distinction between cameras on the front of the device (that
    /// points towards the user, for taking "selfies") and cameras on the back (for
    /// filming in the direction the user is facing).
    ///
    /// ### Parameters
    /// - `instance_id`: the camera device instance ID.
    ///
    /// ### Return value
    /// Returns the position of the camera on the system hardware.
    ///
    /// ### Thread safety
    /// It is safe to call this function from any thread.
    ///
    /// ### Availability
    /// This function is available since SDL 3.1.3.
    ///
    /// ### See also
    /// - [`SDL_GetCameras`]
    pub fn SDL_GetCameraPosition(instance_id: SDL_CameraID) -> SDL_CameraPosition;
}

extern "C" {
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
    /// ### Parameters
    /// - `instance_id`: the camera device instance ID.
    /// - `spec`: the desired format for data the device will provide. Can be
    ///   NULL.
    ///
    /// ### Return value
    /// Returns an [`SDL_Camera`] object or NULL on failure; call [`SDL_GetError()`] for
    ///   more information.
    ///
    /// ### Thread safety
    /// It is safe to call this function from any thread.
    ///
    /// ### Availability
    /// This function is available since SDL 3.1.3.
    ///
    /// ### See also
    /// - [`SDL_GetCameras`]
    /// - [`SDL_GetCameraFormat`]
    pub fn SDL_OpenCamera(
        instance_id: SDL_CameraID,
        spec: *const SDL_CameraSpec,
    ) -> *mut SDL_Camera;
}

extern "C" {
    /// Query if camera access has been approved by the user.
    ///
    /// Cameras will not function between when the device is opened by the app and
    /// when the user permits access to the hardware. On some platforms, this
    /// presents as a popup dialog where the user has to explicitly approve access;
    /// on others the approval might be implicit and not alert the user at all.
    ///
    /// This function can be used to check the status of that approval. It will
    /// return 0 if still waiting for user response, 1 if the camera is approved
    /// for use, and -1 if the user denied access.
    ///
    /// Instead of polling with this function, you can wait for a
    /// [`SDL_EVENT_CAMERA_DEVICE_APPROVED`] (or [`SDL_EVENT_CAMERA_DEVICE_DENIED`]) event
    /// in the standard SDL event loop, which is guaranteed to be sent once when
    /// permission to use the camera is decided.
    ///
    /// If a camera is declined, there's nothing to be done but call
    /// [`SDL_CloseCamera()`] to dispose of it.
    ///
    /// ### Parameters
    /// - `camera`: the opened camera device to query.
    ///
    /// ### Return value
    /// Returns -1 if user denied access to the camera, 1 if user approved access,
    ///   0 if no decision has been made yet.
    ///
    /// ### Thread safety
    /// It is safe to call this function from any thread.
    ///
    /// ### Availability
    /// This function is available since SDL 3.1.3.
    ///
    /// ### See also
    /// - [`SDL_OpenCamera`]
    /// - [`SDL_CloseCamera`]
    pub fn SDL_GetCameraPermissionState(camera: *mut SDL_Camera) -> ::core::ffi::c_int;
}

extern "C" {
    /// Get the instance ID of an opened camera.
    ///
    /// ### Parameters
    /// - `camera`: an [`SDL_Camera`] to query.
    ///
    /// ### Return value
    /// Returns the instance ID of the specified camera on success or 0 on
    ///   failure; call [`SDL_GetError()`] for more information.
    ///
    /// ### Thread safety
    /// It is safe to call this function from any thread.
    ///
    /// ### Availability
    /// This function is available since SDL 3.1.3.
    ///
    /// ### See also
    /// - [`SDL_OpenCamera`]
    pub fn SDL_GetCameraID(camera: *mut SDL_Camera) -> SDL_CameraID;
}

extern "C" {
    /// Get the properties associated with an opened camera.
    ///
    /// ### Parameters
    /// - `camera`: the [`SDL_Camera`] obtained from [`SDL_OpenCamera()`].
    ///
    /// ### Return value
    /// Returns a valid property ID on success or 0 on failure; call
    ///   [`SDL_GetError()`] for more information.
    ///
    /// ### Thread safety
    /// It is safe to call this function from any thread.
    ///
    /// ### Availability
    /// This function is available since SDL 3.1.3.
    pub fn SDL_GetCameraProperties(camera: *mut SDL_Camera) -> SDL_PropertiesID;
}

extern "C" {
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
    /// ### Parameters
    /// - `camera`: opened camera device.
    /// - `spec`: the [`SDL_CameraSpec`] to be initialized by this function.
    ///
    /// ### Return value
    /// Returns true on success or false on failure; call [`SDL_GetError()`] for more
    ///   information.
    ///
    /// ### Thread safety
    /// It is safe to call this function from any thread.
    ///
    /// ### Availability
    /// This function is available since SDL 3.1.3.
    ///
    /// ### See also
    /// - [`SDL_OpenCamera`]
    pub fn SDL_GetCameraFormat(
        camera: *mut SDL_Camera,
        spec: *mut SDL_CameraSpec,
    ) -> ::core::primitive::bool;
}

extern "C" {
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
    /// Do not call [`SDL_FreeSurface()`] on the returned surface! It must be given
    /// back to the camera subsystem with SDL_ReleaseCameraFrame!
    ///
    /// If the system is waiting for the user to approve access to the camera, as
    /// some platforms require, this will return NULL (no frames available); you
    /// should either wait for an [`SDL_EVENT_CAMERA_DEVICE_APPROVED`] (or
    /// [`SDL_EVENT_CAMERA_DEVICE_DENIED`]) event, or poll
    /// [`SDL_GetCameraPermissionState()`] occasionally until it returns non-zero.
    ///
    /// ### Parameters
    /// - `camera`: opened camera device.
    /// - `timestampNS`: a pointer filled in with the frame's timestamp, or 0 on
    ///   error. Can be NULL.
    ///
    /// ### Return value
    /// Returns a new frame of video on success, NULL if none is currently
    ///   available.
    ///
    /// ### Thread safety
    /// It is safe to call this function from any thread.
    ///
    /// ### Availability
    /// This function is available since SDL 3.1.3.
    ///
    /// ### See also
    /// - [`SDL_ReleaseCameraFrame`]
    pub fn SDL_AcquireCameraFrame(
        camera: *mut SDL_Camera,
        timestampNS: *mut Uint64,
    ) -> *mut SDL_Surface;
}

extern "C" {
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
    /// ### Parameters
    /// - `camera`: opened camera device.
    /// - `frame`: the video frame surface to release.
    ///
    /// ### Thread safety
    /// It is safe to call this function from any thread.
    ///
    /// ### Availability
    /// This function is available since SDL 3.1.3.
    ///
    /// ### See also
    /// - [`SDL_AcquireCameraFrame`]
    pub fn SDL_ReleaseCameraFrame(camera: *mut SDL_Camera, frame: *mut SDL_Surface);
}

extern "C" {
    /// Use this function to shut down camera processing and close the camera
    /// device.
    ///
    /// ### Parameters
    /// - `camera`: opened camera device.
    ///
    /// ### Thread safety
    /// It is safe to call this function from any thread, but no
    ///   thread may reference `device` once this function is called.
    ///
    /// ### Availability
    /// This function is available since SDL 3.1.3.
    ///
    /// ### See also
    /// - [`SDL_OpenCameraWithSpec`]
    /// - [`SDL_OpenCamera`]
    pub fn SDL_CloseCamera(camera: *mut SDL_Camera);
}

/// The opaque structure used to identify an opened SDL camera.
///
/// ### Availability
/// This struct is available since SDL 3.1.3.
#[repr(C)]
pub struct SDL_Camera {
    _opaque: [::core::primitive::u8; 0],
}

#[cfg(doc)]
use crate::everything::*;
