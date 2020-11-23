use std::any::Any;

use uuid::Uuid;

use crate::{ray_tracing::matrix::IDENTITY, Intersection, Material, Matrix, Point, Ray, Vector};

use super::Shape;

#[derive(Debug)]
pub struct Group {
    pub id: Uuid,
    pub parent_id: Option<Uuid>,
    pub transform: Matrix,
    pub material: Material,
    pub shapes: Vec<Box<dyn Shape>>,
}

impl Group {
    pub fn new() -> Group {
        Group {
            id: Uuid::new_v4(),
            parent_id: None,
            transform: IDENTITY,
            material: Material::new(),
            shapes: Vec::new(),
        }
    }

    pub fn add_shape(&mut self, shape: Box<dyn Shape>) {
        self.shapes.push(shape);
    }
}

impl Shape for Group {
    fn id(&self) -> Uuid {
        self.id
    }

    fn parent_id(&self) -> Option<Uuid> {
        self.parent_id
    }

    fn set_parent_id(&mut self, id: Uuid) {
        self.parent_id = Some(id);
    }

    fn as_any(&self) -> &dyn Any {
        self
    }

    fn shape_eq(&self, other: &dyn Shape) -> bool {
        self.id == other.id()
    }

    fn transform(&self) -> Matrix {
        self.transform
    }

    fn set_transform(&mut self, transform: Matrix) {
        self.transform = transform;
    }

    fn material(&self) -> &Material {
        &self.material
    }

    fn material_mut(&mut self) -> &mut Material {
        &mut self.material
    }

    fn set_material(&mut self, material: Material) {
        self.material = material;
    }

    fn get_child(&self, id: Uuid) -> Option<&dyn Shape> {
        let mut shape = None;
        for s in &self.shapes {
            if s.id() == id {
                shape = Some(s.as_ref());
                break;
            }
            match s.get_child(id) {
                Some(c) => {
                    shape = Some(c);
                    break;
                }
                None => (),
            }
        }

        shape
    }

    fn local_intersect<'a>(&'a self, ray: Ray) -> Option<Vec<Intersection<'a>>> {
        let mut xs: Vec<Intersection> = Vec::new();

        for o in &self.shapes {
            if let Some(oxs) = o.intersect(ray) {
                for ox in oxs {
                    xs.push(ox);
                }
            }
        }

        if xs.is_empty() {
            None
        } else {
            xs.sort_by(|a, b| a.partial_cmp(b).unwrap());
            Some(xs)
        }
    }

    fn local_normal_at(&self, _point: Point) -> Vector {
        panic!("Should not be called!")
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        shapes::{Sphere, TestShape},
        Transform,
    };

    use super::*;

    // Chapter 14 Groups
    // Page 195
    #[test]
    fn adding_a_child_to_a_group() {
        let mut g = Group::new();
        let mut s = TestShape::new();
        s.parent_id = Some(g.id);
        let s_id = s.id;

        g.add_shape(Box::new(s));

        assert!(!g.shapes.is_empty());
        assert_eq!(g.shapes[0].id(), s_id);
    }

    // Chapter 14 Groups
    // Page 196
    #[test]
    fn intersecting_a_ray_with_an_empty_group() {
        let g = Group::new();
        let r = Ray::new(Point::new(0.0, 0.0, 0.0), Vector::new(0.0, 0.0, 1.0));
        let xs = g.local_intersect(r);
        assert!(xs.is_none());
    }

    // Chapter 14 Groups
    // Page 196
    #[test]
    fn intersecting_a_ray_with_a_none_empty_group() {
        let mut g = Group::new();

        let s1 = Sphere::new();
        let s1_id = s1.id;

        let mut s2 = Sphere::new();
        s2.transform = Transform::new().translation(0.0, 0.0, -3.0).build();
        let s2_id = s2.id;

        let mut s3 = Sphere::new();
        s3.transform = Transform::new().translation(5.0, 0.0, 0.0).build();

        g.add_shape(Box::new(s1));
        g.add_shape(Box::new(s2));
        g.add_shape(Box::new(s3));

        let r = Ray::new(Point::new(0.0, 0.0, -5.0), Vector::new(0.0, 0.0, 1.0));
        let xs = g.intersect(r).unwrap();

        assert_eq!(xs.len(), 4);
        assert_eq!(xs[0].object.id(), s2_id);
        assert_eq!(xs[1].object.id(), s2_id);
        assert_eq!(xs[2].object.id(), s1_id);
        assert_eq!(xs[3].object.id(), s1_id);
    }

    // Chapter 14 Groups
    // Page 197
    #[test]
    fn intersecting_a_transformed_group() {
        let mut g = Group::new();
        g.transform = Transform::new().scaling(2.0, 2.0, 2.0).build();

        let mut s = Sphere::new();
        s.transform = Transform::new().translation(5.0, 0.0, 0.0).build();

        g.add_shape(Box::new(s));

        let r = Ray::new(Point::new(10.0, 0.0, -10.0), Vector::new(0.0, 0.0, 1.0));

        let xs = g.intersect(r).unwrap();
        assert_eq!(xs.len(), 2);
    }
}
