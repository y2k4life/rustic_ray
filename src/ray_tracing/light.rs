use crate::{Color, Point};

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct PointLight {
    pub intensity: Color,
    pub position: Point,
}

impl PointLight {
    pub fn new(position: Point, intensity: Color) -> PointLight {
        PointLight {
            position,
            intensity,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{Color, Point};

    #[test]
    fn point_light_position_intensity() {
        let intensity = Color::new(1.0, 1.0, 1.0);
        let position = Point::new(0.0, 0.0, 0.0);
        let light = PointLight::new(position, intensity);
        assert_eq!(intensity, light.intensity);
        assert_eq!(position, light.position);
    }
}
