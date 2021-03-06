use std::any::Any;

use uuid::Uuid;

use super::Shape;
use crate::{ray_tracing::matrix::IDENTITY, Intersection, Material, Matrix, Point, Ray, Vector};

#[derive(Debug)]
pub struct TestShape {
    pub id: Uuid,
    pub parent_id: Option<Uuid>,
    pub transform: Matrix,
    pub material: Material,
}

impl TestShape {
    pub fn new() -> TestShape {
        TestShape {
            id: Uuid::new_v4(),
            parent_id: None,
            transform: IDENTITY,
            material: Material::new(),
        }
    }
}

impl Shape for TestShape {
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

    fn local_intersect(&self, ray: Ray) -> Option<Vec<Intersection>> {
        Some(vec![
            Intersection::new(ray.origin.x, self),
            Intersection::new(ray.origin.y, self),
            Intersection::new(ray.origin.z, self),
            Intersection::new(ray.direction.x, self),
            Intersection::new(ray.direction.y, self),
            Intersection::new(ray.direction.z, self),
        ])
    }

    fn local_normal_at(&self, point: Point) -> Vector {
        Vector::new(point.x, point.y, point.z)
    }
}

impl PartialEq for TestShape {
    fn eq(&self, other: &Self) -> bool {
        self.transform == other.transform && self.material == other.material
    }
}

#[cfg(test)]
mod tests {
    use super::TestShape;
    use crate::{shapes::Shape, Point, Ray, Transform, Vector};

    #[test]
    fn name() {
        let mut t = TestShape::new();
        let r = Ray::new(Point::new(0.0, 0.0, -5.0), Vector::new(0.0, 0.0, 1.0));
        t.set_transform(Transform::new().scaling(2.0, 2.0, 2.0).build());
        if let Some(xs) = t.intersect(r) {
            assert_eq!(xs[0].t, 0.0);
            assert_eq!(xs[1].t, 0.0);
            assert_eq!(xs[2].t, -2.5);
            assert_eq!(xs[3].t, 0.0);
            assert_eq!(xs[4].t, 0.0);
            assert_eq!(xs[5].t, 0.5);
        }
    }

    #[test]
    fn intersect_translated_shape_with_ray() {
        let mut t = TestShape::new();
        let r = Ray::new(Point::new(0.0, 0.0, -5.0), Vector::new(0.0, 0.0, 1.0));
        t.set_transform(Transform::new().translation(5.0, 0.0, 0.0).build());
        if let Some(xs) = t.intersect(r) {
            assert_eq!(xs[0].t, -5.0);
            assert_eq!(xs[1].t, 0.0);
            assert_eq!(xs[2].t, -5.0);
            assert_eq!(xs[3].t, 0.0);
            assert_eq!(xs[4].t, 0.0);
            assert_eq!(xs[5].t, 1.0);
        }
    }

    #[test]
    fn computing_normal_on_translated_shape() {
        let mut s = TestShape::new();
        s.set_transform(Transform::new().translation(0.0, 1.0, 0.0).build());
        let n = s.normal_at(Point::new(0.0, 1.7071, -0.70711), None);
        assert_eq!(n, Vector::new(0.0, 0.70711, -0.70711));
    }
}
