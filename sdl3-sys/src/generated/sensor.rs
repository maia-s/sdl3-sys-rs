//! SDL sensor management.
//!
//! In order to use these functions, [`SDL_Init()`] must have been called with the
//! [`SDL_INIT_SENSOR`] flag. This causes SDL to scan the system for sensors, and
//! load appropriate drivers.

use super::stdinc::*;

use super::error::*;

use super::properties::*;

/// This is a unique ID for a sensor for the time it is connected to the
/// system, and is never reused for the lifetime of the application.
///
/// The value 0 is an invalid ID.
///
/// ### Availability
/// This datatype is available since SDL 3.1.3.
pub type SDL_SensorID = Uint32;

/// A constant to represent standard gravity for accelerometer sensors.
///
/// The accelerometer returns the current acceleration in SI meters per second
/// squared. This measurement includes the force of gravity, so a device at
/// rest will have an value of [`SDL_STANDARD_GRAVITY`] away from the center of the
/// earth, which is a positive Y value.
///
/// ### Availability
/// This macro is available since SDL 3.1.3.
pub const SDL_STANDARD_GRAVITY: ::core::ffi::c_float = 9.80665_f32;

/// The different sensors defined by SDL.
///
/// Additional sensors may be available, using platform dependent semantics.
///
/// Here are the additional Android sensors:
///
/// <https://developer.android.com/reference/android/hardware/SensorEvent.html#values>
///
/// Accelerometer sensor notes:
///
/// The accelerometer returns the current acceleration in SI meters per second
/// squared. This measurement includes the force of gravity, so a device at
/// rest will have an value of [`SDL_STANDARD_GRAVITY`] away from the center of the
/// earth, which is a positive Y value.
///
/// - `values[0]`: Acceleration on the x axis
/// - `values[1]`: Acceleration on the y axis
/// - `values[2]`: Acceleration on the z axis
///
/// For phones and tablets held in natural orientation and game controllers
/// held in front of you, the axes are defined as follows:
///
/// - -X ... +X : left ... right
/// - -Y ... +Y : bottom ... top
/// - -Z ... +Z : farther ... closer
///
/// The accelerometer axis data is not changed when the device is rotated.
///
/// Gyroscope sensor notes:
///
/// The gyroscope returns the current rate of rotation in radians per second.
/// The rotation is positive in the counter-clockwise direction. That is, an
/// observer looking from a positive location on one of the axes would see
/// positive rotation on that axis when it appeared to be rotating
/// counter-clockwise.
///
/// - `values[0]`: Angular speed around the x axis (pitch)
/// - `values[1]`: Angular speed around the y axis (yaw)
/// - `values[2]`: Angular speed around the z axis (roll)
///
/// For phones and tablets held in natural orientation and game controllers
/// held in front of you, the axes are defined as follows:
///
/// - -X ... +X : left ... right
/// - -Y ... +Y : bottom ... top
/// - -Z ... +Z : farther ... closer
///
/// The gyroscope axis data is not changed when the device is rotated.
///
/// ### Availability
/// This enum is available since SDL 3.1.3.
///
/// ### See also
/// - [`SDL_GetCurrentDisplayOrientation`]
///
/// ### Known values (`sdl3-sys`)
/// | Associated constant | Global constant | Description |
/// | ------------------- | --------------- | ----------- |
/// | [`INVALID`](SDL_SensorType::INVALID) | [`SDL_SENSOR_INVALID`] | Returned for an invalid sensor |
/// | [`UNKNOWN`](SDL_SensorType::UNKNOWN) | [`SDL_SENSOR_UNKNOWN`] | Unknown sensor type |
/// | [`ACCEL`](SDL_SensorType::ACCEL) | [`SDL_SENSOR_ACCEL`] | Accelerometer |
/// | [`GYRO`](SDL_SensorType::GYRO) | [`SDL_SENSOR_GYRO`] | Gyroscope |
/// | [`ACCEL_L`](SDL_SensorType::ACCEL_L) | [`SDL_SENSOR_ACCEL_L`] | Accelerometer for left Joy-Con controller and Wii nunchuk |
/// | [`GYRO_L`](SDL_SensorType::GYRO_L) | [`SDL_SENSOR_GYRO_L`] | Gyroscope for left Joy-Con controller |
/// | [`ACCEL_R`](SDL_SensorType::ACCEL_R) | [`SDL_SENSOR_ACCEL_R`] | Accelerometer for right Joy-Con controller |
/// | [`GYRO_R`](SDL_SensorType::GYRO_R) | [`SDL_SENSOR_GYRO_R`] | Gyroscope for right Joy-Con controller |
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct SDL_SensorType(pub ::core::ffi::c_int);

impl From<SDL_SensorType> for ::core::ffi::c_int {
    #[inline(always)]
    fn from(value: SDL_SensorType) -> Self {
        value.0
    }
}

