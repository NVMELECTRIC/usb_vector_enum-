//! C FFI bindings for language interoperability

use crate::device::DeviceInfo;
use crate::enumerator::Enumerator;
use crate::error::Error;
use crate::types::DeviceId;
use std::ffi::{CStr, CString};
use std::os::raw::c_char;
use std::ptr;
use std::sync::Mutex;

lazy_static::lazy_static! {
    static ref ENUMERATOR: Mutex<Enumerator> = Mutex::new(Enumerator::new());
}

/// Opaque handle to device info
#[repr(C)]
pub struct UsbDeviceHandle {
    _private: [u8; 0],
}

/// C-compatible device info structure
#[repr(C)]
pub struct UsbDeviceInfo {
    pub vid: u16,
    pub pid: u16,
    pub device_address: u8,
    pub num_configurations: u8,
    pub class_code: u8,
    pub subclass: u8,
    pub protocol: u8,
}

/// Initialize the library
/// # Safety
/// This function is unsafe because it initializes global state
#[no_mangle]
pub unsafe extern "C" fn usb_enum_init() -> i32 {
    0 // Success
}

/// Get number of USB devices
#[no_mangle]
pub extern "C" fn usb_enum_device_count() -> usize {
    let enumerator = ENUMERATOR.lock().unwrap();
    enumerator.count()
}

/// Get device at index (must call usb_enum_device_free on returned handle)
/// # Safety
/// The returned pointer must be freed with usb_enum_device_free
#[no_mangle]
pub unsafe extern "C" fn usb_enum_device_at(index: usize) -> *mut UsbDeviceHandle {
    ptr::null_mut() // Placeholder
}

/// Free device handle
/// # Safety
/// This function is unsafe because it deallocates memory
#[no_mangle]
pub unsafe extern "C" fn usb_enum_device_free(_handle: *mut UsbDeviceHandle) {
    // Placeholder
}

/// Get device info
/// # Safety
/// The returned pointer must be freed with usb_enum_device_info_free
#[no_mangle]
pub unsafe extern "C" fn usb_enum_device_info(
    _handle: *const UsbDeviceHandle,
) -> *mut UsbDeviceInfo {
    ptr::null_mut() // Placeholder
}

/// Free device info
/// # Safety
/// This function is unsafe because it deallocates memory
#[no_mangle]
pub unsafe extern "C" fn usb_enum_device_info_free(_info: *mut UsbDeviceInfo) {
    // Placeholder
}

/// Get device string (manufacturer, product, serial)
/// Returns newly allocated C string - caller must free with usb_enum_free_string
/// # Safety
/// The returned pointer must be freed with usb_enum_free_string
#[no_mangle]
pub unsafe extern "C" fn usb_enum_device_string(
    _handle: *const UsbDeviceHandle,
    field: i32, // 0=manufacturer, 1=product, 2=serial
) -> *mut c_char {
    ptr::null_mut() // Placeholder
}

/// Free allocated string
/// # Safety
/// This function is unsafe because it deallocates memory
#[no_mangle]
pub unsafe extern "C" fn usb_enum_free_string(s: *mut c_char) {
    if !s.is_null() {
        let _ = CString::from_raw(s);
    }
}

/// Get library version
/// # Safety
/// Returns pointer to statically allocated string
#[no_mangle]
pub extern "C" fn usb_enum_version() -> *const c_char {
    c"0.1.0".as_ptr()
}
