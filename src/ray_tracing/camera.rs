use crate::{Canvas, Color, Matrix, Point, Ray, World};
use image::{Rgb, RgbImage};
use rand::prelude::*;

use super::matrix::IDENTITY;

pub struct Camera {
    hsize: usize,
    vsize: usize,
    pub transform: Matrix,
    half_width: f64,
    half_height: f64,
    pixel_size: f64,
}

impl Camera {
    pub fn new(hsize: usize, vsize: usize, field_of_view: f64) -> Camera {
        let half_view = (field_of_view / 2.0).tan();
        let aspect = hsize as f64 / vsize as f64;

        let mut half_width = half_view * aspect;
        let mut half_height = half_view;

        if aspect >= 1.0 {
            half_width = half_view;
            half_height = half_view / aspect;
        }

        let pixel_size = (half_width * 2.0) / hsize as f64;

        Camera {
            hsize,
            vsize,
            transform: IDENTITY,
            half_width,
            half_height,
            pixel_size,
        }
    }

    pub fn ray_for_pixel(&mut self, px: f64, py: f64) -> Ray {
        let x_offset = (px + 0.5) * self.pixel_size;
        let y_offset = (py + 0.5) * self.pixel_size;

        let world_x = self.half_width - x_offset;
        let world_y = self.half_height - y_offset;

        let pixel = self.transform.inverse() * Point::new(world_x, world_y, -1.0);
        let origin = self.transform.inverse() * Point::new(0.0, 0.0, 0.0);
        let direction = (pixel - origin).normalize();

        Ray::new(origin, direction)
    }

    pub fn render(&mut self, world: World, ss: usize, rd: usize) -> Canvas {
        let mut canvas = Canvas::new(self.hsize, self.vsize);

        for y in 0..self.vsize {
            for x in 0..self.hsize {
                let ray = self.ray_for_pixel(x as f64, y as f64);
                let mut color = world.color_at(ray, rd);
                if ss > 0 {
                    for _s in 0..ss {
                        let ro = self.get_ray_offset(x as f64, y as f64);
                        let co = world.color_at(ro, rd);
                        color = color + co;
                    }

                    color = Color::new(
                        color.red / ss as f64,
                        color.green / ss as f64,
                        color.blue / ss as f64,
                    );
                }
                canvas.pixels[x][y] = color;
            }
        }

        canvas
    }

    pub fn render_to_file(&mut self, world: World, ss: usize, rd: usize, file_name: &str) {
        let mut img = RgbImage::new(self.hsize as u32, self.vsize as u32);
        for (x, y, pixel) in img.enumerate_pixels_mut() {
            let ray = self.ray_for_pixel(x as f64, y as f64);

            let mut color = world.color_at(ray, rd);

            if ss > 0 {
                for _s in 0..ss {
                    let ro = self.get_ray_offset(x as f64, y as f64);
                    let co = world.color_at(ro, rd);
                    color = color + co;
                }

                color = Color::new(
                    color.red / ss as f64,
                    color.green / ss as f64,
                    color.blue / ss as f64,
                );
            }

            let pixel_color = color.to_rgb();
            *pixel = Rgb([pixel_color.0, pixel_color.1, pixel_color.2]);
        }

        img.save(file_name).unwrap();
    }

    pub fn get_ray_offset(&mut self, x: f64, y: f64) -> Ray {
        let mut rng = thread_rng();
        let xo = rng.gen_range(-0.9, 0.9);
        let yo = rng.gen_range(-0.9, 0.9);
        self.ray_for_pixel(x + xo, y + yo)
    }
}

#[cfg(test)]
mod tests {
    use crate::{float_eq, Camera, Color, Point, Transform, Vector, World};
    use std::f64::consts::PI;

    #[test]
    fn constructing_camera() {
        let hsize = 160;
        let vsize = 120;
        let field_of_view = PI / 2.0;
        let c = Camera::new(hsize, vsize, field_of_view);

        assert_eq!(c.hsize, 160);
        assert_eq!(c.vsize, 120);
    }

    #[test]
    fn the_pixel_size_for_horizontal_canvas() {
        let c = Camera::new(200, 125, PI / 2.0);

        assert_eq!(float_eq(c.pixel_size, 0.01), true);
    }

    #[test]
    fn the_pixel_size_for_vertical_canvas() {
        let c = Camera::new(125, 200, PI / 2.0);

        assert_eq!(float_eq(c.pixel_size, 0.01), true);
    }

    #[test]
    fn constructing_a_ray_through_the_center_of_canvas() {
        let mut c = Camera::new(201, 101, PI / 2.0);
        let r = c.ray_for_pixel(100.0, 50.0);

        assert_eq!(r.origin, Point::new(0.0, 0.0, 0.0));
        assert_eq!(r.direction, Vector::new(0.0, 0.0, -1.0));
    }

    #[test]
    fn constructing_a_ray_through_a_center_of_canvas() {
        let mut c = Camera::new(201, 101, PI / 2.0);
        let r = c.ray_for_pixel(0.0, 0.0);

        assert_eq!(r.origin, Point::new(0.0, 0.0, 0.0));
        assert_eq!(r.direction, Vector::new(0.66519, 0.33259, -0.66851));
    }

    #[test]
    fn constructing_a_ray_through_a_transformed_camera() {
        let mut c = Camera::new(201, 101, PI / 2.0);
        c.transform = Transform::new()
            .translation(0.0, -2.0, 5.0)
            .rotation_y(PI / 4.0)
            .build();
        let r = c.ray_for_pixel(100., 50.0);

        assert_eq!(r.origin, Point::new(0.0, 2.0, -5.0));
        assert_eq!(
            r.direction,
            Vector::new(2.0_f64.sqrt() / 2.0, 0.0, -2.0_f64.sqrt() / 2.0)
        );
    }

    #[test]
    pub fn rendering_world_with_camera() {
        let w = World::default();
        let mut c = Camera::new(11, 11, PI / 2.0);
        let from = Point::new(0.0, 0.0, -5.0);
        let to = Point::new(0.0, 0.0, 0.0);
        let up = Vector::new(0.0, 1.0, 0.0);
        c.transform = Transform::view_transformation(from, to, up);
        let image = c.render(w, 0, 5);

        assert_eq!(image.pixel_at(5, 5), Color::new(0.38066, 0.47583, 0.2855));
    }
}
