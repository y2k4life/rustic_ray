use std::any::Any;

use super::Pattern;
use crate::{ray_tracing::matrix, Color, Matrix, Point};

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Checkers {
    pub a: Color,
    pub b: Color,
    pub transform: Matrix,
}

impl Checkers {
    pub fn new(a: Color, b: Color) -> Checkers {
        Checkers {
            a,
            b,
            transform: matrix::IDENTITY,
        }
    }
}

impl Pattern for Checkers {
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn pattern_eq(&self, other: &dyn Any) -> bool {
        other.downcast_ref::<Self>().map_or(false, |a| self == a)
    }

    fn transform(&self) -> Matrix {
        self.transform
    }

    fn set_transform(&mut self, transform: Matrix) {
        self.transform = transform;
    }

    fn pattern_at(&self, point: Point) -> Color {
        if (point.x.floor() + point.y.floor() + point.z.floor()) % 2.0 == 0.0 {
            self.a
        }
        else {
            self.b
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{ray_tracing::color, Point};

    #[test]
    fn checkers_should_repeat_in_x() {
        let pattern = Checkers::new(color::WHITE, color::BLACK);
        assert_eq!(pattern.pattern_at(Point::new(0.0, 0.0, 0.0)), color::WHITE);
        assert_eq!(pattern.pattern_at(Point::new(0.99, 0.0, 0.0)), color::WHITE);
        assert_eq!(pattern.pattern_at(Point::new(1.01, 0.0, 0.0)), color::BLACK);
    }

    #[test]
    fn checkers_should_repeat_in_y() {
        let pattern = Checkers::new(color::WHITE, color::BLACK);
        assert_eq!(pattern.pattern_at(Point::new(0.0, 0.0, 0.0)), color::WHITE);
        assert_eq!(pattern.pattern_at(Point::new(0.0, 0.99, 0.0)), color::WHITE);
        assert_eq!(pattern.pattern_at(Point::new(0.0, 1.01, 0.708)), color::BLACK);
    }

    #[test]
    fn checkers_should_repeat_in_z() {
        let pattern = Checkers::new(color::WHITE, color::BLACK);
        assert_eq!(pattern.pattern_at(Point::new(0.0,  0.0, 0.0)), color::WHITE);
        assert_eq!(pattern.pattern_at(Point::new(0.0,  0.0, 0.99)), color::WHITE);
        assert_eq!(pattern.pattern_at(Point::new(0.0,  0.0, 1.01)), color::BLACK);
    }
}
