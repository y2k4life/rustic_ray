use crate::{Matrix, Point, Vector};

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Transform {
    data: [[f64; 4]; 4],
}

impl Transform {
    pub fn new() -> Transform {
        Transform {
            data: [
                [1.0, 0.0, 0.0, 0.0],
                [0.0, 1.0, 0.0, 0.0],
                [0.0, 0.0, 1.0, 0.0],
                [0.0, 0.0, 0.0, 1.0],
            ],
        }
    }

    pub fn translation(&self, x: f64, y: f64, z: f64) -> Transform {
        let m = [
            [1.0, 0.0, 0.0, x],
            [0.0, 1.0, 0.0, y],
            [0.0, 0.0, 1.0, z],
            [0.0, 0.0, 0.0, 1.0],
        ];
        Transform {
            data: Matrix::multiple(m, self.data),
        }
    }

    pub fn scaling(&self, x: f64, y: f64, z: f64) -> Transform {
        let m = [
            [x, 0.0, 0.0, 0.0],
            [0.0, y, 0.0, 0.0],
            [0.0, 0.0, z, 0.0],
            [0.0, 0.0, 0.0, 1.0],
        ];
        Transform {
            data: Matrix::multiple(m, self.data),
        }
    }

    pub fn rotation_x(&self, r: f64) -> Transform {
        let m = [
            [1.0, 0.0, 0.0, 0.0],
            [0.0, r.cos(), -r.sin(), 0.0],
            [0.0, r.sin(), r.cos(), 0.0],
            [0.0, 0.0, 0.0, 1.0],
        ];
        Transform {
            data: Matrix::multiple(m, self.data),
        }
    }

    pub fn rotation_y(&self, r: f64) -> Transform {
        let m = [
            [r.cos(), 0.0, r.sin(), 0.0],
            [0.0, 1.0, 0.0, 0.0],
            [-(r.sin()), 0.0, r.cos(), 0.0],
            [0.0, 0.0, 0.0, 1.0],
        ];
        Transform {
            data: Matrix::multiple(m, self.data),
        }
    }

    pub fn rotation_z(&self, r: f64) -> Transform {
        let m = [
            [r.cos(), -(r.sin()), 0.0, 0.0],
            [r.sin(), r.cos(), 0.0, 0.0],
            [0.0, 0.0, 1.0, 0.0],
            [0.0, 0.0, 0.0, 1.0],
        ];
        Transform {
            data: Matrix::multiple(m, self.data),
        }
    }

    pub fn shearing(self, xy: f64, xz: f64, yx: f64, yz: f64, zx: f64, zy: f64) -> Transform {
        let m = [
            [1.0, xy, xz, 0.0],
            [yx, 1.0, yz, 0.0],
            [zx, zy, 1.0, 0.0],
            [0.0, 0.0, 0.0, 1.0],
        ];
        Transform {
            data: Matrix::multiple(m, self.data),
        }
    }

    pub fn view_transformation(from: Point, to: Point, up: Vector) -> Matrix {
        let forward = (to - from).normalize();
        let upn = up.normalize();
        let left = forward.cross(upn);
        let true_up = left.cross(forward);
        let orientation = [
            [left.x, left.y, left.z, 0.0],
            [true_up.x, true_up.y, true_up.z, 0.0],
            [-forward.x, -forward.y, -forward.z, 0.0],
            [0.0, 0.0, 0.0, 1.0],
        ];
        let translation = Transform::new().translation(-from.x, -from.y, -from.z);
        Matrix::new(Matrix::multiple(orientation, translation.data))
    }

    pub fn build(&self) -> Matrix {
        Matrix::new(self.data)
    }
}

#[cfg(test)]
mod tests {
    use super::Transform;
    use crate::{Matrix, Point, Vector};
    use std::f64::consts::PI;

    #[test]
    fn translation_matrix() {
        let transform = Transform::new().translation(5.0, -3.0, 2.0).build();
        let p = Point::new(-3.0, 4.0, 5.0);
        let actual = transform * p;
        let expected = Point::new(2.0, 1.0, 7.0);
        assert_eq!(expected, actual);
    }

    #[test]
    fn translation_inverse_matrix() {
        let transform = Transform::new().translation(5.0, -3.0, 2.0).build();
        let inv = transform.inverse();
        let p = Point::new(-3.0, 4.0, 5.0);
        let actual = inv * p;
        let expected = Point::new(-8.0, 7.0, 3.0);
        assert_eq!(expected, actual);
    }

