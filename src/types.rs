//! Core type definitions for USB device representation

use serde::{Deserialize, Serialize};
use std::fmt;
use uuid::Uuid;

/// Unique device identifier
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct DeviceId(pub Uuid);

impl DeviceId {
    /// Generate a new random device ID
    pub fn new() -> Self {
        Self(Uuid::new_v4())
    }
}

impl Default for DeviceId {
    fn default() -> Self {
        Self::new()
    }
}

impl fmt::Display for DeviceId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

/// USB device speed/class specification
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum DeviceSpeed {
    Low,
    Full,
    High,
    Super,
    SuperPlus,
    Unknown,
}

impl fmt::Display for DeviceSpeed {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Low => write!(f, "Low (1.5 Mbps)"),
            Self::Full => write!(f, "Full (12 Mbps)"),
            Self::High => write!(f, "High (480 Mbps)"),
            Self::Super => write!(f, "Super (5 Gbps)"),
            Self::SuperPlus => write!(f, "Super+ (10+ Gbps)"),
            Self::Unknown => write!(f, "Unknown"),
        }
    }
}

/// USB device class classification
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum DeviceClass {
    MassStorage,
    Printer,
    Keyboard,
    Mouse,
    Monitor,
    Miscellaneous,
    ChipCardInterfaceDevice,
    SmartCard,
    WirelessController,
    PersonalHealthcare,
    AudioVideo,
    Billboard,
    Diagnostic,
    CommonAccessPortProfile,
    Vendor(u8),
    Unknown(u8),
}

impl fmt::Display for DeviceClass {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::MassStorage => write!(f, "Mass Storage"),
            Self::Printer => write!(f, "Printer"),
            Self::Keyboard => write!(f, "Keyboard"),
            Self::Mouse => write!(f, "Mouse"),
            Self::Monitor => write!(f, "Monitor"),
            Self::Miscellaneous => write!(f, "Miscellaneous"),
            Self::ChipCardInterfaceDevice => write!(f, "Chip Card Interface Device"),
            Self::SmartCard => write!(f, "Smart Card"),
            Self::WirelessController => write!(f, "Wireless Controller"),
            Self::PersonalHealthcare => write!(f, "Personal Healthcare"),
            Self::AudioVideo => write!(f, "Audio Video"),
            Self::Billboard => write!(f, "Billboard"),
            Self::Diagnostic => write!(f, "Diagnostic"),
            Self::CommonAccessPortProfile => write!(f, "Common Access Port Profile"),
            Self::Vendor(code) => write!(f, "Vendor (0x{:02x})", code),
            Self::Unknown(code) => write!(f, "Unknown (0x{:02x})", code),
        }
    }
}

/// Platform identifier
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Platform {
    Linux,
    macOS,
    Windows,
    BSD,
    Unknown,
}

impl fmt::Display for Platform {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Linux => write!(f, "Linux"),
            Self::macOS => write!(f, "macOS"),
            Self::Windows => write!(f, "Windows"),
            Self::BSD => write!(f, "BSD"),
            Self::Unknown => write!(f, "Unknown"),
        }
    }
}

impl Platform {
    /// Get current platform
    pub fn current() -> Self {
        #[cfg(target_os = "linux")]
        return Self::Linux;
        #[cfg(target_os = "macos")]
        return Self::macOS;
        #[cfg(target_os = "windows")]
        return Self::Windows;
        #[cfg(target_os = "freebsd")]
        return Self::BSD;
        #[cfg(not(any(
            target_os = "linux",
            target_os = "macos",
            target_os = "windows",
            target_os = "freebsd"
        )))]
        Self::Unknown
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_device_id_generation() {
        let id1 = DeviceId::new();
        let id2 = DeviceId::new();
        assert_ne!(id1, id2);
    }

    #[test]
    fn test_platform_detection() {
        let _platform = Platform::current();
    }
}
