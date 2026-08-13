//! Device filtering and query capabilities

use crate::device::Device;
use crate::types::DeviceClass;
use serde::{Deserialize, Serialize};

/// Device filter criteria
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeviceFilter {
    pub vendor_id: Option<u16>,
    pub product_id: Option<u16>,
    pub manufacturer: Option<String>,
    pub product: Option<String>,
    pub class: Option<DeviceClass>,
    pub serial_number: Option<String>,
}

impl DeviceFilter {
    /// Create empty filter
    pub fn new() -> Self {
        Self {
            vendor_id: None,
            product_id: None,
            manufacturer: None,
            product: None,
            class: None,
            serial_number: None,
        }
    }

    /// Set vendor ID filter
    pub fn with_vendor_id(mut self, vid: u16) -> Self {
        self.vendor_id = Some(vid);
        self
    }

    /// Set product ID filter
    pub fn with_product_id(mut self, pid: u16) -> Self {
        self.product_id = Some(pid);
        self
    }

    /// Set manufacturer filter
    pub fn with_manufacturer(mut self, manufacturer: String) -> Self {
        self.manufacturer = Some(manufacturer);
        self
    }

    /// Set product filter
    pub fn with_product(mut self, product: String) -> Self {
        self.product = Some(product);
        self
    }

    /// Set device class filter
    pub fn with_class(mut self, class: DeviceClass) -> Self {
        self.class = Some(class);
        self
    }

    /// Set serial number filter
    pub fn with_serial_number(mut self, serial: String) -> Self {
        self.serial_number = Some(serial);
        self
    }

    /// Check if device matches all filter criteria
    pub fn matches(&self, device: &Device) -> bool {
        if let Some(vid) = self.vendor_id {
            if device.info.vid != vid {
                return false;
            }
        }

        if let Some(pid) = self.product_id {
            if device.info.pid != pid {
                return false;
            }
        }

        if let Some(ref mfg) = self.manufacturer {
            if device.info.manufacturer.as_ref() != Some(mfg) {
                return false;
            }
        }

        if let Some(ref prod) = self.product {
            if device.info.product.as_ref() != Some(prod) {
                return false;
            }
        }

        if let Some(class) = self.class {
            if device.info.class != class {
                return false;
            }
        }

        if let Some(ref serial) = self.serial_number {
            if device.info.serial_number.as_ref() != Some(serial) {
                return false;
            }
        }

        true
    }
}

impl Default for DeviceFilter {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::device::DeviceInfo;

    #[test]
    fn test_filter_matching() {
        let mut info = DeviceInfo::new(0x1234, 0x5678, 1);
        info.manufacturer = Some("TestMfg".to_string());
        let device = Device::new(info);

        let filter = DeviceFilter::new()
            .with_vendor_id(0x1234)
            .with_manufacturer("TestMfg".to_string());

        assert!(filter.matches(&device));
    }
}