    #[test]
    fn translation_matrix_vector() {
        let transform = Transform::new().translation(5.0, -3.0, 2.0).build();
        let v = Vector::new(-3.0, 4.0, 5.0);
        let actual = transform * v;
        let expected = Vector::new(-3.0, 4.0, 5.0);
        assert_eq!(expected, actual);
    }

    #[test]
    fn scaling_matrix_point() {
        let transform = Transform::new().scaling(2.0, 3.0, 4.0).build();
        let p = Point::new(-4.0, 6.0, 8.0);
        let actual = transform * p;
        let expected = Point::new(-8.0, 18.0, 32.0);
        assert_eq!(expected, actual);
    }

    #[test]
    fn scaling_matrix_vector() {
        let transform = Transform::new().scaling(2.0, 3.0, 4.0).build();
        let v = Vector::new(-4.0, 6.0, 8.0);
        let actual = transform * v;
        let expected = Vector::new(-8.0, 18.0, 32.0);
        assert_eq!(expected, actual);
    }

    #[test]
    fn scaling_inverse_matrix_vector() {
        let transform = Transform::new().scaling(2.0, 3.0, 4.0).build();
        let inv = transform.inverse();
        let v = Vector::new(-4.0, 6.0, 8.0);
        let actual = inv * v;
        let expected = Vector::new(-2.0, 2.0, 2.0);
        assert_eq!(expected, actual);
    }

    #[test]
    fn reflection_scaling_matrix_point() {
        let transform = Transform::new().scaling(-1.0, 1.0, 1.0).build();
        let p = Point::new(2.0, 3.0, 4.0);
        let actual = transform * p;
        let expected = Point::new(-2.0, 3.0, 4.0);
        assert_eq!(expected, actual);
    }

    #[test]
    fn rotating_point_around_x() {
        let t: f64 = 2.0;
        let p = Point::new(0.0, 1.0, 0.0);
        let half_quarter = Transform::new().rotation_x(PI / 4.0).build();
        let full_quarter = Transform::new().rotation_x(PI / 2.0).build();
        let actual_hq = half_quarter * p;
        let expected_hq = Point::new(0.0, t.sqrt() / 2.0, t.sqrt() / 2.0);
        assert_eq!(expected_hq, actual_hq);
        let actual_fq = full_quarter * p;
        let expected_fq = Point::new(0.0, 0.0, 1.0);
        assert_eq!(expected_fq, actual_fq);
    }

    #[test]
    fn rotating_inverse_point_around_x() {
        let t: f64 = 2.0;
        let p = Point::new(0.0, 1.0, 0.0);
        let half_quarter = Transform::new().rotation_x(PI / 4.0).build();
        let inv = half_quarter.inverse();
        let actual = inv * p;
        let expected = Point::new(0.0, t.sqrt() / 2.0, -(t.sqrt() / 2.0));
        assert_eq!(expected, actual);
    }

    #[test]
    fn rotating_point_around_y() {
        let t: f64 = 2.0;
        let p = Point::new(0.0, 0.0, 1.0);
        let half_quarter = Transform::new().rotation_y(PI / 4.0).build();
        let full_quarter = Transform::new().rotation_y(PI / 2.0).build();
        let actual_hq = half_quarter * p;
        let expected_hq = Point::new(t.sqrt() / 2.0, 0.0, t.sqrt() / 2.0);
        assert_eq!(expected_hq, actual_hq);
        let actual_fq = full_quarter * p;
        let expected_fq = Point::new(1.0, 0.0, 0.0);
        assert_eq!(expected_fq, actual_fq);
    }

    #[test]
    fn rotating_point_around_z() {
        let t: f64 = 2.0;
        let p = Point::new(0.0, 1.0, 0.0);
        let half_quarter = Transform::new().rotation_z(PI / 4.0).build();
        let full_quarter = Transform::new().rotation_z(PI / 2.0).build();
        let actual_hq = half_quarter * p;
        let expected_hq = Point::new(-t.sqrt() / 2.0, t.sqrt() / 2.0, 0.0);
        assert_eq!(expected_hq, actual_hq);
        let actual_fq = full_quarter * p;
        let expected_fq = Point::new(-1.0, 0.0, 0.0);
        assert_eq!(expected_fq, actual_fq);
    }

    #[test]
    fn shearing_x_to_y() {
        let transform = Transform::new()
            .shearing(1.0, 0.0, 0.0, 0.0, 0.0, 0.0)
            .build();
        let p = Point::new(2.0, 3.0, 4.0);
        let actual = transform * p;
        let expected = Point::new(5.0, 3.0, 4.0);
        assert_eq!(expected, actual);
    }

