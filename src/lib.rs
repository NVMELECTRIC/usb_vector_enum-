//! # USB Vector Enum
//!
//! The ultimate cross-platform USB device discovery engine with real-time notifications
//! and vector-backed queries.
//!
//! ## Features
//! - Real-time USB device enumeration and connect/disconnect notifications
//! - Full device metadata extraction (VID, PID, manufacturer, product, serial)
//! - Cross-platform support (Linux, macOS, Windows)
//! - Async/await first design with Tokio runtime
//! - FFI-safe C API for language bindings
//! - Vector-backed device storage for blazing-fast queries

pub mod device;
pub mod enumerator;
pub mod error;
pub mod ffi;
pub mod filter;
pub mod notification;
pub mod query;
pub mod types;

pub use device::{Device, DeviceInfo};
pub use enumerator::Enumerator;
pub use error::{Error, Result};
pub use filter::DeviceFilter;
pub use notification::{DeviceNotification, NotificationListener};
pub use query::QueryBuilder;
pub use types::{DeviceClass, DeviceSpeed, Platform};

/// Library version
pub const VERSION: &str = env!("CARGO_PKG_VERSION");

/// Initialize the USB enumeration library
pub async fn init() -> Result<()> {
    tracing_subscriber::fmt()
        .with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
        .init();
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_version() {
        assert!(!VERSION.is_empty());
    }
}
