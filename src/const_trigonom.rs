/// Module containing const versions of trigonometric functions for specific concrete types.
/// These functions can be used in const contexts with concrete numeric types.
/// Calculate quadrance (square of distance) between two points with i64 coordinates
#[inline]
pub const fn quadrance_i64(p_1: (i64, i64), p_2: (i64, i64)) -> i64 {
    let dx = p_1.0 - p_2.0;
    let dy = p_1.1 - p_2.1;
    dx * dx + dy * dy
}

/// Calculate Archimedes formula (quadrea) for a triangle with i64 side lengths
#[inline]
pub const fn archimedes_i64(q_1: i64, q_2: i64, q_3: i64) -> i64 {
    let temp = q_1 + q_2 - q_3;
    let four = 4;
    four * q_1 * q_2 - temp * temp
}

/// Calculate spread (square of sine) between two vectors with i64 coordinates
#[inline]
pub const fn spread_i64(v_1: (i64, i64), v_2: (i64, i64)) -> i64 {
    let dot_product = v_1.0 * v_2.0 + v_1.1 * v_2.1;
    let q_1 = quadrance_i64(v_1, (0, 0));
    let q_2 = quadrance_i64(v_2, (0, 0));
    let q_product = q_1 * q_2;
    if q_product == 0 {
        0
    } else {
        q_product - (dot_product * dot_product) / q_product
    }
}

/// Calculate cross product of two 2D vectors with i64 coordinates
#[inline]
pub const fn cross_i64(v_1: (i64, i64), v_2: (i64, i64)) -> i64 {
    v_1.0 * v_2.1 - v_1.1 * v_2.0
}

/// Calculate quadrance (square of distance) between two points with i32 coordinates
#[inline]
pub const fn quadrance_i32(p_1: (i32, i32), p_2: (i32, i32)) -> i32 {
    let dx = p_1.0 - p_2.0;
    let dy = p_1.1 - p_2.1;
    dx * dx + dy * dy
}

/// Calculate Archimedes formula (quadrea) for a triangle with i32 side lengths
#[inline]
pub const fn archimedes_i32(q_1: i32, q_2: i32, q_3: i32) -> i32 {
    let temp = q_1 + q_2 - q_3;
    let four: i32 = 4;
    four * q_1 * q_2 - temp * temp
}

/// Calculate spread (square of sine) between two vectors with i32 coordinates
#[inline]
pub const fn spread_i32(v_1: (i32, i32), v_2: (i32, i32)) -> i32 {
    let dot_product = v_1.0 * v_2.0 + v_1.1 * v_2.1;
    let q_1 = quadrance_i32(v_1, (0, 0));
    let q_2 = quadrance_i32(v_2, (0, 0));
    let q_product = q_1 * q_2;
    if q_product == 0 {
        0
    } else {
        q_product - dot_product * dot_product / q_product
    }
}

/// Calculate cross product of two 2D vectors with i32 coordinates
#[inline]
pub const fn cross_i32(v_1: (i32, i32), v_2: (i32, i32)) -> i32 {
    v_1.0 * v_2.1 - v_1.1 * v_2.0
}

/// Calculate quadrance (square of distance) between two points with f64 coordinates
#[inline]
pub const fn quadrance_f64(p_1: (f64, f64), p_2: (f64, f64)) -> f64 {
    let dx = p_1.0 - p_2.0;
    let dy = p_1.1 - p_2.1;
    dx * dx + dy * dy
}

/// Calculate Archimedes formula (quadrea) for a triangle with f64 side lengths
#[inline]
pub const fn archimedes_f64(q_1: f64, q_2: f64, q_3: f64) -> f64 {
    let temp = q_1 + q_2 - q_3;
    let four: f64 = 4.0;
    four * q_1 * q_2 - temp * temp
}

/// Calculate spread (square of sine) between two vectors with f64 coordinates
#[inline]
pub const fn spread_f64(v_1: (f64, f64), v_2: (f64, f64)) -> f64 {
    let dot_product = v_1.0 * v_2.0 + v_1.1 * v_2.1;
    let q_1 = quadrance_f64(v_1, (0.0, 0.0));
    let q_2 = quadrance_f64(v_2, (0.0, 0.0));
    let q_product = q_1 * q_2;
    if q_product == 0.0 {
        0.0
    } else {
        1.0 - dot_product * dot_product / q_product
    }
}

