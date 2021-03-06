use super::color;
use crate::{
    shapes::{Shape, Sphere},
    Color, Computations, Intersection, Point, PointLight, Ray, ShapeContainer, Transform,
};
use uuid::Uuid;

#[derive(Debug)]
pub struct World {
    pub lights: Vec<PointLight>,
    shapes: ShapeContainer,
}

impl World {
    pub fn new() -> Self {
        World {
            lights: Vec::new(),
            shapes: ShapeContainer::new(),
        }
    }

    pub fn add_shape(&mut self, shape: Box<dyn Shape>) {
        self.shapes.add_shape(shape);
    }

    pub fn get_shape(&self, id: Uuid) -> Option<&dyn Shape> {
        let mut shape = None;
        for s in self.shapes.all() {
            if s.id() == id {
                shape = Some(s);
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

    pub fn get_shape_at(&self, i: usize) -> &dyn Shape {
        self.shapes.shape_at(i)
    }

    pub fn intersect(&self, r: Ray) -> Option<Vec<Intersection>> {
        let mut xs: Vec<Intersection> = Vec::new();
        for o in self.shapes.all() {
            if let Some(o_xs) = o.intersect(r) {
                for i in o_xs {
                    xs.push(i);
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

    pub fn shade_hit(&self, comps: &Computations, remaining: usize) -> Color {
        let in_shadow = self.is_shadow(comps.over_point);

        let material = comps.object.material();

        let surface = material.lighting(
            comps.object,
            self.lights[0],
            comps.over_point,
            comps.eyev,
            comps.normalv,
            in_shadow,
        );

        let reflected = self.reflected_color(comps, remaining);
        let refracted = self.refracted_color(comps, remaining);

        if material.reflective > 0.0 && material.transparency > 0.0 {
            let reflectance = comps.schlick();
            surface + reflected * reflectance + refracted * (1.0 - reflectance)
        } else {
            surface + reflected + refracted
        }
    }

    pub fn color_at(&self, r: Ray, remaining: usize) -> Color {
        match self.intersect(r) {
            Some(xs) => match Intersection::hit(&xs) {
                Some(h) => {
                    let comps = Intersection::prepare_computations(&h, r, &xs, Some(&self.shapes));
                    self.shade_hit(&comps, remaining)
                }
                None => color::WHITE,
            },
            None => color::WHITE,
        }
    }

    pub fn is_shadow(&self, point: Point) -> bool {
        let v = self.lights[0].position - point;
        let distance = v.magnitude();
        let direction = v.normalize();

        let mut results = false;
        let r = Ray::new(point, direction);
        if let Some(xs) = self.intersect(r) {
            if let Some(hit) = Intersection::hit(&xs) {
                if hit.t < distance && hit.object.cast_shadow() {
                    results = true;
                }
            }
        }
        results
    }

    pub fn reflected_color(&self, comps: &Computations, remaining: usize) -> Color {
        if comps.object.material().reflective == 0.0 || remaining < 1 {
            color::BLACK
        } else {
            let reflect_ray = Ray::new(comps.over_point, comps.reflectv);
            let color = self.color_at(reflect_ray, remaining - 1);
            color * comps.object.material().reflective
        }
    }

    pub fn refracted_color(&self, comps: &Computations, remaining: usize) -> Color {
        if comps.object.material().transparency == 0.0 || remaining == 0 {
            color::BLACK
        } else {
            let n_ratio = comps.n1 / comps.n2;
            let cos_i = comps.eyev.dot(comps.normalv);
            let sin2_t = n_ratio.powf(2.0) * (1.0 - cos_i.powf(2.0));

            if sin2_t > 1.0 {
                color::BLACK
            } else {
                let cos_t = (1.0 - sin2_t).sqrt();
                let direction = comps.normalv * (n_ratio * cos_i - cos_t) - comps.eyev * n_ratio;
                let refract_ray = Ray::new(comps.under_point, direction);
                self.color_at(refract_ray, remaining - 1) * comps.object.material().transparency
            }
        }
    }
}

impl Default for World {
    fn default() -> Self {
        let mut w = World::new();

        let light = PointLight::new(Point::new(-10.0, 10.0, -10.0), Color::new(1.0, 1.0, 1.0));
        w.lights.push(light);

        let mut s1 = Sphere::new();
        s1.material.color = Color::new(0.8, 1.0, 0.6);
        s1.material.diffuse = 0.7;
        s1.material.specular = 0.2;
        w.add_shape(Box::new(s1));

        let mut s2 = Sphere::new();
        s2.transform = Transform::new().scaling(0.5, 0.5, 0.5).build();
        w.add_shape(Box::new(s2));

        w
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        patterns::TestPattern, shapes::Plane, shapes::Sphere, Color, Intersection, Material, Point,
        PointLight, Ray, Transform, Vector,
    };

    #[test]
    fn creating_world() {
        let w = World::new();
        assert_eq!(0, w.lights.len());
        assert_eq!(0, w.shapes.len());
    }

    #[test]
    fn default_world() {
        let light = PointLight::new(Point::new(-10.0, 10.0, -10.0), Color::new(1.0, 1.0, 1.0));
        let mut s1 = Sphere::new();
        s1.material.color = Color::new(0.8, 1.0, 0.6);
        s1.material.diffuse = 0.7;
        s1.material.specular = 0.2;
        let mut s2 = Sphere::new();
        s2.transform = Transform::new().scaling(0.5, 0.5, 0.5).build();

        let w = World::default();
        assert_eq!(1, w.lights.len());
        assert_eq!(2, w.shapes.len());
        assert_eq!(light, w.lights[0]);
        let tr = w.get_shape_at(1).transform();
        assert_eq!(tr, Transform::new().scaling(0.5, 0.5, 0.5).build());
    }

    #[test]
    fn intersecting_world_with_ray() {
        let w = World::default();
        let r = Ray::new(Point::new(0.0, 0.0, -5.0), Vector::new(0.0, 0.0, 1.0));
        let xs = w.intersect(r).unwrap();
        assert_eq!(4, xs.len());
        assert_eq!(4.0, xs[0].t);
        assert_eq!(4.5, xs[1].t);
        assert_eq!(5.5, xs[2].t);
        assert_eq!(6.0, xs[3].t);
    }

    #[test]
    pub fn shading_intersection() {
        let w = World::default();
        let r = Ray::new(Point::new(0.0, 0.0, -5.0), Vector::new(0.0, 0.0, 1.0));
        let shape = w.get_shape_at(0);
        let i = Intersection::new(4.0, shape);
        let comps = Intersection::prepare_computations(&i, r, &vec![i], None);
        let c = w.shade_hit(&comps, 5);
        assert_eq!(Color::new(0.38066, 0.47583, 0.2855), c);
    }

    #[test]
    fn no_shadow_nothing_collinear_point_and_light() {
        let w = World::default();
        let p = Point::new(0.0, 10.0, 0.0);
        assert!(!w.is_shadow(p));
    }

    #[test]
    fn shadow_object_between_point_and_light() {
        let w = World::default();
        let p = Point::new(10.0, -10.0, 10.0);
        assert!(w.is_shadow(p));
    }

    #[test]
    fn no_shadow_object_behind_light() {
        let w = World::default();
        let p = Point::new(-20.0, -20.0, 20.0);
        assert!(!w.is_shadow(p));
    }

    #[test]
    fn no_shadow_object_behind_point() {
        let w = World::default();
        let p = Point::new(-2.0, 2.0, -2.0);
        assert!(!w.is_shadow(p));
    }

    #[test]
    fn shade_hit_is_given_intersection_in_shadow() {
        let mut w = World::new();
        let light = PointLight::new(Point::new(0.0, 0.0, -10.0), Color::new(1.0, 1.0, 1.0));
        w.lights.push(light);

        let s1 = Sphere::new();
        w.add_shape(Box::new(s1));

        let mut s2 = Sphere::new();
        s2.transform = Transform::new().translation(0.0, 0.0, 10.0).build();
        w.add_shape(Box::new(s2));

        let r = Ray::new(Point::new(0.0, 0.0, 5.0), Vector::new(0.0, 0.0, 1.0));
        let i = Intersection::new(4.0, w.get_shape_at(1));
        let comps = Intersection::prepare_computations(&i, r, &vec![i], None);
        let c = w.shade_hit(&comps, 5);
        assert_eq!(c, Color::new(0.1, 0.1, 0.1));
    }

    #[test]
    fn reflected_color_nonreflective_material() {
        let mut w = World::default();
        let r = Ray::new(Point::new(0.0, 0.0, 0.0), Vector::new(0.0, 0.0, 1.0));
        w.shapes.shape_at_as_mut(0).material_mut().ambient = 1.0;
        let i = Intersection::new(1.0, w.get_shape_at(1));
        let comps = Intersection::prepare_computations(&i, r, &vec![i], None);
        let color = w.reflected_color(&comps, 5);
        assert_eq!(color, color::BLACK);
    }

    #[test]
    fn reflected_color_reflective_material() {
        let mut w = World::default();
        let mut shape = Plane::new();
        shape.material.reflective = 0.5;
        shape.transform = Transform::new().translation(0.0, -1.0, 0.0).build();
        w.add_shape(Box::new(shape));
        let r = Ray::new(
            Point::new(0.0, 0.0, -3.0),
            Vector::new(0.0, -2_f64.sqrt() / 2.0, 2_f64.sqrt() / 2.0),
        );
        let i = Intersection::new(2_f64.sqrt(), w.get_shape_at(2));
        let comps = Intersection::prepare_computations(&i, r, &vec![i], None);
        let color = w.reflected_color(&comps, 1);
        assert_eq!(color, Color::new(0.190332, 0.237915, 0.1427492));
    }

    #[test]
    fn shade_hit_with_reflective_material() {
        let mut w = World::default();
        let mut shape = Plane::new();
        shape.material.reflective = 0.5;
        shape.transform = Transform::new().translation(0.0, -1.0, 0.0).build();
        w.add_shape(Box::new(shape));
        let r = Ray::new(
            Point::new(0.0, 0.0, -3.0),
            Vector::new(0.0, -2_f64.sqrt() / 2.0, 2_f64.sqrt() / 2.0),
        );
        let i = Intersection::new(2_f64.sqrt(), w.get_shape_at(2));
        let comps = Intersection::prepare_computations(&i, r, &vec![i], None);
        let color = w.shade_hit(&comps, 1);
        assert_eq!(color, Color::new(0.8767577, 0.924340789, 0.829174629));
    }

    #[test]
    fn color_at_with_mutually_reflective_surfaces() {
        let mut w = World::new();
        w.lights.push(PointLight::new(
            Point::new(0.0, 0.0, 0.0),
            Color::new(1.0, 1.0, 1.0),
        ));
        let mut lower = Plane::new();
        lower.material.reflective = 1.0;
        lower.transform = Transform::new().translation(0.0, -1.0, 0.0).build();
        w.add_shape(Box::new(lower));
        let mut upper = Plane::new();
        upper.material.reflective = 1.0;
        upper.transform = Transform::new().translation(0.0, 1.0, 0.0).build();
        w.add_shape(Box::new(upper));
        let r = Ray::new(Point::new(0.0, 0.0, 0.0), Vector::new(0.0, 1.0, 0.0));
        w.color_at(r, 5);
    }

    #[test]
    fn reflected_color_at_maximum_recursive_depth() {
        let mut w = World::default();
        let mut shape = Plane::new();
        shape.material.reflective = 0.5;
        shape.transform = Transform::new().translation(0.0, -1.0, 0.0).build();
        w.add_shape(Box::new(shape));
        let r = Ray::new(
            Point::new(0.0, 0.0, -3.0),
            Vector::new(0.0, -2_f64.sqrt() / 2.0, 2_f64.sqrt() / 2.0),
        );
        let i = Intersection::new(2_f64.sqrt(), w.get_shape_at(1));
        let comps = Intersection::prepare_computations(&i, r, &vec![i], None);
        let color = w.reflected_color(&comps, 0);
        assert_eq!(color, color::BLACK);
    }

    // Chapter 11 Reflection and Refraction
    // Page 154
    #[test]
    fn refracted_color_with_an_opaque_surface() {
        let w = World::default();
        let shape = w.get_shape_at(0);
        let r = Ray::new(Point::new(0.0, 0.0, -5.0), Vector::new(0.0, 0.0, 1.0));
        let i1 = Intersection::new(4.0, shape);
        let i2 = Intersection::new(6.0, shape);
        let xs = vec![i1.clone(), i2.clone()];
        let comps = Intersection::prepare_computations(&i1, r, &xs, None);
        let c = w.refracted_color(&comps, 5);
        assert_eq!(c, color::BLACK);
    }

    // Chapter 11 Reflection and Refraction
    // Page 156
    #[test]
    fn refracted_color_at_the_maximum_recursive_depth() {
        let w = &mut World::default();

        let mut m = Material::new();
        m.transparency = 1.0;
        m.refractive_index = 1.5;
        w.shapes.shape_at_as_mut(0).set_material(m);

        let r = Ray::new(Point::new(0.0, 0.0, -5.0), Vector::new(0.0, 0.0, 1.0));
        let i1 = Intersection::new(4.0, w.get_shape_at(0));
        let i2 = Intersection::new(6.0, w.get_shape_at(0));
        let xs = vec![i1.clone(), i2.clone()];
        let comps = Intersection::prepare_computations(&i1, r, &xs, None);
        let c = w.refracted_color(&comps, 0);
        assert_eq!(c, color::BLACK);
    }

    // Chapter 11 Reflection and Refraction
    // Page 157
    #[test]
    fn refracted_color_under_total_internal_reflection() {
        let mut w = World::default();

        let mut m = Material::new();
        m.transparency = 1.0;
        m.refractive_index = 1.5;
        w.shapes.shape_at_as_mut(0).set_material(m);

        let r = Ray::new(
            Point::new(0.0, 0.0, 2_f64.sqrt() / 2.0),
            Vector::new(0.0, 1.0, 0.0),
        );
        let i1 = Intersection::new(-2_f64.sqrt() / 2.0, w.get_shape_at(0));
        let i2 = Intersection::new(2_f64.sqrt() / 2.0, w.get_shape_at(0));
        let xs = vec![i1.clone(), i2.clone()];
        let comps = Intersection::prepare_computations(&i2, r, &xs, None);
        let c = w.refracted_color(&comps, 5);
        assert_eq!(c, color::BLACK);
    }

    // Chapter 11 Reflection and Refraction
    // Page 158
    #[test]
    fn the_refracted_color_with_a_refracted_ray() {
        let mut w = World::default();

        let mut am = Material::new();
        am.ambient = 1.0;
        am.pattern = Some(Box::new(TestPattern::new()));
        w.shapes.shape_at_as_mut(0).set_material(am);

        let mut bm = Material::new();
        bm.transparency = 1.0;
        bm.refractive_index = 1.5;
        w.shapes.shape_at_as_mut(1).set_material(bm);

        let r = Ray::new(Point::new(0.0, 0.0, 0.1), Vector::new(0.0, 1.0, 0.0));
        let i1 = Intersection::new(-0.9899, w.get_shape_at(0));
        let i2 = Intersection::new(-0.4899, w.get_shape_at(1));
        let i3 = Intersection::new(0.4899, w.get_shape_at(1));
        let i4 = Intersection::new(0.9899, w.get_shape_at(0));
        let xs = vec![i1.clone(), i2.clone(), i3.clone(), i4.clone()];
        let comps = Intersection::prepare_computations(&i3, r, &xs, None);
        let c = w.refracted_color(&comps, 5);
        assert_eq!(c, Color::new(0.0, 0.99888, 0.04725));
    }

    // Chapter 11 Reflection and Refraction
    // Page 159
    #[test]
    fn shade_hit_with_a_transparent_material() {
        let mut w = World::default();

        let mut floor = Plane::new();
        floor.transform = Transform::new().translation(0.0, -1.0, 0.0).build();
        floor.material.transparency = 0.5;
        floor.material.refractive_index = 1.5;
        w.add_shape(Box::new(floor));

        let mut ball = Sphere::new();
        ball.material.color = Color::new(1.0, 0.0, 0.0);
        ball.material.ambient = 0.5;
        ball.transform = Transform::new().translation(0.0, -3.5, -0.5).build();
        w.add_shape(Box::new(ball));

        let r = Ray::new(
            Point::new(0.0, 0.0, -3.0),
            Vector::new(0.0, -2_f64.sqrt() / 2.0, 2_f64.sqrt() / 2.0),
        );

        let i1 = Intersection::new(2_f64.sqrt(), w.get_shape_at(2));
        let xs = vec![i1.clone()];

        let comps = Intersection::prepare_computations(&i1, r, &xs, None);
        let c = w.shade_hit(&comps, 5);
        assert_eq!(c, Color::new(0.93642, 0.68642, 0.68642));
    }

    // Chapter 11 Reflection and Refraction
    // Page 164
    #[test]
    fn shade_hit_with_a_reflective_transparent_material() {
        let mut w = World::default();

        let mut floor = Plane::new();
        floor.transform = Transform::new().translation(0.0, -1.0, 0.0).build();
        floor.material.reflective = 0.5;
        floor.material.transparency = 0.5;
        floor.material.refractive_index = 1.5;
        w.add_shape(Box::new(floor));

        let mut ball = Sphere::new();
        ball.material.color = Color::new(1.0, 0.0, 0.0);
        ball.material.ambient = 0.5;
        ball.transform = Transform::new().translation(0.0, -3.5, -0.5).build();
        w.add_shape(Box::new(ball));

        let r = Ray::new(
            Point::new(0.0, 0.0, -3.0),
            Vector::new(0.0, -2_f64.sqrt() / 2.0, 2_f64.sqrt() / 2.0),
        );

        let i1 = Intersection::new(2_f64.sqrt(), w.get_shape_at(2));
        let xs = vec![i1.clone()];

        let comps = Intersection::prepare_computations(&i1, r, &xs, None);
        let c = w.shade_hit(&comps, 5);
        assert_eq!(c, Color::new(0.93391, 0.69643, 0.69243));
    }
}
