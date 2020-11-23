use crate::float_eq;
use std::ops::{Add, Div, Mul, Neg, Sub};

#[derive(Debug, Copy, Clone)]
pub struct Vector {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl Vector {
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Vector { x, y, z }
    }

    pub fn magnitude(self) -> f64 {
        (self.x.powf(2.0) + self.y.powf(2.0) + self.z.powf(2.0)).sqrt()
    }

    pub fn normalize(self) -> Self {
        let mag = self.magnitude();
        Self {
            x: self.x / mag,
            y: self.y / mag,
            z: self.z / mag,
        }
    }

    pub fn dot(self, b: Vector) -> f64 {
        self.x * b.x + self.y * b.y + self.z * b.z
    }

    // a: Vector,
    pub fn cross(self, b: Vector) -> Vector {
        Vector {
            x: self.y * b.z - self.z * b.y,
            y: self.z * b.x - self.x * b.z,
            z: self.x * b.y - self.y * b.x,
        }
    }

    pub fn reflect(self, normal: Vector) -> Vector {
        self - normal * 2.0 * self.dot(normal)
    }
}

impl Sub for Vector {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Self {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        }
    }
}

impl Add for Vector {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }
}

impl Mul<f64> for Vector {
    type Output = Self;

    fn mul(self, other: f64) -> Self {
        Self {
            x: self.x * other,
            y: self.y * other,
            z: self.z * other,
        }
    }
}

impl Mul<Vector> for f64 {
    type Output = Vector;

    fn mul(self, other: Vector) -> Vector {
        Vector {
            x: other.x * self,
            y: other.y * self,
            z: other.z * self,
        }
    }
}

impl Div<f64> for Vector {
    type Output = Self;

    fn div(self, other: f64) -> Self {
        Self {
            x: self.x / other,
            y: self.y / other,
            z: self.z / other,
        }
    }
}

impl Neg for Vector {
    type Output = Self;

    fn neg(self) -> Self {
        Self {
            x: -self.x,
            y: -self.y,
            z: -self.z,
        }
    }
}

