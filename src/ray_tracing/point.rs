use crate::float_eq;
use crate::Vector;
use std::ops::{Add, Div, Mul, Neg, Sub};

#[derive(Debug, Copy, Clone)]
pub struct Point {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl Point {
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Point { x, y, z }
    }
}

impl Sub for Point {
    type Output = Vector;

    fn sub(self, other: Point) -> Vector {
        Vector {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        }
    }
}

impl Add for Point {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }
}

impl Sub<Vector> for Point {
    type Output = Self;

    fn sub(self, other: Vector) -> Self {
        Self {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        }
    }
}

impl Add<Vector> for Point {
    type Output = Self;

    fn add(self, other: Vector) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }
}

impl Mul<f64> for Point {
    type Output = Self;

    fn mul(self, other: f64) -> Self {
        Self {
            x: self.x * other,
            y: self.y * other,
            z: self.z * other,
        }
    }
}

impl Mul<Point> for f64 {
    type Output = Point;

    fn mul(self, other: Point) -> Point {
        Point {
            x: other.x * self,
            y: other.y * self,
            z: other.z * self,
        }
    }
}

impl Div<f64> for Point {
    type Output = Point;

    fn div(self, other: f64) -> Point {
        Point {
            x: self.x / other,
            y: self.y / other,
            z: self.z / other,
        }
    }
}

impl Neg for Point {
    type Output = Self;

    fn neg(self) -> Self {
        Self {
            x: -self.x,
            y: -self.y,
            z: -self.z,
        }
    }
}

impl PartialEq for Point {
    fn eq(&self, other: &Point) -> bool {
        float_eq(self.x, other.x) && float_eq(self.y, other.y) && float_eq(self.z, other.z)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Vector;

    // Chapter 1 Tuples, Points, and Vectors
    // page 4
    #[test]
    fn create_point() {
        let p = Point::new(1.0, 2.0, 3.0);

        assert_eq!(p.x, 1.0);
        assert_eq!(p.y, 2.0);
        assert_eq!(p.z, 3.0);
    }

    // Chapter 1 Tuples, Points, and Vectors
    // page 6
    #[test]
    fn add_two_points() {
        let p1 = Point::new(3.0, -2.0, 5.0);
        let p2 = Point::new(-2.0, 3.0, 1.0);

        assert_eq!(p1 + p2, Point::new(1.0, 1.0, 6.0));
    }

    // Chapter 1 Tuples, Points, and Vectors
    // page 6
    #[test]
    fn subtracting_two_points() {
        let p1 = Point::new(3.0, 2.0, 1.0);
        let p2 = Point::new(5.0, 6.0, 7.0);

        assert_eq!(p1 - p2, Vector::new(-2.0, -4.0, -6.0));
    }

    // Chapter 1 Tuples, Points, and Vectors
    // page 6
    #[test]
    fn subtracting_a_vector_from_point() {
        let p = Point::new(3.0, 2.0, 1.0);
        let v = Vector::new(5.0, 6.0, 7.0);

        assert_eq!(p - v, Point::new(-2.0, -4.0, -6.0));
    }

    // Chapter 1 Tuples, Points, and Vectors
    // page 7
    #[test]
    fn negating_point() {
        let p = Point::new(1.0, -2.0, 3.0);

        assert_eq!(-p, Point::new(-1.0, 2.0, -3.0));
    }

    // Chapter 1 Tuples, Points, and Vectors
    // page 8
    #[test]
    fn multiplying_point_by_scalar() {
        let a = Point::new(1.0, -2.0, 3.0);

        assert_eq!(a * 3.5, Point::new(3.5, -7.0, 10.5));
    }

    // Chapter 1 Tuples, Points, and Vectors
    // page 8
    #[test]
    fn multiplying_a_point_by_a_fraction() {
        let a = Point::new(1.0, -2.0, 3.0);

        assert_eq!(a * 0.5, Point::new(0.5, -1.0, 1.5));
    }

    // Chapter 1 Tuples, Points, and Vectors
    // page 8
    #[test]
    fn dividing_a_point_by_a_scalar() {
        let a = Point::new(1.0, -2.0, 3.0);

        assert_eq!(a / 2.0, Point::new(0.5, -1.0, 1.5));
    }
}
