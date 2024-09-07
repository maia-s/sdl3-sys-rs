#![allow(non_camel_case_types, non_upper_case_globals, unused_imports, clippy::approx_constant, clippy::double_parens)]

//! # CategoryHIDAPI
//!
//! Header file for SDL HIDAPI functions.
//!
//! This is an adaptation of the original HIDAPI interface by Alan Ott, and
//! includes source code licensed under the following license:
//!
//! ```
//! HIDAPI - Multi-Platform library for
//! communication with HID devices.
//!
//! Copyright 2009, Alan Ott, Signal 11 Software.
//! All Rights Reserved.
//!
//! This software may be used by anyone for any reason so
//! long as the copyright notice in the source files
//! remains intact.
//! ```
//!
//! (Note that this license is the same as item three of SDL's zlib license, so
//! it adds no new requirements on the user.)
//!
//! If you would like a version of SDL without this code, you can build SDL
//! with SDL_HIDAPI_DISABLED defined to 1. You might want to do this for
//! example on iOS or tvOS to avoid a dependency on the CoreBluetooth
//! framework.

use super::stdinc::*;

use super::error::*;

/// HID underlying bus types.
///
/// \since This enum is available since SDL 3.0.0.
///
/// sdl3-sys note: This is a `C` enum. Known values: [`SDL_HID_API_BUS_UNKNOWN`], [`SDL_HID_API_BUS_USB`], [`SDL_HID_API_BUS_BLUETOOTH`], [`SDL_HID_API_BUS_I2C`], [`SDL_HID_API_BUS_SPI`]
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "debug-impls", derive(Debug))]
pub struct SDL_hid_bus_type(pub ::core::ffi::c_int);
impl SDL_hid_bus_type {
    /// Unknown bus type
    pub const UNKNOWN: Self = Self(0x00);
    /// USB bus
    ///       Specifications:
    ///       https://usb.org/hid
    pub const USB: Self = Self(0x10);
    /// Bluetooth or Bluetooth LE bus
    ///       Specifications:
    ///       https://www.bluetooth.com/specifications/specs/human-interface-device-profile-1-1-1/
    ///       https://www.bluetooth.com/specifications/specs/hid-service-1-0/
    ///       https://www.bluetooth.com/specifications/specs/hid-over-gatt-profile-1-0/
    pub const BLUETOOTH: Self = Self(0x20);
    /// I2C bus
    ///       Specifications:
    ///       https://docs.microsoft.com/previous-versions/windows/hardware/design/dn642101(v=vs.85)
    pub const I2C: Self = Self(0x30);
    /// SPI bus
    ///       Specifications:
    ///       https://www.microsoft.com/download/details.aspx?id=103325
    pub const SPI: Self = Self(0x40);
}
/// Unknown bus type
pub const SDL_HID_API_BUS_UNKNOWN: SDL_hid_bus_type = SDL_hid_bus_type::UNKNOWN;
/// USB bus
///       Specifications:
///       https://usb.org/hid
pub const SDL_HID_API_BUS_USB: SDL_hid_bus_type = SDL_hid_bus_type::USB;
/// Bluetooth or Bluetooth LE bus
///       Specifications:
///       https://www.bluetooth.com/specifications/specs/human-interface-device-profile-1-1-1/
///       https://www.bluetooth.com/specifications/specs/hid-service-1-0/
///       https://www.bluetooth.com/specifications/specs/hid-over-gatt-profile-1-0/
pub const SDL_HID_API_BUS_BLUETOOTH: SDL_hid_bus_type = SDL_hid_bus_type::BLUETOOTH;
/// I2C bus
///       Specifications:
///       https://docs.microsoft.com/previous-versions/windows/hardware/design/dn642101(v=vs.85)
pub const SDL_HID_API_BUS_I2C: SDL_hid_bus_type = SDL_hid_bus_type::I2C;
/// SPI bus
///       Specifications:
///       https://www.microsoft.com/download/details.aspx?id=103325
pub const SDL_HID_API_BUS_SPI: SDL_hid_bus_type = SDL_hid_bus_type::SPI;

