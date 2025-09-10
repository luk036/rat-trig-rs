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
use core::ops::{Add, Div, Mul, Sub};

use num_traits::{One, Zero};

/// The function `archimedes` calculates the area of a triangle using Archimedes' formula with the
/// lengths of the three sides provided as `Fraction<i64>` values.
///
/// Arguments:
///
/// * `q_1`: Represents the length of the first side of the triangle.
/// * `q_2`: The parameters `q_1`, `q_2`, and `q_3` represent the lengths of the sides of a triangle. In
///          the context of Archimedes' formula for the area of a triangle, `q_1`, `q_2`, and `q_3`
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
    T: std::marker::Copy + Add<Output = T> + Sub<Output = T> + Mul<Output = T> + One + Zero,
{
    let temp = *q_1 + *q_2 - *q_3;
    let four = T::one() + T::one() + T::one() + T::one();
    four * *q_1 * *q_2 - temp * temp
}

#[inline]
pub fn quadrance<T>(p_1: (T, T), p_2: (T, T)) -> T
where
    T: std::marker::Copy + Add<Output = T> + Sub<Output = T> + Mul<Output = T>,
{
    let dx = p_1.0 - p_2.0;
    let dy = p_1.1 - p_2.1;
    dx * dx + dy * dy
}

#[inline]
pub fn spread<T>(v_1: (T, T), v_2: (T, T)) -> T
where
    T: std::marker::Copy
        + Add<Output = T>
        + Sub<Output = T>
        + Mul<Output = T>
        + Div<Output = T>
        + One
        + Zero,
{
    let dot_product = v_1.0 * v_2.0 + v_1.1 * v_2.1;
    let q_1 = quadrance(v_1, (T::zero(), T::zero()));
    let q_2 = quadrance(v_2, (T::zero(), T::zero()));
    T::one() - dot_product * dot_product / (q_1 * q_2)
}

#[inline]
pub fn cross<T>(v_1: (T, T), v_2: (T, T)) -> T
where
    T: std::marker::Copy + Sub<Output = T> + Mul<Output = T>,
{
    v_1.0 * v_2.1 - v_1.1 * v_2.0
}

#[inline]
pub fn quadrance_from_line<T>(p: (T, T), l: (T, T, T)) -> T
where
    T: std::marker::Copy + Add<Output = T> + Sub<Output = T> + Mul<Output = T> + Div<Output = T> + Zero,
{
    let temp = l.0 * p.0 + l.1 * p.1 + l.2;
    temp * temp / quadrance((l.0, l.1), (T::zero(), T::zero()))
}

#[inline]
pub fn spread_from_line<T>(l_1: (T, T, T), l_2: (T, T, T)) -> T
where
    T: std::marker::Copy + Add<Output = T> + Sub<Output = T> + Mul<Output = T> + Div<Output = T> + Zero,
{
    let temp = cross((l_1.0, l_1.1), (l_2.0, l_2.1));
    temp * temp / (quadrance((l_1.0, l_1.1), (T::zero(), T::zero())) * quadrance((l_2.0, l_2.1), (T::zero(), T::zero())))
}

#[inline]
pub fn cross_from_line<T>(l_1: (T, T, T), l_2: (T, T, T)) -> T
where
    T: std::marker::Copy + Sub<Output = T> + Mul<Output = T>,
{
    cross((l_1.0, l_1.1), (l_2.0, l_2.1))
}

#[inline]
pub fn quadrance_from_three_points<T>(p_1: (T, T), p_2: (T, T), p_3: (T, T)) -> (T, T, T)
where
    T: std::marker::Copy + Add<Output = T> + Sub<Output = T> + Mul<Output = T>,
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
    T: std::marker::Copy
        + Add<Output = T>
        + Sub<Output = T>
        + Mul<Output = T>
        + Div<Output = T>
        + One
        + Zero,
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
    T: std::marker::Copy + Sub<Output = T> + Mul<Output = T>,
{
    cross(
        (p_2.0 - p_1.0, p_2.1 - p_1.1),
        (p_3.0 - p_1.0, p_3.1 - p_1.1),
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use num_rational::Ratio;
    // use fractions::Fraction;

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
}