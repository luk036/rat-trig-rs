//! Integration tests for real-world scenarios
//!
//! These tests demonstrate practical applications of rational trigonometry

use fractions::Fraction;
use rat_trig_rs::geometry::*;
use rat_trig_rs::trigonom::*;
use rat_trig_rs::validation::*;

#[test]
fn test_triangle_properties_from_coordinates() {
    // Given three points forming a right triangle
    let p1 = (0, 0);
    let p2 = (3, 0);
    let p3 = (0, 4);

    // Calculate quadrances (squared side lengths)
    let (q1, q2, q3) = quadrance_from_three_points(p1, p2, p3);

    // Verify: q1 = 25 (side opposite p1, distance between p2 and p3)
    // q2 = 16 (side opposite p2, distance between p1 and p3)
    // q3 = 9 (side opposite p3, distance between p1 and p2)
    assert_eq!(q1, 25);
    assert_eq!(q2, 16);
    assert_eq!(q3, 9);

    // Calculate spreads (squared sines of angles)
    let (s1, s2, s3) = spread_from_three_points(p1, p2, p3);

    // In a right triangle, one spread should be 1 (right angle)
    assert!(s1 == 1 || s2 == 1 || s3 == 1);

    // Verify Pythagorean theorem in rational form: q2 + q3 = q1
    assert_eq!(q2 + q3, q1);
}

#[test]
fn test_triangle_area_calculation() {
    // Test Archimedes formula for area calculation
    let p1 = (0, 0);
    let p2 = (3, 0);
    let p3 = (0, 4);

    let (q1, q2, q3) = quadrance_from_three_points(p1, p2, p3);
    let quadrea = archimedes(&q1, &q2, &q3);

    // Quadrea = 16 * area^2
    // For a 3-4-5 triangle, area = 6, so quadrea = 16 * 36 = 576
    assert_eq!(quadrea, 576);
}

#[test]
fn test_triangle_type_classification() {
    // Right triangle
    let p1 = (0_i32, 0_i32);
    let p2 = (3_i32, 0_i32);
    let p3 = (0_i32, 4_i32);
    let (s1, s2, s3) = spread_from_three_points(p1, p2, p3);
    assert!(is_right_triangle(s1, s2, s3));
}

#[test]
fn test_point_in_triangle_scenario() {
    // Test if a point is inside a triangle
    let p1 = (0.0, 0.0);
    let p2 = (2.0, 0.0);
    let p3 = (0.0, 2.0);

    // Point inside
    let inside_point = (0.5, 0.5);
    assert!(point_in_triangle(inside_point, p1, p2, p3));

    // Point outside
    let outside_point = (2.0, 2.0);
    assert!(!point_in_triangle(outside_point, p1, p2, p3));

    // Point on edge
    let edge_point = (1.0, 0.0);
    assert!(point_in_triangle(edge_point, p1, p2, p3));
}

#[test]
fn test_rational_triangle_properties() {
    // Test with rational numbers for exact calculations
    let p1 = (Fraction::<i32>::new(0, 1), Fraction::<i32>::new(0, 1));
    let p2 = (Fraction::<i32>::new(3, 1), Fraction::<i32>::new(0, 1));
    let p3 = (Fraction::<i32>::new(0, 1), Fraction::<i32>::new(4, 1));

    let (q1, q2, q3) = quadrance_from_three_points(p1, p2, p3);

    assert_eq!(q1, Fraction::<i32>::new(25, 1));
    assert_eq!(q2, Fraction::<i32>::new(16, 1));
    assert_eq!(q3, Fraction::<i32>::new(9, 1));

    let quadrea = archimedes(&q1, &q2, &q3);
    assert_eq!(quadrea, Fraction::<i32>::new(576, 1));
}

#[test]
fn test_3d_tetrahedron_face() {
    // Test 3D operations on a face of a tetrahedron
    let p1 = (0, 0, 0);
    let p2 = (1, 0, 0);
    let p3 = (0, 1, 0);

    // Calculate quadrances in 3D
    let q1 = quadrance3d(p2, p3);
    let q2 = quadrance3d(p1, p3);
    let q3 = quadrance3d(p1, p2);

    assert_eq!(q1, 2);
    assert_eq!(q2, 1);
    assert_eq!(q3, 1);

    // Calculate cross product
    let v1 = (p2.0 - p1.0, p2.1 - p1.1, p2.2 - p1.2);
    let v2 = (p3.0 - p1.0, p3.1 - p1.1, p3.2 - p1.2);
    let cross = cross3d(v1, v2);

    // Cross product should be (0, 0, 1)
    assert_eq!(cross, (0, 0, 1));
}

#[test]
fn test_geometry_primitives_triangle() {
    // Test using geometry primitives
    let p1 = Point2D::new(0, 0);
    let p2 = Point2D::new(3, 0);
    let p3 = Point2D::new(0, 4);

    let triangle = Triangle2D::new(p1, p2, p3);

    // Test quadrances
    let (q1, q2, q3) = triangle.quadrances();
    assert_eq!(q1, 25);
    assert_eq!(q2, 16);
    assert_eq!(q3, 9);

    // Test area
    let area = triangle.area();
    assert_eq!(area, 576);

    // Test twist
    let twist = triangle.twist();
    assert_eq!(twist, 12);

    // Test degenerate check
    assert!(!triangle.is_degenerate());
}

#[test]
fn test_degenerate_triangle_detection() {
    // Collinear points should form a degenerate triangle
    let p1 = (0_i32, 0_i32);
    let p2 = (1_i32, 1_i32);
    let p3 = (2_i32, 2_i32);

    assert!(are_collinear(p1, p2, p3));
    assert!(!is_valid_triangle(p1, p2, p3));

    let triangle = Triangle2D::new(p1.into(), p2.into(), p3.into());
    assert!(triangle.is_degenerate());
}

#[test]
fn test_vector_operations() {
    // Test vector arithmetic
    let v1 = Vector2D::new(1, 2);
    let v2 = Vector2D::new(3, 4);

    let sum = v1 + v2;
    assert_eq!(sum.x, 4);
    assert_eq!(sum.y, 6);

    let diff = v2 - v1;
    assert_eq!(diff.x, 2);
    assert_eq!(diff.y, 2);
}

#[test]
fn test_turn_and_orientation() {
    // Test turn function for orientation detection
    let p1 = (0.0, 0.0);
    let p2 = (1.0, 0.0);
    let p3 = (1.0, 1.0);

    let (s, sign) = turn(p1, p2, p3);

    // Spread should be positive and in valid range
    assert!((0.0..=1.0).contains(&s));
    // Sign should be true for counter-clockwise turn
    assert!(sign);

    // Test clockwise turn
    let p3 = (1.0, -1.0);
    let (_s, sign) = turn(p1, p2, p3);
    assert!(!sign);
}