/// Information about a connected HID device
///
/// \since This struct is available since SDL 3.0.0.
#[repr(C)]
#[derive(Clone, Copy)]
#[cfg_attr(feature = "debug-impls", derive(Debug))]
pub struct SDL_hid_device_info {
    /// Platform-specific device path
    pub path: *mut ::core::ffi::c_char,
    /// Device Vendor ID
    pub vendor_id: ::core::ffi::c_ushort,
    /// Device Product ID
    pub product_id: ::core::ffi::c_ushort,
    /// Serial Number
    pub serial_number: *mut crate::ffi::c_wchar_t,
    /// Device Release Number in binary-coded decimal,
    ///        also known as Device Version Number
    pub release_number: ::core::ffi::c_ushort,
    /// Manufacturer String
    pub manufacturer_string: *mut crate::ffi::c_wchar_t,
    /// Product string
    pub product_string: *mut crate::ffi::c_wchar_t,
    /// Usage Page for this Device/Interface
    ///        (Windows/Mac/hidraw only)
    pub usage_page: ::core::ffi::c_ushort,
    /// Usage for this Device/Interface
    ///        (Windows/Mac/hidraw only)
    pub usage: ::core::ffi::c_ushort,
    /// The USB interface which this logical device
    ///        represents.
    ///
    ///        Valid only if the device is a USB HID device.
    ///        Set to -1 in all other cases.
    pub interface_number: ::core::ffi::c_int,
    /// Additional information about the USB interface.
    ///        Valid on libusb and Android implementations.
    pub interface_class: ::core::ffi::c_int,
    pub interface_subclass: ::core::ffi::c_int,
    pub interface_protocol: ::core::ffi::c_int,
    /// Underlying bus type
    pub bus_type: SDL_hid_bus_type,
    /// Pointer to the next device
    pub next: *mut SDL_hid_device_info,
}

extern_sdlcall! {{
    /// Initialize the HIDAPI library.
    ///
    /// This function initializes the HIDAPI library. Calling it is not strictly
    /// necessary, as it will be called automatically by SDL_hid_enumerate() and
    /// any of the SDL_hid_open_*() functions if it is needed. This function should
    /// be called at the beginning of execution however, if there is a chance of
    /// HIDAPI handles being opened by different threads simultaneously.
    ///
    /// Each call to this function should have a matching call to SDL_hid_exit()
    ///
    /// \returns 0 on success or a negative error code on failure; call
    ///          SDL_GetError() for more information.
    ///
    /// \since This function is available since SDL 3.0.0.
    ///
    /// \sa SDL_hid_exit
    pub fn SDL_hid_init() -> ::core::ffi::c_int;
}}

extern_sdlcall! {{
    /// Finalize the HIDAPI library.
    ///
    /// This function frees all of the static data associated with HIDAPI. It
    /// should be called at the end of execution to avoid memory leaks.
    ///
    /// \returns 0 on success or a negative error code on failure; call
    ///          SDL_GetError() for more information.
    ///
    /// \since This function is available since SDL 3.0.0.
    ///
    /// \sa SDL_hid_init
    pub fn SDL_hid_exit() -> ::core::ffi::c_int;
}}

extern_sdlcall! {{
    /// Check to see if devices may have been added or removed.
    ///
    /// Enumerating the HID devices is an expensive operation, so you can call this
    /// to see if there have been any system device changes since the last call to
    /// this function. A change in the counter returned doesn't necessarily mean
    /// that anything has changed, but you can call SDL_hid_enumerate() to get an
    /// updated device list.
    ///
    /// Calling this function for the first time may cause a thread or other system
    /// resource to be allocated to track device change notifications.
    ///
    /// \returns a change counter that is incremented with each potential device
    ///          change, or 0 if device change detection isn't available.
    ///
    /// \since This function is available since SDL 3.0.0.
    ///
    /// \sa SDL_hid_enumerate
    pub fn SDL_hid_device_change_count() -> Uint32;
}}

