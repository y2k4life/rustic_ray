use uuid::Uuid;

use crate::{Intersection, Material, Matrix, Point, Ray, ShapeContainer, Vector};
use std::any::Any;
use std::fmt;

pub trait Shape: Any + fmt::Debug {
    fn id(&self) -> Uuid;

    fn parent_id(&self) -> Option<Uuid> {
        None
    }

    fn set_parent_id(&mut self, id: Uuid);

    fn as_any(&self) -> &dyn Any;

    fn shape_eq(&self, other: &dyn Shape) -> bool;

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

    fn get_child(&self, _id: Uuid) -> Option<&dyn Shape> {
        None
    }

    fn intersect(&self, ray: Ray) -> Option<Vec<Intersection>> {
        let local_ray = ray.transform(&self.transform().inverse());
        self.local_intersect(local_ray)
    }

    fn normal_at(&self, point: Point, sc: Option<&ShapeContainer>) -> Vector {
        match sc {
            Some(sc) => {
                let local_point = self.world_to_object(point, sc);
                let local_normal = self.local_normal_at(local_point);
                self.normal_to_world(local_normal, sc)
            }
            None => {
                let local_point = self.transform().inverse() * point;
                let local_normal = self.local_normal_at(local_point);
                (self.transform().inverse().transpose() * local_normal).normalize()
            }
        }
    }

    fn world_to_object(&self, point: Point, sc: &ShapeContainer) -> Point {
        let object_point = match self.parent_id() {
            Some(id) => {
                let parent = sc.get_shape(id).expect("Shape not found!");
                parent.world_to_object(point, sc)
            }
            None => point,
        };

        self.transform().inverse() * object_point
    }

    fn normal_to_world(&self, normal: Vector, w: &ShapeContainer) -> Vector {
        let world_normal = (self.transform().inverse().transpose() * normal).normalize();

        match self.parent_id() {
            Some(id) => {
                let parent = w.get_shape(id).expect("Shape not found!");
                parent.normal_to_world(world_normal, w)
            }
            None => world_normal,
        }
    }
}

impl PartialEq for dyn Shape {
    fn eq(&self, other: &Self) -> bool {
        self.id() == other.id()
    }
}
