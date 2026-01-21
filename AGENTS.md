# Agent Guidelines for rat-trig-rs

This file provides guidance for AI agents working on the rat-trig-rs codebase.

## Build, Test, and Lint Commands

### Essential Commands
```bash
# Build
cargo build                    # Debug build
cargo build --release          # Release build with LTO enabled

# Tests
cargo test                    # Run all tests
cargo test <testname>         # Run single test by name
cargo test --all-features --workspace  # CI test command

# Formatting
cargo fmt --all               # Format all code
cargo fmt --all --check       # Check formatting without modifying

# Linting
cargo clippy --all-targets --all-features --workspace  # Run Clippy
cargo doc --no-deps --document-private-items --all-features --workspace --examples  # Check docs

# Security/License checks
cargo deny check              # Verify licenses and vulnerabilities
```

### Running Specific Tests
- Run tests in a specific file: `cargo test --lib <module_name>::tests`
- Run a single test: `cargo test <test_function_name>`
- Run integration tests: `cargo test --test integration`

## Code Style Guidelines

### Language and Edition
- Rust 2021 edition
- `#![no_std]` library (no standard library dependencies)
- Minimum Rust version: 1.70.0

### Generic Programming Patterns

**Use Generics with Trait Bounds**
- Functions should be generic over type parameter `T` where possible
- Common trait bounds: `Copy`, `Add`, `Sub`, `Mul`, `Div`, `One`, `Zero`, `PartialEq`, `PartialOrd`
- Example:
```rust
pub fn quadrance<T>(p1: (T, T), p2: (T, T)) -> T
where
    T: Copy + Add<Output = T> + Sub<Output = T> + Mul<Output = T>,
{
    let dx = p1.0 - p2.0;
    let dy = p1.1 - p2.1;
    dx * dx + dy * dy
}
```

**Performance Attributes**
- Add `#[inline]` to small, frequently-called functions
- This is a project-wide pattern for performance optimization

### Naming Conventions

**Functions**: `snake_case`
- Core operations: `quadrance`, `spread`, `cross`, `archimedes`
- Variants: `quadrance_from_line`, `spread_from_three_points`
- Validation: `is_valid_triangle`, `are_collinear`

**Types**: `PascalCase`
- Structs: `Point2D<T>`, `Vector2D<T>`, `Triangle2D<T>`, `Line2D<T>`
- Module names: `trigonom`, `geometry`, `validation`, `const_trigonom`

**Variables**: `snake_case`
- Generic type parameter: `T`
- Coordinates: `x`, `y`, `z` for points; `a`, `b`, `c` for line coefficients
- Geometry: `q1`, `q2`, `q3` for quadrances; `s1`, `s2`, `s3` for spreads
- Points: `p1`, `p2`, `p3` or `p_1`, `p_2`, `p_3`
- Vectors: `v1`, `v2` or `v_1`, `v_2`

### Imports and Module Structure

**Standard Pattern**:
```rust
use core::ops::{Add, Div, Mul, Sub};  // Use core::ops, not std::ops (no_std)
use num_traits::{One, Zero};

// Then declare module functions or types
pub fn function_name<T>(...) -> T
where
    T: Copy + Add<Output = T> + ...,
{
    // implementation
}
```

**Module Organization**:
- `src/lib.rs`: Entry point, declare public modules
- `src/trigonom.rs`: Core rational trigonometry functions
- `src/geometry.rs`: Structured geometry primitives (Point2D, Vector2D, Triangle2D, etc.)
- `src/validation.rs`: Validation utilities
- `src/const_trigonom.rs`: Const-evaluable functions for concrete types (i32, i64, f64)

### Documentation Standards

**All public functions must have documentation with**:
1. Brief description
2. Arguments section
3. Returns section
4. Example with `assert_eq!` for expected behavior

