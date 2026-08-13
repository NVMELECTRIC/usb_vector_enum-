//! Device connection/disconnection notifications

use crate::device::Device;
use crate::error::Result;
use crossbeam_channel::{bounded, Receiver, Sender};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tokio::task::JoinHandle;

/// Device notification event
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum DeviceNotification {
    Connected(Device),
    Disconnected(Device),
    Error(String),
}

/// Device event listener
pub struct NotificationListener {
    rx: Receiver<DeviceNotification>,
    tx: Sender<DeviceNotification>,
}

impl NotificationListener {
    /// Create new notification listener
    pub fn new(capacity: usize) -> Self {
        let (tx, rx) = bounded(capacity);
        Self { rx, tx }
    }

    /// Get sender for notifications
    pub fn sender(&self) -> Sender<DeviceNotification> {
        self.tx.clone()
    }

    /// Receive next notification
    pub fn recv(&self) -> Result<DeviceNotification> {
        self.rx.recv().map_err(|e| crate::error::Error::Channel(e.to_string()))
    }

    /// Try to receive notification without blocking
    pub fn try_recv(&self) -> Option<DeviceNotification> {
        self.rx.try_recv().ok()
    }

    /// Check if listener is empty
    pub fn is_empty(&self) -> bool {
        self.rx.is_empty()
    }
}

impl Clone for NotificationListener {
    fn clone(&self) -> Self {
        Self {
            rx: self.rx.clone(),
            tx: self.tx.clone(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::DeviceId;
    use crate::device::DeviceInfo;

    #[test]
    fn test_notification_listener() {
        let listener = NotificationListener::new(10);
        let device_info = DeviceInfo::new(0x1234, 0x5678, 1);
        let device = Device::new(device_info);

        let sender = listener.sender();
        sender
            .send(DeviceNotification::Connected(device))
            .expect("Send failed");

        let notification = listener.recv().expect("Recv failed");
        match notification {
            DeviceNotification::Connected(_) => {},
            _ => panic!("Expected Connected notification"),
        }
    }
}
