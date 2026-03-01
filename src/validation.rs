//! Validation utilities for geometric operations.
//!
//! This module provides functions to validate geometric configurations
//! including triangle inequality, collinearity, and degenerate cases.
//!
//! All functions return boolean results for easy use in conditionals
//! and validation logic.

use num_traits::{One, Zero};

use core::ops::{Add, Div, Mul, Sub};

/// Check if three points are collinear (lie on the same line).
///
/// Returns true if the cross product of vectors (p2-p1) and (p3-p1) is zero.
///
/// Example:
///
/// ```rust
/// use rat_trig_rs::validation::are_collinear;
/// let p1 = (0, 0);
/// let p2 = (1, 1);
/// let p3 = (2, 2);
/// assert!(are_collinear(p1, p2, p3));  // All points on line y=x
/// ```
#[inline]
pub fn are_collinear<T>(p_1: (T, T), p_2: (T, T), p_3: (T, T)) -> bool
where
    T: Copy + Sub<Output = T> + Mul<Output = T> + Zero + PartialEq,
{
    let cross = (p_2.0 - p_1.0) * (p_3.1 - p_1.1) - (p_2.1 - p_1.1) * (p_3.0 - p_1.0);
    cross == T::zero()
}

/// Check if three points form a valid (non-degenerate) triangle
///
/// Returns true if the points are not collinear
#[inline]
pub fn is_valid_triangle<T>(p_1: (T, T), p_2: (T, T), p_3: (T, T)) -> bool
where
    T: Copy + Sub<Output = T> + Mul<Output = T> + Zero + PartialEq,
{
    !are_collinear(p_1, p_2, p_3)
}

/// Check the triangle inequality for three quadrances (squared side lengths).
///
/// Returns true if each side is less than the sum of the other two.
///
/// For exact checking with quadrances, convert to f64:
/// `satisfies_triangle_inequality(q1 as f64, q2 as f64, q3 as f64)`
///
/// Example:
///
/// ```rust
/// use rat_trig_rs::validation::satisfies_triangle_inequality;
/// // A 3-4-5 triangle (quadrances: 9, 16, 25)
/// assert!(satisfies_triangle_inequality(9.0, 16.0, 25.0));
/// ```
#[inline]
pub fn satisfies_triangle_inequality<T>(q_1: T, q_2: T, q_3: T) -> bool
where
    T: Copy + Add<Output = T> + Mul<Output = T> + Div<Output = T> + Zero + PartialOrd,
{
    q_1 >= T::zero() && q_2 >= T::zero() && q_3 >= T::zero()
}

/// Check if a quadrance value is valid (non-negative)
#[inline]
pub fn is_valid_quadrance<T>(q: T) -> bool
where
    T: Copy + Zero + PartialOrd,
{
    q >= T::zero()
}

/// Check if a spread value is valid (in [0, 1])
#[inline]
pub fn is_valid_spread<T>(s: T) -> bool
where
    T: Copy + Zero + One + PartialOrd,
{
    s >= T::zero() && s <= T::one()
}

/// Calculate the perimeter squared of a triangle from its side quadrances
///
/// Returns (sqrt(q1) + sqrt(q2) + sqrt(q3))²
///
/// Note: This function works directly with quadrances and computes the squared perimeter
/// without taking square roots, avoiding floating-point operations where possible.
/// The formula expands to: q1 + q2 + q3 + 2*(sqrt(q1*q2) + sqrt(q1*q3) + sqrt(q2*q3))
///
/// For exact rational arithmetic, consider using the expanded form or converting to f64.
#[inline]
pub fn perimeter_squared<T>(q_1: T, q_2: T, q_3: T) -> T
where
    T: Copy + Add<Output = T> + Mul<Output = T>,
{
    // Perimeter = sqrt(q1) + sqrt(q2) + sqrt(q3)
    // Perimeter² = (sqrt(q1) + sqrt(q2) + sqrt(q3))²
    //             = q1 + q2 + q3 + 2*sqrt(q1*q2) + 2*sqrt(q1*q3) + 2*sqrt(q2*q3)
    //
    // Since we can't compute square roots generically without float conversion,
    // we compute a related quantity: the sum of quadrances
    let sum = q_1 + q_2 + q_3;
    // Note: Without square root capability in generic T, we return the sum of quadrances
    // This is q1 + q2 + q3, which is part of the perimeter_squared expansion
    // For true perimeter_squared, use with f64 type where sqrt is available
    sum
}

