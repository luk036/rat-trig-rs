/// Rational Trigonometry is a new approach to classical trigonometry, developed by Norman
/// Wildberger, that aims to simplify and clarify the subject by using only rational numbers
/// and operations, rather than irrational numbers and limits.
///
/// In traditional trigonometry, concepts such as the sine, cosine, and tangent of an angle
/// are typically defined using circles and the unit circle in particular. These definitions
/// involve irrational numbers and limits, which can make the subject more difficult to
/// understand and work with.
///
/// In rational trigonometry, Wildberger replaces these circular definitions with ones based
/// on lines and line segments, which allows for a more straightforward and intuitive approach.
/// The fundamental concepts in rational trigonometry are the "quadaverage" and the "dilated
/// directed angle," which are defined in terms of lines and line segments, rather than circles.
///
/// Rational trigonometry has been gaining popularity in recent years, as it provides a useful
/// alternative to traditional trigonometry for certain applications, such as computer graphics,
/// robotics, and physics. It can also be a helpful tool for students who struggle with the
/// irrational numbers and limits used in traditional trigonometry.
///
/// In summary, Rational Trigonometry is a new approach to classical trigonometry that uses
/// rational numbers and operations, rather than irrational numbers and limits, making it a more
/// straightforward and intuitive subject to understand and work with.
use num_traits::{One, Zero};

use core::ops::{Add, Div, Mul, Sub};

/// The function `archimedes` calculates the area of a triangle using Archimedes' formula with the
/// lengths of the three sides provided as `Fraction<i64>` values.
///
/// Arguments:
///
/// * `q_1`: Represents the length of the first side of the triangle.
/// * `q_2`: The parameters `q_1`, `q_2`, and `q_3` represent the lengths of the sides of a triangle. In
///   the context of Archimedes' formula for the area of a triangle, `q_1`, `q_2`, and `q_3`
/// * `q_3`: The parameter `q_3` represents the length of the third side of the triangle.
///
/// Returns:
///
/// The function `archimedes` returns the area of a triangle computed using Archimedes' formula, given
/// the lengths of the 3 sides.
///
/// Example:
///
/// ```rust
/// use num_rational::Rational32;
/// use rat_trig_rs::trigonom::archimedes;
/// let q_1 = Rational32::new(1, 2);
/// let q_2 = Rational32::new(1, 4);
/// let q_3 = Rational32::new(1, 6);
/// let quadrea = archimedes(&q_1, &q_2, &q_3);
/// assert_eq!(quadrea, Rational32::new(23, 144));
/// ```
#[inline]
pub fn archimedes<T>(q_1: &T, q_2: &T, q_3: &T) -> T
where
    T: Copy + Add<Output = T> + Sub<Output = T> + Mul<Output = T> + One + Zero,
{
    let temp = *q_1 + *q_2 - *q_3;
    let four = T::one() + T::one() + T::one() + T::one();
    four * *q_1 * *q_2 - temp * temp
}

#[inline]
pub fn quadrance<T>(p_1: (T, T), p_2: (T, T)) -> T
where
    T: Copy + Add<Output = T> + Sub<Output = T> + Mul<Output = T>,
{
    let dx = p_1.0 - p_2.0;
    let dy = p_1.1 - p_2.1;
    dx * dx + dy * dy
}

#[inline]
pub fn spread<T>(v_1: (T, T), v_2: (T, T)) -> T
where
    T: Copy + Add<Output = T> + Sub<Output = T> + Mul<Output = T> + Div<Output = T> + One + Zero,
{
    let dot_product = v_1.0 * v_2.0 + v_1.1 * v_2.1;
    let q_1 = quadrance(v_1, (T::zero(), T::zero()));
    let q_2 = quadrance(v_2, (T::zero(), T::zero()));
    T::one() - dot_product * dot_product / (q_1 * q_2)
}

/// Calculate spread (square of sine) between two vectors with error checking
///
/// Returns `MathError::DivisionByZero` if either vector has zero magnitude.
#[inline]
pub fn safe_spread<T>(v_1: (T, T), v_2: (T, T)) -> Result<T, crate::error::MathError>
where
    T: Copy
        + Add<Output = T>
        + Sub<Output = T>
        + Mul<Output = T>
        + Div<Output = T>
        + One
        + Zero
        + PartialEq,
{
    let dot_product = v_1.0 * v_2.0 + v_1.1 * v_2.1;
    let q_1 = quadrance(v_1, (T::zero(), T::zero()));
    let q_2 = quadrance(v_2, (T::zero(), T::zero()));

    if q_1 == T::zero() || q_2 == T::zero() {
        return Err(crate::error::MathError::DivisionByZero);
    }

    Ok(T::one() - dot_product * dot_product / (q_1 * q_2))
}

