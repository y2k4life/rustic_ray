use crate::{patterns::Pattern, shapes::Shape, Color, Point, PointLight, Vector};

#[derive(Debug, PartialEq)]
pub struct Material {
    pub color: Color,
    pub ambient: f64,
    pub diffuse: f64,
    pub specular: f64,
    pub shininess: f64,
    pub reflective: f64,
    pub transparency: f64,
    pub refractive_index: f64,
    pub pattern: Option<Box<dyn Pattern>>,
}

impl Material {
    pub fn new() -> Self {
        Material {
            color: Color::new(1.0, 1.0, 1.0),
            ambient: 0.1,
            diffuse: 0.9,
            specular: 0.9,
            shininess: 200.0,
            reflective: 0.0,
            transparency: 0.0,
            refractive_index: 1.0,
            pattern: None,
        }
    }

    pub fn lighting(
        &self,
        object: &dyn Shape,
        light: PointLight,
        point: Point,
        eyev: Vector,
        normalv: Vector,
        in_shadow: bool,
    ) -> Color {
        let color = match self.pattern.as_ref() {
            Some(pattern) => pattern.pattern_at_object(object, point),
            None => self.color,
        };
        let effective_color = color * light.intensity;
        let lightv = (light.position - point).normalize();
        let ambient = effective_color * self.ambient;
        let light_dot_normal = lightv.dot(normalv);
        let mut diffuse = Color::new(0.0, 0.0, 0.0);
        let mut specular = Color::new(0.0, 0.0, 0.0);
        if light_dot_normal >= 0.0 && !in_shadow {
            diffuse = effective_color * self.diffuse * light_dot_normal;
            let reflectv = (-lightv).reflect(normalv);
            let reflect_dot_eye = reflectv.dot(eyev);
            if reflect_dot_eye <= 0.0 {
                specular = Color::new(0.0, 0.0, 0.0);
            } else {
                let factor = reflect_dot_eye.powf(self.shininess);
                specular = light.intensity * self.specular * factor;
            }
        }
        ambient + diffuse + specular
    }
}

impl Default for Material {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        patterns::Stripe, ray_tracing::color, shapes::Sphere, Color, Point, PointLight, Vector,
    };

    #[test]
    fn lighting_eye_between_light_and_surface() {
        let m = Material::new();
        let position = Point::new(0.0, 0.0, 0.0);
        let eyev = Vector::new(0.0, 0.0, -1.0);
        let normalv = Vector::new(0.0, 0.0, -1.0);
        let light = PointLight::new(Point::new(0.0, 0.0, -10.0), Color::new(1.0, 1.0, 1.0));
        let results = m.lighting(&Sphere::new(), light, position, eyev, normalv, false);
        assert_eq!(Color::new(1.9, 1.9, 1.9), results);
    }

    #[test]
    fn lighting_eye_between_light_and_surface_eye_offset_45() {
        let m = Material::new();
        let position = Point::new(0.0, 0.0, 0.0);
        let eyev = Vector::new(0.0, 2_f64.sqrt() / 2.0, 2_f64.sqrt() / 2.0);
        let normalv = Vector::new(0.0, 0.0, -1.0);
        let light = PointLight::new(Point::new(0.0, 0.0, -10.0), Color::new(1.0, 1.0, 1.0));
        let results = m.lighting(&Sphere::new(), light, position, eyev, normalv, false);
        assert_eq!(Color::new(1.0, 1.0, 1.0), results);
    }

    #[test]
    fn lighting_eye_between_light_and_surface_light_offset_45() {
        let m = Material::new();
        let position = Point::new(0.0, 0.0, 0.0);
        let eyev = Vector::new(0.0, 0.0, -1.0);
        let normalv = Vector::new(0.0, 0.0, -1.0);
        let light = PointLight::new(Point::new(0.0, 10.0, -10.0), Color::new(1.0, 1.0, 1.0));
        let results = m.lighting(&Sphere::new(), light, position, eyev, normalv, false);
        assert_eq!(Color::new(0.7364, 0.7364, 0.7364), results);
    }

    #[test]
    fn lighting_eye_in_path_reflection_vector() {
        let m = Material::new();
        let position = Point::new(0.0, 0.0, 0.0);
        let eyev = Vector::new(0.0, -2_f64.sqrt() / 2.0, -2_f64.sqrt() / 2.0);
        let normalv = Vector::new(0.0, 0.0, -1.0);
        let light = PointLight::new(Point::new(0.0, 10.0, -10.0), Color::new(1.0, 1.0, 1.0));
        let results = m.lighting(&Sphere::new(), light, position, eyev, normalv, false);
        assert_eq!(results, Color::new(1.6364, 1.6364, 1.6364));
    }

    #[test]
    fn lighting_light_behind_surface() {
        let m = Material::new();
        let position = Point::new(0.0, 0.0, 0.0);
        let eyev = Vector::new(0.0, 0.0, -1.0);
        let normalv = Vector::new(0.0, 0.0, -1.0);
        let light = PointLight::new(Point::new(0.0, 0.0, 10.0), Color::new(1.0, 1.0, 1.0));
        let results = m.lighting(&Sphere::new(), light, position, eyev, normalv, false);
        assert_eq!(Color::new(0.1, 0.1, 0.1), results);
    }

    #[test]
    fn lighting_with_surface_in_shadow() {
        let m = Material::new();
        let position = Point::new(0.0, 0.0, 0.0);
        let eyev = Vector::new(0.0, 0.0, -1.0);
        let normalv = Vector::new(0.0, 0.0, -1.0);
        let light = PointLight::new(Point::new(0.0, 0.0, -10.0), Color::new(1.0, 1.0, 1.0));
        let in_shadow = true;
        let result = m.lighting(&Sphere::new(), light, position, eyev, normalv, in_shadow);
        assert_eq!(result, Color::new(0.1, 0.1, 0.1));
    }

    #[test]
    fn lighting_with_pattern_applied() {
        let mut m = Material::new();

        m.pattern = Some(Box::new(Stripe::new(color::WHITE, color::BLACK)));
        m.ambient = 1.0;
        m.diffuse = 0.0;
        m.specular = 0.0;
        let eyev = Vector::new(0.0, 0.0, -1.0);
        let normalv = Vector::new(0.0, 0.0, -1.0);
        let light = PointLight::new(Point::new(0.0, 0.0, -10.0), Color::new(1.0, 1.0, 1.0));
        let c1 = m.lighting(
            &Sphere::new(),
            light,
            Point::new(0.9, 0.0, 0.0),
            eyev,
            normalv,
            false,
        );
        let c2 = m.lighting(
            &Sphere::new(),
            light,
            Point::new(1.1, 0.0, 0.0),
            eyev,
            normalv,
            false,
        );
        assert_eq!(c1, color::WHITE);
        assert_eq!(c2, color::BLACK);
    }

    #[test]
    fn reflectivity_for_default_material() {
        let m = Material::new();
        assert_eq!(m.reflective, 0.0);
    }

    // Chapter 11 - Reflection and Refraction
    // Page 150
    #[test]
    fn transparency_refractive_index_default_material() {
        let m = Material::new();
        assert_eq!(m.transparency, 0.0);
        assert_eq!(m.refractive_index, 1.0);
    }
}
