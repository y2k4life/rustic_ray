use crate::{EPSILON, Point, Ray, Vector, float_cmp};
use crate::shapes::Shape;
use std::cmp::Ordering;

#[derive(Debug, Clone)]
pub struct Intersection<'a> {
    pub t: f64,
    pub object: &'a dyn Shape,
}

#[derive(Debug, Clone)]
pub struct Intersection2<'a> {
    pub t: f64,
    pub object: &'a Box<dyn Shape>,
}

pub struct Computations<'a> {
    pub t: f64,
    pub object: &'a dyn Shape,
    pub point: Point,
    pub eyev: Vector,
    pub normalv: Vector,
    pub inside: bool,
    pub over_point: Point,
    pub under_point: Point,
    pub reflectv: Vector,
    pub n1: f64,
    pub n2: f64,
}

impl Intersection<'_> {
    pub fn hit<'a>(xs: &'a [Intersection]) -> Option<Intersection<'a>> {
        let mut hit = None;
        let mut min_time = 0.0;
        for i in xs.iter().filter(|x| x.t >= 0.0) {
            if i.t < min_time || min_time == 0.0 {
                hit = Some(Intersection::new(i.t, i.object));
                min_time = i.t;
            }
        }

        hit
    }
}

impl<'a> Intersection<'a> {
    pub fn new(t: f64, object:&dyn Shape) -> Intersection {
        Intersection { t, object: object }
    }

    pub fn prepare_computations<'h>(hit: &'h Intersection, r: Ray, xs: &[Intersection]) -> Computations<'h> {
        let point = r.position(hit.t);
        let mut normalv = hit.object.normal_at(point);
        let mut inside = false;
        if normalv.dot(-r.direction) < 0.0 {
            inside = true;
            normalv = -normalv;
        }
        let over_point = point + normalv * EPSILON;
        let under_point = point - normalv * EPSILON;

        let reflectv = r.direction.reflect(normalv);

        let mut n1 = 0.0;
        let mut n2 = 0.0;
        let mut container: Vec<&dyn Shape> = Vec::new();
        for i in xs.iter() {
            if i == hit {
                if container.len() == 0 {
                    n1 = 1.0;
                }
                else if let Some(object) = container.last() {
                    n1 = object.material().refractive_index;
                }
            }

            if container.contains(&i.object) {
                container = container.into_iter().filter(|o| *o != i.object.clone()).collect();
            }
            else {
                container.push(i.object.clone());
            }

            if i == hit {
                if container.len() == 0 {
                    n2 = 1.0;
                }
                else if let Some(object) = container.last() {
                    n2 = object.material().refractive_index;
                }

                break;
            }
        }

        Computations {
            t: hit.t,
            object: hit.object,
            point,
            eyev: -r.direction,
            normalv,
            inside,
            over_point,
            under_point,
            reflectv,
            n1,
            n2,
        }
    }
}


impl PartialEq for Intersection<'_> {
    fn eq(&self, other: &Intersection) -> bool {
        self.t == other.t && &self.object == &other.object
    }
}

impl PartialOrd for Intersection<'_> {
    fn partial_cmp(&self, other: &Intersection) -> Option<Ordering> {
        Some(float_cmp(self.t, other.t))
    }
}

impl Computations<'_> {
    pub fn schlick(&self) -> f64 {
        // find the cosine of the angle between the eye and normal vector
        let mut cos = self.eyev.dot(self.normalv);

        // total internal reflection can only occur if n1 > n2
        if self.n1 > self.n2 {
            let n = self.n1 /self.n2;
            let sin2_t = n.powf(2.0) * (1.0 - cos.powf(2.0));
            if sin2_t > 1.0 {
                return 1.0;
            }

            // computer cosine of theta_t using trig identity
            // when n1 > n2 use cos(theta_t) instead
            cos = (1.0 - sin2_t).sqrt();
        }

        let r0 = ((self.n1 - self.n2) / (self.n1 + self.n2)).powf(2.0);
        r0 + (1.0 - r0) * (1.0 - cos).powf(5.0)
    }
}

#[cfg(test)]
mod tests {
    use crate::{EPSILON, Point, Ray, Transform, Vector, shapes::Plane, shapes::Sphere, float_eq};
    use super::*;
    
    // Chapter 5 Ray-Sphere Intersections
    // Page 63
    #[test]
    fn intersection_encapsulates_t_and_shape() {
        let s = Sphere::new();
        let i = Intersection::new(3.5, &s);
        assert_eq!(3.5, i.t);
        assert!(i.object.shape_eq(&s));
    }

