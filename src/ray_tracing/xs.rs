// use std::cmp::Ordering;
use std::cmp::Ordering;

use crate::shapes::Shape;
use crate::float_cmp;

#[derive(Clone)]
pub struct XS {
    pub t: f64,
    pub object: Box<dyn Shape>
}

impl XS {
    pub fn new(t: f64, object: Box<dyn Shape>) -> Self {
        XS {t, object: object}
    }
}

impl PartialEq for XS {
    fn eq(&self, other: &XS) -> bool {
        self.t == other.t &&
        &self.object == &other.object
    }
}

impl PartialOrd for XS {
    fn partial_cmp(&self, other: &XS) -> Option<Ordering> {
        Some(float_cmp(self.t,other.t))
    }
}