extern_sdlcall! {{
    /// Enumerate the HID Devices.
    ///
    /// This function returns a linked list of all the HID devices attached to the
    /// system which match vendor_id and product_id. If `vendor_id` is set to 0
    /// then any vendor matches. If `product_id` is set to 0 then any product
    /// matches. If `vendor_id` and `product_id` are both set to 0, then all HID
    /// devices will be returned.
    ///
    /// By default SDL will only enumerate controllers, to reduce risk of hanging
    /// or crashing on bad drivers, but SDL_HINT_HIDAPI_ENUMERATE_ONLY_CONTROLLERS
    /// can be set to "0" to enumerate all HID devices.
    ///
    /// \param vendor_id the Vendor ID (VID) of the types of device to open, or 0
    ///                  to match any vendor.
    /// \param product_id the Product ID (PID) of the types of device to open, or 0
    ///                   to match any product.
    /// \returns a pointer to a linked list of type SDL_hid_device_info, containing
    ///          information about the HID devices attached to the system, or NULL
    ///          in the case of failure. Free this linked list by calling
    ///          SDL_hid_free_enumeration().
    ///
    /// \since This function is available since SDL 3.0.0.
    ///
    /// \sa SDL_hid_device_change_count
    pub fn SDL_hid_enumerate(vendor_id: ::core::ffi::c_ushort, product_id: ::core::ffi::c_ushort) -> *mut SDL_hid_device_info;
}}

extern_sdlcall! {{
    /// Free an enumeration linked list.
    ///
    /// This function frees a linked list created by SDL_hid_enumerate().
    ///
    /// \param devs pointer to a list of struct_device returned from
    ///             SDL_hid_enumerate().
    ///
    /// \since This function is available since SDL 3.0.0.
    pub fn SDL_hid_free_enumeration(devs: *mut SDL_hid_device_info);
}}

extern_sdlcall! {{
    /// Open a HID device using a Vendor ID (VID), Product ID (PID) and optionally
    /// a serial number.
    ///
    /// If `serial_number` is NULL, the first device with the specified VID and PID
    /// is opened.
    ///
    /// \param vendor_id the Vendor ID (VID) of the device to open.
    /// \param product_id the Product ID (PID) of the device to open.
    /// \param serial_number the Serial Number of the device to open (Optionally
    ///                      NULL).
    /// \returns a pointer to a SDL_hid_device object on success or NULL on
    ///          failure; call SDL_GetError() for more information.
    ///
    /// \since This function is available since SDL 3.0.0.
    pub fn SDL_hid_open(vendor_id: ::core::ffi::c_ushort, product_id: ::core::ffi::c_ushort, serial_number: *const crate::ffi::c_wchar_t) -> *mut SDL_hid_device;
}}

extern_sdlcall! {{
    /// Open a HID device by its path name.
    ///
    /// The path name be determined by calling SDL_hid_enumerate(), or a
    /// platform-specific path name can be used (eg: /dev/hidraw0 on Linux).
    ///
    /// \param path the path name of the device to open.
    /// \returns a pointer to a SDL_hid_device object on success or NULL on
    ///          failure; call SDL_GetError() for more information.
    ///
    /// \since This function is available since SDL 3.0.0.
    pub fn SDL_hid_open_path(path: *const ::core::ffi::c_char) -> *mut SDL_hid_device;
}}

extern_sdlcall! {{
    /// Write an Output report to a HID device.
    ///
    /// The first byte of `data` must contain the Report ID. For devices which only
    /// support a single report, this must be set to 0x0. The remaining bytes
    /// contain the report data. Since the Report ID is mandatory, calls to
    /// SDL_hid_write() will always contain one more byte than the report contains.
    /// For example, if a hid report is 16 bytes long, 17 bytes must be passed to
    /// SDL_hid_write(), the Report ID (or 0x0, for devices with a single report),
    /// followed by the report data (16 bytes). In this example, the length passed
    /// in would be 17.
    ///
    /// SDL_hid_write() will send the data on the first OUT endpoint, if one
    /// exists. If it does not, it will send the data through the Control Endpoint
    /// (Endpoint 0).
    ///
    /// \param dev a device handle returned from SDL_hid_open().
    /// \param data the data to send, including the report number as the first
    ///             byte.
    /// \param length the length in bytes of the data to send.
    /// \returns the actual number of bytes written and -1 on on failure; call
    ///          SDL_GetError() for more information.
    ///
    /// \since This function is available since SDL 3.0.0.
    pub fn SDL_hid_write(dev: *mut SDL_hid_device, data: *const ::core::ffi::c_uchar, length: ::core::primitive::usize) -> ::core::ffi::c_int;
}}

