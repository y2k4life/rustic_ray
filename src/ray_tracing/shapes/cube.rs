use super::Shape;
use crate::{ray_tracing::matrix::IDENTITY, Intersection, Material, Matrix, Point, Ray, Vector};
use std::any::Any;

#[derive(Debug, Clone)]
pub struct Cube {
    pub transform: Matrix,
    pub material: Material,
}

impl Cube {
    pub fn new() -> Cube {
        Cube {
            transform: IDENTITY,
            material: Material::new(),
        }
    }
}

impl Shape for Cube {
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn shape_eq(&self, other: &dyn Any) -> bool {
        other.downcast_ref::<Self>().map_or(false, |a| self == a)
    }

    fn shape_clone(&self) -> Box<dyn Shape> {
        Box::new((*self).clone())
    }

    fn transform(&self) -> Matrix {
        self.transform
    }

    fn set_transform(&mut self, transform: Matrix) {
        self.transform = transform;
    }

    fn material(&self) -> Material {
        self.material.clone()
    }

    fn set_material(&mut self, material: Material) {
        self.material = material;
    }

    fn local_intersect(&self, ray: Ray) -> Option<Vec<Intersection>> {
        Some(vec![
            Intersection::new(ray.origin.x, Box::new(self.clone())),
            Intersection::new(ray.origin.y, Box::new(self.clone())),
            Intersection::new(ray.origin.z, Box::new(self.clone())),
            Intersection::new(ray.direction.x, Box::new(self.clone())),
            Intersection::new(ray.direction.y, Box::new(self.clone())),
            Intersection::new(ray.direction.z, Box::new(self.clone())),
            ])
        
    }

    fn local_normal_at(&self, point: Point) -> Vector {
        Vector::new(point.x, point.y, point.z)
    }
}

impl PartialEq for Cube {
    fn eq(&self, other: &Self) -> bool {
        self.transform == other.transform && self.material == other.material
    }
}

#[cfg(test)]
mod tests {

}