    // Chapter 5 Ray-Sphere Intersections
    // Page 64
    #[test]
    fn aggregating_intersections() {
        let s = Sphere::new();
        let xs = vec![
            Intersection::new(1.0, &s),
            Intersection::new(2.0, &s),
        ];
        assert_eq!(2, xs.len());
        assert_eq!(1.0, xs[0].t);
        assert_eq!(2.0, xs[1].t);
    }

    // Chapter 5 Ray-Sphere Intersections
    // Page 65
    #[test]
    fn the_hit_all_intersections_positive_t() {
        let s = Sphere::new();
        let xs = vec![
            Intersection::new(2.0, &s),
            Intersection::new(1.0, &s),
        ];
        let hit = Intersection::hit(&xs);
        if let Some(i) = hit {
            assert_eq!(1.0, i.t);
        }
    }
  
    // Chapter 5 Ray-Sphere Intersections
    // Page 65
    #[test]
    fn the_hit_all_intersections_some_negative_t() {
        let s = Sphere::new();
        let xs = vec![
            Intersection::new(-1.0, &s),
            Intersection::new(1.0, &s),
        ];
        let hit = Intersection::hit(&xs);
        if let Some(i) = hit {
            assert_eq!(1.0, i.t);
        }
    }

    // Chapter 5 Ray-Sphere Intersections
    // Page 65
    #[test]
    fn the_hit_all_intersections_have_negative_t() {
        let s = Sphere::new();
        let xs = vec![
            Intersection::new(-2.0, &s),
            Intersection::new(-1.0, &s),
        ];
        let hit = Intersection::hit(&xs);
        assert_eq!(None, hit);
    }

    // Chapter 5 Ray-Sphere Intersections
    // Page 66
    #[test]
    fn the_hit_always_lowest_nonnegative_intersections() {
        let s = Sphere::new();
        let xs = vec![
            Intersection::new(52.0, &s),
            Intersection::new(7.0, &s),
            Intersection::new(-3.0, &s),
            Intersection::new(2.0, &s),
        ];
        let hit = Intersection::hit(&xs);
        if let Some(i) = hit {
            assert_eq!(2.0, i.t);
        }
    }

    // Chapter 7 Making a Scene
    // Page 93
    #[test]
    fn precomputing_state_of_intersection() {
        let r = Ray::new(Point::new(0.0, 0.0, -5.0), Vector::new(0.0, 0.0, 1.0));
        let shape = Sphere::new();
        let i = Intersection::new(4.0, &shape);
        let comps = Intersection::prepare_computations(&i, r, &vec![i.clone()]);
        assert_eq!(comps.t, i.t);
        assert_eq!(comps.point, Point::new(0.0, 0.0, -1.0));
        assert_eq!(comps.eyev, Vector::new(0.0, 0.0, -1.0));
        assert_eq!(comps.normalv, Vector::new(0.0, 0.0, -1.0));
    }

    // Chapter 7 Making a Scene
    // Page 94
    #[test]
    fn hit_when_occurs_intersection_occurs_outside() {
        let r = Ray::new(Point::new(0.0, 0.0, -5.0), Vector::new(0.0, 0.0, 1.0));
        let shape = Sphere::new();
        let i = Intersection::new(4.0, &shape);
        let comps = Intersection::prepare_computations(&i, r, &vec![i.clone()]);
        assert_eq!(comps.inside, false);
    }

    // Chapter 7 Making a Scene
    // Page 95
    #[test]
    fn hit_when_intersection_occurs_inside() {
        let r = Ray::new(Point::new(0.0, 0.0, 0.0), Vector::new(0.0, 0.0, 1.0));
        let shape = Sphere::new();
        let i = Intersection::new(1.0, &shape);
        let comps = Intersection::prepare_computations(&i, r, &vec![i.clone()]);
        assert_eq!(comps.point, Point::new(0.0, 0.0, 1.0));
        assert_eq!(comps.eyev, Vector::new(0.0, 0.0, -1.0));
        assert_eq!(comps.inside, true);
        assert_eq!(comps.normalv, Vector::new(0.0, 0.0, -1.0));
    }
    
    // Chapter 8 Shadows
    // Page 115
    #[test]
    fn the_hit_should_offset_point() {
        let r = Ray::new(Point::new(0.0, 0.0, -5.0), Vector::new(0.0, 0.0, 1.0));
        let mut shape = Sphere::new();
        shape.transform = Transform::new().translation(0.0, 0.0, 1.0).build();
        let i = Intersection::new(5.0, &shape);
        let comps = Intersection::prepare_computations(&i, r, &vec![i.clone()]);
        assert!(comps.over_point.z < -EPSILON / 2.0);
        assert!(comps.point.z > comps.over_point.z);
    }

