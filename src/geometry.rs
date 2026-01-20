//! Geometry primitive types for rational trigonometry
//!
//! This module provides structured types for geometric primitives
//! including points, vectors, lines, and triangles.

use core::ops::{Add, Mul, Sub};
use num_traits::{One, Zero};

/// A 2D point with coordinates of type T
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Point2D<T> {
    pub x: T,
    pub y: T,
}

impl<T> Point2D<T> {
    pub fn new(x: T, y: T) -> Self {
        Self { x, y }
    }
}

impl<T> From<(T, T)> for Point2D<T> {
    fn from(tuple: (T, T)) -> Self {
        Self {
            x: tuple.0,
            y: tuple.1,
        }
    }
}

/// A 3D point with coordinates of type T
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Point3D<T> {
    pub x: T,
    pub y: T,
    pub z: T,
}

impl<T> Point3D<T> {
    pub fn new(x: T, y: T, z: T) -> Self {
        Self { x, y, z }
    }
}

impl<T> From<(T, T, T)> for Point3D<T> {
    fn from(tuple: (T, T, T)) -> Self {
        Self {
            x: tuple.0,
            y: tuple.1,
            z: tuple.2,
        }
    }
}

/// A 2D vector with components of type T
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Vector2D<T> {
    pub x: T,
    pub y: T,
}

impl<T> Vector2D<T> {
    pub fn new(x: T, y: T) -> Self {
        Self { x, y }
    }
}

impl<T> From<(T, T)> for Vector2D<T> {
    fn from(tuple: (T, T)) -> Self {
        Self {
            x: tuple.0,
            y: tuple.1,
        }
    }
}

impl<T> From<Point2D<T>> for Vector2D<T> {
    fn from(point: Point2D<T>) -> Self {
        Self {
            x: point.x,
            y: point.y,
        }
    }
}

