//! Error types for rational trigonometry operations.
//!
//! This module provides error types for functions that can fail,
//! replacing silent returns with explicit `Result` types.

use core::fmt;

/// Errors that can occur during geometric operations
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MathError {
    /// Division by zero attempted
    DivisionByZero,

    /// Invalid input provided (e.g., negative quadrance)
    InvalidInput,

    /// Calculation overflow
    Overflow,
}

impl fmt::Display for MathError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            MathError::DivisionByZero => write!(f, "division by zero"),
            MathError::InvalidInput => write!(f, "invalid input provided"),
            MathError::Overflow => write!(f, "calculation overflow"),
        }
    }
}
