use super::Pattern;
use crate::{ray_tracing::matrix, Color, Matrix, Point};
use std::any::Any;

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Gradient {
    a: Color,
    b: Color,
    pub transform: Matrix,
}

impl Gradient {
    pub fn new(a: Color, b: Color) -> Gradient {
        Gradient {
            a,
            b,
            transform: matrix::IDENTITY,
        }
    }
}

impl Pattern for Gradient {
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
        self.a + (self.b - self.a) * (point.x - point.x.floor()) 
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ray_tracing::color;
    
    #[test]
    fn gradient_linearly_interpolates_between_colors() {
        let pattern = Gradient::new(color::WHITE, color::BLACK);
        assert_eq!(pattern.pattern_at(Point::new(0.0, 0.0, 0.0)), color::WHITE);
        assert_eq!(pattern.pattern_at(Point::new(0.25, 0.0, 0.0)), Color::new(0.75, 0.75, 0.75));
        assert_eq!(pattern.pattern_at(Point::new(0.5, 0.0, 0.0)), Color::new(0.5, 0.5, 0.5));
        assert_eq!(pattern.pattern_at(Point::new(0.75, 0.0, 0.0)), Color::new(0.25, 0.25, 0.25));
    }
}
