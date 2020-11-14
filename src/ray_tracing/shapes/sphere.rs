use uuid::Uuid;

use super::Shape;
use crate::{ray_tracing::matrix::IDENTITY, Intersection, Material, Matrix, Point, Ray, Vector};
use std::any::Any;

#[derive(Debug, Clone, PartialEq)]
pub struct Sphere {
    id: Uuid,
    pub transform: Matrix,
    pub material: Material,
    pub cast_shadow: bool,
}

impl Sphere {
    pub fn new() -> Self {
        Self {
            id: Uuid::new_v4(),
            transform: IDENTITY,
            material: Material::new(),
            cast_shadow: true,
        }
    }

    pub fn glass_sphere() -> Self {
        let mut m = Material::new();
        m.refractive_index = 1.5;
        m.transparency = 1.0;
        Self {
            id: Uuid::new_v4(),
            transform: IDENTITY,
            material: m,
            cast_shadow: true,
        }
    }
}

impl Shape for Sphere {
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

    fn cast_shadow(&self) -> bool {
        self.cast_shadow
    }

    fn local_intersect(&self, r2: Ray) -> Option<Vec<Intersection>> {
        let mut xs: Vec<Intersection> = Vec::new();

        let sphere_to_ray = r2.origin - Point::new(0.0, 0.0, 0.0);
        let a = r2.direction.dot(r2.direction);

        let b = 2.0 * r2.direction.dot(sphere_to_ray);
        let c = sphere_to_ray.dot(sphere_to_ray) - 1.0;

        let discriminant = b.powf(2.0) - 4.0 * a * c;

        if discriminant >= 0.0 {
            let t1 = (-b - discriminant.sqrt()) / (2.0 * a);
            xs.push(Intersection::new(t1, self));
            let t2 = (-b + discriminant.sqrt()) / (2.0 * a);
            xs.push(Intersection::new(t2, self));
        }

        if xs.len() > 0 {
            Some(xs)
        } else {
            None
        }
    }