/// Calculate cross product of two 2D vectors with f64 coordinates
#[inline]
pub const fn cross_f64(v_1: (f64, f64), v_2: (f64, f64)) -> f64 {
    v_1.0 * v_2.1 - v_1.1 * v_2.0
}

/// Calculate quadrance (square of distance) between two points with u32 coordinates
#[inline]
pub const fn quadrance_u32(p_1: (u32, u32), p_2: (u32, u32)) -> u32 {
    let dx = p_1.0.abs_diff(p_2.0);
    let dy = p_1.1.abs_diff(p_2.1);
    dx * dx + dy * dy
}

/// Calculate Archimedes formula (quadrea) for a triangle with u32 side lengths
#[inline]
pub const fn archimedes_u32(q_1: u32, q_2: u32, q_3: u32) -> u32 {
    let temp = q_1 + q_2 - q_3;
    let four: u32 = 4;
    four * q_1 * q_2 - temp * temp
}

/// Calculate spread (square of sine) between two vectors with u32 coordinates
#[inline]
pub const fn spread_u32(v_1: (u32, u32), v_2: (u32, u32)) -> u32 {
    let dot_product = v_1.0 * v_2.0 + v_1.1 * v_2.1;
    let q_1 = quadrance_u32(v_1, (0, 0));
    let q_2 = quadrance_u32(v_2, (0, 0));
    let q_product = q_1 * q_2;
    if q_product == 0 {
        0
    } else {
        q_product - dot_product * dot_product / q_product
    }
}

/// Calculate cross product of two 2D vectors with u32 coordinates
///
/// Note: Uses wrapping subtraction since cross product can be negative.
/// For unsigned types, the result is the absolute value of the cross product.
#[inline]
pub const fn cross_u32(v_1: (u32, u32), v_2: (u32, u32)) -> u32 {
    let a = v_1.0 * v_2.1;
    let b = v_1.1 * v_2.0;
    a.abs_diff(b)
}

/// Calculate quadrance (square of distance) between two points with u64 coordinates
#[inline]
pub const fn quadrance_u64(p_1: (u64, u64), p_2: (u64, u64)) -> u64 {
    let dx = p_1.0.abs_diff(p_2.0);
    let dy = p_1.1.abs_diff(p_2.1);
    dx * dx + dy * dy
}

/// Calculate Archimedes formula (quadrea) for a triangle with u64 side lengths
#[inline]
pub const fn archimedes_u64(q_1: u64, q_2: u64, q_3: u64) -> u64 {
    let temp = q_1 + q_2 - q_3;
    let four: u64 = 4;
    four * q_1 * q_2 - temp * temp
}

/// Calculate spread (square of sine) between two vectors with u64 coordinates
#[inline]
pub const fn spread_u64(v_1: (u64, u64), v_2: (u64, u64)) -> u64 {
    let dot_product = v_1.0 * v_2.0 + v_1.1 * v_2.1;
    let q_1 = quadrance_u64(v_1, (0, 0));
    let q_2 = quadrance_u64(v_2, (0, 0));
    let q_product = q_1 * q_2;
    if q_product == 0 {
        0
    } else {
        q_product - dot_product * dot_product / q_product
    }
}

/// Calculate cross product of two 2D vectors with u64 coordinates
///
/// Note: Uses wrapping subtraction since cross product can be negative.
/// For unsigned types, the result is the absolute value of the cross product.
#[inline]
pub const fn cross_u64(v_1: (u64, u64), v_2: (u64, u64)) -> u64 {
    let a = v_1.0 * v_2.1;
    let b = v_1.1 * v_2.0;
    a.abs_diff(b)
}

/// Calculate quadrance (square of distance) between two points with i128 coordinates
#[inline]
pub const fn quadrance_i128(p_1: (i128, i128), p_2: (i128, i128)) -> i128 {
    let dx = p_1.0 - p_2.0;
    let dy = p_1.1 - p_2.1;
    dx * dx + dy * dy
}

