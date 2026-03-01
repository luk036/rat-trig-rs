#![no_std]

//! Rational Trigonometry Library
//!
//! This library implements Norman Wildberger's Rational Trigonometry, an alternative approach
//! to classical trigonometry that uses rational numbers and squared distances
//! instead of traditional trigonometric functions.
//!
//! # Core Concepts
//!
//! - **Quadrance**: The square of distance between two points
//! - **Spread**: The square of the sine of an angle
//! - **Cross**: Signed area calculations
//!
//! # Features
//!
//! - `#![no_std]` compatible - works in embedded environments
//! - Generic over numeric types - supports i32, i64, f64, and rational numbers
//! - SIMD optimizations for bulk calculations (x86_64 + avx2)
//! - Optional serde serialization support
//! - Optional logging support (via `std` feature)
//! - Safe variants with proper error handling for fallible operations
//!
//! # Quick Start
//!
//! ```rust
//! use rat_trig_rs::trigonom::quadrance;
//!
//! let p1 = (0, 0);
//! let p2 = (3, 4);
//! let q = quadrance(p1, p2);  // q = 25
//! ```
//!
//! # Error Handling
//!
//! For operations that may fail (division by zero), use the `safe_*` variants:
//!
//! ```rust
//! use rat_trig_rs::trigonom::safe_spread;
//!
//! let v1 = (1.0, 0.0);
//! let v2 = (0.0, 0.0);
//! let result = safe_spread(v1, v2);  // Returns Err(MathError::DivisionByZero)
//! ```
//!
//! # Modules
//!
//! - [`trigonom`] - Core rational trigonometry functions
//! - [`geometry`] - Structured geometry primitives
//! - [`validation`] - Validation utilities
//! - [`const_trigonom`] - Const-evaluable functions for concrete types
//! - [`error`] - Error types for operations that may fail
//! - [`logging`] - Logging utilities (requires `std` feature)

pub mod const_trigonom;
pub mod error;
pub mod geometry;
#[cfg(feature = "std")]
pub mod logging;
pub mod trigonom;
pub mod validation;
