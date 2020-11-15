use crate::{Intersection, Material, Matrix, Point, Ray, Vector};
use std::any::Any;
use std::fmt;

pub trait Shape: Any + fmt::Debug {
    fn shape_eq(&self, other: &dyn Any) -> bool;

    fn as_any(&self) -> &dyn Any;

    fn transform(&self) -> Matrix;

    fn set_transform(&mut self, transform: Matrix);

    fn material(&self) -> &Material;

    fn material_mut(&mut self) -> &mut Material;

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

impl PartialEq for dyn Shape {
    fn eq(&self, other: &Self) -> bool {
        self.shape_eq(other.as_any())
    }
}