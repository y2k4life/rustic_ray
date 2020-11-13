use crate::{float_eq, Point, Vector};
use std::ops::{Index, Mul};

#[derive(Debug, Clone, Copy)]
pub struct Matrix {
    data: [[f64; 4]; 4],
    inverse: [[f64; 4]; 4],
}

pub const IDENTITY: Matrix = Matrix {
    data: [
        [1.0, 0.0, 0.0, 0.0],
        [0.0, 1.0, 0.0, 0.0],
        [0.0, 0.0, 1.0, 0.0],
        [0.0, 0.0, 0.0, 1.0],
    ],
    inverse: [
        [1.0, 0.0, 0.0, 0.0],
        [0.0, 1.0, 0.0, 0.0],
        [0.0, 0.0, 1.0, 0.0],
        [0.0, 0.0, 0.0, 1.0],
    ],
};

impl Matrix {
    pub fn new(data: [[f64; 4]; 4]) -> Self {
        let mut i = [[0.; 4]; 4];
        let d = Matrix::determinant(data, 4);
        for row in 0..4 {
            for col in 0..4 {
                i[col][row] = Matrix::cofactor(data, row, col, 3) / d;
            }
        }
        
        Self { 
            data, 
            inverse: i, 
        }
    }

    pub fn transpose(&self) -> Self {
        Matrix {
            data: [ 
                [self.data[0][0], self.data[1][0], self.data[2][0], self.data[3][0]],
                [self.data[0][1], self.data[1][1], self.data[2][1], self.data[3][1]],
                [self.data[0][2], self.data[1][2], self.data[2][2], self.data[3][2]],
                [self.data[0][3], self.data[1][3], self.data[2][3], self.data[3][3]]],
            inverse: [
                [self.inverse[0][0], self.inverse[1][0], self.inverse[2][0], self.inverse[3][0]],
                [self.inverse[0][1], self.inverse[1][1], self.inverse[2][1], self.inverse[3][1]],
                [self.inverse[0][2], self.inverse[1][2], self.inverse[2][2], self.inverse[3][2]],
                [self.inverse[0][3], self.inverse[1][3], self.inverse[2][3], self.inverse[3][3]]],
        }
    }

    pub fn inverse(&self) -> Matrix {
        Matrix {
            data: self.inverse,
            inverse: self.data,
        }
    }

    pub fn is_invertible(m: [[f64; 4]; 4]) -> bool {
        let mut invertible = true;
        if Matrix::determinant(m, 4) == 0.0 {
            invertible = false
        }
        invertible
    }

    fn determinant(a: [[f64; 4]; 4], s: usize) -> f64 {
        let mut det = 0.;

        if s == 2 {
            det = a[0][0] * a[1][1] - a[0][1] * a[1][0];
        } else {
            for col in 0..4 {
                det += a[0][col] * Matrix::cofactor(a, 0, col, s - 1);
            }
        }

        det
    }

    fn sub_matrix(a: [[f64; 4]; 4], r_row: usize, r_col: usize) -> [[f64; 4]; 4] {
        let mut m = [[0.; 4]; 4];

        for (nri, ri) in [0, 1, 2, 3].iter().filter(|&&x| x != r_row).enumerate() {
            for (nci, ci) in [0, 1, 2, 3].iter().filter(|&&x| x != r_col).enumerate() {
                m[nri][nci] = a[*ri][*ci];
            }
        }
        
        m
    }

    fn minor(a: [[f64; 4]; 4], r_row: usize, r_col: usize, s: usize) -> f64 {
        Matrix::determinant(Matrix::sub_matrix(a, r_row, r_col), s)
    }

    pub fn cofactor(a: [[f64; 4]; 4], r_row: usize, r_col: usize, s: usize) -> f64 {
        let mut minor = Matrix::minor(a, r_row, r_col, s);
        if (r_row + r_col) % 2 == 1 {
            minor *= -1.0
        }
        minor
    }

