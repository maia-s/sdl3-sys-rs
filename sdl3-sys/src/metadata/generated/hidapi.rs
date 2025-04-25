//! Metadata for items in the `crate::hidapi` module

use super::*;

pub const METADATA_SDL_hid_bus_type: Group = Group {
    module: "hidapi",
    kind: GroupKind::Enum,
    name: "SDL_hid_bus_type",
    short_name: "hid_bus_type",
    doc: Some("HID underlying bus types.\n\n## Availability\nThis enum is available since SDL 3.2.0.\n"),
    available_since: Some(SDL_VERSIONNUM(3, 2, 0)),
    values: &[
        GroupValue {
            name: "SDL_HID_API_BUS_UNKNOWN",
            short_name: "UNKNOWN",
            doc: Some("Unknown bus type\n"),
            available_since: None,
        },
        GroupValue {
            name: "SDL_HID_API_BUS_USB",
            short_name: "USB",
            doc: Some("USB bus\nSpecifications:\n<https://usb.org/hid>\n"),
            available_since: None,
        },
        GroupValue {
            name: "SDL_HID_API_BUS_BLUETOOTH",
            short_name: "BLUETOOTH",
            doc: Some("Bluetooth or Bluetooth LE bus\nSpecifications:\n<https://www.bluetooth.com/specifications/specs/human-interface-device-profile-1-1-1/>\n<https://www.bluetooth.com/specifications/specs/hid-service-1-0/>\n<https://www.bluetooth.com/specifications/specs/hid-over-gatt-profile-1-0/>\n"),
            available_since: None,
        },
        GroupValue {
            name: "SDL_HID_API_BUS_I2C",
            short_name: "I2C",
            doc: Some("I2C bus\nSpecifications:\n<https://docs.microsoft.com/previous-versions/windows/hardware/design/dn642101(v=vs.85)>\n"),
            available_since: None,
        },
        GroupValue {
            name: "SDL_HID_API_BUS_SPI",
            short_name: "SPI",
            doc: Some("SPI bus\nSpecifications:\n<https://www.microsoft.com/download/details.aspx?id=103325>\n"),
            available_since: None,
        },
    ],
};
