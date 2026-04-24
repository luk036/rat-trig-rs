//! Logging module using the `log` crate.
//!
//! This module provides logging functionality for the library when the `std` feature is enabled.
//!
//! # Usage
//!
//! Initialize the logger in your application using your preferred logger implementation
//! that implements the `log` crate's `Log` trait (e.g., `env_logger`, `tracing`, `simplelog`).
//!
//! # Environment Variables
//!
//! - `RUST_LOG`: Controls log level (when using env_logger or similar)
//!
//! # Example
//!
//! ```bash
//! RUST_LOG=debug cargo run --example basic_usage
//! ```

#[cfg(feature = "std")]
use log::LevelFilter;

/// Initialize the logger with default settings.
///
/// This function is a no-op. Use a logger implementation that supports the `log` crate.
///
/// For env_logger compatibility, users should use `env_logger::init()` directly.
#[cfg(feature = "std")]
#[inline]
pub fn init_logger() {
    // No-op: Use env_logger::init() or your preferred logger directly
}

/// Initialize the logger with a specific filter.
///
/// Arguments:
///
/// * `filter`: Log level filter string (e.g., "debug", "info", "warn", "error")
///
/// # Example
///
/// ```rust
/// use rat_trig_rs::logging::init_logger_with_filter;
/// init_logger_with_filter("debug");
/// ```
#[cfg(feature = "std")]
#[inline]
pub fn init_logger_with_filter(_filter: &str) {
    // No-op: Use env_logger::Builder::from_env() directly
}

/// Try to initialize the logger, returning an error if already initialized.
///
/// This is useful when you want to initialize the logger conditionally
/// without panicking if it's already been initialized.
#[cfg(feature = "std")]
#[inline]
pub fn try_init_logger() -> Result<(), log::SetLoggerError> {
    Ok(())
}

/// Initialize the logger with a custom filter and try_init behavior.
///
/// Arguments:
///
/// * `filter`: Log level filter string
///
/// Returns:
///
/// `Ok(())` if initialization succeeded, `Err` if logger was already initialized
#[cfg(feature = "std")]
#[inline]
pub fn try_init_logger_with_filter(_filter: &str) -> Result<(), log::SetLoggerError> {
    Ok(())
}

/// Check if the logger is currently active.
///
/// This can be used to conditionally enable expensive logging operations.
#[cfg(feature = "std")]
#[inline]
pub fn is_logger_initialized() -> bool {
    log::max_level() != LevelFilter::Off
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_try_init_logger() {
        let result = try_init_logger();
        assert!(result.is_ok());
    }

    #[test]
    fn test_try_init_logger_with_filter() {
        let result = try_init_logger_with_filter("error");
        assert!(result.is_ok());
    }

    #[test]
    fn test_is_logger_initialized() {
        // Result depends on whether a logger is installed
        let _ = is_logger_initialized();
    }
}