    fn local_normal_at(&self, object_point: Point) -> Vector {
        //object_normal = object_point - Point::new(0.0, 0.0, 0.0);
        //let word_normal = self.transform.inverse().transpose() * object_normal;
        //word_normal.normalize()
        object_point - Point::new(0.0, 0.0, 0.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{Material, Point, Ray, Transform, Vector, ray_tracing::matrix};
    use std::f64::consts::PI;

    #[test]
    fn a_ray_intersects_a_sphere_at_two_points() {
        let r = Ray::new(Point::new(0.0, 0.0, -5.0), Vector::new(0.0, 0.0, 1.0));
        let s = Sphere::new();
        let xs = s.intersect(r).unwrap();
        assert_eq!(2, xs.len());
        assert_eq!(4.0, xs[0].t);
        assert_eq!(6.0, xs[1].t);
    }

    #[test]
    fn a_ray_intersects_a_sphere_at_a_tangent() {
        let r = Ray::new(Point::new(0.0, 1.0, -5.0), Vector::new(0.0, 0.0, 1.0));
        let s = Sphere::new();
        let xs = s.intersect(r).unwrap();
        assert_eq!(2, xs.len());
        assert_eq!(5.0, xs[0].t);
        assert_eq!(5.0, xs[1].t);
    }

    #[test]
    fn a_ray_misses_a_sphere() {
        let r = Ray::new(Point::new(0.0, 2.0, -5.0), Vector::new(0.0, 0.0, 1.0));
        let s = Sphere::new();
        assert_eq!(s.intersect(r).is_none(), true);
    }

    #[test]
    fn a_ray_originates_inside_a_sphere() {
        let r = Ray::new(Point::new(0.0, 0.0, 0.0), Vector::new(0.0, 0.0, 1.0));
        let s = Sphere::new();
        let xs = s.intersect(r).unwrap();
        assert_eq!(2, xs.len());
        assert_eq!(-1.0, xs[0].t);
        assert_eq!(1.0, xs[1].t);
    }

    #[test]
    fn a_sphere_behind_a_ray() {
        let r = Ray::new(Point::new(0.0, 0.0, 5.0), Vector::new(0.0, 0.0, 1.0));
        let s = Sphere::new();
        let xs = s.intersect(r).unwrap();
        assert_eq!(2, xs.len());
        assert_eq!(-6.0, xs[0].t);
        assert_eq!(-4.0, xs[1].t);
    }

    #[test]
    fn intersecting_a_scaled_sphere_with_ray() {
        let r = Ray::new(Point::new(0.0, 0.0, -5.0), Vector::new(0.0, 0.0, 1.0));
        let mut s = Sphere::new();
        s.transform = Transform::new().scaling(2.0, 2.0, 2.0).build();
        let xs = s.intersect(r).unwrap();
        assert_eq!(2, xs.len());
        assert_eq!(3.0, xs[0].t);
        assert_eq!(7.0, xs[1].t)
    }

    #[test]
    fn the_normal_on_sphere_at_point_x_axis() {
        let s = Sphere::new();
        let n = s.normal_at(Point::new(1.0, 0.0, 0.0));
        assert_eq!(Vector::new(1.0, 0.0, 0.0), n);
    }

    #[test]
    fn the_normal_on_sphere_at_point_y_axis() {
        let s = Sphere::new();
        let n = s.normal_at(Point::new(0.0, 1.0, 0.0));
        assert_eq!(Vector::new(0.0, 1.0, 0.0), n);
    }

    #[test]
    fn the_normal_on_sphere_at_point_z_axis() {
        let s = Sphere::new();
        let n = s.normal_at(Point::new(0.0, 0.0, 1.0));
        assert_eq!(Vector::new(0.0, 0.0, 1.0), n);
    }

    #[test]
    fn the_normal_on_sphere_at_point_non_axial() {
        let s = Sphere::new();
        let n = s.normal_at(Point::new(
            3_f64.sqrt() / 3.0,
            3_f64.sqrt() / 3.0,
            3_f64.sqrt() / 3.0,
        ));
        assert_eq!(
            Vector::new(3_f64.sqrt() / 3.0, 3_f64.sqrt() / 3.0, 3_f64.sqrt() / 3.0),
            n
        );
    }

    #[test]
    fn the_normal_is_a_normalized_vector() {
        let s = Sphere::new();
        let n = s.normal_at(Point::new(
            3_f64.sqrt() / 3.0,
            3_f64.sqrt() / 3.0,
            3_f64.sqrt() / 3.0,
        ));
        assert_eq!(n, n.normalize());
    }

    #[test]
    fn computing_normal_on_translated_sphere() {
        let mut s = Sphere::new();
        s.transform = Transform::new().translation(0.0, 1.0, 0.0).build();
        let n = s.normal_at(Point::new(0.0, 1.70711, -0.70711));
        assert_eq!(Vector::new(0.0, 0.70711, -0.70711), n);
    }

    #[test]
    fn computing_normal_on_transformed_sphere() {
        let mut s = Sphere::new();
        s.transform = Transform::new()
            .rotation_z(PI / 5.0)
            .scaling(1.0, 0.5, 1.0)
            .build();
        let n = s.normal_at(Point::new(0.0, 2_f64.sqrt() / 2.0, -2_f64.sqrt() / 2.0));
        assert_eq!(Vector::new(0.0, 0.97014, -0.24254), n);
    }

    #[test]
    fn a_sphere_may_be_assigned_material() {
        let mut s = Sphere::new();
        let mut m = Material::new();
        m.ambient = 1.0;
        s.material = m;
        assert_eq!(1.0, s.material.ambient);
    }

    // Chapter 11 - Reflection and Refraction
    // Page 151
    #[test]
    fn helper_producing_sphere_with_glassy_material() {
        let s = Sphere::glass_sphere();
        assert_eq!(s.transform, matrix::IDENTITY);
        assert_eq!(s.material.transparency, 1.0);
        assert_eq!(s.material.refractive_index, 1.5);
    }
}
