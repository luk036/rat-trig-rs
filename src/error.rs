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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_math_error_clone() {
        let err1 = MathError::DivisionByZero;
        let err2 = err1;
        assert_eq!(err1, err2);
    }

    #[test]
    fn test_math_error_copy() {
        let err1 = MathError::InvalidInput;
        let err2 = err1;
        assert_eq!(err1, err2);
    }

    #[test]
    fn test_math_error_partial_eq() {
        let err1 = MathError::Overflow;
        let err2 = MathError::Overflow;
        let err3 = MathError::DivisionByZero;
        assert_eq!(err1, err2);
        assert_ne!(err1, err3);
    }

    #[test]
    fn test_math_error_eq() {
        let err1 = MathError::Overflow;
        let err2 = MathError::Overflow;
        assert_eq!(err1, err2);
    }
}
