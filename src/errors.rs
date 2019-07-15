//! Utility module for containing error-handling functions and types

/// Specific `Result` type for Cookhub
pub type Result<T> = std::result::Result<T, failure::Error>;
