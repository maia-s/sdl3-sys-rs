//! A property is a variable that can be created and retrieved by name at
//! runtime.
//!
//! All properties are part of a property group ([`SDL_PropertiesID`]). A property
//! group can be created with the [`SDL_CreateProperties`] function and destroyed
//! with the [`SDL_DestroyProperties`] function.
//!
//! Properties can be added to and retrieved from a property group through the
//! following functions:
//!
//! - [`SDL_SetPointerProperty`] and [`SDL_GetPointerProperty`] operate on `void*`
//!   pointer types.
//! - [`SDL_SetStringProperty`] and [`SDL_GetStringProperty`] operate on string types.
//! - [`SDL_SetNumberProperty`] and [`SDL_GetNumberProperty`] operate on signed 64-bit
//!   integer types.
//! - [`SDL_SetFloatProperty`] and [`SDL_GetFloatProperty`] operate on floating point
//!   types.
//! - [`SDL_SetBooleanProperty`] and [`SDL_GetBooleanProperty`] operate on boolean
//!   types.
//!
//! Properties can be removed from a group by using [`SDL_ClearProperty`].

use super::stdinc::*;

use super::error::*;

/// SDL properties ID
///
/// ### Availability
/// This datatype is available since SDL 3.1.3.
pub type SDL_PropertiesID = Uint32;

/// SDL property type
///
/// ### Availability
/// This enum is available since SDL 3.1.3.
///
/// ### Known values (`sdl3-sys`)
/// | Associated constant | Global constant | Description |
/// | ------------------- | --------------- | ----------- |
/// | [`INVALID`](SDL_PropertyType::INVALID) | [`SDL_PROPERTY_TYPE_INVALID`] | |
/// | [`POINTER`](SDL_PropertyType::POINTER) | [`SDL_PROPERTY_TYPE_POINTER`] | |
/// | [`STRING`](SDL_PropertyType::STRING) | [`SDL_PROPERTY_TYPE_STRING`] | |
/// | [`NUMBER`](SDL_PropertyType::NUMBER) | [`SDL_PROPERTY_TYPE_NUMBER`] | |
/// | [`FLOAT`](SDL_PropertyType::FLOAT) | [`SDL_PROPERTY_TYPE_FLOAT`] | |
/// | [`BOOLEAN`](SDL_PropertyType::BOOLEAN) | [`SDL_PROPERTY_TYPE_BOOLEAN`] | |
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct SDL_PropertyType(pub ::core::ffi::c_int);

impl From<SDL_PropertyType> for ::core::ffi::c_int {
    #[inline(always)]
    fn from(value: SDL_PropertyType) -> Self {
        value.0
    }
}

#[cfg(feature = "debug-impls")]
impl ::core::fmt::Debug for SDL_PropertyType {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        #[allow(unreachable_patterns)]
        f.write_str(match *self {
            Self::INVALID => "SDL_PROPERTY_TYPE_INVALID",
            Self::POINTER => "SDL_PROPERTY_TYPE_POINTER",
            Self::STRING => "SDL_PROPERTY_TYPE_STRING",
            Self::NUMBER => "SDL_PROPERTY_TYPE_NUMBER",
            Self::FLOAT => "SDL_PROPERTY_TYPE_FLOAT",
            Self::BOOLEAN => "SDL_PROPERTY_TYPE_BOOLEAN",

            _ => return write!(f, "SDL_PropertyType({})", self.0),
        })
    }
}

impl SDL_PropertyType {
    pub const INVALID: Self = Self(0);
    pub const POINTER: Self = Self(1);
    pub const STRING: Self = Self(2);
    pub const NUMBER: Self = Self(3);
    pub const FLOAT: Self = Self(4);
    pub const BOOLEAN: Self = Self(5);
}

pub const SDL_PROPERTY_TYPE_INVALID: SDL_PropertyType = SDL_PropertyType::INVALID;
pub const SDL_PROPERTY_TYPE_POINTER: SDL_PropertyType = SDL_PropertyType::POINTER;
pub const SDL_PROPERTY_TYPE_STRING: SDL_PropertyType = SDL_PropertyType::STRING;
pub const SDL_PROPERTY_TYPE_NUMBER: SDL_PropertyType = SDL_PropertyType::NUMBER;
pub const SDL_PROPERTY_TYPE_FLOAT: SDL_PropertyType = SDL_PropertyType::FLOAT;
pub const SDL_PROPERTY_TYPE_BOOLEAN: SDL_PropertyType = SDL_PropertyType::BOOLEAN;