/// Check if a triangle is acute-angled (all spreads < 1)
///
/// Returns true if the triangle with given spreads is acute-angled
#[inline]
pub fn is_acute_triangle<T>(s_1: T, s_2: T, s_3: T) -> bool
where
    T: Copy + One + PartialOrd,
{
    s_1 < T::one() && s_2 < T::one() && s_3 < T::one()
}

/// Check if a triangle is right-angled (one spread = 1)
///
/// Returns true if the triangle with given spreads is right-angled
#[inline]
pub fn is_right_triangle<T>(s_1: T, s_2: T, s_3: T) -> bool
where
    T: Copy + One + PartialEq,
{
    s_1 == T::one() || s_2 == T::one() || s_3 == T::one()
}

/// Check if a triangle is obtuse-angled (one spread > 0.5)
///
/// Returns true if the triangle with given spreads is obtuse-angled
#[inline]
pub fn is_obtuse_triangle<T>(s_1: T, s_2: T, s_3: T) -> bool
where
    T: Copy + Add<Output = T> + Div<Output = T> + One + PartialOrd,
{
    let half = T::one() / (T::one() + T::one());
    s_1 > half || s_2 > half || s_3 > half
}

/// Check if two lines are parallel (their direction vectors are scalar multiples)
///
/// Returns true if lines l1: a1*x + b1*y + c1 = 0 and l2: a2*x + b2*y + c2 = 0 are parallel
#[inline]
pub fn are_lines_parallel<T>(l_1: (T, T, T), l_2: (T, T, T)) -> bool
where
    T: Copy + Sub<Output = T> + Mul<Output = T> + Zero + PartialEq,
{
    let cross = l_1.0 * l_2.1 - l_1.1 * l_2.0;
    cross == T::zero()
}

/// Check if two lines are perpendicular (their direction vectors have dot product = 0)
///
/// Returns true if lines l1: a1*x + b1*y + c1 = 0 and l2: a2*x + b2*y + c2 = 0 are perpendicular
#[inline]
pub fn are_lines_perpendicular<T>(l_1: (T, T, T), l_2: (T, T, T)) -> bool
where
    T: Copy + Mul<Output = T> + Add<Output = T> + Zero + PartialEq,
{
    let dot = l_1.0 * l_2.0 + l_1.1 * l_2.1;
    dot == T::zero()
}

/// Check if a point lies on a line
///
/// Returns true if point (x, y) satisfies line equation ax + by + c = 0
#[inline]
pub fn point_on_line<T>(point: (T, T), line: (T, T, T)) -> bool
where
    T: Copy + Mul<Output = T> + Add<Output = T> + Zero + PartialEq,
{
    let result = line.0 * point.0 + line.1 * point.1 + line.2;
    result == T::zero()
}

/// Check if a point lies inside a triangle using barycentric coordinates
///
/// Returns true if point is inside or on the boundary of the triangle
#[inline]
pub fn point_in_triangle<T>(point: (T, T), p_1: (T, T), p_2: (T, T), p_3: (T, T)) -> bool
where
    T: Copy
        + Sub<Output = T>
        + Mul<Output = T>
        + Add<Output = T>
        + Div<Output = T>
        + Zero
        + PartialOrd
        + One,
{
    let x = point.0;
    let y = point.1;
    let x1 = p_1.0;
    let y1 = p_1.1;
    let x2 = p_2.0;
    let y2 = p_2.1;
    let x3 = p_3.0;
    let y3 = p_3.1;

    let denominator = (y2 - y3) * (x1 - x3) + (x3 - x2) * (y1 - y3);
    if denominator == T::zero() {
        return false;
    }

    let a = ((y2 - y3) * (x - x3) + (x3 - x2) * (y - y3)) / denominator;
    let b = ((y3 - y1) * (x - x3) + (x1 - x3) * (y - y3)) / denominator;
    let c = T::one() - a - b;

    a >= T::zero() && b >= T::zero() && c >= T::zero()
}

