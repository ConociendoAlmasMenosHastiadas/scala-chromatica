//! Error types for colormap I/O operations

use std::io;

/// Error types for colormap operations
#[derive(Debug)]
pub enum ColorMapError {
    /// I/O error (file read/write)
    IoError(io::Error),
    /// JSON parsing/serialization error
    JsonError(serde_json::Error),
    /// Colormap not found by name
    NotFound(String),
    /// Could not determine config directory
    NoConfigDirectory,
}

impl std::fmt::Display for ColorMapError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ColorMapError::IoError(e) => write!(f, "I/O error: {}", e),
            ColorMapError::JsonError(e) => write!(f, "JSON error: {}", e),
            ColorMapError::NotFound(name) => write!(f, "ColorMap '{}' not found", name),
            ColorMapError::NoConfigDirectory => write!(f, "Could not find config directory"),
        }
    }
}

impl std::error::Error for ColorMapError {}

impl From<io::Error> for ColorMapError {
    fn from(err: io::Error) -> Self {
        ColorMapError::IoError(err)
    }
}

impl From<serde_json::Error> for ColorMapError {
    fn from(err: serde_json::Error) -> Self {
        ColorMapError::JsonError(err)
    }
}

/// Result type for colormap operations
pub type Result<T> = std::result::Result<T, ColorMapError>;
