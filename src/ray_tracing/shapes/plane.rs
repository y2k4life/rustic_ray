use std::any::Any;
use crate::{EPSILON, ray_tracing::matrix::IDENTITY, Intersection, Material, Matrix, Point, Ray, Vector};
use super::Shape;

#[derive(Debug)]
pub struct Plane {
    pub transform: Matrix,
    pub material: Material,
    pub cast_shadow: bool,
}

impl Plane {
    pub fn new() -> Plane {
        Plane {
            transform: IDENTITY,
            material: Material::new(),
            cast_shadow: true,
        }
    }
}

impl Shape for Plane {
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn shape_eq(&self, other: &dyn Any) -> bool {
        other.downcast_ref::<Self>().map_or(false, |a| self == a)
    }

    /*
    fn shape_clone(&self) -> Box<dyn Shape> {
        Box::new((*self).clone())
    }
    */

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
        if ray.direction.y.abs() < EPSILON {
            None
        }
        else {
            Some(vec![
                Intersection::new(-ray.origin.y / ray.direction.y, self),
            ])
        }
    }
    
    fn local_normal_at(&self, _point: Point) -> Vector {
        Vector::new(0.0, 1.0, 0.0)
    }
}

impl PartialEq for Plane {
    fn eq(&self, other: &Self) -> bool {
        self.transform == other.transform && self.material == other.material
    }
}


#[cfg(test)]
mod tests {
    use crate::{Point, Ray, Vector, shapes::Shape};
    use super::Plane;

    #[test]
    fn normal_plane_constant_everywhere() {
        let p = Plane::new();
        let n1 = p.local_normal_at(Point::new(0.0, 0.0, 0.0));
        let n2 = p.local_normal_at(Point::new(0.0, 0.0, 0.0));
        let n3 = p.local_normal_at(Point::new(0.0, 0.0, 0.0));
        assert_eq!(n1, Vector::new(0.0, 1.0, 0.0));
        assert_eq!(n2, Vector::new(0.0, 1.0, 0.0));
        assert_eq!(n3, Vector::new(0.0, 1.0, 0.0));
    }

    #[test]
    fn intersect_ray_parallel_to_plane() {
        let p = Plane::new();
        let r = Ray::new(Point::new(0.0, 10.0, 0.0), Vector::new(0.0, 0.0, 1.0));
        let xs = p.local_intersect(r);
        assert_eq!(xs, None);
    }

    #[test]
    fn ray_intersecting_plane_from_above() {
        let p = Plane::new();
        let r = Ray::new(Point::new(0.0, 1.0, 0.0), Vector::new(0.0, -1.0, 0.0));
        if let Some(xs) =  p.local_intersect(r) {
            assert_eq!(xs.len(), 1);
            assert_eq!(xs[0].t, 1.0);
            //assert!(p.shape_eq(xs[0].object));
        }
    }
}