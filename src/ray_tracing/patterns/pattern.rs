use crate::{shapes::Shape, Color, Matrix, Point};
use std::any::Any;
use std::fmt;

pub trait Pattern: Send + Any + fmt::Debug {
    fn pattern_eq(&self, other: &dyn Any) -> bool;

    fn as_any(&self) -> &dyn Any;

    fn transform(&self) -> Matrix;

    fn set_transform(&mut self, transform: Matrix);

    fn pattern_at(&self, point: Point) -> Color;

    fn pattern_at_object(&self, object: &dyn Shape, word_point: Point) -> Color {
        let object_point = object.transform().inverse() * word_point;
        let pattern_point = self.transform().inverse() * object_point;
        self.pattern_at(pattern_point)
    }
}

impl PartialEq for Box<dyn Pattern> {
    fn eq(&self, other: &Box<dyn Pattern>) -> bool {
        self.pattern_eq(other.as_any())
    }
}