impl PartialEq for Vector {
    fn eq(&self, other: &Vector) -> bool {
        float_eq(self.x, other.x) && float_eq(self.y, other.y) && float_eq(self.z, other.z)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Chapter 1 Tuples, Points, and Vectors
    // page 4
    #[test]
    fn create_vector() {
        let p = Vector::new(1.0, 2.0, 3.0);
        assert_eq!(1.0, p.x);
        assert_eq!(2.0, p.y);
        assert_eq!(3.0, p.z);
    }

    // Chapter 1 Tuples, Points, and Vectors
    // page 6
    #[test]
    fn add_two_vectors() {
        let v1 = Vector::new(3.0, -2.0, 5.0);
        let v2 = Vector::new(-2.0, 3.0, 1.0);

        assert_eq!(v1 + v2, Vector::new(1.0, 1.0, 6.0));
    }

    // Chapter 1 Tuples, Points, and Vectors
    // page 7
    #[test]
    fn subtracting_two_vectors() {
        let v1 = Vector::new(3.0, 2.0, 1.0);
        let v2 = Vector::new(5.0, 6.0, 7.0);

        assert_eq!(v1 - v2, Vector::new(-2.0, -4.0, -6.0));
    }

    // Chapter 1 Tuples, Points, and Vectors
    // page 7
    #[test]
    fn subtracting_a_vector_from_the_zero_vector() {
        let zero = Vector::new(0.0, 0.0, 0.0);
        let v = Vector::new(1.0, -2.0, 3.0);

        assert_eq!(zero - v, Vector::new(-1.0, 2.0, -3.0));
    }

    // Chapter 1 Tuples, Points, and Vectors
    // page 7
    #[test]
    fn negating_vector() {
        let v = Vector::new(1.0, -2.0, 3.0);

        assert_eq!(-v, Vector::new(-1.0, 2.0, -3.0));
    }

    // Chapter 1 Tuples, Points, and Vectors
    // page 8
    #[test]
    fn multiplying_a_vector_by_a_scalar() {
        let v = Vector::new(1.0, -2.0, 3.0);

        assert_eq!(3.5 * v, Vector::new(3.5, -7.0, 10.5));
    }

    // Chapter 1 Tuples, Points, and Vectors
    // page 8
    #[test]
    fn multiplying_vector_by_faction() {
        let v = Vector::new(1.0, -2.0, 3.0);

        assert_eq!(0.5 * v, Vector::new(0.5, -1.0, 1.5));
    }

    // Chapter 1 Tuples, Points, and Vectors
    // page 8
    #[test]
    fn divide_vector_by_float() {
        let v = Vector::new(1.0, -2.0, 3.0);

        assert_eq!(v / 2.0, Vector::new(0.5, -1.0, 1.5));
    }

    #[test]
    fn compute_magnitude_for_vector_1_0_0() {
        let v = Vector::new(1.0, 0.0, 0.0);
        let results = v.magnitude();
        let expected = 1.0;
        assert_eq!(expected, results);
    }

    #[test]
    fn compute_magnitude_for_vector_0_1_0() {
        let v = Vector::new(0.0, 1.0, 0.0);
        let results = v.magnitude();
        let expected = 1.0;
        assert_eq!(expected, results);
    }

    #[test]
    fn compute_magnitude_for_vector_0_0_1() {
        let v = Vector::new(0.0, 0.0, 1.0);
        let results = v.magnitude();
        let expected = 1.0;
        assert_eq!(expected, results);
    }

    #[test]
    fn compute_magnitude_for_vector_1_2_3() {
        let v = Vector::new(1.0, 2.0, 3.0);
        let results = v.magnitude();
        let expected: f64 = 14.0;
        assert_eq!(expected.sqrt(), results);
    }

    #[test]
    fn compute_magnitude_for_vector_neg_1_2_3() {
        let v = Vector::new(-1.0, -2.0, -3.0);
        let results = v.magnitude();
        let expected: f64 = 14.0;
        assert_eq!(expected.sqrt(), results);
    }

    #[test]
    fn normalizing_vector_4_0_0() {
        let v = Vector::new(4.0, 0.0, 0.0);
        let results = v.normalize();
        let expected = Vector::new(1.0, 0.0, 0.0);
        assert_eq!(expected, results);
    }

    #[test]
    fn normalizing_vector_1_2_3() {
        let v = Vector::new(1.0, 2.0, 3.0);
        let results = v.normalize();
        let expected = Vector::new(0.26726, 0.53452, 0.80178);
        assert_eq!(expected, results);
    }

    #[test]
    fn normalizing_vector_1_2_3_normalize() {
        let v = Vector::new(1.0, 2.0, 3.0);
        let norm = v.normalize();
        let results = norm.magnitude();
        assert_eq!(1.0, results);
    }

    #[test]
    fn dot_product() {
        let v1 = Vector::new(1.0, 2.0, 3.0);
        let v2 = Vector::new(2.0, 3.0, 4.0);
        let results = v1.dot(v2);
        assert_eq!(20.0, results);
    }

    #[test]
    fn cross_product() {
        let v1 = Vector::new(1.0, 2.0, 3.0);
        let v2 = Vector::new(2.0, 3.0, 4.0);
        let results1 = v1.cross(v2);
        let expected1 = Vector::new(-1.0, 2.0, -1.0);
        assert_eq!(expected1, results1);
        let results2 = v2.cross(v1);
        let expected2 = Vector::new(1.0, -2.0, 1.0);
        assert_eq!(expected2, results2);
    }

    #[test]
    fn reflecting_a_vector_approaching_45() {
        let v = Vector::new(1.0, -1.0, 0.0);
        let n = Vector::new(0.0, 1.0, 0.0);
        let r = v.reflect(n);
        assert_eq!(Vector::new(1.0, 1.0, 0.0), r);
    }

    #[test]
    fn reflecting_a_vector_slant() {
        let v = Vector::new(0.0, -1.0, 0.0);
        let n = Vector::new(2_f64.sqrt() / 2.0, 2_f64.sqrt() / 2.0, 0.0);
        let r = v.reflect(n);
        assert_eq!(Vector::new(1.0, 0.0, 0.0), r);
    }
}