extern "C" {
    /// Get the global SDL properties.
    ///
    /// ### Return value
    /// Returns a valid property ID on success or 0 on failure; call
    ///   [`SDL_GetError()`] for more information.
    ///
    /// ### Availability
    /// This function is available since SDL 3.1.3.
    pub fn SDL_GetGlobalProperties() -> SDL_PropertiesID;
}

extern "C" {
    /// Create a group of properties.
    ///
    /// All properties are automatically destroyed when [`SDL_Quit()`] is called.
    ///
    /// ### Return value
    /// Returns an ID for a new group of properties, or 0 on failure; call
    ///   [`SDL_GetError()`] for more information.
    ///
    /// ### Thread safety
    /// It is safe to call this function from any thread.
    ///
    /// ### Availability
    /// This function is available since SDL 3.1.3.
    ///
    /// ### See also
    /// - [`SDL_DestroyProperties`]
    pub fn SDL_CreateProperties() -> SDL_PropertiesID;
}

extern "C" {
    /// Copy a group of properties.
    ///
    /// Copy all the properties from one group of properties to another, with the
    /// exception of properties requiring cleanup (set using
    /// [`SDL_SetPointerPropertyWithCleanup()`]), which will not be copied. Any
    /// property that already exists on `dst` will be overwritten.
    ///
    /// ### Parameters
    /// - `src`: the properties to copy.
    /// - `dst`: the destination properties.
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
    pub fn SDL_CopyProperties(
        src: SDL_PropertiesID,
        dst: SDL_PropertiesID,
    ) -> ::core::primitive::bool;
}

extern "C" {
    /// Lock a group of properties.
    ///
    /// Obtain a multi-threaded lock for these properties. Other threads will wait
    /// while trying to lock these properties until they are unlocked. Properties
    /// must be unlocked before they are destroyed.
    ///
    /// The lock is automatically taken when setting individual properties, this
    /// function is only needed when you want to set several properties atomically
    /// or want to guarantee that properties being queried aren't freed in another
    /// thread.
    ///
    /// ### Parameters
    /// - `props`: the properties to lock.
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
    /// - [`SDL_UnlockProperties`]
    pub fn SDL_LockProperties(props: SDL_PropertiesID) -> ::core::primitive::bool;
}

extern "C" {
    /// Unlock a group of properties.
    ///
    /// ### Parameters
    /// - `props`: the properties to unlock.
    ///
    /// ### Thread safety
    /// It is safe to call this function from any thread.
    ///
    /// ### Availability
    /// This function is available since SDL 3.1.3.
    ///
    /// ### See also
    /// - [`SDL_LockProperties`]
    pub fn SDL_UnlockProperties(props: SDL_PropertiesID);
}

/// A callback used to free resources when a property is deleted.
///
/// This should release any resources associated with `value` that are no
/// longer needed.
///
/// This callback is set per-property. Different properties in the same group
/// can have different cleanup callbacks.
///
/// This callback will be called _during_ [`SDL_SetPointerPropertyWithCleanup`] if
/// the function fails for any reason.
///
/// ### Parameters
/// - `userdata`: an app-defined pointer passed to the callback.
/// - `value`: the pointer assigned to the property to clean up.
///
/// ### Thread safety
/// This callback may fire without any locks held; if this is a
///   concern, the app should provide its own locking.
///
/// ### Availability
/// This datatype is available since SDL 3.1.3.
///
/// ### See also
/// - [`SDL_SetPointerPropertyWithCleanup`]
pub type SDL_CleanupPropertyCallback = ::core::option::Option<
    unsafe extern "C" fn(userdata: *mut ::core::ffi::c_void, value: *mut ::core::ffi::c_void),
>;

extern "C" {
    /// Set a pointer property in a group of properties with a cleanup function
    /// that is called when the property is deleted.
    ///
    /// The cleanup function is also called if setting the property fails for any
    /// reason.
    ///
    /// For simply setting basic data types, like numbers, bools, or strings, use
    /// [`SDL_SetNumberProperty`], [`SDL_SetBooleanProperty`], or [`SDL_SetStringProperty`]
    /// instead, as those functions will handle cleanup on your behalf. This
    /// function is only for more complex, custom data.
    ///
    /// ### Parameters
    /// - `props`: the properties to modify.
    /// - `name`: the name of the property to modify.
    /// - `value`: the new value of the property, or NULL to delete the property.
    /// - `cleanup`: the function to call when this property is deleted, or NULL
    ///   if no cleanup is necessary.
    /// - `userdata`: a pointer that is passed to the cleanup function.
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
    /// - [`SDL_GetPointerProperty`]
    /// - [`SDL_SetPointerProperty`]
    /// - [`SDL_CleanupPropertyCallback`]
    pub fn SDL_SetPointerPropertyWithCleanup(
        props: SDL_PropertiesID,
        name: *const ::core::ffi::c_char,
        value: *mut ::core::ffi::c_void,
        cleanup: SDL_CleanupPropertyCallback,
        userdata: *mut ::core::ffi::c_void,
    ) -> ::core::primitive::bool;
}

