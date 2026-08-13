//! Advanced query builder for device searches

use crate::device::Device;
use crate::types::DeviceClass;
use std::collections::HashMap;

/// Fluent query builder for device enumeration
#[derive(Debug, Clone)]
pub struct QueryBuilder {
    filters: Vec<Box<dyn Fn(&Device) -> bool + Send + Sync>>,
}

impl QueryBuilder {
    /// Create new query builder
    pub fn new() -> Self {
        Self {
            filters: Vec::new(),
        }
    }

    /// Filter by vendor ID
    pub fn vendor_id(mut self, vid: u16) -> Self {
        self.filters.push(Box::new(move |d| d.info.vid == vid));
        self
    }

    /// Filter by product ID
    pub fn product_id(mut self, pid: u16) -> Self {
        self.filters.push(Box::new(move |d| d.info.pid == pid));
        self
    }

    /// Filter by manufacturer name pattern
    pub fn manufacturer_contains(mut self, pattern: String) -> Self {
        self.filters.push(Box::new(move |d| {
            d.info
                .manufacturer
                .as_ref()
                .map(|m| m.contains(&pattern))
                .unwrap_or(false)
        }));
        self
    }

    /// Filter by product name pattern
    pub fn product_contains(mut self, pattern: String) -> Self {
        self.filters.push(Box::new(move |d| {
            d.info
                .product
                .as_ref()
                .map(|p| p.contains(&pattern))
                .unwrap_or(false)
        }));
        self
    }

    /// Filter by device class
    pub fn class(mut self, class: DeviceClass) -> Self {
        self.filters.push(Box::new(move |d| d.info.class == class));
        self
    }

    /// Apply all filters to device list
    pub fn apply(&self, devices: &[Device]) -> Vec<Device> {
        devices
            .iter()
            .filter(|d| self.filters.iter().all(|f| f(d)))
            .cloned()
            .collect()
    }
}

impl Default for QueryBuilder {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::device::DeviceInfo;

    #[test]
    fn test_query_builder() {
        let mut info = DeviceInfo::new(0x1234, 0x5678, 1);
        info.manufacturer = Some("TestMfg".to_string());
        let device = Device::new(info);

        let query = QueryBuilder::new().vendor_id(0x1234);
        let results = query.apply(&[device]);
        assert_eq!(results.len(), 1);
    }
}