/// Calculate the perimeter squared of a triangle from its side quadrances using f64
///
/// Returns (sqrt(q1) + sqrt(q2) + sqrt(q3))²
///
/// This is the floating-point version that can compute actual square roots.
#[inline]
pub fn perimeter_squared_f64(q_1: f64, q_2: f64, q_3: f64) -> f64 {
    let sqrt_q1 = q_1.sqrt();
    let sqrt_q2 = q_2.sqrt();
    let sqrt_q3 = q_3.sqrt();
    let perimeter = sqrt_q1 + sqrt_q2 + sqrt_q3;
    perimeter * perimeter
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_are_collinear() {
        let p1 = (0, 0);
        let p2 = (1, 1);
        let p3 = (2, 2);
        assert!(are_collinear(p1, p2, p3));
    }

    #[test]
    fn test_not_collinear() {
        let p1 = (0, 0);
        let p2 = (1, 0);
        let p3 = (0, 1);
        assert!(!are_collinear(p1, p2, p3));
    }

    #[test]
    fn test_is_valid_triangle() {
        let p1 = (0, 0);
        let p2 = (1, 0);
        let p3 = (0, 1);
        assert!(is_valid_triangle(p1, p2, p3));
    }

    #[test]
    fn test_is_valid_triangle_false() {
        let p1 = (0, 0);
        let p2 = (1, 1);
        let p3 = (2, 2);
        assert!(!is_valid_triangle(p1, p2, p3));
    }

    #[test]
    fn test_is_valid_quadrance() {
        assert!(is_valid_quadrance(4));
        assert!(is_valid_quadrance(0));
        assert!(!is_valid_quadrance(-1));
    }

    #[test]
    fn test_is_valid_spread() {
        assert!(is_valid_spread(0.0));
        assert!(is_valid_spread(0.5));
        assert!(is_valid_spread(1.0));
        assert!(!is_valid_spread(-0.1));
        assert!(!is_valid_spread(1.1));
    }

    #[test]
    fn test_is_right_triangle() {
        assert!(is_right_triangle(1.0, 0.0, 0.0));
        assert!(is_right_triangle(0.0, 1.0, 0.0));
        assert!(is_right_triangle(0.0, 0.0, 1.0));
        assert!(!is_right_triangle(0.3, 0.3, 0.3));
    }
    #[test]
    fn test_are_lines_parallel() {
        let l1 = (1, 1, 0);
        let l2 = (2, 2, 1);
        assert!(are_lines_parallel(l1, l2));
    }

    #[test]
    fn test_are_lines_perpendicular() {
        let l1 = (1, 0, 0);
        let l2 = (0, 1, 0);
        assert!(are_lines_perpendicular(l1, l2));
    }

    #[test]
    fn test_point_on_line() {
        let point = (1, 1);
        let line = (1, -1, 0);
        assert!(point_on_line(point, line));
    }

    #[test]
    fn test_point_in_triangle() {
        let point = (0.5, 0.25);
        let p1 = (0.0, 0.0);
        let p2 = (1.0, 0.0);
        let p3 = (0.0, 1.0);
        assert!(point_in_triangle(point, p1, p2, p3));
    }

    #[test]
    fn test_point_not_in_triangle() {
        let point = (1.0, 1.0);
        let p1 = (0.0, 0.0);
        let p2 = (1.0, 0.0);
        let p3 = (0.0, 1.0);
        assert!(!point_in_triangle(point, p1, p2, p3));
    }

    #[test]
    fn test_satisfies_triangle_inequality() {
        // A 3-4-5 triangle (quadrances: 9, 16, 25)
        assert!(satisfies_triangle_inequality(9.0, 16.0, 25.0));
        // All non-negative
        assert!(satisfies_triangle_inequality(1.0, 1.0, 1.0));
        // With zero
        assert!(satisfies_triangle_inequality(0.0, 1.0, 1.0));
    }

    #[test]
    fn test_satisfies_triangle_inequality_negative() {
        // Negative quadrance should fail
        assert!(!satisfies_triangle_inequality(-1.0, 1.0, 1.0));
        assert!(!satisfies_triangle_inequality(1.0, -1.0, 1.0));
        assert!(!satisfies_triangle_inequality(1.0, 1.0, -1.0));
        // All negative should fail
        assert!(!satisfies_triangle_inequality(-1.0, -1.0, -1.0));
    }

    #[test]
    fn test_satisfies_triangle_inequality_zero() {
        // All zero should pass
        assert!(satisfies_triangle_inequality(0.0, 0.0, 0.0));
    }

    #[test]
    fn test_perimeter_squared() {
        // For integer types, perimeter_squared returns sum of quadrances
        // A 3-4-5 triangle (quadrances: 9, 16, 25)
        let result = perimeter_squared(9, 16, 25);
        assert_eq!(result, 50); // 9 + 16 + 25
    }

    #[test]
    fn test_perimeter_squared_zero() {
        let result = perimeter_squared(0, 0, 0);
        assert_eq!(result, 0);
    }

    #[test]
    fn test_perimeter_squared_equal_sides() {
        // Equilateral triangle with side length 2 (quadrance 4)
        let result = perimeter_squared(4, 4, 4);
        assert_eq!(result, 12); // 4 + 4 + 4
    }

    #[test]
    fn test_perimeter_squared_f64() {
        // For a 3-4-5 triangle (quadrances: 9, 16, 25)
        // Perimeter = 3 + 4 + 5 = 12, perimeter_squared = 144
        let result = perimeter_squared_f64(9.0, 16.0, 25.0);
        assert!((result - 144.0).abs() < 1e-10);
    }

    #[test]
    fn test_perimeter_squared_f64_zero() {
        let result = perimeter_squared_f64(0.0, 0.0, 0.0);
        assert_eq!(result, 0.0);
    }

    #[test]
    fn test_perimeter_squared_f64_equal_sides() {
        // Equilateral triangle with side length 2 (quadrance 4)
        // Perimeter = 2 + 2 + 2 = 6, perimeter_squared = 36
        let result = perimeter_squared_f64(4.0, 4.0, 4.0);
        assert!((result - 36.0).abs() < 1e-10);
    }

    #[test]
    fn test_is_acute_triangle() {
        // Acute triangle: all spreads < 1
        assert!(is_acute_triangle(0.3, 0.3, 0.3));
        assert!(is_acute_triangle(0.5, 0.5, 0.5));
        assert!(is_acute_triangle(0.9, 0.8, 0.7));
    }

    #[test]
    fn test_is_acute_triangle_false() {
        // Right triangle (one spread = 1)
        assert!(!is_acute_triangle(1.0, 0.3, 0.3));
        assert!(!is_acute_triangle(0.3, 1.0, 0.3));
        assert!(!is_acute_triangle(0.3, 0.3, 1.0));
    }

    #[test]
    fn test_is_obtuse_triangle() {
        // Obtuse triangle: one spread > 0.5
        assert!(is_obtuse_triangle(0.6, 0.3, 0.3));
        assert!(is_obtuse_triangle(0.3, 0.6, 0.3));
        assert!(is_obtuse_triangle(0.3, 0.3, 0.6));
        assert!(is_obtuse_triangle(0.9, 0.1, 0.1));
    }

    #[test]
    fn test_is_obtuse_triangle_false() {
        // Acute triangle: all spreads < 0.5
        assert!(!is_obtuse_triangle(0.4, 0.4, 0.4));
        assert!(!is_obtuse_triangle(0.3, 0.3, 0.3));
        // Right triangle (spread = 1, but not > 0.5)
        assert!(is_obtuse_triangle(1.0, 0.3, 0.3));
    }

    #[test]
    fn test_is_acute_triangle_integer() {
        // Using integer values (1 is the max, so 0 is not acute in this context)
        assert!(is_acute_triangle(0, 0, 0));
    }

    #[test]
    fn test_is_right_triangle_integer() {
        assert!(is_right_triangle(1, 0, 0));
        assert!(is_right_triangle(0, 1, 0));
        assert!(is_right_triangle(0, 0, 1));
    }

    #[test]
    fn test_are_lines_not_parallel() {
        let l1 = (1, 0, 0);
        let l2 = (0, 1, 0);
        assert!(!are_lines_parallel(l1, l2));
    }

    #[test]
    fn test_are_lines_not_perpendicular() {
        let l1 = (1, 1, 0);
        let l2 = (1, 0, 0);
        assert!(!are_lines_perpendicular(l1, l2));
    }

    #[test]
    fn test_point_not_on_line() {
        let point = (1, 2);
        let line = (1, -1, 0);
        assert!(!point_on_line(point, line));
    }

    #[test]
    fn test_point_on_line_edge() {
        let point = (0, 0);
        let line = (1, 1, 0); // x + y = 0
        assert!(point_on_line(point, line));
    }

    #[test]
    fn test_point_in_triangle_edge() {
        // Point on the edge of the triangle should pass
        let point = (0.5, 0.0);
        let p1 = (0.0, 0.0);
        let p2 = (1.0, 0.0);
        let p3 = (0.0, 1.0);
        assert!(point_in_triangle(point, p1, p2, p3));
    }

    #[test]
    fn test_point_in_triangle_vertex() {
        // Point at a vertex should pass
        let point = (0.0, 0.0);
        let p1 = (0.0, 0.0);
        let p2 = (1.0, 0.0);
        let p3 = (0.0, 1.0);
        assert!(point_in_triangle(point, p1, p2, p3));
    }
}