extern_sdlcall! {{
    /// Read an Input report from a HID device with timeout.
    ///
    /// Input reports are returned to the host through the INTERRUPT IN endpoint.
    /// The first byte will contain the Report number if the device uses numbered
    /// reports.
    ///
    /// \param dev a device handle returned from SDL_hid_open().
    /// \param data a buffer to put the read data into.
    /// \param length the number of bytes to read. For devices with multiple
    ///               reports, make sure to read an extra byte for the report
    ///               number.
    /// \param milliseconds timeout in milliseconds or -1 for blocking wait.
    /// \returns the actual number of bytes read and -1 on on failure; call
    ///          SDL_GetError() for more information. If no packet was available to
    ///          be read within the timeout period, this function returns 0.
    ///
    /// \since This function is available since SDL 3.0.0.
    pub fn SDL_hid_read_timeout(dev: *mut SDL_hid_device, data: *mut ::core::ffi::c_uchar, length: ::core::primitive::usize, milliseconds: ::core::ffi::c_int) -> ::core::ffi::c_int;
}}

extern_sdlcall! {{
    /// Read an Input report from a HID device.
    ///
    /// Input reports are returned to the host through the INTERRUPT IN endpoint.
    /// The first byte will contain the Report number if the device uses numbered
    /// reports.
    ///
    /// \param dev a device handle returned from SDL_hid_open().
    /// \param data a buffer to put the read data into.
    /// \param length the number of bytes to read. For devices with multiple
    ///               reports, make sure to read an extra byte for the report
    ///               number.
    /// \returns the actual number of bytes read and -1 on failure; call
    ///          SDL_GetError() for more information. If no packet was available to
    ///          be read and the handle is in non-blocking mode, this function
    ///          returns 0.
    ///
    /// \since This function is available since SDL 3.0.0.
    pub fn SDL_hid_read(dev: *mut SDL_hid_device, data: *mut ::core::ffi::c_uchar, length: ::core::primitive::usize) -> ::core::ffi::c_int;
}}

extern_sdlcall! {{
    /// Set the device handle to be non-blocking.
    ///
    /// In non-blocking mode calls to SDL_hid_read() will return immediately with a
    /// value of 0 if there is no data to be read. In blocking mode, SDL_hid_read()
    /// will wait (block) until there is data to read before returning.
    ///
    /// Nonblocking can be turned on and off at any time.
    ///
    /// \param dev a device handle returned from SDL_hid_open().
    /// \param nonblock enable or not the nonblocking reads - 1 to enable
    ///                 nonblocking - 0 to disable nonblocking.
    /// \returns 0 on success or a negative error code on failure; call
    ///          SDL_GetError() for more information.
    ///
    /// \since This function is available since SDL 3.0.0.
    pub fn SDL_hid_set_nonblocking(dev: *mut SDL_hid_device, nonblock: ::core::ffi::c_int) -> ::core::ffi::c_int;
}}

