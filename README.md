# 📐 rat-trig-rs

Rational Trigonometry - A modern approach to classical trigonometry using rational numbers instead of irrational values.

[![Crates.io](https://img.shields.io/crates/v/rat-trig-rs.svg)](https://crates.io/crates/rat-trig-rs)
[![Docs.rs](https://docs.rs/rat-trig-rs/badge.svg)](https://docs.rs/rat-trig-rs)
[![CI](https://github.com/luk036/rat-trig-rs/workflows/CI/badge.svg)](https://github.com/luk036/rat-trig-rs/actions)
[![codecov](https://codecov.io/gh/luk036/rat-trig-rs/graph/badge.svg?token=H7oT1T5LV5)](https://codecov.io/gh/luk036/rat-trig-rs)
[![License](https://img.shields.io/badge/license-MIT%20OR%20Apache--2.0-blue.svg)](LICENSE-APACHE)

## 📖 About

**Rational Trigonometry** is an alternative approach developed by Norman Wildberger that replaces traditional circular definitions with line-based operations. Instead of sine, cosine, and tangent, it uses:

- **Quadrance** - The square of distance (replaces distance)
- **Spread** - The square of sine (replaces angle)
- **Cross** - Signed area calculations

This approach offers:
- ✅ **Exact calculations** with rational numbers (no floating-point errors)
- ✅ **No irrational numbers** - all operations use rational arithmetic
- ✅ **Simpler formulas** - often more elegant than traditional trigonometry
- ✅ **`#![no_std]` compatible** - works in embedded systems

## 🚀 Getting Started

### Installation

Add to your `Cargo.toml`:

```toml
[dependencies]
rat-trig-rs = "0.1"
```

### Quick Example

```rust
use rat_trig_rs::trigonom::quadrance;
use rat_trig_rs::geometry::{Point2D, Triangle2D};

fn main() {
    // Calculate quadrance (squared distance) between two points
    let p1 = (1, 2);
    let p2 = (4, 6);
    let q = quadrance(p1, p2);
    println!("Quadrance: {}", q);  // Output: 25

    // Work with geometry primitives
    let triangle = Triangle2D::new(
        Point2D::new(0, 0),
        Point2D::new(3, 0),
        Point2D::new(0, 4),
    );
    let (q1, q2, q3) = triangle.quadrances();
    println!("Triangle quadrances: {}, {}, {}", q1, q2, q3);
}
```

## 📚 Core Concepts

### Quadrance

Quadrance is the square of distance. Instead of `d = √[(x₂-x₁)² + (y₂-y₁)²]`, we work directly with the squared value:

```rust
use rat_trig_rs::trigonom::quadrance;

let p1 = (0, 0);
let p2 = (3, 4);
let q = quadrance(p1, p2);  // q = 25 (distance²)
```

### Spread

Spread represents the square of the sine of an angle between two vectors.
For exact rational arithmetic, use rational numbers:

```rust
use num_rational::Ratio;
use rat_trig_rs::trigonom::spread;

let v1 = (Ratio::new(1, 1), Ratio::new(1, 1));
let v2 = (Ratio::new(1, 1), Ratio::new(0, 1));
let s = spread(v1, v2);  // s = 1/2 (sin²(45°))
```

### Archimedes' Formula

Calculate 16×(area)² from three side quadrances:

```rust
use rat_trig_rs::trigonom::{quadrance_from_three_points, archimedes};

let p1 = (0, 0);
let p2 = (3, 0);
let p3 = (0, 4);
let (q1, q2, q3) = quadrance_from_three_points(p1, p2, p3);
let quadrea = archimedes(&q1, &q2, &q3);
// area = (quadrea as f64).sqrt() / 4.0 = 6.0  (when converted to float)
```

## 🔧 API Overview

### Tuple API

Simple and fast - works with tuples:

```rust
use rat_trig_rs::trigonom::*;

// 2D operations
let q = quadrance((0, 0), (3, 4));
let s = spread((1, 1), (1, 0));
let c = cross((1, 1), (1, 0));

// 3D operations
let q3d = quadrance3d((0, 0, 0), (1, 2, 2));
let cross3d = cross3d((1, 0, 0), (0, 1, 0));
```

### Struct API

Organized and extensible - works with structs:

```rust
use rat_trig_rs::geometry::*;

let p1 = Point2D::new(0, 0);
let p2 = Point2D::new(3, 4);
let triangle = Triangle2D::new(p1, p2, Point2D::new(0, 4));

let (q1, q2, q3) = triangle.quadrances();
let area = triangle.area();
```

### Validation API

Check geometric properties:

```rust
use rat_trig_rs::validation::*;

let p1 = (0, 0);
let p2 = (1, 0);
let p3 = (0, 1);

assert!(is_valid_triangle(p1, p2, p3));
assert!(!are_collinear(p1, p2, p3));

let (s1, s2, s3) = spread_from_three_points(p1, p2, p3);
assert!(is_right_triangle(s1, s2, s3));
```

## 📦 Features

- **`#![no_std]`** - Works in embedded environments
- **Generic** - Works with i32, i64, and rational types
- **Exact arithmetic** - No floating-point errors with rational types
- **Const-friendly** - Const-evaluable functions available for concrete types

## 📖 Examples

See the `examples/` directory for more detailed examples:

```bash
cargo run --example basic_usage
cargo run --example triangle_analysis
cargo run --example line_operations
```

## 🧪 Testing

Run the test suite:

```bash
cargo test
```

Run with specific test names:

```bash
cargo test test_quadrance
cargo test test_spread
cargo test trigonom::tests  # All tests in trigonom module
```

## 📈 Performance

The library is optimized for performance:

- All functions marked `#[inline]`
- LTO enabled in release builds
- Minimal dependencies (only `num-traits`)

Benchmark your code:

```bash
cargo bench
```

## 🛠️ Development

```bash
# Build
cargo build

# Build release
cargo build --release

# Format code
cargo fmt --all

# Run clippy
cargo clippy --all-targets --all-features --workspace

# Run tests
cargo test --all-features --workspace

# Check documentation
cargo doc --no-deps --document-private-items --all-features --workspace
```

## 📜 License

Licensed under either of

- Apache License, Version 2.0
  ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
- MIT license
  ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

## 🤝 Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.

See [CONTRIBUTING.md](CONTRIBUTING.md).