    pub fn multiple(a: [[f64; 4]; 4], b: [[f64; 4]; 4]) -> [[f64; 4]; 4] {
        let mut results = [[0.0; 4]; 4];

        for row in 0..4 {
            for col in 0..4 {
                results[row][col] = a[row][0] * b[0][col]
                    + a[row][1] * b[1][col]
                    + a[row][2] * b[2][col]
                    + a[row][3] * b[3][col];
            }
        }
        results
    }
}

impl Mul for Matrix {
    type Output = Self;

    fn mul(self, other: Matrix) -> Self {
        Matrix::new(Matrix::multiple(self.data, other.data))
    }
}

impl Mul<Point> for Matrix {
    type Output = Point;

    fn mul(self, other: Point) -> Point {
        let c1 = (self[0][0] * other.x)
            + (self[0][1] * other.y)
            + (self[0][2] * other.z)
            + (self[0][3] * 1.0);
        let c2 = (self[1][0] * other.x)
            + (self[1][1] * other.y)
            + (self[1][2] * other.z)
            + (self[1][3] * 1.0);
        let c3 = (self[2][0] * other.x)
            + (self[2][1] * other.y)
            + (self[2][2] * other.z)
            + (self[2][3] * 1.0);

        Point::new(c1, c2, c3)
    }
}

impl Mul<Vector> for Matrix {
    type Output = Vector;

    fn mul(self, other: Vector) -> Vector {
        let c1 = (self[0][0] * other.x)
            + (self[0][1] * other.y)
            + (self[0][2] * other.z)
            + (self[0][3] * 0.0);
        let c2 = (self[1][0] * other.x)
            + (self[1][1] * other.y)
            + (self[1][2] * other.z)
            + (self[1][3] * 0.0);
        let c3 = (self[2][0] * other.x)
            + (self[2][1] * other.y)
            + (self[2][2] * other.z)
            + (self[2][3] * 0.0);

        Vector::new(c1, c2, c3)
    }
}

impl Index<usize> for Matrix {
    type Output = [f64; 4];

    fn index(&self, i: usize) -> &Self::Output {
        &self.data[i]
    }
}

impl PartialEq for Matrix {
    fn eq(&self, other: &Matrix) -> bool {
        let mut equal = true;
        for r in 0..4 {
            for c in 0..4 {
                if !float_eq(self[r][c], other[r][c]) {
                    equal = false;
                    break;
                }
            }
            if !equal {
                break;
            }
        }
        equal
    }
}

#[cfg(test)]
mod tests {
    use crate::float_eq;
    use super::{Matrix, IDENTITY};

    #[test]
    fn determinant() {
        let m = [
            [1.0, 5.0, 0.0, 0.],
            [-3.0, 2.0, 0.0, 0.],
            [0.0, 0.0, 0.0, 0.],
            [0.0, 0.0, 0.0, 0.],
        ];
        let d = Matrix::determinant(m, 2);
        assert_eq!(17.0, d);
    }

    #[test]
    fn matrix_equal() {
        let m1 = Matrix::new([
            [1.0, 2.0, 3.0, 4.0],
            [5.0, 6.0, 7.0, 8.0],
            [9.0, 8.0, 7.0, 6.0],
            [5.0, 4.0, 3.0, 2.0],
        ]);
        let m2 = Matrix::new([
            [1.0, 2.0, 3.0, 4.0],
            [5.0, 6.0, 7.0, 8.0],
            [9.0, 8.0, 7.0, 6.0],
            [5.0, 4.0, 3.0, 2.0],
        ]);
        assert_eq!(m1, m2);
    }