#[inline]
pub fn cross<T>(v_1: (T, T), v_2: (T, T)) -> T
where
    T: Copy + Sub<Output = T> + Mul<Output = T>,
{
    v_1.0 * v_2.1 - v_1.1 * v_2.0
}

#[inline]
pub fn quadrance_from_line<T>(p: (T, T), l: (T, T, T)) -> T
where
    T: Copy + Add<Output = T> + Sub<Output = T> + Mul<Output = T> + Div<Output = T> + Zero,
{
    let temp = l.0 * p.0 + l.1 * p.1 + l.2;
    temp * temp / quadrance((l.0, l.1), (T::zero(), T::zero()))
}

/// Calculate quadrance from line with error checking
///
/// Returns `MathError::DivisionByZero` if the line has zero magnitude.
#[inline]
pub fn safe_quadrance_from_line<T>(p: (T, T), l: (T, T, T)) -> Result<T, crate::error::MathError>
where
    T: Copy
        + Add<Output = T>
        + Sub<Output = T>
        + Mul<Output = T>
        + Div<Output = T>
        + Zero
        + PartialEq,
{
    let temp = l.0 * p.0 + l.1 * p.1 + l.2;
    let q = quadrance((l.0, l.1), (T::zero(), T::zero()));

    if q == T::zero() {
        return Err(crate::error::MathError::DivisionByZero);
    }

    Ok(temp * temp / q)
}

#[inline]
pub fn spread_from_line<T>(l_1: (T, T, T), l_2: (T, T, T)) -> T
where
    T: Copy + Add<Output = T> + Sub<Output = T> + Mul<Output = T> + Div<Output = T> + Zero,
{
    let temp = cross((l_1.0, l_1.1), (l_2.0, l_2.1));
    temp * temp
        / (quadrance((l_1.0, l_1.1), (T::zero(), T::zero()))
            * quadrance((l_2.0, l_2.1), (T::zero(), T::zero())))
}

/// Calculate spread from line with error checking
///
/// Returns `MathError::DivisionByZero` if either line has zero magnitude.
#[inline]
pub fn safe_spread_from_line<T>(
    l_1: (T, T, T),
    l_2: (T, T, T),
) -> Result<T, crate::error::MathError>
where
    T: Copy
        + Add<Output = T>
        + Sub<Output = T>
        + Mul<Output = T>
        + Div<Output = T>
        + Zero
        + PartialEq,
{
    let temp = cross((l_1.0, l_1.1), (l_2.0, l_2.1));
    let q_1 = quadrance((l_1.0, l_1.1), (T::zero(), T::zero()));
    let q_2 = quadrance((l_2.0, l_2.1), (T::zero(), T::zero()));

    if q_1 == T::zero() || q_2 == T::zero() {
        return Err(crate::error::MathError::DivisionByZero);
    }

    Ok(temp * temp / (q_1 * q_2))
}

#[inline]
pub fn cross_from_line<T>(l_1: (T, T, T), l_2: (T, T, T)) -> T
where
    T: Copy + Sub<Output = T> + Mul<Output = T>,
{
    cross((l_1.0, l_1.1), (l_2.0, l_2.1))
}

#[inline]
pub fn quadrance_from_three_points<T>(p_1: (T, T), p_2: (T, T), p_3: (T, T)) -> (T, T, T)
where
    T: Copy + Add<Output = T> + Sub<Output = T> + Mul<Output = T>,
{
    (
        quadrance(p_2, p_3),
        quadrance(p_1, p_3),
        quadrance(p_1, p_2),
    )
}

