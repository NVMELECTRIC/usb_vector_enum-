//! Device representation and metadata

use crate::types::{DeviceClass, DeviceId, DeviceSpeed, Platform};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Comprehensive USB device information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeviceInfo {
    /// Unique device identifier
    pub id: DeviceId,
    /// Vendor ID (VID)
    pub vid: u16,
    /// Product ID (PID)
    pub pid: u16,
    /// Manufacturer name
    pub manufacturer: Option<String>,
    /// Product name
    pub product: Option<String>,
    /// Serial number
    pub serial_number: Option<String>,
    /// Device class
    pub class: DeviceClass,
    /// Device subclass
    pub subclass: u8,
    /// Device protocol
    pub protocol: u8,
    /// USB speed capability
    pub speed: DeviceSpeed,
    /// Bus number (Linux/BSD)
    pub bus_number: Option<u8>,
    /// Device address
    pub device_address: u8,
    /// Number of configuration descriptors
    pub num_configurations: u8,
    /// Platform where device is connected
    pub platform: Platform,
    /// Device path (OS-specific)
    pub path: Option<String>,
    /// Additional metadata
    pub metadata: HashMap<String, String>,
}

impl DeviceInfo {
    /// Create new device info
    pub fn new(vid: u16, pid: u16, device_address: u8) -> Self {
        Self {
            id: DeviceId::new(),
            vid,
            pid,
            manufacturer: None,
            product: None,
            serial_number: None,
            class: DeviceClass::Unknown(0),
            subclass: 0,
            protocol: 0,
            speed: DeviceSpeed::Unknown,
            bus_number: None,
            device_address,
            num_configurations: 0,
            platform: Platform::current(),
            path: None,
            metadata: HashMap::new(),
        }
    }

    /// Get human-readable device name
    pub fn name(&self) -> String {
        self.product
            .clone()
            .or_else(|| self.manufacturer.clone())
            .unwrap_or_else(|| format!("USB Device {:04x}:{:04x}", self.vid, self.pid))
    }

    /// Get full device descriptor
    pub fn descriptor(&self) -> String {
        format!(
            "{} ({}:{}) @ {}",
            self.name(),
            format_hex(self.vid),
            format_hex(self.pid),
            self.path
                .as_deref()
                .unwrap_or("Unknown")
        )
    }
}

impl PartialEq for DeviceInfo {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl Eq for DeviceInfo {}

/// Device abstraction layer
#[derive(Debug, Clone)]
pub struct Device {
    pub info: DeviceInfo,
}

impl Device {
    /// Create new device wrapper
    pub fn new(info: DeviceInfo) -> Self {
        Self { info }
    }

    /// Check if device matches criteria
    pub fn matches(&self, vid: Option<u16>, pid: Option<u16>) -> bool {
        match (vid, pid) {
            (Some(v), Some(p)) => self.info.vid == v && self.info.pid == p,
            (Some(v), None) => self.info.vid == v,
            (None, Some(p)) => self.info.pid == p,
            (None, None) => true,
        }
    }
}

/// Format u16 as hex string
fn format_hex(value: u16) -> String {
    format!("0x{:04x}", value)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_device_creation() {
        let info = DeviceInfo::new(0x1234, 0x5678, 1);
        assert_eq!(info.vid, 0x1234);
        assert_eq!(info.pid, 0x5678);
    }

    #[test]
    fn test_device_name() {
        let mut info = DeviceInfo::new(0x1234, 0x5678, 1);
        info.product = Some("Test Device".to_string());
        assert_eq!(info.name(), "Test Device");
    }

    #[test]
    fn test_device_matching() {
        let info = DeviceInfo::new(0x1234, 0x5678, 1);
        let device = Device::new(info);
        assert!(device.matches(Some(0x1234), Some(0x5678)));
        assert!(device.matches(Some(0x1234), None));
        assert!(!device.matches(Some(0x9999), None));
    }
}
