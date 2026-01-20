//! Triangle analysis example
//!
//! This example demonstrates comprehensive triangle analysis using rational trigonometry

use rat_trig_rs::trigonom::*;
use rat_trig_rs::validation::*;

fn analyze_triangle(p1: (f64, f64), p2: (f64, f64), p3: (f64, f64)) {
    println!("=== Triangle Analysis ===");
    println!("Vertices:");
    println!("  P1: ({}, {})", p1.0, p1.1);
    println!("  P2: ({}, {})", p2.0, p2.1);
    println!("  P3: ({}, {})", p3.0, p3.1);

    // Check if valid
    if !is_valid_triangle(p1, p2, p3) {
        println!("  ERROR: Points are collinear, cannot form a valid triangle!\n");
        return;
    }

    // Calculate quadrances
    let (q1, q2, q3) = quadrance_from_three_points(p1, p2, p3);
    println!("\nSide lengths (squared):");
    println!("  q1 (opposite P1): {} (length: {})", q1, q1.sqrt());
    println!("  q2 (opposite P2): {} (length: {})", q2, q2.sqrt());
    println!("  q3 (opposite P3): {} (length: {})", q3, q3.sqrt());

    // Calculate spreads
    let (s1, s2, s3) = spread_from_three_points(p1, p2, p3);
    println!("\nAngle measures (squared sines):");
    println!(
        "  s1 (at P1): {} (angle: {:.4}°)",
        s1,
        s1.sqrt().asin().to_degrees()
    );
    println!(
        "  s2 (at P2): {} (angle: {:.4}°)",
        s2,
        s2.sqrt().asin().to_degrees()
    );
    println!(
        "  s3 (at P3): {} (angle: {:.4}°)",
        s3,
        s3.sqrt().asin().to_degrees()
    );

    // Calculate area
    let quadrea = archimedes(&q1, &q2, &q3);
    let area = quadrea.sqrt() / 4.0;
    println!("\nArea: {}", area);

    // Classify triangle type
    println!("\nTriangle classification:");
    if is_right_triangle(s1, s2, s3) {
        println!("  Type: Right triangle");
        if s1 == 1.0 {
            println!("  Right angle at: P1");
        } else if s2 == 1.0 {
            println!("  Right angle at: P2");
        } else {
            println!("  Right angle at: P3");
        }
    } else if is_acute_triangle(s1, s2, s3) {
        println!("  Type: Acute triangle");
    } else if is_obtuse_triangle(s1, s2, s3) {
        println!("  Type: Obtuse triangle");
        if s1 > 0.5 {
            println!("  Obtuse angle at: P1");
        } else if s2 > 0.5 {
            println!("  Obtuse angle at: P2");
        } else {
            println!("  Obtuse angle at: P3");
        }
    }

    // Check for special triangles
    let side1 = q1.sqrt();
    let side2 = q2.sqrt();
    let side3 = q3.sqrt();

    if (side1 - side2).abs() < 1e-10 && (side2 - side3).abs() < 1e-10 {
        println!("  Special: Equilateral triangle");
    } else if (side1 - side2).abs() < 1e-10
        || (side2 - side3).abs() < 1e-10
        || (side1 - side3).abs() < 1e-10
    {
        println!("  Special: Isosceles triangle");
    }

    // Verify Pythagorean theorem
    let sides = [q1, q2, q3];
    let max_q = sides.iter().cloned().fold(0.0 / 0.0, f64::max);
    let sum_other: f64 = sides.iter().filter(|&&x| x < max_q).sum();

    println!("\nPythagorean theorem check:");
    if (max_q - sum_other).abs() < 1e-10 {
        println!("  ✓ Satisfies a² + b² = c² (right triangle)");
    } else {
        println!("  Does not satisfy a² + b² = c²");
    }

    // Verify sine law
    let product1 = q1 * s1;
    let product2 = q2 * s2;
    let product3 = q3 * s3;
    println!("\nSine law check (q1*s1 = q2*s2 = q3*s3):");
    println!("  q1*s1 = {}", product1);
    println!("  q2*s2 = {}", product2);
    println!("  q3*s3 = {}", product3);
    if (product1 - product2).abs() < 1e-10 && (product2 - product3).abs() < 1e-10 {
        println!("  ✓ Sine law holds");
    }

    // Calculate twist (signed area)
    let twist = twist(p1, p2, p3);
    println!("\nTwist (2 * signed area): {}", twist);
    if twist > 0.0 {
        println!("  Orientation: Counter-clockwise");
    } else if twist < 0.0 {
        println!("  Orientation: Clockwise");
    } else {
        println!("  Orientation: Collinear");
    }

    println!("\n");
}

fn main() {
    // Example 1: Right triangle (3-4-5)
    println!("Example 1: Right triangle (3-4-5)\n");
    analyze_triangle((0.0, 0.0), (3.0, 0.0), (0.0, 4.0));

    // Example 2: Equilateral triangle
    println!("Example 2: Equilateral triangle\n");
    analyze_triangle((0.0, 0.0), (2.0, 0.0), (1.0, 1.7320508075688772));

    // Example 3: Isosceles triangle
    println!("Example 3: Isosceles triangle\n");
    analyze_triangle((0.0, 0.0), (2.0, 0.0), (1.0, 1.5));

    // Example 4: Obtuse triangle
    println!("Example 4: Obtuse triangle\n");
    analyze_triangle((0.0, 0.0), (1.0, 0.0), (0.1, 0.1));

    // Example 5: Acute triangle
    println!("Example 5: Acute triangle\n");
    analyze_triangle((0.0, 0.0), (2.0, 0.0), (1.0, 1.0));

    // Example 6: Degenerate triangle (collinear points)
    println!("Example 6: Degenerate triangle (collinear points)\n");
    analyze_triangle((0.0, 0.0), (1.0, 1.0), (2.0, 2.0));
}