extern "C" {
    /// Set a pointer property in a group of properties.
    ///
    /// ### Parameters
    /// - `props`: the properties to modify.
    /// - `name`: the name of the property to modify.
    /// - `value`: the new value of the property, or NULL to delete the property.
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
    /// - [`SDL_GetPointerProperty`]
    /// - [`SDL_HasProperty`]
    /// - [`SDL_SetBooleanProperty`]
    /// - [`SDL_SetFloatProperty`]
    /// - [`SDL_SetNumberProperty`]
    /// - [`SDL_SetPointerPropertyWithCleanup`]
    /// - [`SDL_SetStringProperty`]
    pub fn SDL_SetPointerProperty(
        props: SDL_PropertiesID,
        name: *const ::core::ffi::c_char,
        value: *mut ::core::ffi::c_void,
    ) -> ::core::primitive::bool;
}

extern "C" {
    /// Set a string property in a group of properties.
    ///
    /// This function makes a copy of the string; the caller does not have to
    /// preserve the data after this call completes.
    ///
    /// ### Parameters
    /// - `props`: the properties to modify.
    /// - `name`: the name of the property to modify.
    /// - `value`: the new value of the property, or NULL to delete the property.
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
    /// - [`SDL_GetStringProperty`]
    pub fn SDL_SetStringProperty(
        props: SDL_PropertiesID,
        name: *const ::core::ffi::c_char,
        value: *const ::core::ffi::c_char,
    ) -> ::core::primitive::bool;
}

extern "C" {
    /// Set an integer property in a group of properties.
    ///
    /// ### Parameters
    /// - `props`: the properties to modify.
    /// - `name`: the name of the property to modify.
    /// - `value`: the new value of the property.
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
    /// - [`SDL_GetNumberProperty`]
    pub fn SDL_SetNumberProperty(
        props: SDL_PropertiesID,
        name: *const ::core::ffi::c_char,
        value: Sint64,
    ) -> ::core::primitive::bool;
}

extern "C" {
    /// Set a floating point property in a group of properties.
    ///
    /// ### Parameters
    /// - `props`: the properties to modify.
    /// - `name`: the name of the property to modify.
    /// - `value`: the new value of the property.
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
    /// - [`SDL_GetFloatProperty`]
    pub fn SDL_SetFloatProperty(
        props: SDL_PropertiesID,
        name: *const ::core::ffi::c_char,
        value: ::core::ffi::c_float,
    ) -> ::core::primitive::bool;
}

extern "C" {
    /// Set a boolean property in a group of properties.
    ///
    /// ### Parameters
    /// - `props`: the properties to modify.
    /// - `name`: the name of the property to modify.
    /// - `value`: the new value of the property.
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
    /// - [`SDL_GetBooleanProperty`]
    pub fn SDL_SetBooleanProperty(
        props: SDL_PropertiesID,
        name: *const ::core::ffi::c_char,
        value: ::core::primitive::bool,
    ) -> ::core::primitive::bool;
}

extern "C" {
    /// Return whether a property exists in a group of properties.
    ///
    /// ### Parameters
    /// - `props`: the properties to query.
    /// - `name`: the name of the property to query.
    ///
    /// ### Return value
    /// Returns true if the property exists, or false if it doesn't.
    ///
    /// ### Thread safety
    /// It is safe to call this function from any thread.
    ///
    /// ### Availability
    /// This function is available since SDL 3.1.3.
    ///
    /// ### See also
    /// - [`SDL_GetPropertyType`]
    pub fn SDL_HasProperty(
        props: SDL_PropertiesID,
        name: *const ::core::ffi::c_char,
    ) -> ::core::primitive::bool;
}

