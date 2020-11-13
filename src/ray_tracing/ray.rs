use crate::{Matrix, Point, Vector};

#[derive(Debug, Copy, Clone)]
pub struct Ray {
    pub origin: Point,
    pub direction: Vector,
}

impl Ray {
    pub fn new(origin: Point, direction: Vector) -> Ray {
        Ray { origin, direction }
    }

    pub fn position(&self, t: f64) -> Point {
        self.origin + self.direction * t
    }

    pub fn transform(&self, m: &Matrix) -> Ray {
        Ray::new(*m * self.origin, *m * self.direction)
    }
}

#[cfg(test)]
mod tests {
    use crate::{Point, Transform, Vector};
    use super::*;

    #[test]
    fn creating_ray() {
        let origin = Point::new(1.0, 2.0, 3.0);
        let direction = Vector::new(4.0, 5.0, 6.0);
        let r = Ray::new(origin, direction);
        assert_eq!(origin, r.origin);
        assert_eq!(direction, r.direction);
    }

    #[test]
    fn computing_a_point_from_a_distance() {
        let r = Ray::new(Point::new(2.0, 3.0, 4.0), Vector::new(1.0, 0.0, 0.0));
        assert_eq!(Point::new(2.0, 3.0, 4.0), r.position(0.0));
        assert_eq!(Point::new(3.0, 3.0, 4.0), r.position(1.0));
        assert_eq!(Point::new(1.0, 3.0, 4.0), r.position(-1.0));
        assert_eq!(Point::new(4.5, 3.0, 4.0), r.position(2.5));
    }

    #[test]
    fn translating_a_ray() {
        let r1 = Ray::new(Point::new(1.0, 2.0, 3.0), Vector::new(0.0, 1.0, 0.0));
        let m = Transform::new().translation(3.0, 4.0, 5.0).build();
        let r2 = r1.transform(&m);
        assert_eq!(Point::new(4.0, 6.0, 8.0), r2.origin);
        assert_eq!(Vector::new(0.0, 1.0, 0.0), r2.direction);
    }

    #[test]
    fn scaling_a_ray() {
        let r1 = Ray::new(Point::new(1.0, 2.0, 3.0), Vector::new(0.0, 1.0, 0.0));
        let m = Transform::new().scaling(2.0, 3.0, 4.0).build();
        let r2 = r1.transform(&m);
        assert_eq!(Point::new(2.0, 6.0, 12.0), r2.origin);
        assert_eq!(Vector::new(0.0, 3.0, 0.0), r2.direction);
    }
}