#[cfg(feature = "debug-impls")]
impl ::core::fmt::Debug for SDL_SensorType {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        #[allow(unreachable_patterns)]
        f.write_str(match *self {
            Self::INVALID => "SDL_SENSOR_INVALID",
            Self::UNKNOWN => "SDL_SENSOR_UNKNOWN",
            Self::ACCEL => "SDL_SENSOR_ACCEL",
            Self::GYRO => "SDL_SENSOR_GYRO",
            Self::ACCEL_L => "SDL_SENSOR_ACCEL_L",
            Self::GYRO_L => "SDL_SENSOR_GYRO_L",
            Self::ACCEL_R => "SDL_SENSOR_ACCEL_R",
            Self::GYRO_R => "SDL_SENSOR_GYRO_R",

            _ => return write!(f, "SDL_SensorType({})", self.0),
        })
    }
}

impl SDL_SensorType {
    /// Returned for an invalid sensor
    pub const INVALID: Self = Self(-1_i32);
    /// Unknown sensor type
    pub const UNKNOWN: Self = Self(0_i32);
    /// Accelerometer
    pub const ACCEL: Self = Self(1_i32);
    /// Gyroscope
    pub const GYRO: Self = Self(2_i32);
    /// Accelerometer for left Joy-Con controller and Wii nunchuk
    pub const ACCEL_L: Self = Self(3_i32);
    /// Gyroscope for left Joy-Con controller
    pub const GYRO_L: Self = Self(4_i32);
    /// Accelerometer for right Joy-Con controller
    pub const ACCEL_R: Self = Self(5_i32);
    /// Gyroscope for right Joy-Con controller
    pub const GYRO_R: Self = Self(6_i32);
}

/// Returned for an invalid sensor
pub const SDL_SENSOR_INVALID: SDL_SensorType = SDL_SensorType::INVALID;
/// Unknown sensor type
pub const SDL_SENSOR_UNKNOWN: SDL_SensorType = SDL_SensorType::UNKNOWN;
/// Accelerometer
pub const SDL_SENSOR_ACCEL: SDL_SensorType = SDL_SensorType::ACCEL;
/// Gyroscope
pub const SDL_SENSOR_GYRO: SDL_SensorType = SDL_SensorType::GYRO;
/// Accelerometer for left Joy-Con controller and Wii nunchuk
pub const SDL_SENSOR_ACCEL_L: SDL_SensorType = SDL_SensorType::ACCEL_L;
/// Gyroscope for left Joy-Con controller
pub const SDL_SENSOR_GYRO_L: SDL_SensorType = SDL_SensorType::GYRO_L;
/// Accelerometer for right Joy-Con controller
pub const SDL_SENSOR_ACCEL_R: SDL_SensorType = SDL_SensorType::ACCEL_R;
/// Gyroscope for right Joy-Con controller
pub const SDL_SENSOR_GYRO_R: SDL_SensorType = SDL_SensorType::GYRO_R;

extern "C" {
    /// Get a list of currently connected sensors.
    ///
    /// ### Parameters
    /// - `count`: a pointer filled in with the number of sensors returned, may
    ///   be NULL.
    ///
    /// ### Return value
    /// Returns a 0 terminated array of sensor instance IDs or NULL on failure;
    ///   call [`SDL_GetError()`] for more information. This should be freed
    ///   with [`SDL_free()`] when it is no longer needed.
    ///
    /// ### Availability
    /// This function is available since SDL 3.1.3.
    pub fn SDL_GetSensors(count: *mut ::core::ffi::c_int) -> *mut SDL_SensorID;
}

extern "C" {
    /// Get the implementation dependent name of a sensor.
    ///
    /// This can be called before any sensors are opened.
    ///
    /// ### Parameters
    /// - `instance_id`: the sensor instance ID.
    ///
    /// ### Return value
    /// Returns the sensor name, or NULL if `instance_id` is not valid.
    ///
    /// ### Availability
    /// This function is available since SDL 3.1.3.
    pub fn SDL_GetSensorNameForID(instance_id: SDL_SensorID) -> *const ::core::ffi::c_char;
}

extern "C" {
    /// Get the type of a sensor.
    ///
    /// This can be called before any sensors are opened.
    ///
    /// ### Parameters
    /// - `instance_id`: the sensor instance ID.
    ///
    /// ### Return value
    /// Returns the [`SDL_SensorType`], or [`SDL_SENSOR_INVALID`] if `instance_id` is
    ///   not valid.
    ///
    /// ### Availability
    /// This function is available since SDL 3.1.3.
    pub fn SDL_GetSensorTypeForID(instance_id: SDL_SensorID) -> SDL_SensorType;
}

extern "C" {
    /// Get the platform dependent type of a sensor.
    ///
    /// This can be called before any sensors are opened.
    ///
    /// ### Parameters
    /// - `instance_id`: the sensor instance ID.
    ///
    /// ### Return value
    /// Returns the sensor platform dependent type, or -1 if `instance_id` is not
    ///   valid.
    ///
    /// ### Availability
    /// This function is available since SDL 3.1.3.
    pub fn SDL_GetSensorNonPortableTypeForID(instance_id: SDL_SensorID) -> ::core::ffi::c_int;
}