    // Chapter 11 Reflection and Refraction
    // Page 143
    #[test]
    fn precomputing_reflection_vector() {
        let shape = Plane::new();
        let r = Ray::new(
            Point::new(0.0, 1.0, -1.0),
            Vector::new(0.0, -2_f64.sqrt() / 2.0, 2_f64.sqrt() / 2.0),
        );
        let i = Intersection::new(2_f64.sqrt(), &shape);
        let comps = Intersection::prepare_computations(&i, r, &vec![i.clone()]);
        assert_eq!(
            comps.reflectv,
            Vector::new(0.0, 2_f64.sqrt() / 2.0, 2_f64.sqrt() / 2.0)
        );
    }

    // Chapter 11 Reflection and Refraction
    // Page 152
    #[test]
    fn finding_n1_n2_at_various_intersections() {
        let mut a = Sphere::glass_sphere();
        a.transform = Transform::new().scaling(2.0, 2.0, 2.0).build();
        a.material.refractive_index = 1.5;
        let ia1 = Intersection::new(2.0, &a);
        let ia2 = Intersection::new(6.0, &a);

        let mut b = Sphere::glass_sphere();
        b.transform = Transform::new().translation(0.0, 0.0, -0.25).build();
        b.material.refractive_index = 2.0;
        let ib1 = Intersection::new(2.75, &b);
        let ib2 = Intersection::new(4.75, &b);

        let mut c = Sphere::glass_sphere();
        c.transform = Transform::new().translation(0.0, 0.0, 0.25).build();
        c.material.refractive_index = 2.5;
        let ic1 = Intersection::new(3.25, &c);
        let ic2 = Intersection::new(5.35, &c);

        let r = Ray::new(Point::new(0.0, 0.0, -4.0), Vector::new(0.0, 0.0, 1.0));
        let xs = vec![ia1, ib1, ic1, ib2, ic2, ia2];

        let expected = vec![
            (1.0, 1.5),
            (1.5, 2.0),
            (2.0, 2.5),
            (2.5, 2.5),
            (2.5, 1.5),
            (1.5, 1.0)];

        
        for i in 0..5 {
            let comps = Intersection::prepare_computations(&xs[i], r, &xs);
            assert_eq!(expected[i].0, comps.n1);
            assert_eq!(expected[i].1, comps.n2);
        }
    }

    // Chapter 11 Reflection and Refraction
    // Page 154
    #[test]
    fn under_point_offset_below_surface() {
        let r = Ray::new(Point::new(0.0, 0.0, -5.0), Vector::new(0.0, 0.0, 1.0));
        let mut shape = Sphere::glass_sphere();
        shape.transform = Transform::new().translation(0.0, 0.0, 1.0).build();
        let hit = Intersection::new(5.0, &shape);
        let xs = vec![hit.clone()];
        let comps = Intersection::prepare_computations(&hit, r, &xs);
        assert!(comps.under_point.z > EPSILON / 2.0);
        assert!(comps.point.z < comps.under_point.z);
    }

    // Chapter 11 Reflection and Refraction
    // Page 161
    #[test]
    fn the_schlick_approximation_under_total_internal_reflection() {
        let shape = Sphere::glass_sphere();
        let r = Ray::new(Point::new(0.0, 0.0, 2_f64.sqrt()), Vector::new(0.0, 1.0, 0.0));
        let i1 = Intersection::new(-2_f64.sqrt()/2.0, &shape);
        let i2 = Intersection::new(2_f64.sqrt()/2.0, &shape);
        let xs = vec![i1.clone(), i2.clone()];
        let comps = Intersection::prepare_computations(&i2, r, &xs);
        let reflectance = comps.schlick();
        assert_eq!(reflectance, 1.0);
    }

    // Chapter 11 Reflection and Refraction
    // Page 162
    #[test]
    fn the_schlick_approximation_with_a_perpendicular_viewing_angle() {
        let shape = Sphere::glass_sphere();
        let r = Ray::new(Point::new(0.0, 0.0, 0.0), Vector::new(0.0, 1.0, 0.0));
        let i1 = Intersection::new(-1.0, &shape);
        let i2 = Intersection::new(1.0, &shape);
        let xs = vec![i1.clone(), i2.clone()];
        let comps = Intersection::prepare_computations(&i2, r, &xs);
        let reflectance = comps.schlick();
        assert!(float_eq(reflectance, 0.04));
    }

    // Chapter 11 Reflection and Refraction
    // Page 163
    #[test]
    fn the_schlick_approximation_with_small_angle_and_n2_greater_than_n1() {
        let shape = Sphere::glass_sphere();
        let r = Ray::new(Point::new(0.0, 0.99, -2.0), Vector::new(0.0, 0.0, 1.0));
        let i1 = Intersection::new(1.8589, &shape);
        let xs = vec![i1.clone()];
        let comps = Intersection::prepare_computations(&i1, r, &xs);
        let reflectance = comps.schlick();
        assert!(float_eq(reflectance, 0.48873));
    }
}
