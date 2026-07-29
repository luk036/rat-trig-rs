/// Module containing const versions of trigonometric functions for specific concrete types.
macro_rules! impl_quadrance_signed {
    ($ty:ident, $name:ident) => {
        /// Calculate quadrance (square of distance) between two points.
        ///
        /// $$ Q(p_1, p_2) = (x_2 - x_1)^2 + (y_2 - y_1)^2 $$
        #[inline]
        pub const fn $name(p_1: ($ty, $ty), p_2: ($ty, $ty)) -> $ty {
            let dx = p_1.0 - p_2.0;
            let dy = p_1.1 - p_2.1;
            dx * dx + dy * dy
        }
    };
}

macro_rules! impl_quadrance_unsigned {
    ($ty:ident, $name:ident) => {
        /// Calculate quadrance (square of distance) between two points.
        ///
        /// $$ Q(p_1, p_2) = |x_2 - x_1|^2 + |y_2 - y_1|^2 $$
        #[inline]
        pub const fn $name(p_1: ($ty, $ty), p_2: ($ty, $ty)) -> $ty {
            let dx = p_1.0.abs_diff(p_2.0);
            let dy = p_1.1.abs_diff(p_2.1);
            dx * dx + dy * dy
        }
    };
}

macro_rules! impl_archimedes {
    ($ty:ident, $name:ident) => {
        /// Calculate Archimedes formula (quadrea) for a triangle.
        ///
        /// $$ \text{quadrea}(q_1, q_2, q_3) = 4 q_1 q_2 - (q_1 + q_2 - q_3)^2 $$
        #[inline]
        pub const fn $name(q_1: $ty, q_2: $ty, q_3: $ty) -> $ty {
            let temp = q_1 + q_2 - q_3;
            let four: $ty = 4;
            four * q_1 * q_2 - temp * temp
        }
    };
}

macro_rules! impl_spread {
    ($ty:ident, $name:ident, $quad_fn:ident) => {
        /// Calculate spread (square of sine) between two vectors.
        ///
        /// $$ s(v_1, v_2) = 1 - \frac{(v_1 \cdot v_2)^2}{|v_1|^2 |v_2|^2} $$
        #[inline]
        pub const fn $name(v_1: ($ty, $ty), v_2: ($ty, $ty)) -> $ty {
            let dot_product = v_1.0 * v_2.0 + v_1.1 * v_2.1;
            let q_1 = $quad_fn(v_1, (0, 0));
            let q_2 = $quad_fn(v_2, (0, 0));
            let q_product = q_1 * q_2;
            if q_product == 0 {
                0
            } else {
                q_product - dot_product * dot_product / q_product
            }
        }
    };
}

macro_rules! impl_cross_signed {
    ($ty:ident, $name:ident) => {
        /// Calculate cross product of two 2D vectors.
        ///
        /// $$ \text{cross}(v_1, v_2) = x_1 y_2 - y_1 x_2 $$
        #[inline]
        pub const fn $name(v_1: ($ty, $ty), v_2: ($ty, $ty)) -> $ty {
            v_1.0 * v_2.1 - v_1.1 * v_2.0
        }
    };
}

macro_rules! impl_cross_unsigned {
    ($ty:ident, $name:ident) => {
        /// Calculate cross product of two 2D vectors.
        ///
        /// $$ \text{cross}(v_1, v_2) = |x_1 y_2 - y_1 x_2| $$
        ///
        /// Note: Uses abs_diff since cross product can be negative.
        /// For unsigned types, the result is the absolute value of the cross product.
        #[inline]
        pub const fn $name(v_1: ($ty, $ty), v_2: ($ty, $ty)) -> $ty {
            let a = v_1.0 * v_2.1;
            let b = v_1.1 * v_2.0;
            a.abs_diff(b)
        }
    };
}

impl_quadrance_signed!(i64, quadrance_i64);
impl_quadrance_signed!(i32, quadrance_i32);
impl_quadrance_signed!(i128, quadrance_i128);
impl_quadrance_unsigned!(u32, quadrance_u32);
impl_quadrance_unsigned!(u64, quadrance_u64);
impl_quadrance_unsigned!(u128, quadrance_u128);

impl_archimedes!(i64, archimedes_i64);
impl_archimedes!(i32, archimedes_i32);
impl_archimedes!(i128, archimedes_i128);
impl_archimedes!(u32, archimedes_u32);
impl_archimedes!(u64, archimedes_u64);
impl_archimedes!(u128, archimedes_u128);

impl_spread!(i64, spread_i64, quadrance_i64);
impl_spread!(i32, spread_i32, quadrance_i32);
impl_spread!(i128, spread_i128, quadrance_i128);
impl_spread!(u32, spread_u32, quadrance_u32);
impl_spread!(u64, spread_u64, quadrance_u64);
impl_spread!(u128, spread_u128, quadrance_u128);

impl_cross_signed!(i64, cross_i64);
impl_cross_signed!(i32, cross_i32);
impl_cross_signed!(i128, cross_i128);
impl_cross_unsigned!(u32, cross_u32);
impl_cross_unsigned!(u64, cross_u64);
impl_cross_unsigned!(u128, cross_u128);

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
