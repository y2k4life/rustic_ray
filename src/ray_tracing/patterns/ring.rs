use super::Pattern;
use crate::{Color, Matrix, Point, ray_tracing::matrix};
use std::any::Any;

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Ring {
    pub a: Color,
    pub b: Color,
    pub transform: Matrix,
}

impl Ring {
    pub fn new(a: Color, b: Color) -> Ring {
        Ring {
            a,
            b,
            transform: matrix::IDENTITY,
        }
    }
}

impl Pattern for Ring {
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
        let x = (point.x * 100.0).round() / 100.0;
        let z = (point.z * 100.0).round() / 100.0;
        let t = (x.powf(2.0) + z.powf(2.0)).sqrt().floor();
        if t % 2.0 == 0.0 {
            self.a
        } else {
            self.b
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::ray_tracing::color;

    use super::*;

    #[test]
    fn ring_should_extend_both_x_and_z() {
        let pattern = Ring::new(color::WHITE, color::BLACK);
        assert_eq!(pattern.pattern_at(Point::new(0.0, 0.0, 0.0)), color::WHITE);
        assert_eq!(pattern.pattern_at(Point::new(1.0, 0.0, 0.0)), color::BLACK);
        assert_eq!(pattern.pattern_at(Point::new(0.0, 0.0, 1.0)), color::BLACK);
        assert_eq!(pattern.pattern_at(Point::new(0.708, 0.0, 0.708)), color::BLACK);
    }
}
