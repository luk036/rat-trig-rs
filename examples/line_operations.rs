//! Line operations example
//!
//! This example demonstrates line-related operations in rational trigonometry

use rat_trig_rs::trigonom::*;
use rat_trig_rs::validation::*;

fn main() {
    println!("=== Line Operations in Rational Trigonometry ===\n");

    // Example 1: Calculate distance from point to line
    println!("Example 1: Distance from point to line");
    let point = (1.0, 1.0);
    let line = (1.0, 1.0, 0.0); // Line: x + y = 0
    let q: f64 = quadrance_from_line(point, line);
    println!("  Point: ({}, {})", point.0, point.1);
    println!("  Line: {}x + {}y + {} = 0", line.0, line.1, line.2);
    println!("  Distance squared: {}", q);
    println!("  Distance: {}\n", q.sqrt());

    // Example 2: Calculate angle between two lines
    println!("Example 2: Angle between two lines");
    let line1 = (1.0, 0.0, 0.0); // Line: x = 0 (vertical)
    let line2 = (0.0, 1.0, 0.0); // Line: y = 0 (horizontal)
    let s: f64 = spread_from_line(line1, line2);
    println!(
        "  Line 1: {}x + {}y + {} = 0 (vertical)",
        line1.0, line1.1, line1.2
    );
    println!(
        "  Line 2: {}x + {}y + {} = 0 (horizontal)",
        line2.0, line2.1, line2.2
    );
    println!("  Spread (squared sine): {}", s);
    println!("  Angle: {:.4}°\n", s.sqrt().asin().to_degrees());

    // Example 3: Check if lines are parallel
    println!("Example 3: Checking parallel lines");
    let line1 = (1.0, 1.0, 0.0);
    let line2 = (2.0, 2.0, 1.0);
    let line3 = (1.0, 0.0, 0.0);
    println!("  Line 1: {}x + {}y + {} = 0", line1.0, line1.1, line1.2);
    println!("  Line 2: {}x + {}y + {} = 0", line2.0, line2.1, line2.2);
    println!("  Line 3: {}x + {}y + {} = 0", line3.0, line3.1, line3.2);
    println!("  Line 1 �?Line 2: {}", are_lines_parallel(line1, line2));
    println!("  Line 1 �?Line 3: {}\n", are_lines_parallel(line1, line3));

    // Example 4: Check if lines are perpendicular
    println!("Example 4: Checking perpendicular lines");
    println!("  Line 1: {}x + {}y + {} = 0", line1.0, line1.1, line1.2);
    println!("  Line 2: {}x + {}y + {} = 0", line2.0, line2.1, line2.2);
    println!("  Line 3: {}x + {}y + {} = 0", line3.0, line3.1, line3.2);
    println!(
        "  Line 1 �?Line 2: {}",
        are_lines_perpendicular(line1, line2)
    );
    println!(
        "  Line 1 �?Line 3: {}\n",
        are_lines_perpendicular(line1, line3)
    );

    // Example 5: Check if point lies on line
    println!("Example 5: Point on line check");
    let line = (1.0, -1.0, 0.0); // Line: x - y = 0
    let point1 = (1.0, 1.0);
    let point2 = (1.0, 2.0);
    println!("  Line: {}x + {}y + {} = 0", line.0, line.1, line.2);
    println!("  Point 1: ({}, {})", point1.0, point1.1);
    println!("  Point 2: ({}, {})", point2.0, point2.1);
    println!("  Point 1 on line: {}", point_on_line(point1, line));
    println!("  Point 2 on line: {}\n", point_on_line(point2, line));

    // Example 6: Cross product of line direction vectors
    println!("Example 6: Cross product of line direction vectors");
    let line1 = (1.0, 1.0, 0.0);
    let line2 = (1.0, 0.0, 0.0);
    let c = cross_from_line(line1, line2);
    println!("  Line 1: {}x + {}y + {} = 0", line1.0, line1.1, line1.2);
    println!("  Line 2: {}x + {}y + {} = 0", line2.0, line2.1, line2.2);
    println!("  Cross product of direction vectors: {}\n", c);

    // Example 7: Multiple distance calculations
    println!("Example 7: Distance from multiple points to a line");
    let line = (1.0, 1.0, -5.0); // Line: x + y = 5
    let points = [(0.0, 0.0), (1.0, 1.0), (2.0, 3.0), (5.0, 0.0)];
    println!("  Line: {}x + {}y + {} = 0", line.0, line.1, line.2);
    println!("  Distances:");
    for point in points.iter() {
        let q: f64 = quadrance_from_line(*point, line);
        println!("    ({}, {}): {}", point.0, point.1, q.sqrt());
    }
    println!();

    // Example 8: Angle between lines at different orientations
    println!("Example 8: Angles between lines at different orientations");
    let base_line = (1.0, 0.0, 0.0); // Horizontal line
    let angles: [f64; 5] = [0.0, 30.0, 45.0, 60.0, 90.0];
    println!(
        "  Base line: {}x + {}y + {} = 0 (horizontal)",
        base_line.0, base_line.1, base_line.2
    );
    println!("  Angles with lines at different orientations:");
    for &angle in angles.iter() {
        let rad = angle.to_radians();
        let line = (-rad.sin(), rad.cos(), 0.0);
        let s: f64 = spread_from_line(base_line, line);
        println!(
            "    {:.1}°: spread = {:.6}, angle = {:.4}°",
            angle,
            s,
            s.sqrt().asin().to_degrees()
        );
    }
    println!();

    // Example 9: Using line operations in triangle context
    println!("Example 9: Line operations in triangle context");
    let p1 = (0.0, 0.0);
    let p2 = (3.0, 0.0);
    let p3 = (0.0, 4.0);

    // Create lines from triangle sides
    let line12 = line_from_points(p1, p2);
    let line23 = line_from_points(p2, p3);
    let line31 = line_from_points(p3, p1);

    println!("  Triangle vertices:");
    println!("    P1: ({}, {})", p1.0, p1.1);
    println!("    P2: ({}, {})", p2.0, p2.1);
    println!("    P3: ({}, {})", p3.0, p3.1);

    println!("  Lines from sides:");
    println!(
        "    P1-P2: {}x + {}y + {} = 0",
        line12.0, line12.1, line12.2
    );
    println!(
        "    P2-P3: {}x + {}y + {} = 0",
        line23.0, line23.1, line23.2
    );
    println!(
        "    P3-P1: {}x + {}y + {} = 0",
        line31.0, line31.1, line31.2
    );

    // Check if adjacent sides are perpendicular
    let s12_23 = spread_from_line(line12, line23);
    let s23_31 = spread_from_line(line23, line31);
    let s31_12 = spread_from_line(line31, line12);

    println!("  Spreads between adjacent sides:");
    println!(
        "    P1-P2 and P2-P3: {} ({:.4}°)",
        s12_23,
        s12_23.sqrt().asin().to_degrees()
    );
    println!(
        "    P2-P3 and P3-P1: {} ({:.4}°)",
        s23_31,
        s23_31.sqrt().asin().to_degrees()
    );
    println!(
        "    P3-P1 and P1-P2: {} ({:.4}°)",
        s31_12,
        s31_12.sqrt().asin().to_degrees()
    );
}

/// Helper function to create line equation from two points
fn line_from_points(p1: (f64, f64), p2: (f64, f64)) -> (f64, f64, f64) {
    let a = p1.1 - p2.1;
    let b = p2.0 - p1.0;
    let c = p1.0 * p2.1 - p2.0 * p1.1;
    (a, b, c)
}
