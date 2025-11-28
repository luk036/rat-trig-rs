/// Module containing const versions of trigonometric functions for specific concrete types.
/// These functions can be used in const contexts with concrete numeric types.

/// Calculate quadrance (square of distance) between two points with i64 coordinates
#[inline]
pub const fn quadrance_i64(p_1: (i64, i64), p_2: (i64, i64)) -> i64 {
    let dx = p_1.0 - p_2.0;
    let dy = p_1.1 - p_2.1;
    dx * dx + dy * dy
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
    fn test_cross_f64() {
        let v1 = (1.0, 1.0);
        let v2 = (1.0, 0.0);
        assert_eq!(cross_f64(v1, v2), -1.0);
    }
}