#[inline]
pub fn spread_from_three_points<T>(p_1: (T, T), p_2: (T, T), p_3: (T, T)) -> (T, T, T)
where
    T: Copy + Add<Output = T> + Sub<Output = T> + Mul<Output = T> + Div<Output = T> + One + Zero,
{
    let q_1 = quadrance(p_2, p_3);
    let q_2 = quadrance(p_1, p_3);
    let q_3 = quadrance(p_1, p_2);
    let four = T::one() + T::one() + T::one() + T::one();
    let s_1 = T::one() - (q_2 + q_3 - q_1) * (q_2 + q_3 - q_1) / (four * q_2 * q_3);
    let s_2 = T::one() - (q_1 + q_3 - q_2) * (q_1 + q_3 - q_2) / (four * q_1 * q_3);
    let s_3 = T::one() - (q_1 + q_2 - q_3) * (q_1 + q_2 - q_3) / (four * q_1 * q_2);
    (s_1, s_2, s_3)
}

#[inline]
pub fn cross_from_three_points<T>(p_1: (T, T), p_2: (T, T), p_3: (T, T)) -> T
where
    T: Copy + Sub<Output = T> + Mul<Output = T>,
{
    cross(
        (p_2.0 - p_1.0, p_2.1 - p_1.1),
        (p_3.0 - p_1.0, p_3.1 - p_1.1),
    )
}

/// Calculate quadrance (square of distance) between two 3D points
#[inline]
pub fn quadrance3d<T>(p_1: (T, T, T), p_2: (T, T, T)) -> T
where
    T: Copy + Add<Output = T> + Sub<Output = T> + Mul<Output = T>,
{
    let dx = p_1.0 - p_2.0;
    let dy = p_1.1 - p_2.1;
    let dz = p_1.2 - p_2.2;
    dx * dx + dy * dy + dz * dz
}

/// Calculate cross product of two 3D vectors
#[inline]
pub fn cross3d<T>(v_1: (T, T, T), v_2: (T, T, T)) -> (T, T, T)
where
    T: Copy + Sub<Output = T> + Mul<Output = T> + Add<Output = T>,
{
    (
        v_1.1 * v_2.2 - v_1.2 * v_2.1,
        v_1.2 * v_2.0 - v_1.0 * v_2.2,
        v_1.0 * v_2.1 - v_1.1 * v_2.0,
    )
}

/// Calculate spread (square of sine) between two 3D vectors
#[inline]
pub fn spread3d<T>(v_1: (T, T, T), v_2: (T, T, T)) -> T
where
    T: Copy + Add<Output = T> + Sub<Output = T> + Mul<Output = T> + Div<Output = T> + One + Zero,
{
    let dot_product = v_1.0 * v_2.0 + v_1.1 * v_2.1 + v_1.2 * v_2.2;
    let q_1 = quadrance3d(v_1, (T::zero(), T::zero(), T::zero()));
    let q_2 = quadrance3d(v_2, (T::zero(), T::zero(), T::zero()));
    T::one() - dot_product * dot_product / (q_1 * q_2)
}

/// Calculate twist (signed area) of triangle formed by three points
/// Twist is twice the signed area of the triangle
#[inline]
pub fn twist<T>(p_1: (T, T), p_2: (T, T), p_3: (T, T)) -> T
where
    T: Copy + Sub<Output = T> + Mul<Output = T>,
{
    cross_from_three_points(p_1, p_2, p_3)
}

/// Calculate turn (oriented angle measure) between three points
/// Returns the spread and its sign based on orientation
#[inline]
pub fn turn<T>(p_1: (T, T), p_2: (T, T), p_3: (T, T)) -> (T, bool)
where
    T: Copy
        + Add<Output = T>
        + Sub<Output = T>
        + Mul<Output = T>
        + Div<Output = T>
        + One
        + Zero
        + PartialOrd,
{
    let v1 = (p_2.0 - p_1.0, p_2.1 - p_1.1);
    let v2 = (p_3.0 - p_2.0, p_3.1 - p_2.1);
    let s = spread(v1, v2);
    let sign = cross(v1, v2) >= T::zero();
    (s, sign)
}

/// Calculate dilatation between two vectors
/// Dilatation is the ratio of lengths squared
#[inline]
pub fn dilatation<T>(v_1: (T, T), v_2: (T, T)) -> T
where
    T: Copy
        + Add<Output = T>
        + Sub<Output = T>
        + Mul<Output = T>
        + Div<Output = T>
        + Zero
        + PartialEq,
{
    let q_1 = quadrance(v_1, (T::zero(), T::zero()));
    let q_2 = quadrance(v_2, (T::zero(), T::zero()));
    if q_1 == T::zero() {
        T::zero()
    } else {
        q_2 / q_1
    }
}