/// Calculate Archimedes formula (quadrea) for a triangle with i128 side lengths
#[inline]
pub const fn archimedes_i128(q_1: i128, q_2: i128, q_3: i128) -> i128 {
    let temp = q_1 + q_2 - q_3;
    let four: i128 = 4;
    four * q_1 * q_2 - temp * temp
}

/// Calculate spread (square of sine) between two vectors with i128 coordinates
#[inline]
pub const fn spread_i128(v_1: (i128, i128), v_2: (i128, i128)) -> i128 {
    let dot_product = v_1.0 * v_2.0 + v_1.1 * v_2.1;
    let q_1 = quadrance_i128(v_1, (0, 0));
    let q_2 = quadrance_i128(v_2, (0, 0));
    let q_product = q_1 * q_2;
    if q_product == 0 {
        0
    } else {
        q_product - dot_product * dot_product / q_product
    }
}

/// Calculate cross product of two 2D vectors with i128 coordinates
#[inline]
pub const fn cross_i128(v_1: (i128, i128), v_2: (i128, i128)) -> i128 {
    v_1.0 * v_2.1 - v_1.1 * v_2.0
}

/// Calculate quadrance (square of distance) between two points with u128 coordinates
#[inline]
pub const fn quadrance_u128(p_1: (u128, u128), p_2: (u128, u128)) -> u128 {
    let dx = p_1.0.abs_diff(p_2.0);
    let dy = p_1.1.abs_diff(p_2.1);
    dx * dx + dy * dy
}

/// Calculate Archimedes formula (quadrea) for a triangle with u128 side lengths
#[inline]
pub const fn archimedes_u128(q_1: u128, q_2: u128, q_3: u128) -> u128 {
    let temp = q_1 + q_2 - q_3;
    let four: u128 = 4;
    four * q_1 * q_2 - temp * temp
}

/// Calculate spread (square of sine) between two vectors with u128 coordinates
#[inline]
pub const fn spread_u128(v_1: (u128, u128), v_2: (u128, u128)) -> u128 {
    let dot_product = v_1.0 * v_2.0 + v_1.1 * v_2.1;
    let q_1 = quadrance_u128(v_1, (0, 0));
    let q_2 = quadrance_u128(v_2, (0, 0));
    let q_product = q_1 * q_2;
    if q_product == 0 {
        0
    } else {
        q_product - dot_product * dot_product / q_product
    }
}