extern "C" {
    /// Open a sensor for use.
    ///
    /// ### Parameters
    /// - `instance_id`: the sensor instance ID.
    ///
    /// ### Return value
    /// Returns an [`SDL_Sensor`] object or NULL on failure; call [`SDL_GetError()`] for
    ///   more information.
    ///
    /// ### Availability
    /// This function is available since SDL 3.1.3.
    pub fn SDL_OpenSensor(instance_id: SDL_SensorID) -> *mut SDL_Sensor;
}

extern "C" {
    /// Return the [`SDL_Sensor`] associated with an instance ID.
    ///
    /// ### Parameters
    /// - `instance_id`: the sensor instance ID.
    ///
    /// ### Return value
    /// Returns an [`SDL_Sensor`] object or NULL on failure; call [`SDL_GetError()`] for
    ///   more information.
    ///
    /// ### Availability
    /// This function is available since SDL 3.1.3.
    pub fn SDL_GetSensorFromID(instance_id: SDL_SensorID) -> *mut SDL_Sensor;
}

extern "C" {
    /// Get the properties associated with a sensor.
    ///
    /// ### Parameters
    /// - `sensor`: the [`SDL_Sensor`] object.
    ///
    /// ### Return value
    /// Returns a valid property ID on success or 0 on failure; call
    ///   [`SDL_GetError()`] for more information.
    ///
    /// ### Availability
    /// This function is available since SDL 3.1.3.
    pub fn SDL_GetSensorProperties(sensor: *mut SDL_Sensor) -> SDL_PropertiesID;
}

extern "C" {
    /// Get the implementation dependent name of a sensor.
    ///
    /// ### Parameters
    /// - `sensor`: the [`SDL_Sensor`] object.
    ///
    /// ### Return value
    /// Returns the sensor name or NULL on failure; call [`SDL_GetError()`] for more
    ///   information.
    ///
    /// ### Availability
    /// This function is available since SDL 3.1.3.
    pub fn SDL_GetSensorName(sensor: *mut SDL_Sensor) -> *const ::core::ffi::c_char;
}

extern "C" {
    /// Get the type of a sensor.
    ///
    /// ### Parameters
    /// - `sensor`: the [`SDL_Sensor`] object to inspect.
    ///
    /// ### Return value
    /// Returns the [`SDL_SensorType`] type, or [`SDL_SENSOR_INVALID`] if `sensor` is
    ///   NULL.
    ///
    /// ### Availability
    /// This function is available since SDL 3.1.3.
    pub fn SDL_GetSensorType(sensor: *mut SDL_Sensor) -> SDL_SensorType;
}

extern "C" {
    /// Get the platform dependent type of a sensor.
    ///
    /// ### Parameters
    /// - `sensor`: the [`SDL_Sensor`] object to inspect.
    ///
    /// ### Return value
    /// Returns the sensor platform dependent type, or -1 if `sensor` is NULL.
    ///
    /// ### Availability
    /// This function is available since SDL 3.1.3.
    pub fn SDL_GetSensorNonPortableType(sensor: *mut SDL_Sensor) -> ::core::ffi::c_int;
}

extern "C" {
    /// Get the instance ID of a sensor.
    ///
    /// ### Parameters
    /// - `sensor`: the [`SDL_Sensor`] object to inspect.
    ///
    /// ### Return value
    /// Returns the sensor instance ID, or 0 on failure; call [`SDL_GetError()`] for
    ///   more information.
    ///
    /// ### Availability
    /// This function is available since SDL 3.1.3.
    pub fn SDL_GetSensorID(sensor: *mut SDL_Sensor) -> SDL_SensorID;
}

extern "C" {
    /// Get the current state of an opened sensor.
    ///
    /// The number of values and interpretation of the data is sensor dependent.
    ///
    /// ### Parameters
    /// - `sensor`: the [`SDL_Sensor`] object to query.
    /// - `data`: a pointer filled with the current sensor state.
    /// - `num_values`: the number of values to write to data.
    ///
    /// ### Return value
    /// Returns true on success or false on failure; call [`SDL_GetError()`] for more
    ///   information.
    ///
    /// ### Availability
    /// This function is available since SDL 3.1.3.
    pub fn SDL_GetSensorData(
        sensor: *mut SDL_Sensor,
        data: *mut ::core::ffi::c_float,
        num_values: ::core::ffi::c_int,
    ) -> ::core::primitive::bool;
}

extern "C" {
    /// Close a sensor previously opened with [`SDL_OpenSensor()`].
    ///
    /// ### Parameters
    /// - `sensor`: the [`SDL_Sensor`] object to close.
    ///
    /// ### Availability
    /// This function is available since SDL 3.1.3.
    pub fn SDL_CloseSensor(sensor: *mut SDL_Sensor);
}

extern "C" {
    /// Update the current state of the open sensors.
    ///
    /// This is called automatically by the event loop if sensor events are
    /// enabled.
    ///
    /// This needs to be called from the thread that initialized the sensor
    /// subsystem.
    ///
    /// ### Availability
    /// This function is available since SDL 3.1.3.
    pub fn SDL_UpdateSensors();
}

#[repr(C)]
pub struct SDL_Sensor {
    _opaque: [::core::primitive::u8; 0],
}

#[cfg(doc)]
use crate::everything::*;
