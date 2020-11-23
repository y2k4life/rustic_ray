use std::{fs::File, io::Write, path::Path};

use rustic_ray::{
    shapes::{Shape, Sphere},
    Canvas, Color, Intersection, Point, PointLight, Ray,
};

fn main() {
    for i in 0..9 {
        material(i, i as f64 / 10.0)
    }
}

fn material(i: usize, a: f64) {
    let ray_origin = Point::new(0.0, 0.0, -5.0);
    let wall_z = 10.0;

    let wall_size = 7.0;
    let canvas_size = 800;

    let mut canvas = Canvas::new(canvas_size, canvas_size);

    let pixel_size = wall_size / canvas_size as f64;

    let half = wall_size / 2.0;

    let mut sphere = Sphere::new();
    sphere.material.color = Color::new(1.0, 0.2, 1.0);

    // changes this to be the range value
    sphere.material.ambient = a as f64;

    let light = PointLight::new(Point::new(-10.0, 10.0, -10.0), Color::new(1.0, 1.0, 1.0));

    for y in 0..canvas_size {
        let world_y = half - pixel_size * y as f64;
        for x in 0..canvas_size {
            let world_x = -half + pixel_size * x as f64;

            let position = Point::new(world_x, world_y, wall_z);

            let direction = (position - ray_origin).normalize();
            let ray = Ray::new(ray_origin, direction);

            match sphere.local_intersect(ray) {
                Some(xs) => match Intersection::hit(&xs) {
                    Some(hit) => {
                        let point = ray.position(hit.t);
                        let normal = hit.object.normal_at(point, None);
                        let eye = -ray.direction;

                        let color = sphere
                            .material
                            .lighting(&sphere, light, point, eye, normal, false);
                        canvas.pixels[x][y] = color;
                    }
                    _ => (),
                },
                None => (),
            }
        }
    }

    let file = format!("marble{}.ppm", i);
    let path = Path::new(&file);
    let display = path.display();

    let mut file = match File::create(&path) {
        Err(why) => panic!("couldn't create {}: {}", display, why),
        Ok(file) => file,
    };

    let ppm = canvas.canvas_to_ppm();
    match file.write_all(ppm.as_bytes()) {
        Err(why) => panic!("couldn't write to {}: {}", display, why),
        Ok(_) => println!("successfully wrote to {}", display),
    };
}