    #[test]
    fn matrix_not_equal() {
        let m1 = Matrix::new([
            [1.0, 2.0, 3.0, 4.0],
            [5.0, 6.0, 7.0, 8.0],
            [9.0, 8.0, 7.0, 6.0],
            [5.0, 4.0, 3.0, 2.0],
        ]);
        let m2 = Matrix::new([
            [2.0, 3.0, 4.0, 5.0],
            [6.0, 7.0, 8.0, 9.0],
            [8.0, 7.0, 6.0, 5.0],
            [4.0, 3.0, 2.0, 1.0],
        ]);
        assert_ne!(m1, m2);
    }

    #[test]
    fn multiple_two_matrix() {
        let m1 = Matrix::new([
            [1.0, 2.0, 3.0, 4.0],
            [5.0, 6.0, 7.0, 8.0],
            [9.0, 8.0, 7.0, 6.0],
            [5.0, 4.0, 3.0, 2.0],
        ]);
        let m2 = Matrix::new([
            [-2.0, 1.0, 2.0, 3.0],
            [3.0, 2.0, 1.0, -1.0],
            [4.0, 3.0, 6.0, 5.0],
            [1.0, 2.0, 7.0, 8.0],
        ]);
        let actual = m1 * m2;
        let epected = Matrix::new([
            [20.0, 22.0, 50.0, 48.0],
            [44.0, 54.0, 114.0, 108.0],
            [40.0, 58.0, 110.0, 102.0],
            [16.0, 26.0, 46.0, 42.0],
        ]);
        assert_eq!(epected, actual);
    }

    #[test]
    fn mutliple_by_identity() {
        let m1 = Matrix::new([
            [0.0, 1.0, 2.0, 4.0],
            [1.0, 2.0, 4.0, 8.0],
            [2.0, 4.0, 8.0, 16.0],
            [4.0, 8.0, 16.0, 32.0],
        ]);
        let m2 = IDENTITY;
        let actual = m1 * m2;
        assert_eq!(m1, actual);
    }

    #[test]
    fn transpose_matrix() {
        let m1 = Matrix::new([
            [0.0, 9.0, 3.0, 0.0],
            [9.0, 8.0, 0.0, 8.0],
            [1.0, 8.0, 5.0, 3.0],
            [0.0, 0.0, 5.0, 8.0],
        ]);
        let actual = m1.transpose();
        let expected = Matrix::new([
            [0.0, 9.0, 1.0, 0.0],
            [9.0, 8.0, 8.0, 0.0],
            [3.0, 0.0, 5.0, 5.0],
            [0.0, 8.0, 3.0, 8.0],
        ]);
        assert_eq!(expected, actual);
    }

    #[test]
    fn transpose_identity() {
        let m1 = IDENTITY;
        let actual = m1.transpose();
        assert_eq!(m1, actual);
    }

    #[test]
    fn sub_matrix() {
        let m = [
            [1.0, 5.0, 0.0, 0.],
            [-3.0, 2.0, 7.0, 0.],
            [0.0, 6.0, -3.0, 0.],
            [0.0, 0.0, 0.0, 0.],
        ];
        let actual = Matrix::sub_matrix(m, 0, 2);
        let e = [
            [-3.0, 2.0, 0.0, 0.],
            [0.0, 6.0, 0.0, 0.],
            [0.0, 0.0, 0.0, 0.],
            [0.0, 0.0, 0.0, 0.],
        ];
        assert_eq!(e, actual);
    }

    #[test]
    fn sub_matrix_of_4x4_is_3x3() {
        let m = [
            [-6.0, 1.0, 1.0, 6.0],
            [-8.0, 5.0, 8.0, 6.0],
            [-1.0, 0.0, 8.0, 2.0],
            [-7.0, 1.0, -1.0, 1.0],
        ];
        let actual = Matrix::sub_matrix(m, 2, 1);
        let expected = [
            [-6.0, 1.0, 6.0, 0.],
            [-8.0, 8.0, 6.0, 0.],
            [-7.0, -1.0, 1.0, 0.],
            [0.0, 0.0, 0.0, 0.],
        ];
        assert_eq!(expected, actual);
    }

