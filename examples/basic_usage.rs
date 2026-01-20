//! Basic usage examples for rational trigonometry
//!
//! This example demonstrates the fundamental operations of rational trigonometry

use num_rational::Ratio;
use rat_trig_rs::geometry::*;
use rat_trig_rs::trigonom::*;
use rat_trig_rs::validation::*;

fn main() {
    println!("=== Rational Trigonometry Basic Usage ===\n");

    // Example 1: Calculate quadrance (squared distance)
    println!("Example 1: Quadrance (squared distance)");
    let p1 = (1, 2);
    let p2 = (4, 6);
    let q = quadrance(p1, p2);
    println!(
        "  Distance squared between ({}, {}) and ({}, {}): {}",
        p1.0, p1.1, p2.0, p2.1, q
    );
    println!("  Actual distance: {}\n", (q as f64).sqrt());

    // Example 2: Calculate spread (squared sine)
    println!("Example 2: Spread (squared sine of angle)");
    let v1 = (1.0, 1.0);
    let v2 = (1.0, 0.0);
    let s: f64 = spread(v1, v2);
    println!(
        "  Spread between vectors ({}, {}) and ({}, {}): {}",
        v1.0, v1.1, v2.0, v2.1, s
    );
    println!("  Actual sine: {}\n", s.sqrt());

    // Example 3: Calculate cross product
    println!("Example 3: Cross product");
    let v1 = (1, 1);
    let v2 = (1, 0);
    let c = cross(v1, v2);
    println!(
        "  Cross product of ({}, {}) and ({}, {}): {}\n",
        v1.0, v1.1, v2.0, v2.1, c
    );

    // Example 4: Triangle properties
    println!("Example 4: Triangle properties");
    let p1 = (0, 0);
    let p2 = (3, 0);
    let p3 = (0, 4);

    let (q1, q2, q3) = quadrance_from_three_points(p1, p2, p3);
    println!("  Triangle with vertices:");
    println!("    P1: ({}, {})", p1.0, p1.1);
    println!("    P2: ({}, {})", p2.0, p2.1);
    println!("    P3: ({}, {})", p3.0, p3.1);
    println!(
        "  Quadrances (squared side lengths): q1={}, q2={}, q3={}",
        q1, q2, q3
    );

    let (s1, s2, s3) = spread_from_three_points(p1, p2, p3);
    println!("  Spreads (squared sines): s1={}, s2={}, s3={}", s1, s2, s3);

    let quadrea = archimedes(&q1, &q2, &q3);
    println!("  Quadrea (16 * area^2): {}", quadrea);
    println!("  Area: {}\n", (quadrea as f64).sqrt() / 4.0);

    // Example 5: Using geometry primitives
    println!("Example 5: Using geometry primitives");
    let p1 = Point2D::new(0, 0);
    let p2 = Point2D::new(3, 0);
    let p3 = Point2D::new(0, 4);
    let triangle = Triangle2D::new(p1, p2, p3);

    println!("  Triangle area: {}", triangle.area());
    println!("  Triangle twist (2 * signed area): {}", triangle.twist());
    println!("  Is degenerate: {}\n", triangle.is_degenerate());

    // Example 6: Rational calculations
    println!("Example 6: Exact rational calculations");
    let q1 = Ratio::new(1, 2);
    let q2 = Ratio::new(1, 4);
    let q3 = Ratio::new(1, 6);
    let quadrea = archimedes(&q1, &q2, &q3);
    println!("  Archimedes formula with rational sides:");
    println!("    q1 = {}, q2 = {}, q3 = {}", q1, q2, q3);
    println!("    Quadrea = {} (exact rational result)\n", quadrea);

    // Example 7: Validation
    println!("Example 7: Geometric validation");
    let p1 = (0, 0);
    let p2 = (1, 1);
    let p3 = (2, 2);
    println!(
        "  Points: ({}, {}), ({}, {}), ({}, {})",
        p1.0, p1.1, p2.0, p2.1, p3.0, p3.1
    );
    println!("  Are collinear: {}", are_collinear(p1, p2, p3));
    println!("  Form valid triangle: {}\n", is_valid_triangle(p1, p2, p3));

    // Example 8: 3D operations
    println!("Example 8: 3D operations");
    let p1 = (0, 0, 0);
    let p2 = (1, 2, 3);
    let q3d = quadrance3d(p1, p2);
    println!(
        "  3D quadrance between ({}, {}, {}) and ({}, {}, {}): {}",
        p1.0, p1.1, p1.2, p2.0, p2.1, p2.2, q3d
    );

    let v1 = (1, 0, 0);
    let v2 = (0, 1, 0);
    let cross3d = cross3d(v1, v2);
    println!(
        "  3D cross product of ({}, {}, {}) and ({}, {}, {}): ({}, {}, {})",
        v1.0, v1.1, v1.2, v2.0, v2.1, v2.2, cross3d.0, cross3d.1, cross3d.2
    );
}
