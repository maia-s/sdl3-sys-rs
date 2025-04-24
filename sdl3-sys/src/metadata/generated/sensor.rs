//! Metadata for items in the `crate::sensor` module

use super::*;

pub static METADATA_SDL_SensorID: Group = Group {
    module: "sensor",
    kind: GroupKind::Id,
    name: "SDL_SensorID",
    short_name: "SensorID",
    doc: Some("This is a unique ID for a sensor for the time it is connected to the\nsystem, and is never reused for the lifetime of the application.\n\nThe value 0 is an invalid ID.\n\n## Availability\nThis datatype is available since SDL 3.2.0.\n"),
    available_since: Some(SDL_VERSIONNUM(3, 2, 0)),
    values: &[
    ],
};
pub static METADATA_SDL_SensorType: Group = Group {
    module: "sensor",
    kind: GroupKind::Enum,
    name: "SDL_SensorType",
    short_name: "SensorType",
    doc: Some("The different sensors defined by SDL.\n\nAdditional sensors may be available, using platform dependent semantics.\n\nHere are the additional Android sensors:\n\n<https://developer.android.com/reference/android/hardware/SensorEvent.html#values>\n\nAccelerometer sensor notes:\n\nThe accelerometer returns the current acceleration in SI meters per second\nsquared. This measurement includes the force of gravity, so a device at\nrest will have an value of [`SDL_STANDARD_GRAVITY`] away from the center of the\nearth, which is a positive Y value.\n\n- `values[0]`: Acceleration on the x axis\n- `values[1]`: Acceleration on the y axis\n- `values[2]`: Acceleration on the z axis\n\nFor phones and tablets held in natural orientation and game controllers\nheld in front of you, the axes are defined as follows:\n\n- -X ... +X : left ... right\n- -Y ... +Y : bottom ... top\n- -Z ... +Z : farther ... closer\n\nThe accelerometer axis data is not changed when the device is rotated.\n\nGyroscope sensor notes:\n\nThe gyroscope returns the current rate of rotation in radians per second.\nThe rotation is positive in the counter-clockwise direction. That is, an\nobserver looking from a positive location on one of the axes would see\npositive rotation on that axis when it appeared to be rotating\ncounter-clockwise.\n\n- `values[0]`: Angular speed around the x axis (pitch)\n- `values[1]`: Angular speed around the y axis (yaw)\n- `values[2]`: Angular speed around the z axis (roll)\n\nFor phones and tablets held in natural orientation and game controllers\nheld in front of you, the axes are defined as follows:\n\n- -X ... +X : left ... right\n- -Y ... +Y : bottom ... top\n- -Z ... +Z : farther ... closer\n\nThe gyroscope axis data is not changed when the device is rotated.\n\n## Availability\nThis enum is available since SDL 3.2.0.\n\n## See also\n- [`SDL_GetCurrentDisplayOrientation`]\n"),
    available_since: Some(SDL_VERSIONNUM(3, 2, 0)),
    values: &[
        GroupValue {
            name: "SDL_SENSOR_INVALID",
            short_name: "INVALID",
            doc: Some("Returned for an invalid sensor\n"),
            available_since: None,
        },
        GroupValue {
            name: "SDL_SENSOR_UNKNOWN",
            short_name: "UNKNOWN",
            doc: Some("Unknown sensor type\n"),
            available_since: None,
        },
        GroupValue {
            name: "SDL_SENSOR_ACCEL",
            short_name: "ACCEL",
            doc: Some("Accelerometer\n"),
            available_since: None,
        },
        GroupValue {
            name: "SDL_SENSOR_GYRO",
            short_name: "GYRO",
            doc: Some("Gyroscope\n"),
            available_since: None,
        },
        GroupValue {
            name: "SDL_SENSOR_ACCEL_L",
            short_name: "ACCEL_L",
            doc: Some("Accelerometer for left Joy-Con controller and Wii nunchuk\n"),
            available_since: None,
        },
        GroupValue {
            name: "SDL_SENSOR_GYRO_L",
            short_name: "GYRO_L",
            doc: Some("Gyroscope for left Joy-Con controller\n"),
            available_since: None,
        },
        GroupValue {
            name: "SDL_SENSOR_ACCEL_R",
            short_name: "ACCEL_R",
            doc: Some("Accelerometer for right Joy-Con controller\n"),
            available_since: None,
        },
        GroupValue {
            name: "SDL_SENSOR_GYRO_R",
            short_name: "GYRO_R",
            doc: Some("Gyroscope for right Joy-Con controller\n"),
            available_since: None,
        },
    ],
};