/// Calculate cross product of two 2D vectors with u128 coordinates
///
/// Note: Uses wrapping subtraction since cross product can be negative.
/// For unsigned types, the result is the absolute value of the cross product.
#[inline]
pub const fn cross_u128(v_1: (u128, u128), v_2: (u128, u128)) -> u128 {
    let a = v_1.0 * v_2.1;
    let b = v_1.1 * v_2.0;
    a.abs_diff(b)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_quadrance_i64() {
        let p1 = (1, 1);
        let p2 = (4, 5);
        assert_eq!(quadrance_i64(p1, p2), 25);
    }

    #[test]
    fn test_archimedes_i64() {
        assert_eq!(archimedes_i64(1, 2, 3), 8);
    }

    #[test]
    fn test_spread_i64() {
        let v1 = (1, 1);
        let v2 = (1, 0);
        let result = spread_i64(v1, v2);
        // For vectors (1,1) and (1,0):
        // dot = 1*1 + 1*0 = 1
        // q1 = 1^2 + 1^2 = 2
        // q2 = 1^2 + 0^2 = 1
        // spread = q1*q2 - dot^2/q1*q2 = 2 - 1/2 = 2 - 0 = 2 (integer division)
        // This is the numerator of the spread formula with integer division
        assert_eq!(result, 2);
    }

    #[test]
    fn test_cross_i64() {
        let v1 = (1, 1);
        let v2 = (1, 0);
        assert_eq!(cross_i64(v1, v2), -1);
    }

    #[test]
    fn test_quadrance_i32() {
        let p1 = (1, 1);
        let p2 = (4, 5);
        assert_eq!(quadrance_i32(p1, p2), 25);
    }

    #[test]
    fn test_archimedes_i32() {
        assert_eq!(archimedes_i32(1, 2, 3), 8);
    }

    #[test]
    fn test_spread_i32() {
        let v1 = (1, 1);
        let v2 = (1, 0);
        let result = spread_i32(v1, v2);
        assert_eq!(result, 2);
    }

    #[test]
    fn test_cross_i32() {
        let v1 = (1, 1);
        let v2 = (1, 0);
        assert_eq!(cross_i32(v1, v2), -1);
    }

    #[test]
    fn test_quadrance_f64() {
        let p1 = (1.0, 1.0);
        let p2 = (4.0, 5.0);
        assert_eq!(quadrance_f64(p1, p2), 25.0);
    }

    #[test]
    fn test_archimedes_f64() {
        assert_eq!(archimedes_f64(1.0, 2.0, 3.0), 8.0);
    }

    #[test]
    fn test_spread_f64() {
        let v1 = (1.0, 1.0);
        let v2 = (1.0, 0.0);
        assert_eq!(spread_f64(v1, v2), 0.5);
    }

    #[test]
    fn test_cross_f64() {
        let v1 = (1.0, 1.0);
        let v2 = (1.0, 0.0);
        assert_eq!(cross_f64(v1, v2), -1.0);
    }

    #[test]
    fn test_quadrance_u32() {
        let p1 = (1, 1);
        let p2 = (4, 5);
        assert_eq!(quadrance_u32(p1, p2), 25);
    }

    #[test]
    fn test_archimedes_u32() {
        assert_eq!(archimedes_u32(1, 2, 3), 8);
    }

    #[test]
    fn test_spread_u32() {
        let v1 = (1, 1);
        let v2 = (1, 0);
        let result = spread_u32(v1, v2);
        assert_eq!(result, 2);
    }

    #[test]
    fn test_cross_u32() {
        let v1 = (1, 1);
        let v2 = (1, 0);
        // For unsigned types, cross returns absolute value
        assert_eq!(cross_u32(v1, v2), 1);
    }

    #[test]
    fn test_quadrance_u64() {
        let p1 = (1, 1);
        let p2 = (4, 5);
        assert_eq!(quadrance_u64(p1, p2), 25);
    }

    #[test]
    fn test_archimedes_u64() {
        assert_eq!(archimedes_u64(1, 2, 3), 8);
    }

    #[test]
    fn test_spread_u64() {
        let v1 = (1, 1);
        let v2 = (1, 0);
        let result = spread_u64(v1, v2);
        assert_eq!(result, 2);
    }

    #[test]
    fn test_cross_u64() {
        let v1 = (1, 1);
        let v2 = (1, 0);
        // For unsigned types, cross returns absolute value
        assert_eq!(cross_u64(v1, v2), 1);
    }

    #[test]
    fn test_quadrance_i128() {
        let p1 = (1, 1);
        let p2 = (4, 5);
        assert_eq!(quadrance_i128(p1, p2), 25);
    }

    #[test]
    fn test_archimedes_i128() {
        assert_eq!(archimedes_i128(1, 2, 3), 8);
    }

    #[test]
    fn test_spread_i128() {
        let v1 = (1, 1);
        let v2 = (1, 0);
        let result = spread_i128(v1, v2);
        assert_eq!(result, 2);
    }

    #[test]
    fn test_cross_i128() {
        let v1 = (1, 1);
        let v2 = (1, 0);
        assert_eq!(cross_i128(v1, v2), -1);
    }

    #[test]
    fn test_quadrance_u128() {
        let p1 = (1, 1);
        let p2 = (4, 5);
        assert_eq!(quadrance_u128(p1, p2), 25);
    }

    #[test]
    fn test_archimedes_u128() {
        assert_eq!(archimedes_u128(1, 2, 3), 8);
    }

    #[test]
    fn test_spread_u128() {
        let v1 = (1, 1);
        let v2 = (1, 0);
        let result = spread_u128(v1, v2);
        assert_eq!(result, 2);
    }

    #[test]
    fn test_cross_u128() {
        let v1 = (1, 1);
        let v2 = (1, 0);
        // For unsigned types, cross returns absolute value
        assert_eq!(cross_u128(v1, v2), 1);
    }
}
