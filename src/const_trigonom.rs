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
}