/// Calculate dilatation between two vectors with error checking
///
/// Returns `MathError::DivisionByZero` if the first vector has zero magnitude.
#[inline]
pub fn safe_dilatation<T>(v_1: (T, T), v_2: (T, T)) -> Result<T, crate::error::MathError>
where
    T: Copy
        + Add<Output = T>
        + Sub<Output = T>
        + Mul<Output = T>
        + Div<Output = T>
        + Zero
        + PartialEq,
{
    let q_1 = quadrance(v_1, (T::zero(), T::zero()));
    let q_2 = quadrance(v_2, (T::zero(), T::zero()));

    if q_1 == T::zero() {
        return Err(crate::error::MathError::DivisionByZero);
    }

    Ok(q_2 / q_1)
}

/// Calculate the sine law equivalent in rational trigonometry
/// For a triangle with sides q1, q2, q3 and corresponding spreads s1, s2, s3
/// This verifies: q1 * s1 = q2 * s2 = q3 * s3
#[inline]
pub fn sine_law_product<T>(q: T, s: T) -> T
where
    T: Copy + Mul<Output = T>,
{
    q * s
}

/// Calculate the cosine law equivalent in rational trigonometry
/// For a triangle with quadrances q1, q2, q3 and spread s1 opposite q1
/// s1 = 1 - (q2 + q3 - q1)² / (4 * q2 * q3)
#[inline]
pub fn cosine_law<T>(q_1: T, q_2: T, q_3: T) -> T
where
    T: Copy
        + Add<Output = T>
        + Sub<Output = T>
        + Mul<Output = T>
        + Div<Output = T>
        + One
        + Zero
        + PartialEq,
{
    let four = T::one() + T::one() + T::one() + T::one();
    if q_2 == T::zero() || q_3 == T::zero() {
        T::zero()
    } else {
        T::one() - (q_2 + q_3 - q_1) * (q_2 + q_3 - q_1) / (four * q_2 * q_3)
    }
}

/// Calculate the cosine law equivalent in rational trigonometry with error checking
///
/// Returns `MathError::DivisionByZero` if either q2 or q3 is zero.
#[inline]
pub fn safe_cosine_law<T>(q_1: T, q_2: T, q_3: T) -> Result<T, crate::error::MathError>
where
    T: Copy
        + Add<Output = T>
        + Sub<Output = T>
        + Mul<Output = T>
        + Div<Output = T>
        + One
        + Zero
        + PartialEq,
{
    let four = T::one() + T::one() + T::one() + T::one();

    if q_2 == T::zero() || q_3 == T::zero() {
        return Err(crate::error::MathError::DivisionByZero);
    }

    Ok(T::one() - (q_2 + q_3 - q_1) * (q_2 + q_3 - q_1) / (four * q_2 * q_3))
}

#[cfg(test)]
mod tests {
    use super::*;
    use num_rational::Ratio;

    #[test]
    fn test_archimedes2() {
        let q_1: i64 = 1;
        let q_2: i64 = 2;
        let q_3: i64 = 3;
        assert_eq!(archimedes(&q_1, &q_2, &q_3), 8);
    }

    #[test]
    fn test_archimedes3() {
        let q_1 = 1.0;
        let q_2 = 2.0;
        let q_3 = 3.0;
        assert_eq!(archimedes(&q_1, &q_2, &q_3), 8.0);
    }

    #[test]
    fn test_archimedes() {
        let q_1 = Ratio::<i32>::new(1, 2);
        let q_2 = Ratio::<i32>::new(1, 4);
        let q_3 = Ratio::<i32>::new(1, 6);
        assert_eq!(archimedes(&q_1, &q_2, &q_3), Ratio::<i32>::new(23, 144));
    }

    #[test]
    fn test_archimedes_zero() {
        let q_1 = Ratio::<i64>::new(0, 1);
        let q_2 = Ratio::<i64>::new(0, 1);
        let q_3 = Ratio::<i64>::new(0, 1);
        assert_eq!(archimedes(&q_1, &q_2, &q_3), Ratio::<i64>::new(0, 1));
    }

    #[test]
    fn test_archimedes_negative() {
        let q_1 = Ratio::<i64>::new(-1, 2);
        let q_2 = Ratio::<i64>::new(-1, 4);
        let q_3 = Ratio::<i64>::new(-1, 6);
        assert_eq!(archimedes(&q_1, &q_2, &q_3), Ratio::<i64>::new(23, 144));
    }

