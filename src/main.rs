use rustic_ray::{
    patterns::*, ray_tracing::color, shapes::Plane, shapes::Shape, shapes::Sphere, Camera, Color,
    Point, PointLight, Transform, Vector, World,
};
use std::f64::consts::PI;

// use std::{fs::File, io::Write, path::Path};

fn main() {
    let mut world = World::new();

    let mut pattern = Checkers::new(color::WHITE, color::BLACK);
    pattern.transform = Transform::new()
        .scaling(0.1, 0.1, 0.1)
        .rotation_y(0.174)
        .translation(10.0, 0.0, 10.0)
        .build();

    let mut floor = Plane::new();
    // floor.material.reflective = 0.1;
    floor.material.pattern = Some(pattern.pattern_clone());
    world.objects.push(Box::new(floor));

    let mut ceiling = Plane::new();
    ceiling.transform = Transform::new().translation(0.0, 5.0, 0.0).build();
    ceiling.material.reflective = 0.1;
    ceiling.material.pattern = Some(pattern.pattern_clone());
    world.objects.push(Box::new(ceiling));

    let mut checkers = Checkers::new(color::WHITE, color::BLACK);
    checkers.transform = Transform::new().translation(10.0, 0.0, 10.0).build();

    let mut left_wall = Plane::new();
    left_wall.transform = Transform::new()
        .rotation_x(PI / 2.0)
        .rotation_y(-PI / 4.0)
        .translation(0.0, 0.0, 10.0)
        .build();
    left_wall.material.pattern = Some(checkers.pattern_clone());
    world.objects.push(Box::new(left_wall));

    let mut right_wall = Plane::new();
    right_wall.transform = Transform::new()
        .rotation_x(PI / 2.0)
        .rotation_y(PI / 4.0)
        .translation(10.0, 0.0, 0.0)
        .build();
    right_wall.material.pattern = Some(checkers.pattern_clone());
    world.objects.push(Box::new(right_wall));

    let mut ball1 = Sphere::new();
    ball1.transform = Transform::new().translation(-0.5, 1.0, -1.0).build();
    ball1.material.transparency = 1.0;
    ball1.material.refractive_index = 1.5;
    ball1.material.ambient = 0.1;
    ball1.material.diffuse = 0.05;
    world.objects.push(Box::new(ball1));

    let mut ball2 = Sphere::new();
    ball2.transform = Transform::new()
        .scaling(0.65, 0.65, 0.65)
        .translation(-0.5, 1.0, -1.0)
        .build();
    ball2.material.color = Color::new(1.0, 0.0, 0.0);
    ball2.material.ambient = 0.5;
    ball2.material.reflective = 0.25;
    world.objects.push(Box::new(ball2));

    let mut ball3 = Sphere::new();
    ball3.transform = Transform::new()
        .scaling(2.0, 2.0, 2.0)
        .translation(2.25, 2.0, -4.25)
        .build();
    ball3.material.color = Color::new(0.0, 1.0, 0.0);
    ball3.material.ambient = 0.8;
    ball3.material.reflective = 1.0;
    world.objects.push(Box::new(ball3));

    let mut water = Plane::new();
    water.transform = Transform::new().translation(0.0, 1.25, 0.0).build();
    water.material.reflective = 0.75;
    water.material.transparency = 4.0;
    water.material.refractive_index = 1.33;
    water.material.ambient = 0.5;
    water.material.diffuse = 0.25;
    water.cast_shadow = false;
    water.material.color = Color::new_rgb(21, 144, 219);
    // world.objects.push(Box::new(water));

    /*
    let mut middle = Sphere::new();
    middle.transform = Transform::new()
        .translation(0.0, 1.0, -6.0).build();
    middle.material.ambient = 0.01;
    middle.material.diffuse = 0.1;
    middle.material.transparency = 1.0;
    middle.material.refractive_index = 1.52;
    middle.material.color = color::WHITE;
    world.objects.push(Box::new(middle));


    */

    let light = PointLight::new(Point::new(10.0, 3.5, -10.0), Color::new(1.0, 1.0, 1.0));
    world.lights.push(light);

    let mut c = Camera::new(100, 100, PI / 3.0);

    c.transform = Transform::view_transformation(
        Point::new(0.0, 1.5, -4.0),
        Point::new(0.0, 1.0, 0.0),
        Vector::new(0.0, 1.0, 0.0),
    );

    c.render_to_file(world, 1, 5, "fractal.png");

    // let canvas = c.render(world, 0);
    /*
    println!("Done writing file.");
    let file = "balls.ppm".to_string();
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
    */
}