extern "C" {
    /// Get the type of a property in a group of properties.
    ///
    /// ### Parameters
    /// - `props`: the properties to query.
    /// - `name`: the name of the property to query.
    ///
    /// ### Return value
    /// Returns the type of the property, or [`SDL_PROPERTY_TYPE_INVALID`] if it is
    ///   not set.
    ///
    /// ### Thread safety
    /// It is safe to call this function from any thread.
    ///
    /// ### Availability
    /// This function is available since SDL 3.1.3.
    ///
    /// ### See also
    /// - [`SDL_HasProperty`]
    pub fn SDL_GetPropertyType(
        props: SDL_PropertiesID,
        name: *const ::core::ffi::c_char,
    ) -> SDL_PropertyType;
}

extern "C" {
    /// Get a pointer property from a group of properties.
    ///
    /// By convention, the names of properties that SDL exposes on objects will
    /// start with "SDL.", and properties that SDL uses internally will start with
    /// "SDL.internal.". These should be considered read-only and should not be
    /// modified by applications.
    ///
    /// ### Parameters
    /// - `props`: the properties to query.
    /// - `name`: the name of the property to query.
    /// - `default_value`: the default value of the property.
    ///
    /// ### Return value
    /// Returns the value of the property, or `default_value` if it is not set or
    ///   not a pointer property.
    ///
    /// ### Thread safety
    /// It is safe to call this function from any thread, although
    ///   the data returned is not protected and could potentially be
    ///   freed if you call [`SDL_SetPointerProperty()`] or
    ///   [`SDL_ClearProperty()`] on these properties from another thread.
    ///   If you need to avoid this, use [`SDL_LockProperties()`] and
    ///   [`SDL_UnlockProperties()`].
    ///
    /// ### Availability
    /// This function is available since SDL 3.1.3.
    ///
    /// ### See also
    /// - [`SDL_GetBooleanProperty`]
    /// - [`SDL_GetFloatProperty`]
    /// - [`SDL_GetNumberProperty`]
    /// - [`SDL_GetPropertyType`]
    /// - [`SDL_GetStringProperty`]
    /// - [`SDL_HasProperty`]
    /// - [`SDL_SetPointerProperty`]
    pub fn SDL_GetPointerProperty(
        props: SDL_PropertiesID,
        name: *const ::core::ffi::c_char,
        default_value: *mut ::core::ffi::c_void,
    ) -> *mut ::core::ffi::c_void;
}

extern "C" {
    /// Get a string property from a group of properties.
    ///
    /// ### Parameters
    /// - `props`: the properties to query.
    /// - `name`: the name of the property to query.
    /// - `default_value`: the default value of the property.
    ///
    /// ### Return value
    /// Returns the value of the property, or `default_value` if it is not set or
    ///   not a string property.
    ///
    /// ### Thread safety
    /// It is safe to call this function from any thread, although
    ///   the data returned is not protected and could potentially be
    ///   freed if you call [`SDL_SetStringProperty()`] or
    ///   [`SDL_ClearProperty()`] on these properties from another thread.
    ///   If you need to avoid this, use [`SDL_LockProperties()`] and
    ///   [`SDL_UnlockProperties()`].
    ///
    /// ### Availability
    /// This function is available since SDL 3.1.3.
    ///
    /// ### See also
    /// - [`SDL_GetPropertyType`]
    /// - [`SDL_HasProperty`]
    /// - [`SDL_SetStringProperty`]
    pub fn SDL_GetStringProperty(
        props: SDL_PropertiesID,
        name: *const ::core::ffi::c_char,
        default_value: *const ::core::ffi::c_char,
    ) -> *const ::core::ffi::c_char;
}

extern "C" {
    /// Get a number property from a group of properties.
    ///
    /// You can use [`SDL_GetPropertyType()`] to query whether the property exists and
    /// is a number property.
    ///
    /// ### Parameters
    /// - `props`: the properties to query.
    /// - `name`: the name of the property to query.
    /// - `default_value`: the default value of the property.
    ///
    /// ### Return value
    /// Returns the value of the property, or `default_value` if it is not set or
    ///   not a number property.
    ///
    /// ### Thread safety
    /// It is safe to call this function from any thread.
    ///
    /// ### Availability
    /// This function is available since SDL 3.1.3.
    ///
    /// ### See also
    /// - [`SDL_GetPropertyType`]
    /// - [`SDL_HasProperty`]
    /// - [`SDL_SetNumberProperty`]
    pub fn SDL_GetNumberProperty(
        props: SDL_PropertiesID,
        name: *const ::core::ffi::c_char,
        default_value: Sint64,
    ) -> Sint64;
}