    #[test]
    fn minor_3x3() {
        let a = [
            [3.0, 5.0, 0.0, 0.],
            [2.0, -1.0, -7.0, 0.],
            [6.0, -1.0, 5.0, 0.],
            [0.0, -0.0, 0.0, 0.],
        ];
        let b = Matrix::sub_matrix(a, 1, 0);
        let actual_det = Matrix::determinant(b, 2);
        assert_eq!(25.0, actual_det);
        let actual_minor = Matrix::minor(a, 1, 0, 2);
        assert_eq!(25.0, actual_minor);
    }

    #[test]
    fn cofactor_3x3() {
        let a = [
            [3.0, 5.0, 0.0, 0.],
            [2.0, -1.0, -7.0, 0.],
            [6.0, -1.0, 5.0, 0.],
            [0.0, 0.0, 0.0, 0.],
        ];
        let m1 = Matrix::minor(a, 0, 0, 2);
        assert_eq!(-12.0, m1);
        let c1 = Matrix::cofactor(a, 0, 0, 2);
        assert_eq!(-12.0, c1);
        let m2 = Matrix::minor(a, 1, 0, 2);
        assert_eq!(25.0, m2);
        let c2 = Matrix::cofactor(a, 1, 0, 2);
        assert_eq!(-25.0, c2);
    }

    #[test]
    fn determinant_3x3() {
        let a = [
            [1.0, 2.0, 6.0, 0.],
            [-5.0, 8.0, -4.0, 0.],
            [2.0, 6.0, 4.0, 0.],
            [0.0, 0.0, 0.0, 0.],
        ];
        assert_eq!(56.0, Matrix::cofactor(a, 0, 0, 2));
        assert_eq!(12.0, Matrix::cofactor(a, 0, 1, 2));
        assert_eq!(-46.0, Matrix::cofactor(a, 0, 2, 2));
        assert_eq!(-196.0, Matrix::determinant(a, 3));
    }

    #[test]
    fn determinant_4x4() {
        let a = [
            [-2.0, -8.0, 3.0, 5.],
            [-3.0, 1.0, 7.0, 3.],
            [1.0, 2.0, -9.0, 6.],
            [-6.0, 7.0, 7.0, -9.],
        ];
        assert_eq!(690.0, Matrix::cofactor(a, 0, 0, 3));
        assert_eq!(447.0, Matrix::cofactor(a, 0, 1, 3));
        assert_eq!(210.0, Matrix::cofactor(a, 0, 2, 3));
        assert_eq!(51.0, Matrix::cofactor(a, 0, 3, 3));
        assert_eq!(-4071.0, Matrix::determinant(a, 4));
    }

    #[test]
    fn invertible_true() {
        let a = [
            [6.0, 4.0, 4.0, 4.0],
            [5.0, 5.0, 7.0, 6.0],
            [4.0, -9.0, 3.0, -7.0],
            [9.0, 1.0, 7.0, -6.0],
        ];
        assert_eq!(-2120.0, Matrix::determinant(a, 4));
        assert_eq!(true, Matrix::is_invertible(a))
    }

    #[test]
    fn invertible_false() {
        let a = [
            [-4.0, 2.0, -2.0, -3.0],
            [9.0, 6.0, 2.0, 6.0],
            [0.0, -5.0, 1.0, -5.0],
            [0.0, 0.0, 0.0, 0.0],
        ];
        assert_eq!(0.0, Matrix::determinant(a, 4));
        assert_eq!(false, Matrix::is_invertible(a))
    }

