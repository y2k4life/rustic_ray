use std::fmt::Debug;
use uuid::Uuid;

use crate::shapes::Shape;

#[derive(Debug)]
pub struct ShapeRelationship {
    pub parent: Uuid,
    pub child: Uuid,
}

#[derive(Debug)]
pub struct ShapeContainer {
    pub shapes: Vec<Box<dyn Shape>>,
}

impl ShapeContainer {
    pub fn new() -> ShapeContainer {
        ShapeContainer { shapes: Vec::new() }
    }

    pub fn add_shape(&mut self, shape: Box<dyn Shape>) {
        self.shapes.push(shape);
    }

    pub fn all(&self) -> Vec<&dyn Shape> {
        self.shapes.iter().map(|s| s.as_ref()).collect()
    }

    pub fn shape_at(&self, i: usize) -> &dyn Shape {
        self.shapes[i].as_ref()
    }

    pub fn shape_at_as_mut(&mut self, i: usize) -> &mut dyn Shape {
        self.shapes[i].as_mut()
    }

    pub fn len(&self) -> usize {
        self.shapes.len()
    }

    pub fn get_shape(&self, id: Uuid) -> Option<&dyn Shape> {
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
}

#[cfg(test)]
mod tests {
    use std::f64::consts::PI;

    use crate::{
        shapes::{Group, Sphere},
        Point, Transform, Vector,
    };

    use super::*;

    // Chapter 14 Groups
    // Page 198
    #[test]
    fn converting_a_point_from_world_to_object_space() {
        let mut go = ShapeContainer::new();

        let mut g1 = Group::new();
        g1.transform = Transform::new().rotation_y(PI / 2.0).build();

        let mut g2 = Group::new();
        g2.transform = Transform::new().scaling(2.0, 2.0, 2.0).build();
        g2.parent_id = Some(g1.id);

        let mut s = Sphere::new();
        s.transform = Transform::new().translation(5.0, 0.0, 0.0).build();
        s.parent_id = Some(g2.id);
        let s_id = s.id;

        g2.add_shape(Box::new(s));
        g1.add_shape(Box::new(g2));
        go.add_shape(Box::new(g1));

        let s = go.get_shape(s_id).unwrap();
        let p = s.world_to_object(Point::new(-2.0, 0.0, -10.0), &go);
        assert_eq!(p, Point::new(0.0, 0.0, -1.0));
    }

    // Chapter 14 Groups
    // Page 198 & 199
    #[test]
    fn converting_a_normal_from_object_to_world_space() {
        let mut g1 = Group::new();
        g1.transform = Transform::new().rotation_y(PI / 2.0).build();

        let mut g2 = Group::new();
        g2.transform = Transform::new().scaling(1.0, 2.0, 3.0).build();
        g2.parent_id = Some(g1.id);

        let mut s = Sphere::new();
        s.transform = Transform::new().translation(5.0, 0.0, 0.0).build();
        s.parent_id = Some(g2.id);
        let s_id = s.id;

        let mut go = ShapeContainer::new();

        g2.add_shape(Box::new(s));
        g1.add_shape(Box::new(g2));
        go.add_shape(Box::new(g1));

        let p = go.get_shape(s_id).unwrap().normal_to_world(
            Vector::new(3_f64.sqrt() / 3.0, 3_f64.sqrt() / 3.0, 3_f64.sqrt() / 3.0),
            &go,
        );
        assert_eq!(p, Vector::new(0.2857, 0.4286, -0.8571));
    }
}