extern_sdlcall! {{
    /// Send a Feature report to the device.
    ///
    /// Feature reports are sent over the Control endpoint as a Set_Report
    /// transfer. The first byte of `data` must contain the Report ID. For devices
    /// which only support a single report, this must be set to 0x0. The remaining
    /// bytes contain the report data. Since the Report ID is mandatory, calls to
    /// SDL_hid_send_feature_report() will always contain one more byte than the
    /// report contains. For example, if a hid report is 16 bytes long, 17 bytes
    /// must be passed to SDL_hid_send_feature_report(): the Report ID (or 0x0, for
    /// devices which do not use numbered reports), followed by the report data (16
    /// bytes). In this example, the length passed in would be 17.
    ///
    /// \param dev a device handle returned from SDL_hid_open().
    /// \param data the data to send, including the report number as the first
    ///             byte.
    /// \param length the length in bytes of the data to send, including the report
    ///               number.
    /// \returns the actual number of bytes written and -1 on failure; call
    ///          SDL_GetError() for more information.
    ///
    /// \since This function is available since SDL 3.0.0.
    pub fn SDL_hid_send_feature_report(dev: *mut SDL_hid_device, data: *const ::core::ffi::c_uchar, length: ::core::primitive::usize) -> ::core::ffi::c_int;
}}

extern_sdlcall! {{
    /// Get a feature report from a HID device.
    ///
    /// Set the first byte of `data` to the Report ID of the report to be read.
    /// Make sure to allow space for this extra byte in `data`. Upon return, the
    /// first byte will still contain the Report ID, and the report data will start
    /// in data[1].
    ///
    /// \param dev a device handle returned from SDL_hid_open().
    /// \param data a buffer to put the read data into, including the Report ID.
    ///             Set the first byte of `data` to the Report ID of the report to
    ///             be read, or set it to zero if your device does not use numbered
    ///             reports.
    /// \param length the number of bytes to read, including an extra byte for the
    ///               report ID. The buffer can be longer than the actual report.
    /// \returns the number of bytes read plus one for the report ID (which is
    ///          still in the first byte), or -1 on on failure; call SDL_GetError()
    ///          for more information.
    ///
    /// \since This function is available since SDL 3.0.0.
    pub fn SDL_hid_get_feature_report(dev: *mut SDL_hid_device, data: *mut ::core::ffi::c_uchar, length: ::core::primitive::usize) -> ::core::ffi::c_int;
}}

extern_sdlcall! {{
    /// Get an input report from a HID device.
    ///
    /// Set the first byte of `data` to the Report ID of the report to be read.
    /// Make sure to allow space for this extra byte in `data`. Upon return, the
    /// first byte will still contain the Report ID, and the report data will start
    /// in data[1].
    ///
    /// \param dev a device handle returned from SDL_hid_open().
    /// \param data a buffer to put the read data into, including the Report ID.
    ///             Set the first byte of `data` to the Report ID of the report to
    ///             be read, or set it to zero if your device does not use numbered
    ///             reports.
    /// \param length the number of bytes to read, including an extra byte for the
    ///               report ID. The buffer can be longer than the actual report.
    /// \returns the number of bytes read plus one for the report ID (which is
    ///          still in the first byte), or -1 on on failure; call SDL_GetError()
    ///          for more information.
    ///
    /// \since This function is available since SDL 3.0.0.
    pub fn SDL_hid_get_input_report(dev: *mut SDL_hid_device, data: *mut ::core::ffi::c_uchar, length: ::core::primitive::usize) -> ::core::ffi::c_int;
}}

extern_sdlcall! {{
    /// Close a HID device.
    ///
    /// \param dev a device handle returned from SDL_hid_open().
    /// \returns 0 on success or a negative error code on failure; call
    ///          SDL_GetError() for more information.
    ///
    /// \since This function is available since SDL 3.0.0.
    pub fn SDL_hid_close(dev: *mut SDL_hid_device) -> ::core::ffi::c_int;
}}

extern_sdlcall! {{
    /// Get The Manufacturer String from a HID device.
    ///
    /// \param dev a device handle returned from SDL_hid_open().
    /// \param string a wide string buffer to put the data into.
    /// \param maxlen the length of the buffer in multiples of wchar_t.
    /// \returns 0 on success or a negative error code on failure; call
    ///          SDL_GetError() for more information.
    ///
    /// \since This function is available since SDL 3.0.0.
    pub fn SDL_hid_get_manufacturer_string(dev: *mut SDL_hid_device, string: *mut crate::ffi::c_wchar_t, maxlen: ::core::primitive::usize) -> ::core::ffi::c_int;
}}