extern "C" {
    /// Get a floating point property from a group of properties.
    ///
    /// You can use [`SDL_GetPropertyType()`] to query whether the property exists and
    /// is a floating point property.
    ///
    /// ### Parameters
    /// - `props`: the properties to query.
    /// - `name`: the name of the property to query.
    /// - `default_value`: the default value of the property.
    ///
    /// ### Return value
    /// Returns the value of the property, or `default_value` if it is not set or
    ///   not a float property.
    ///
    /// ### Thread safety
    /// It is safe to call this function from any thread.
    ///
    /// ### Availability
    /// This function is available since SDL 3.1.3.
    ///
    /// ### See also
    /// - [`SDL_GetPropertyType`]
    /// - [`SDL_HasProperty`]
    /// - [`SDL_SetFloatProperty`]
    pub fn SDL_GetFloatProperty(
        props: SDL_PropertiesID,
        name: *const ::core::ffi::c_char,
        default_value: ::core::ffi::c_float,
    ) -> ::core::ffi::c_float;
}

extern "C" {
    /// Get a boolean property from a group of properties.
    ///
    /// You can use [`SDL_GetPropertyType()`] to query whether the property exists and
    /// is a boolean property.
    ///
    /// ### Parameters
    /// - `props`: the properties to query.
    /// - `name`: the name of the property to query.
    /// - `default_value`: the default value of the property.
    ///
    /// ### Return value
    /// Returns the value of the property, or `default_value` if it is not set or
    ///   not a boolean property.
    ///
    /// ### Thread safety
    /// It is safe to call this function from any thread.
    ///
    /// ### Availability
    /// This function is available since SDL 3.1.3.
    ///
    /// ### See also
    /// - [`SDL_GetPropertyType`]
    /// - [`SDL_HasProperty`]
    /// - [`SDL_SetBooleanProperty`]
    pub fn SDL_GetBooleanProperty(
        props: SDL_PropertiesID,
        name: *const ::core::ffi::c_char,
        default_value: ::core::primitive::bool,
    ) -> ::core::primitive::bool;
}

extern "C" {
    /// Clear a property from a group of properties.
    ///
    /// ### Parameters
    /// - `props`: the properties to modify.
    /// - `name`: the name of the property to clear.
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
    pub fn SDL_ClearProperty(
        props: SDL_PropertiesID,
        name: *const ::core::ffi::c_char,
    ) -> ::core::primitive::bool;
}

/// A callback used to enumerate all the properties in a group of properties.
///
/// This callback is called from [`SDL_EnumerateProperties()`], and is called once
/// per property in the set.
///
/// ### Parameters
/// - `userdata`: an app-defined pointer passed to the callback.
/// - `props`: the [`SDL_PropertiesID`] that is being enumerated.
/// - `name`: the next property name in the enumeration.
///
/// ### Thread safety
/// [`SDL_EnumerateProperties`] holds a lock on `props` during this
///   callback.
///
/// ### Availability
/// This datatype is available since SDL 3.1.3.
///
/// ### See also
/// - [`SDL_EnumerateProperties`]
pub type SDL_EnumeratePropertiesCallback = ::core::option::Option<
    unsafe extern "C" fn(
        userdata: *mut ::core::ffi::c_void,
        props: SDL_PropertiesID,
        name: *const ::core::ffi::c_char,
    ),
>;

extern "C" {
    /// Enumerate the properties contained in a group of properties.
    ///
    /// The callback function is called for each property in the group of
    /// properties. The properties are locked during enumeration.
    ///
    /// ### Parameters
    /// - `props`: the properties to query.
    /// - `callback`: the function to call for each property.
    /// - `userdata`: a pointer that is passed to `callback`.
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
    pub fn SDL_EnumerateProperties(
        props: SDL_PropertiesID,
        callback: SDL_EnumeratePropertiesCallback,
        userdata: *mut ::core::ffi::c_void,
    ) -> ::core::primitive::bool;
}

extern "C" {
    /// Destroy a group of properties.
    ///
    /// All properties are deleted and their cleanup functions will be called, if
    /// any.
    ///
    /// ### Parameters
    /// - `props`: the properties to destroy.
    ///
    /// ### Thread safety
    /// This function should not be called while these properties are
    ///   locked or other threads might be setting or getting values
    ///   from these properties.
    ///
    /// ### Availability
    /// This function is available since SDL 3.1.3.
    ///
    /// ### See also
    /// - [`SDL_CreateProperties`]
    pub fn SDL_DestroyProperties(props: SDL_PropertiesID);
}

#[cfg(doc)]
use crate::everything::*;
