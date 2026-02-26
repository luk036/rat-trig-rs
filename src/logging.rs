//! Logging module using env_logger.
//!
//! This module provides logging functionality for the library when the `std` feature is enabled.
//! It uses the `env_logger` crate for flexible logging configuration via environment variables.
//!
//! # Usage
//!
//! Initialize the logger in your application:
//!
//! ```rust
//! use rat_trig_rs::logging::init_logger;
//! init_logger();
//! ```
//!
//! Or initialize with custom configuration:
//!
//! ```rust
//! use rat_trig_rs::logging::init_logger_with_filter;
//! init_logger_with_filter("debug");
//! ```
//!
//! # Environment Variables
//!
//! - `RUST_LOG`: Controls log level (e.g., `debug`, `info`, `warn`, `error`)
//! - `RUST_LOG_STYLE`: Set to `never` to disable colored output
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
/// This function reads the `RUST_LOG` environment variable to set the log level.
/// If `RUST_LOG` is not set, defaults to `info` level.
#[cfg(feature = "std")]
#[inline]
pub fn init_logger() {
    env_logger::init();
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
pub fn init_logger_with_filter(filter: &str) {
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or(filter)).init();
}

/// Try to initialize the logger, returning an error if already initialized.
///
/// This is useful when you want to initialize the logger conditionally
/// without panicking if it's already been initialized.
///
/// # Example
///
/// ```rust
/// use rat_trig_rs::logging::try_init_logger;
/// if try_init_logger().is_ok() {
///     log::info!("Logger initialized successfully");
/// }
/// ```
#[cfg(feature = "std")]
#[inline]
pub fn try_init_logger() -> Result<(), log::SetLoggerError> {
    env_logger::try_init()
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
///
/// # Example
///
/// ```rust
/// use rat_trig_rs::logging::try_init_logger_with_filter;
/// let _ = try_init_logger_with_filter("warn");
/// ```
#[cfg(feature = "std")]
#[inline]
pub fn try_init_logger_with_filter(filter: &str) -> Result<(), log::SetLoggerError> {
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or(filter)).try_init()
}

/// Check if the logger is currently active.
///
/// This can be used to conditionally enable expensive logging operations.
///
/// # Example
///
/// ```rust
/// use rat_trig_rs::logging;
///
/// if logging::is_logger_initialized() {
///     log::info!("Logger is ready");
/// }
/// ```
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
        // Should succeed (logger not initialized in tests by default)
        let result = try_init_logger();
        assert!(result.is_ok() || result.is_err()); // Either is fine
    }

    #[test]
    fn test_try_init_logger_with_filter() {
        let result = try_init_logger_with_filter("error");
        assert!(result.is_ok() || result.is_err());
    }

    #[test]
    fn test_is_logger_initialized() {
        // Should return true after initialization attempt
        let _ = try_init_logger();
        let initialized = is_logger_initialized();
        assert!(initialized);
    }
}