extern_sdlcall! {{
    /// Get The Product String from a HID device.
    ///
    /// \param dev a device handle returned from SDL_hid_open().
    /// \param string a wide string buffer to put the data into.
    /// \param maxlen the length of the buffer in multiples of wchar_t.
    /// \returns 0 on success or a negative error code on failure; call
    ///          SDL_GetError() for more information.
    ///
    /// \since This function is available since SDL 3.0.0.
    pub fn SDL_hid_get_product_string(dev: *mut SDL_hid_device, string: *mut crate::ffi::c_wchar_t, maxlen: ::core::primitive::usize) -> ::core::ffi::c_int;
}}

extern_sdlcall! {{
    /// Get The Serial Number String from a HID device.
    ///
    /// \param dev a device handle returned from SDL_hid_open().
    /// \param string a wide string buffer to put the data into.
    /// \param maxlen the length of the buffer in multiples of wchar_t.
    /// \returns 0 on success or a negative error code on failure; call
    ///          SDL_GetError() for more information.
    ///
    /// \since This function is available since SDL 3.0.0.
    pub fn SDL_hid_get_serial_number_string(dev: *mut SDL_hid_device, string: *mut crate::ffi::c_wchar_t, maxlen: ::core::primitive::usize) -> ::core::ffi::c_int;
}}

extern_sdlcall! {{
    /// Get a string from a HID device, based on its string index.
    ///
    /// \param dev a device handle returned from SDL_hid_open().
    /// \param string_index the index of the string to get.
    /// \param string a wide string buffer to put the data into.
    /// \param maxlen the length of the buffer in multiples of wchar_t.
    /// \returns 0 on success or a negative error code on failure; call
    ///          SDL_GetError() for more information.
    ///
    /// \since This function is available since SDL 3.0.0.
    pub fn SDL_hid_get_indexed_string(dev: *mut SDL_hid_device, string_index: ::core::ffi::c_int, string: *mut crate::ffi::c_wchar_t, maxlen: ::core::primitive::usize) -> ::core::ffi::c_int;
}}

extern_sdlcall! {{
    /// Get the device info from a HID device.
    ///
    /// \param dev a device handle returned from SDL_hid_open().
    /// \returns a pointer to the SDL_hid_device_info for this hid_device or NULL
    ///          on failure; call SDL_GetError() for more information. This struct
    ///          is valid until the device is closed with SDL_hid_close().
    ///
    /// \since This function is available since SDL 3.0.0.
    pub fn SDL_hid_get_device_info(dev: *mut SDL_hid_device) -> *mut SDL_hid_device_info;
}}

extern_sdlcall! {{
    /// Get a report descriptor from a HID device.
    ///
    /// User has to provide a preallocated buffer where descriptor will be copied
    /// to. The recommended size for a preallocated buffer is 4096 bytes.
    ///
    /// \param dev a device handle returned from SDL_hid_open().
    /// \param buf the buffer to copy descriptor into.
    /// \param buf_size the size of the buffer in bytes.
    /// \returns the number of bytes actually copied or -1 on failure; call
    ///          SDL_GetError() for more information.
    ///
    /// \since This function is available since SDL 3.0.0.
    pub fn SDL_hid_get_report_descriptor(dev: *mut SDL_hid_device, buf: *mut ::core::ffi::c_uchar, buf_size: ::core::primitive::usize) -> ::core::ffi::c_int;
}}

extern_sdlcall! {{
    /// Start or stop a BLE scan on iOS and tvOS to pair Steam Controllers.
    ///
    /// \param active SDL_TRUE to start the scan, SDL_FALSE to stop the scan.
    ///
    /// \since This function is available since SDL 3.0.0.
    pub fn SDL_hid_ble_scan(active: SDL_bool);
}}

/// An opaque handle representing an open HID device.
///
/// \since This struct is available since SDL 3.0.0.
#[repr(C)]
#[non_exhaustive]
pub struct SDL_hid_device { _opaque: [::core::primitive::u8; 0] }

