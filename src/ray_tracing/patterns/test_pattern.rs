use std::any::Any;

use super::Pattern;
use crate::{ray_tracing::matrix, Color, Matrix, Point};

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct TestPattern {}

impl TestPattern {
    pub fn new() -> TestPattern {
        TestPattern {}
    }
}

impl Pattern for TestPattern {
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn pattern_eq(&self, other: &dyn Any) -> bool {
        other.downcast_ref::<Self>().map_or(false, |a| self == a)
    }

    fn transform(&self) -> Matrix {
        matrix::IDENTITY
    }

    fn set_transform(&mut self, _transform: Matrix) {}

    fn pattern_at(&self, point: Point) -> Color {
        Color::new(point.x, point.y, point.z)
    }
}

impl Default for TestPattern {
    fn default() -> Self {
        Self::new()
    }
}
