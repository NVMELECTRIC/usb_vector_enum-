//! Main USB device enumerator

use crate::{
    device::{Device, DeviceInfo},
    error::{Error, Result},
    types::{DeviceClass, DeviceSpeed, Platform},
};
use async_trait::async_trait;
use std::sync::{Arc, Mutex};
use tokio::task::JoinHandle;

/// Main enumeration interface
#[derive(Clone)]
pub struct Enumerator {
    devices: Arc<Mutex<Vec<Device>>>,
}

impl Enumerator {
    /// Create new enumerator
    pub fn new() -> Self {
        Self {
            devices: Arc::new(Mutex::new(Vec::new())),
        }
    }

    /// Enumerate all USB devices
    pub async fn enumerate(&self) -> Result<Vec<Device>> {
        self.enumerate_internal().await
    }

    /// Enumerate devices with specific vendor ID
    pub async fn enumerate_by_vendor(&self, vid: u16) -> Result<Vec<Device>> {
        let all_devices = self.enumerate().await?;
        Ok(all_devices
            .into_iter()
            .filter(|d| d.info.vid == vid)
            .collect())
    }

    /// Enumerate devices with specific product ID
    pub async fn enumerate_by_product(&self, pid: u16) -> Result<Vec<Device>> {
        let all_devices = self.enumerate().await?;
        Ok(all_devices
            .into_iter()
            .filter(|d| d.info.pid == pid)
            .collect())
    }

    /// Enumerate devices by class
    pub async fn enumerate_by_class(&self, class: DeviceClass) -> Result<Vec<Device>> {
        let all_devices = self.enumerate().await?;
        Ok(all_devices
            .into_iter()
            .filter(|d| d.info.class == class)
            .collect())
    }

    /// Get total device count
    pub fn count(&self) -> usize {
        self.devices
            .lock()
            .map(|devices| devices.len())
            .unwrap_or(0)
    }

    async fn enumerate_internal(&self) -> Result<Vec<Device>> {
        let mut devices = Vec::new();

        // Query libusb for devices
        match rusb::devices() {
            Ok(device_list) => {
                for device in device_list.iter() {
                    if let Ok(device_info) = self.extract_device_info(device).await {
                        devices.push(Device::new(device_info));
                    }
                }
            }
            Err(e) => {
                tracing::warn!("Failed to enumerate USB devices: {:?}", e);
            }
        }

        // Cache results
        *self.devices.lock().unwrap() = devices.clone();

        Ok(devices)
    }

    async fn extract_device_info(&self, device: &rusb::Device<rusb::GlobalContext>) -> Result<DeviceInfo> {
        let descriptor = device.device_descriptor()?;
        let address = device.address();

        let mut info = DeviceInfo::new(descriptor.vendor_id(), descriptor.product_id(), address);
        info.class = self.classify_device(descriptor.class_code());
        info.subclass = descriptor.sub_class_code();
        info.protocol = descriptor.protocol_code();
        info.speed = self.detect_speed(device);
        info.num_configurations = descriptor.num_configurations();
        info.platform = Platform::current();

        // Extract string descriptors
        if let Ok(handle) = device.open() {
            if let Ok(lang) = handle.read_languages(std::time::Duration::from_secs(1)) {
                if let Some(lang_id) = lang.first() {
                    if let Ok(mfg) = handle.read_manufacturer_string(*lang_id, &descriptor, std::time::Duration::from_secs(1)) {
                        info.manufacturer = Some(mfg);
                    }
                    if let Ok(prod) = handle.read_product_string(*lang_id, &descriptor, std::time::Duration::from_secs(1)) {
                        info.product = Some(prod);
                    }
                    if let Ok(serial) = handle.read_serial_number_string(*lang_id, &descriptor, std::time::Duration::from_secs(1)) {
                        info.serial_number = Some(serial);
                    }
                }
            }
        }

        Ok(info)
    }

    fn classify_device(&self, class_code: u8) -> DeviceClass {
        match class_code {
            0x08 => DeviceClass::MassStorage,
            0x07 => DeviceClass::Printer,
            0x03 => DeviceClass::Keyboard,
            0x06 => DeviceClass::Monitor,
            0x0e => DeviceClass::AudioVideo,
            0xff => DeviceClass::Vendor(class_code),
            _ => DeviceClass::Unknown(class_code),
        }
    }

    fn detect_speed(&self, _device: &rusb::Device<rusb::GlobalContext>) -> DeviceSpeed {
        // Basic speed detection; can be enhanced with device descriptor analysis
        DeviceSpeed::High
    }
}

impl Default for Enumerator {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_enumerator_creation() {
        let enumerator = Enumerator::new();
        let devices = enumerator.enumerate().await;
        assert!(devices.is_ok());
    }
}