**Template**:
```rust
/// Calculate the [function description].
///
/// Arguments:
///
/// * `arg1`: Description of first argument
/// * `arg2`: Description of second argument
///
/// Returns:
///
/// The [description of return value].
///
/// Example:
///
/// ```rust
/// use rat_trig_rs::module_name::function_name;
/// let result = function_name(1, 2);
/// assert_eq!(result, expected_value);
/// ```
#[inline]
pub fn function_name<T>(arg1: T, arg2: T) -> T
where
    T: Copy + ...,
{
    // implementation
}
```

### Testing Guidelines

**All tests in `#[cfg(test)]` modules within each source file**
- Test module at end of file
- Use descriptive test names: `test_<function_name>`, `test_<function_name>_<scenario>`
- Test multiple numeric types: i32, i64, f64, Rational32
- Use `assert_eq!` for exact comparisons
- Use `assert!` for boolean checks

**Example**:
```rust
#[cfg(test)]
mod tests {
    use super::*;
    use num_rational::Ratio;

    #[test]
    fn test_quadrance() {
        let p1 = (1, 1);
        let p2 = (4, 5);
        assert_eq!(quadrance(p1, p2), 25);
    }

    #[test]
    fn test_quadrance_rational() {
        let p1 = (Ratio::new(1, 2), Ratio::new(1, 2));
        let p2 = (Ratio::new(4, 1), Ratio::new(5, 1));
        assert_eq!(quadrance(p1, p2), Ratio::new(25, 1));
    }
}
```

### Data Structures

**Coordinate Representations**:
- Tuples for simple operations: `(x, y)`, `(x, y, z)`, `(a, b, c)`
- Structs for structured operations: `Point2D<T>`, `Vector2D<T>`, `Triangle2D<T>`

**Derive Traits**:
- Always derive `Debug`, `Clone`, `Copy`, `PartialEq`, `Eq` for simple structs
- Don't derive `Eq` for float-based structs (not safe)

**Example**:
```rust
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Point2D<T> {
    pub x: T,
    pub y: T,
}
```

### Trait Implementations

**Implement `From` for type conversions**:
```rust
impl<T> From<(T, T)> for Point2D<T> {
    fn from(tuple: (T, T)) -> Self {
        Self { x: tuple.0, y: tuple.1 }
    }
}
```

**Implement operator traits for math types**:
```rust
impl<T> Add for Vector2D<T>
where
    T: Add<Output = T>,
{
    type Output = Self;
    fn add(self, other: Self) -> Self::Output {
        Self { x: self.x + other.x, y: self.y + other.y }
    }
}
```

### Error Handling

**Prevent panic in generic code by checking zero**:
```rust
pub fn safe_divide<T>(numerator: T, denominator: T) -> T
where
    T: Copy + Div<Output = T> + Zero + PartialEq,
{
    if denominator == T::zero() {
        return T::zero();  // Or handle appropriately
    }
    numerator / denominator
}
```

### Const Evaluation

**For const contexts, provide concrete implementations** in `const_trigonom.rs`:
- Function pattern: `<function_name>_<type>` (e.g., `quadrance_i64`, `spread_f64`)
- Use `const fn` instead of `fn`
- No trait bounds, concrete types only

### Linting Requirements

CI pipeline runs the following checks (all must pass):
- `cargo fmt --all --check` - Code must be formatted
- `cargo clippy --all-targets --all-features --workspace` - No Clippy warnings
- `cargo test --all-features --workspace` - All tests must pass
- `cargo doc --no-deps --document-private-items --all-features --workspace --examples` - Documentation must build without warnings

### Dependencies

**Production**: `num-traits` only (minimal dependency footprint)
**Dev**: `num-rational`, `fractions-rs`, `quickcheck`, `quickcheck_macros`, `criterion`

When adding dependencies:
- Prefer minimal, well-maintained crates
- Check `deny.toml` for allowed licenses (MIT, Apache-2.0 preferred)
- Run `cargo deny check` before committing

### Release Profile

The project uses aggressive release optimizations (see `[profile.release]` in Cargo.toml):
- LTO enabled (`lto = true`)
- Single codegen unit (`codegen-units = 1`)
- Panic on abort (`panic = "abort"`)

New code should be performance-conscious and compatible with these optimizations.