pub fn get_ball(x: f64, y: f64, z: f64) -> Box<dyn Shape> {
    let mut left = Sphere::new();
    left.transform = Transform::new()
        .scaling(0.33, 0.33, 0.33)
        .translation(x, y, z)
        //.translation(-1.5, 0.33, -0.75)
        .build();
    left.material.color = Color::random();
    left.material.diffuse = 0.7;
    left.material.specular = 0.3;

    Box::new(left)
}

/*
fn ambient(i: usize, a: usize) {
    let ray_origin = Point::new(0.0, 0.0, -5.0);
    let wall_z = 10.0;

    let wall_size = 7.0;
    let canvas_size = 800;

    let mut canvas = Canvas::new(canvas_size, canvas_size);

    let pixel_size = wall_size / canvas_size as f64;

    let half = wall_size / 2.0;

    let mut sphere = Sphere::new();
    sphere.material.color = Color::new(1.0, 0.2, 1.0);
    sphere.material.shininess = a as f64;

    let light = PointLight::new(Point::new(-10.0, 10.0, -10.0), Color::new(1.0, 1.0, 1.0));

    for y in 0..canvas_size {
        let world_y = half - pixel_size * y as f64;
        for x in 0..canvas_size {
            let world_x = -half + pixel_size * x as f64;

            let position = Point::new(world_x, world_y, wall_z);

            let direction = (position - ray_origin).normalize();
            let ray = Ray::new(ray_origin, direction);

            match sphere.intersect(ray) {
                Some(xs) => {
                    match intersections::hit(xs) {
                        Some(hit) => {
                            let point = ray.position(hit.t);
                            let normal = hit.object.normal_at(point);
                            let eye = -ray.direction;

                            let color = sphere.material.lighting(light, point, eye, normal);
                            canvas.pixels[x][y] = color;
                        }
                        _ => ()
                    }
                },
                None => ()
            }
        }
    }

    let file = format!("clock{}.ppm", i);
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


fn clock() {
    let canvas = &mut Canvas::new(200, 200);
    canvas.pixels[100][100] = Color::new(1.0, 0.0, 0.0);

    let mut hour = 1.0;
    loop {
        let xy = clock_hour(hour);

        println!("{} {} {}", hour, xy.0, xy.1);

        canvas.pixels[xy.0][xy.1] = Color::new(1.0, 0.0, 0.0);
        hour += 1.0;

        if hour > 12.0 {
            break;
        }
    }

    let path = Path::new("clock.ppm");
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


fn clock_hour(hour: f64) -> (usize, usize) {
    let r = Transform::new().rotation_y(hour * PI / 6.0);
    let hour_point = r * Point::new(0.0, 0.0, 1.0);

    println!("{:?}", hour_point);
    let radius = (3.0/8.0) * 200.0;

    let x = (100.0 + (hour_point.x * radius)) as usize;
    let y = (100.0 + (hour_point.z * radius)) as usize;
    (x, y)
}

struct Projectile {
    position: Point,
    velocity: Vector,
}

struct Environment {
    gravity: Vector,
    wind: Vector,
}

fn _cannon() {
    let start = Point::new(0.0, 1.0, 0.0);
    let velocity = Vector::new(1.0, 1.8, 0.0).normalize() * 11.25;

    let p = &mut Projectile {
        position: start,
        velocity,
    };
    let e = &Environment {
        gravity: Vector::new(0.0, -0.1, 0.0),
        wind: Vector::new(-0.01, 0.0, 0.0),
    };

    let c = &mut Canvas::new(900, 500);

    loop {
        tick(e, p);
        if p.position.y <= 0.0 {
            break;
        }
        draw(c, &p.position);
    }
    let _m = Matrix::new([
        [1.0, 2.0, 3.0, 4.0],
        [1.0, 2.0, 3.0, 4.0],
        [1.0, 2.0, 3.0, 4.0],
        [1.0, 2.0, 3.0, 4.0],
    ]);

    let path = Path::new("image.ppm");
    let display = path.display();

    let mut file = match File::create(&path) {
        Err(why) => panic!("couldn't create {}: {}", display, why),
        Ok(file) => file,
    };

    let ppm = c.canvas_to_ppm();
    match file.write_all(ppm.as_bytes()) {
        Err(why) => panic!("couldn't write to {}: {}", display, why),
        Ok(_) => println!("successfully wrote to {}", display),
    }
}

fn tick(env: &Environment, proj: &mut Projectile) {
    proj.position = proj.position + proj.velocity;
    proj.velocity = proj.velocity + env.gravity + env.wind;
}

fn draw(canvas: &mut Canvas, position: &Point) {
    let x = position.x as usize;
    let y = canvas.height - (position.y as usize);

    if x <= canvas.width - 1 && y <= canvas.height - 1 {
        canvas.pixels[x][y] = Color::new(1.0, 0.0, 0.0);
    }
}
*/