    #[test]
    fn test_archimedes_i32() {
        let q_1: i32 = 1;
        let q_2: i32 = 2;
        let q_3: i32 = 3;
        assert_eq!(archimedes(&q_1, &q_2, &q_3), 8);
    }

    #[test]
    fn test_quadrance() {
        let p1 = (1, 1);
        let p2 = (4, 5);
        assert_eq!(quadrance(p1, p2), 25);
    }

    #[test]
    fn test_spread() {
        let v1 = (1.0, 1.0);
        let v2 = (1.0, 0.0);
        assert_eq!(spread(v1, v2), 0.5);
    }

    #[test]
    fn test_cross() {
        let v1 = (1, 1);
        let v2 = (1, 0);
        assert_eq!(cross(v1, v2), -1);
    }

    #[test]
    fn test_quadrance_from_line() {
        let p1 = (1.0, 1.0);
        let l1 = (1.0, 1.0, 1.0);
        assert_eq!(quadrance_from_line(p1, l1), 4.5);
    }

    #[test]
    fn test_spread_from_line() {
        let l1 = (1.0, 1.0, 1.0);
        let l2 = (1.0, 0.0, 0.0);
        assert_eq!(spread_from_line(l1, l2), 0.5);
    }

    #[test]
    fn test_cross_from_line() {
        let l1 = (1, 1, 1);
        let l2 = (1, 0, 0);
        assert_eq!(cross_from_line(l1, l2), -1);
    }

    #[test]
    fn test_quadrance_from_three_points() {
        let p1 = (0, 0);
        let p2 = (1, 0);
        let p3 = (0, 1);
        assert_eq!(quadrance_from_three_points(p1, p2, p3), (2, 1, 1));
    }

    #[test]
    fn test_spread_from_three_points() {
        let p1 = (0.0, 0.0);
        let p2 = (1.0, 0.0);
        let p3 = (0.0, 1.0);
        assert_eq!(spread_from_three_points(p1, p2, p3), (1.0, 0.5, 0.5));
    }

    #[test]
    fn test_cross_from_three_points() {
        let p1 = (0, 0);
        let p2 = (1, 0);
        let p3 = (0, 1);
        assert_eq!(cross_from_three_points(p1, p2, p3), 1);
    }

    // #[test]
    // fn test_archimedes4() {
    //     let q_1 = Fraction::<i64>::new(1, 2);

    #[test]
    fn test_quadrance3d() {
        let p1 = (0, 0, 0);
        let p2 = (1, 2, 2);
        assert_eq!(quadrance3d(p1, p2), 9);
    }

    #[test]
    fn test_cross3d() {
        let v1 = (1, 0, 0);
        let v2 = (0, 1, 0);
        assert_eq!(cross3d(v1, v2), (0, 0, 1));
    }

    #[test]
    fn test_spread3d() {
        let v1 = (1.0, 0.0, 0.0);
        let v2 = (0.0, 1.0, 0.0);
        assert_eq!(spread3d(v1, v2), 1.0);
    }

    #[test]
    fn test_quadrance3d_rational() {
        let p1 = (
            Ratio::<i32>::new(0, 1),
            Ratio::<i32>::new(0, 1),
            Ratio::<i32>::new(0, 1),
        );
        let p2 = (
            Ratio::<i32>::new(1, 1),
            Ratio::<i32>::new(2, 1),
            Ratio::<i32>::new(2, 1),
        );
        assert_eq!(quadrance3d(p1, p2), Ratio::<i32>::new(9, 1));
    }

    #[test]
    fn test_twist() {
        let p1 = (0, 0);
        let p2 = (1, 0);
        let p3 = (0, 1);
        assert_eq!(twist(p1, p2, p3), 1);
    }

    #[test]
    fn test_turn() {
        let p1 = (0.0, 0.0);
        let p2 = (1.0, 0.0);
        let p3 = (1.0, 1.0);
        let (s, sign) = turn(p1, p2, p3);
        // v1 = (1,0), v2 = (0,1) - perpendicular, spread = 1.0
        assert_eq!(s, 1.0);
        assert!(sign);
    }

    #[test]
    fn test_dilatation() {
        let v1 = (1.0, 0.0);
        let v2 = (2.0, 0.0);
        assert_eq!(dilatation(v1, v2), 4.0);
    }

