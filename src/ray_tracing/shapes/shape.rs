use crate::{Intersection, Material, Matrix, Point, Ray, Vector};
use std::any::Any;
use std::fmt;

pub trait Shape: Any + fmt::Debug {
    fn shape_clone(&self) -> Box<dyn Shape>;

    /// This method test for `self` and `other` shapes to be equal.
    fn shape_eq(&self, other: &dyn Any) -> bool;

    fn as_any(&self) -> &dyn Any;

    /// Gets the transformation of a shape.
    fn transform(&self) -> Matrix;

    /// Sets the material of a shape
    fn set_transform(&mut self, transform: Matrix);

    /// Returns the material of a shape
    fn material(&self) -> Material;

    fn set_material(&mut self, material: Material);

    fn local_intersect(&self, ray: Ray) -> Option<Vec<Intersection>>;

    fn local_normal_at(&self, point: Point) -> Vector;

    fn cast_shadow(&self) -> bool {
        true
    }

    fn intersect(&self, ray: Ray)  -> Option<Vec<Intersection>> {
        let local_ray = ray.transform(&self.transform().inverse());
        self.local_intersect(local_ray)
    }

    fn normal_at(&self, point: Point) -> Vector {
        let local_point = self.transform().inverse() * point;
        let local_normal = self.local_normal_at(local_point);
        (self.transform().inverse().transpose() * local_normal).normalize()
    }
}

impl Clone for Box<dyn Shape> {
    fn clone(&self) -> Self {
        self.shape_clone()
    }
}

/*
impl PartialEq for Box<dyn Shape> {
    fn eq(&self, other: &Box<dyn Shape>) -> bool {
        self.shape_eq(other.as_any())
    }
}
*/

impl PartialEq for dyn Shape {
    fn eq(&self, other: &Self) -> bool {
        self.shape_eq(other.as_any())
    }
}