impl<T> Add for Vector2D<T>
where
    T: Add<Output = T>,
{
    type Output = Self;
    fn add(self, other: Self) -> Self::Output {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl<T> Sub for Vector2D<T>
where
    T: Sub<Output = T>,
{
    type Output = Self;
    fn sub(self, other: Self) -> Self::Output {
        Self {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}

/// A 3D vector with components of type T
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Vector3D<T> {
    pub x: T,
    pub y: T,
    pub z: T,
}

impl<T> Vector3D<T> {
    pub fn new(x: T, y: T, z: T) -> Self {
        Self { x, y, z }
    }
}

impl<T> From<(T, T, T)> for Vector3D<T> {
    fn from(tuple: (T, T, T)) -> Self {
        Self {
            x: tuple.0,
            y: tuple.1,
            z: tuple.2,
        }
    }
}

impl<T> From<Point3D<T>> for Vector3D<T> {
    fn from(point: Point3D<T>) -> Self {
        Self {
            x: point.x,
            y: point.y,
            z: point.z,
        }
    }
}

impl<T> Add for Vector3D<T>
where
    T: Add<Output = T>,
{
    type Output = Self;
    fn add(self, other: Self) -> Self::Output {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }
}

impl<T> Sub for Vector3D<T>
where
    T: Sub<Output = T>,
{
    type Output = Self;
    fn sub(self, other: Self) -> Self::Output {
        Self {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        }
    }
}

/// A 2D line represented as ax + by + c = 0
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Line2D<T> {
    pub a: T,
    pub b: T,
    pub c: T,
}

impl<T> Line2D<T> {
    pub fn new(a: T, b: T, c: T) -> Self {
        Self { a, b, c }
    }
}

impl<T> From<(T, T, T)> for Line2D<T> {
    fn from(tuple: (T, T, T)) -> Self {
        Self {
            a: tuple.0,
            b: tuple.1,
            c: tuple.2,
        }
    }
}

/// A 2D triangle defined by three points
#[derive(Debug, Clone, Copy)]
pub struct Triangle2D<T> {
    pub p1: Point2D<T>,
    pub p2: Point2D<T>,
    pub p3: Point2D<T>,
}

impl<T> Triangle2D<T> {
    pub fn new(p1: Point2D<T>, p2: Point2D<T>, p3: Point2D<T>) -> Self {
        Self { p1, p2, p3 }
    }

    /// Calculate the quadrances of the triangle sides
    pub fn quadrances(&self) -> (T, T, T)
    where
        T: Copy + Add<Output = T> + Sub<Output = T> + Mul<Output = T>,
    {
        let q1 = (self.p2.x - self.p3.x) * (self.p2.x - self.p3.x)
            + (self.p2.y - self.p3.y) * (self.p2.y - self.p3.y);
        let q2 = (self.p1.x - self.p3.x) * (self.p1.x - self.p3.x)
            + (self.p1.y - self.p3.y) * (self.p1.y - self.p3.y);
        let q3 = (self.p1.x - self.p2.x) * (self.p1.x - self.p2.x)
            + (self.p1.y - self.p2.y) * (self.p1.y - self.p2.y);
        (q1, q2, q3)
    }

    /// Calculate the area using Archimedes' formula
    pub fn area(&self) -> T
    where
        T: Copy + Add<Output = T> + Sub<Output = T> + Mul<Output = T> + One + Zero,
    {
        let (q1, q2, q3) = self.quadrances();
        let temp = q1 + q2 - q3;
        let four = T::one() + T::one() + T::one() + T::one();
        four * q1 * q2 - temp * temp
    }

    /// Calculate the twist (twice the signed area) of the triangle
    pub fn twist(&self) -> T
    where
        T: Copy + Sub<Output = T> + Mul<Output = T>,
    {
        (self.p2.x - self.p1.x) * (self.p3.y - self.p1.y)
            - (self.p2.y - self.p1.y) * (self.p3.x - self.p1.x)
    }

    /// Check if the triangle is degenerate (all points collinear)
    pub fn is_degenerate(&self) -> bool
    where
        T: Copy + Sub<Output = T> + Mul<Output = T> + Zero + PartialEq,
    {
        self.twist() == T::zero()
    }
}

/// A 3D triangle defined by three points
#[derive(Debug, Clone, Copy)]
pub struct Triangle3D<T> {
    pub p1: Point3D<T>,
    pub p2: Point3D<T>,
    pub p3: Point3D<T>,
}

impl<T> Triangle3D<T> {
    pub fn new(p1: Point3D<T>, p2: Point3D<T>, p3: Point3D<T>) -> Self {
        Self { p1, p2, p3 }
    }

    /// Calculate the quadrances of the triangle sides
    pub fn quadrances(&self) -> (T, T, T)
    where
        T: Copy + Add<Output = T> + Sub<Output = T> + Mul<Output = T>,
    {
        let q1 = (self.p2.x - self.p3.x) * (self.p2.x - self.p3.x)
            + (self.p2.y - self.p3.y) * (self.p2.y - self.p3.y)
            + (self.p2.z - self.p3.z) * (self.p2.z - self.p3.z);
        let q2 = (self.p1.x - self.p3.x) * (self.p1.x - self.p3.x)
            + (self.p1.y - self.p3.y) * (self.p1.y - self.p3.y)
            + (self.p1.z - self.p3.z) * (self.p1.z - self.p3.z);
        let q3 = (self.p1.x - self.p2.x) * (self.p1.x - self.p2.x)
            + (self.p1.y - self.p2.y) * (self.p1.y - self.p2.y)
            + (self.p1.z - self.p2.z) * (self.p1.z - self.p2.z);
        (q1, q2, q3)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_point2d_from_tuple() {
        let p: Point2D<i32> = (1, 2).into();
        assert_eq!(p.x, 1);
        assert_eq!(p.y, 2);
    }

    #[test]
    fn test_vector2d_add() {
        let v1 = Vector2D::new(1, 2);
        let v2 = Vector2D::new(3, 4);
        let result = v1 + v2;
        assert_eq!(result.x, 4);
        assert_eq!(result.y, 6);
    }

    #[test]
    fn test_vector2d_sub() {
        let v1 = Vector2D::new(3, 4);
        let v2 = Vector2D::new(1, 2);
        let result = v1 - v2;
        assert_eq!(result.x, 2);
        assert_eq!(result.y, 2);
    }

    #[test]
    fn test_triangle2d_quadrances() {
        let p1 = Point2D::new(0, 0);
        let p2 = Point2D::new(1, 0);
        let p3 = Point2D::new(0, 1);
        let triangle = Triangle2D::new(p1, p2, p3);
        let (q1, q2, q3) = triangle.quadrances();
        assert_eq!(q1, 2);
        assert_eq!(q2, 1);
        assert_eq!(q3, 1);
    }

    #[test]
    fn test_triangle2d_area() {
        let p1 = Point2D::new(0, 0);
        let p2 = Point2D::new(1, 0);
        let p3 = Point2D::new(0, 1);
        let triangle = Triangle2D::new(p1, p2, p3);
        let area = triangle.area();
        assert_eq!(area, 4);
    }

    #[test]
    fn test_triangle2d_twist() {
        let p1 = Point2D::new(0, 0);
        let p2 = Point2D::new(1, 0);
        let p3 = Point2D::new(0, 1);
        let triangle = Triangle2D::new(p1, p2, p3);
        assert_eq!(triangle.twist(), 1);
    }

    #[test]
    fn test_triangle2d_is_degenerate() {
        let p1 = Point2D::new(0, 0);
        let p2 = Point2D::new(1, 1);
        let p3 = Point2D::new(2, 2);
        let triangle = Triangle2D::new(p1, p2, p3);
        assert!(triangle.is_degenerate());
    }

    #[test]
    fn test_triangle2d_not_degenerate() {
        let p1 = Point2D::new(0, 0);
        let p2 = Point2D::new(1, 0);
        let p3 = Point2D::new(0, 1);
        let triangle = Triangle2D::new(p1, p2, p3);
        assert!(!triangle.is_degenerate());
    }

    #[test]
    fn test_triangle3d_quadrances() {
        let p1 = Point3D::new(0, 0, 0);
        let p2 = Point3D::new(1, 0, 0);
        let p3 = Point3D::new(0, 1, 0);
        let triangle = Triangle3D::new(p1, p2, p3);
        let (q1, q2, q3) = triangle.quadrances();
        assert_eq!(q1, 2);
        assert_eq!(q2, 1);
        assert_eq!(q3, 1);
    }
}