    #[test]
    fn shearing_x_to_z() {
        let transform = Transform::new()
            .shearing(0.0, 1.0, 0.0, 0.0, 0.0, 0.0)
            .build();
        let p = Point::new(2.0, 3.0, 4.0);
        let actual = transform * p;
        let expected = Point::new(6.0, 3.0, 4.0);
        assert_eq!(expected, actual);
    }

    #[test]
    fn shearing_y_to_x() {
        let transform = Transform::new()
            .shearing(0.0, 0.0, 1.0, 0.0, 0.0, 0.0)
            .build();
        let p = Point::new(2.0, 3.0, 4.0);
        let actual = transform * p;
        let expected = Point::new(2.0, 5.0, 4.0);
        assert_eq!(expected, actual);
    }

    #[test]
    fn shearing_y_to_z() {
        let transform = Transform::new()
            .shearing(0.0, 0.0, 0.0, 1.0, 0.0, 0.0)
            .build();
        let p = Point::new(2.0, 3.0, 4.0);
        let actual = transform * p;
        let expected = Point::new(2.0, 7.0, 4.0);
        assert_eq!(expected, actual);
    }

    #[test]
    fn shearing_z_to_x() {
        let transform = Transform::new()
            .shearing(0.0, 0.0, 0.0, 0.0, 1.0, 0.0)
            .build();
        let p = Point::new(2.0, 3.0, 4.0);
        let actual = transform * p;
        let expected = Point::new(2.0, 3.0, 6.0);
        assert_eq!(expected, actual);
    }

    #[test]
    fn shearing_z_to_y() {
        let transform = Transform::new()
            .shearing(0.0, 0.0, 0.0, 0.0, 0.0, 1.0)
            .build();
        let p = Point::new(2.0, 3.0, 4.0);
        let actual = transform * p;
        let expected = Point::new(2.0, 3.0, 7.0);
        assert_eq!(expected, actual);
    }

    #[test]
    fn chaining_transformations() {
        let p = Point::new(1.0, 0.0, 1.0);
        let transform = Transform::new()
            .rotation_x(PI / 2.0)
            .scaling(5.0, 5.0, 5.0)
            .translation(10.0, 5.0, 7.0)
            .build();
        let actual = transform * p;
        let expected = Point::new(15.0, 0.0, 7.0);
        assert_eq!(expected, actual);
    }

    #[test]
    fn clock() {
        let twelve = Point::new(0.0, 0.0, 1.0);
        let hour = PI / 6.0;
        let r = Transform::new().rotation_y(3.0 * hour).build();
        let three = r * twelve;
        assert_eq!(Point::new(1.0, 0.0, 0.0), three);
    }

    #[test]
    fn view_transformation_matrix_looking_positive_z_direction() {
        let from = Point::new(0.0, 0.0, 0.0);
        let to = Point::new(0.0, 0.0, 1.0);
        let up = Vector::new(0.0, 1.0, 0.0);
        let t = Transform::view_transformation(from, to, up);

        assert_eq!(t, Transform::new().scaling(-1.0, 1.0, -1.0).build());
    }

    #[test]
    fn view_transformation_moves_the_world() {
        let from = Point::new(0.0, 0.0, 8.0);
        let to = Point::new(0.0, 0.0, 0.0);
        let up = Vector::new(0.0, 1.0, 0.0);
        let t = Transform::view_transformation(from, to, up);

        assert_eq!(t, Transform::new().translation(0.0, 0.0, -8.0).build());
    }

    #[test]
    fn view_transformation_matrix_for_default_orientation() {
        let from = Point::new(0.0, 0.0, 0.0);
        let to = Point::new(0.0, 0.0, -1.0);
        let up = Vector::new(0.0, 1.0, 0.0);
        let t = Transform::view_transformation(from, to, up);

        let e = Matrix::new([
            [1.0, 0.0, 0.0, 0.0],
            [0.0, 1.0, 0.0, 0.0],
            [0.0, 0.0, 1.0, 0.0],
            [0.0, 0.0, 0.0, 1.0],
        ]);

        assert_eq!(e, t)
    }

    #[test]
    fn arbitrary_view_transformation() {
        let from = Point::new(1.0, 3.0, 2.0);
        let to = Point::new(4.0, -2.0, 8.0);
        let up = Vector::new(1.0, 1.0, 0.0);
        let t = Transform::view_transformation(from, to, up);

        let e = Matrix::new([
            [-0.50709, 0.50709, 0.67612, -2.36643],
            [0.76772, 0.60609, 0.12122, -2.82843],
            [-0.35857, 0.59761, -0.71714, 0.00000],
            [0.00000, 0.00000, 0.00000, 1.00000],
        ]);

        assert_eq!(t, e);
    }
}
