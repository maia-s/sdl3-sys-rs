//! Metadata for items in the `crate::hidapi` module

use super::*;

pub const METADATA_SDL_PROP_HIDAPI_LIBUSB_DEVICE_HANDLE_POINTER: Property = Property {
    module: "hidapi",
    name: "SDL_PROP_HIDAPI_LIBUSB_DEVICE_HANDLE_POINTER",
    short_name: "HIDAPI_LIBUSB_DEVICE_HANDLE_POINTER",
    value: crate::hidapi::SDL_PROP_HIDAPI_LIBUSB_DEVICE_HANDLE_POINTER,
    ty: PropertyType::POINTER,
    doc: None,
    available_since: None,
};
pub const METADATA_SDL_hid_bus_type: Group = Group {
    module: "hidapi",
    kind: GroupKind::Enum,
    name: "SDL_hid_bus_type",
    short_name: "hid_bus_type",
    doc: Some(
        "HID underlying bus types.\n\n## Availability\nThis enum is available since SDL 3.2.0.\n",
    ),
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
            doc: Some(
                "Bluetooth or Bluetooth LE bus\nSpecifications:\n<https://www.bluetooth.com/specifications/specs/human-interface-device-profile-1-1-1/>\n<https://www.bluetooth.com/specifications/specs/hid-service-1-0/>\n<https://www.bluetooth.com/specifications/specs/hid-over-gatt-profile-1-0/>\n",
            ),
            available_since: None,
        },
        GroupValue {
            name: "SDL_HID_API_BUS_I2C",
            short_name: "I2C",
            doc: Some(
                "I2C bus\nSpecifications:\n<https://docs.microsoft.com/previous-versions/windows/hardware/design/dn642101(v=vs.85)>\n",
            ),
            available_since: None,
        },
        GroupValue {
            name: "SDL_HID_API_BUS_SPI",
            short_name: "SPI",
            doc: Some(
                "SPI bus\nSpecifications:\n<https://www.microsoft.com/download/details.aspx?id=103325>\n",
            ),
            available_since: None,
        },
    ],
};
pub const METADATA_SDL_hid_device_info: Struct = Struct {
    module: "hidapi",
    kind: StructKind::Struct,
    name: "SDL_hid_device_info",
    doc: Some(
        "Information about a connected HID device\n\n## Availability\nThis struct is available since SDL 3.2.0.\n",
    ),
    available_since: Some(SDL_VERSIONNUM(3, 2, 0)),
    fields: &[
        Field {
            name: "path",
            doc: Some("Platform-specific device path\n"),
            available_since: None,
            ty: "*mut ::core::ffi::c_char",
        },
        Field {
            name: "vendor_id",
            doc: Some("Device Vendor ID\n"),
            available_since: None,
            ty: "::core::ffi::c_ushort",
        },
        Field {
            name: "product_id",
            doc: Some("Device Product ID\n"),
            available_since: None,
            ty: "::core::ffi::c_ushort",
        },
        Field {
            name: "serial_number",
            doc: Some("Serial Number\n"),
            available_since: None,
            ty: "*mut crate::ffi::c_wchar_t",
        },
        Field {
            name: "release_number",
            doc: Some(
                "Device Release Number in binary-coded decimal,\nalso known as Device Version Number\n",
            ),
            available_since: None,
            ty: "::core::ffi::c_ushort",
        },
        Field {
            name: "manufacturer_string",
            doc: Some("Manufacturer String\n"),
            available_since: None,
            ty: "*mut crate::ffi::c_wchar_t",
        },
        Field {
            name: "product_string",
            doc: Some("Product string\n"),
            available_since: None,
            ty: "*mut crate::ffi::c_wchar_t",
        },
        Field {
            name: "usage_page",
            doc: Some("Usage Page for this Device/Interface\n(Windows/Mac/hidraw only)\n"),
            available_since: None,
            ty: "::core::ffi::c_ushort",
        },
        Field {
            name: "usage",
            doc: Some("Usage for this Device/Interface\n(Windows/Mac/hidraw only)\n"),
            available_since: None,
            ty: "::core::ffi::c_ushort",
        },
        Field {
            name: "interface_number",
            doc: Some(
                "The USB interface which this logical device\nrepresents.\n\nValid only if the device is a USB HID device.\nSet to -1 in all other cases.\n",
            ),
            available_since: None,
            ty: "::core::ffi::c_int",
        },
        Field {
            name: "interface_class",
            doc: Some(
                "Additional information about the USB interface.\nValid on libusb and Android implementations.\n",
            ),
            available_since: None,
            ty: "::core::ffi::c_int",
        },
        Field {
            name: "interface_subclass",
            doc: None,
            available_since: None,
            ty: "::core::ffi::c_int",
        },
        Field {
            name: "interface_protocol",
            doc: None,
            available_since: None,
            ty: "::core::ffi::c_int",
        },
        Field {
            name: "bus_type",
            doc: Some("Underlying bus type\n"),
            available_since: None,
            ty: "SDL_hid_bus_type",
        },
        Field {
            name: "next",
            doc: Some("Pointer to the next device\n"),
            available_since: None,
            ty: "*mut SDL_hid_device_info",
        },
    ],
};
