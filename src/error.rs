//! Error types and handling

use thiserror::Error;

/// Result type alias
pub type Result<T> = std::result::Result<T, Error>;

/// USB enumeration error types
#[derive(Debug, Error)]
pub enum Error {
    #[error("USB error: {0}")]
    Usb(String),

    #[error("Device not found")]
    DeviceNotFound,

    #[error("Device not available")]
    DeviceNotAvailable,

    #[error("Invalid argument: {0}")]
    InvalidArgument(String),

    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    #[error("Permission denied")]
    PermissionDenied,

    #[error("Timeout occurred")]
    Timeout,

    #[error("Configuration error: {0}")]
    Configuration(String),

    #[error("Notification error: {0}")]
    Notification(String),

    #[error("Serialization error: {0}")]
    Serialization(#[from] serde_json::Error),

    #[error("Channel error: {0}")]
    Channel(String),

    #[error("Runtime error: {0}")]
    Runtime(String),

    #[error("Unknown error: {0}")]
    Unknown(String),
}

impl From<rusb::Error> for Error {
    fn from(err: rusb::Error) -> Self {
        Error::Usb(format!("{:?}", err))
    }
}

impl From<crossbeam_channel::RecvError> for Error {
    fn from(err: crossbeam_channel::RecvError) -> Self {
        Error::Channel(err.to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_error_display() {
        let err = Error::DeviceNotFound;
        assert_eq!(err.to_string(), "Device not found");
    }
}