    #[test]
    fn test_sine_law_product() {
        let q = 4.0;
        let s = 0.5;
        assert_eq!(sine_law_product(q, s), 2.0);
    }

    #[test]
    fn test_cosine_law() {
        let q1 = 2.0;
        let q2 = 1.0;
        let q3 = 1.0;
        // For a right triangle with sides sqrt(2), 1, 1
        // The angle opposite q1 is 90°, so spread = 1.0
        assert_eq!(cosine_law(q1, q2, q3), 1.0);
    }

    #[test]
    fn test_twist_negative() {
        let p1 = (0, 0);
        let p2 = (1, 0);
        let p3 = (0, -1);
        assert_eq!(twist(p1, p2, p3), -1);
    }

    #[test]
    fn test_safe_spread_success() {
        let v1: (f64, f64) = (1.0, 1.0);
        let v2: (f64, f64) = (1.0, 0.0);
        let result = safe_spread(v1, v2);
        assert!(result.is_ok());
        assert!((result.unwrap() - 0.5_f64).abs() < 1e-10);
    }

    #[test]
    fn test_safe_spread_zero_vector() {
        let v1: (f64, f64) = (0.0, 0.0);
        let v2: (f64, f64) = (1.0, 0.0);
        let result = safe_spread(v1, v2);
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), crate::error::MathError::DivisionByZero);
    }

    #[test]
    fn test_safe_quadrance_from_line_success() {
        let p: (f64, f64) = (1.0, 1.0);
        let l: (f64, f64, f64) = (1.0, 1.0, 1.0);
        let result = safe_quadrance_from_line(p, l);
        assert!(result.is_ok());
        assert!((result.unwrap() - 4.5_f64).abs() < 1e-10);
    }

    #[test]
    fn test_safe_quadrance_from_line_zero_line() {
        let p: (f64, f64) = (1.0, 1.0);
        let l: (f64, f64, f64) = (0.0, 0.0, 1.0);
        let result = safe_quadrance_from_line(p, l);
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), crate::error::MathError::DivisionByZero);
    }

    #[test]
    fn test_safe_spread_from_line_success() {
        let l1: (f64, f64, f64) = (1.0, 1.0, 1.0);
        let l2: (f64, f64, f64) = (1.0, 0.0, 0.0);
        let result = safe_spread_from_line(l1, l2);
        assert!(result.is_ok());
        assert!((result.unwrap() - 0.5_f64).abs() < 1e-10);
    }

    #[test]
    fn test_safe_spread_from_line_zero_line() {
        let l1: (f64, f64, f64) = (0.0, 0.0, 1.0);
        let l2: (f64, f64, f64) = (1.0, 0.0, 0.0);
        let result = safe_spread_from_line(l1, l2);
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), crate::error::MathError::DivisionByZero);
    }

    #[test]
    fn test_safe_dilatation_success() {
        let v1: (f64, f64) = (1.0, 0.0);
        let v2: (f64, f64) = (2.0, 0.0);
        let result = safe_dilatation(v1, v2);
        assert!(result.is_ok());
        assert!((result.unwrap() - 4.0_f64).abs() < 1e-10);
    }

    #[test]
    fn test_safe_dilatation_zero_vector() {
        let v1: (f64, f64) = (0.0, 0.0);
        let v2: (f64, f64) = (1.0, 0.0);
        let result = safe_dilatation(v1, v2);
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), crate::error::MathError::DivisionByZero);
    }

    #[test]
    fn test_safe_cosine_law_success() {
        let q1: f64 = 2.0;
        let q2: f64 = 1.0;
        let q3: f64 = 1.0;
        let result = safe_cosine_law(q1, q2, q3);
        assert!(result.is_ok());
        assert!((result.unwrap() - 1.0_f64).abs() < 1e-10);
    }

    #[test]
    fn test_safe_cosine_law_zero_side() {
        let q1: f64 = 1.0;
        let q2: f64 = 0.0;
        let q3: f64 = 1.0;
        let result = safe_cosine_law(q1, q2, q3);
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), crate::error::MathError::DivisionByZero);
    }
}

#[cfg(test)]
mod quickcheck_tests {
    // Note: Quickcheck tests are disabled for now due to integer overflow issues
    // These tests work better with bounded types or f64
}