    #[test]
    fn inverse_matrix() {
        let m = [
            [-5.0, 2.0, 6.0, -8.0],
            [1.0, -5.0, 1.0, 8.0],
            [7.0, 7.0, -6.0, -7.0],
            [1.0, -3.0, 7.0, 4.0],
        ];
        let a = Matrix::new(m);
        let b = a.inverse();
        assert_eq!(532.0, Matrix::determinant(m, 4));
        assert_eq!(-160.0 / 532.0, b[3][2]);
        let expected = Matrix::new([
            [0.21805, 0.45113, 0.24060, -0.04511],
            [-0.80827, -1.45677, -0.44361, 0.52068],
            [-0.07895, -0.22368, -0.05263, 0.19737],
            [-0.52256, -0.81391, -0.30075, 0.30639],
        ]);
        for row in 0..4 {
            for col in 0..4 {
                let are_equal = float_eq(b.data[row][col], expected.data[row][col]);
                assert_eq!(true, are_equal);
            }
        }
    }

    #[test]
    fn test_inverse_matrix_2() {
        let m = [
            [8.0, -5.0, 9.0, 2.0],
            [7.0, 5.0, 6.0, 1.0],
            [-6.0, 0.0, 9.0, 6.0],
            [-3.0, 0.0, -9.0, -4.0],
        ];
        let a = Matrix::new(m);
        let b = a.inverse();
        let expected = Matrix::new([
            [-0.15385, -0.15385, -0.28205, -0.53846],
            [-0.07692, 0.12308, 0.02564, 0.03077],
            [0.35897, 0.35897, 0.43590, 0.92308],
            [-0.69231, -0.69231, -0.76923, -1.92308],
        ]);
        for row in 0..4 {
            for col in 0..4 {
                assert_eq!(true, float_eq(b.data[row][col], expected.data[row][col]));
            }
        }
    }

    #[test]
    fn test_multiple_inverse() {
        let a = Matrix::new([
            [3.0, -9.0, 7.0, 3.0],
            [3.0, -8.0, 2.0, -9.0],
            [-4.0, 4.0, 4.0, 1.0],
            [-6.0, 5.0, -1.0, 1.0],
        ]);
        let b = Matrix::new([
            [8.0, 2.0, 2.0, 2.0],
            [3.0, -1.0, 7.0, 0.0],
            [7.0, 0.0, 5.0, 4.0],
            [6.0, -2.0, 0.0, 5.0],
        ]);
        let ib = b.inverse();
        let c = a * b;
        let actual = c * ib;
        for row in 0..4 {
            for col in 0..4 {
                assert_eq!(true, float_eq(actual.data[row][col], a.data[row][col]));
            }
        }
    }

    #[test]
    fn determinant_should_be() {
        let a = [
            [-2.0, -8.0, 3.0, 5.0],
            [-3.0, 1.0, 7.0, 3.0],
            [1.0, 2.0, -9.0, 6.0],
            [-6.0, 7.0, 7.0, -9.0],
        ];
        assert_eq!(-4071.0, Matrix::determinant(a, 4));
    }

    #[test]
    fn transpose_inverse() {
        let a = Matrix::new([
            [8.0, -5.0, 9.0, 2.0],
            [7.0, 5.0, 6.0, 1.0],
            [-6.0, 0.0, 9.0, 6.0],
            [-3.0, 0.0, -9.0, -4.0],
        ]);
        let b = a.inverse();
        let expected = Matrix::new([
            [-0.15385, -0.15385, -0.28205, -0.53846],
            [-0.07692, 0.12308, 0.02564, 0.03077],
            [0.35897, 0.35897, 0.43590, 0.92308],
            [-0.69231, -0.69231, -0.76923, -1.92308],
        ]);

        for row in 0..4 {
            for col in 0..4 {
                assert_eq!(true, float_eq(b.data[row][col], expected.data[row][col]));
            }
        }

        let t = a.transpose();
        
        let te = [
            [-0.15385, -0.07692, 0.35897, -0.69231],
            [-0.15385, 0.12308, 0.35897, -0.69231],
            [-0.28205, 0.02564, 0.43590, -0.76923],
            [-0.53846, 0.03077, 0.92308, -1.92308]];

        for row in 0..4 {
            for col in 0..4 {
                assert_eq!(true, float_eq(t.inverse[row][col], te[row][col]));
            }
        }
        
    }
}
