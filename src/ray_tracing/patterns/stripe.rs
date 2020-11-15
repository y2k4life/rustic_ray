use std::any::Any;

use crate::{Color, Matrix, Point, ray_tracing::matrix};
use super::Pattern;

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Stripe {
    pub a: Color,
    pub b: Color,
    pub transform: Matrix,
}

impl Stripe {
    pub fn new (a: Color, b: Color) -> Stripe {
        Stripe {a, b, transform: matrix::IDENTITY}
    }
}

impl Pattern for Stripe {
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn pattern_eq(&self, other: &dyn Any) -> bool {
        other.downcast_ref::<Self>().map_or(false, |a| self == a)
    }

    fn pattern_clone(&self) -> Box<dyn Pattern> {
        Box::new((*self).clone())
    }

    fn transform(&self) -> Matrix {
        self.transform
    }

    fn set_transform(&mut self, transform: Matrix) {
        self.transform = transform;
    }

    fn pattern_at(&self, point: Point) -> Color {
        if point.x.floor() % 2.0 == 0.0 {
            self.a
        }
        else {
            self.b
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{Point, ray_tracing::color, shapes::Shape, shapes::Sphere, Transform};
    use super::*;

    #[test]
    fn creating_stripe_patter() {
        let pattern = Stripe::new(color::WHITE, color::BLACK);
        assert_eq!(pattern.a, color::WHITE);
        assert_eq!(pattern.b, color::BLACK);
    }

    #[test]
    fn stripe_pattern_constant_in_y() {
        let pattern = Stripe::new(color::WHITE, color::BLACK);
        assert_eq!(pattern.pattern_at(Point::new(0.0, 0.0, 0.0)), color::WHITE);
        assert_eq!(pattern.pattern_at(Point::new(0.0, 1.0, 0.0)), color::WHITE);
        assert_eq!(pattern.pattern_at(Point::new(0.0, 2.0, 0.0)), color::WHITE);
    }

    #[test]
    fn stripe_pattern_constant_in_z() {
        let pattern = Stripe::new(color::WHITE, color::BLACK);
        assert_eq!(pattern.pattern_at(Point::new(0.0, 0.0, 0.0)), color::WHITE);
        assert_eq!(pattern.pattern_at(Point::new(0.0, 0.0, 1.0)), color::WHITE);
        assert_eq!(pattern.pattern_at(Point::new(0.0, 0.0, 2.0)), color::WHITE);
    }

    #[test]
    fn stripe_pattern_alternates_in_x() {
        let pattern = Stripe::new(color::WHITE, color::BLACK);
        assert_eq!(pattern.pattern_at(Point::new(0.0, 0.0, 0.0)), color::WHITE);
        assert_eq!(pattern.pattern_at(Point::new(0.9, 0.0, 0.0)), color::WHITE);
        assert_eq!(pattern.pattern_at(Point::new(1.0, 0.0, 0.0)), color::BLACK);
        assert_eq!(pattern.pattern_at(Point::new(-0.1, 0.0, 0.0)), color::BLACK);
        assert_eq!(pattern.pattern_at(Point::new(-1.0, 0.0, 0.0)), color::BLACK);
        assert_eq!(pattern.pattern_at(Point::new(-1.1, 0.0, 0.0)), color::WHITE);
    }

    #[test]
    fn stripes_with_object_transformation() {
        let mut object = Sphere::new();
        object.set_transform(Transform::new().scaling(2.0, 2.0, 2.0).build());
        let pattern = Stripe::new(color::WHITE, color::BLACK);
        let c = pattern.pattern_at_object(&object, Point::new(1.5, 0.0, 0.0));
        assert_eq!(c, color::WHITE);
    }

    #[test]
    fn stripes_with_pattern_transformation() {
        let object = Sphere::new();
        let mut pattern = Stripe::new(color::WHITE, color::BLACK);
        pattern.set_transform(Transform::new().scaling(2.0, 2.0, 2.0).build());
        let c = pattern.pattern_at_object(&object, Point::new(1.5, 0.0, 0.0));
        assert_eq!(c, color::WHITE);
    }

    #[test]
    fn stripes_with_both_object_and_pattern_transformation() {
        let mut object = Sphere::new();
        object.set_transform(Transform::new().scaling(2.0, 2.0, 2.0).build());
        let mut pattern = Stripe::new(color::WHITE, color::BLACK);
        pattern.set_transform(Transform::new().translation(0.5, 0.0, 0.0).build());
        let c = pattern.pattern_at_object(&object, Point::new(2.5, 0.0, 0.0));
        assert_eq!(c, color::WHITE);
    }
}