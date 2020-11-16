use rustic_ray::{ray_tracing::camera::AntiAlias, Camera, Color, Point, PointLight, Transform, Vector, World, patterns::*, ray_tracing::color, shapes::Plane, shapes::Shape, shapes::Sphere};
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
    floor.material.pattern = Some(Box::new(pattern));
    world.objects.push(Box::new(floor));

    let mut ceiling = Plane::new();
    ceiling.transform = Transform::new().translation(0.0, 10.0, 0.0).build();
    ceiling.material.reflective = 0.1;
    ceiling.material.pattern = Some(Box::new(pattern));
    world.objects.push(Box::new(ceiling));

    let mut checkers = Checkers::new(color::WHITE, color::BLACK);
    checkers.transform = Transform::new().translation(10.0, 0.0, 10.0).build();

    let mut front_wall = Plane::new();
    front_wall.transform = Transform::new()
        .rotation_x(PI / 2.0)
        .rotation_y(-PI / 4.0)
        .translation(0.0, 0.0, 10.0)
        .build();
    front_wall.material.pattern = Some(Box::new(checkers));
    world.objects.push(Box::new(front_wall));

    let mut right_wall = Plane::new();
    right_wall.transform = Transform::new()
        .rotation_x(PI / 2.0)
        .rotation_y(PI / 4.0)
        .translation(10.0, 0.0, 0.0)
        .build();
    right_wall.material.pattern = Some(Box::new(checkers));
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

    let light = PointLight::new(Point::new(10.0, 3.5, -10.0), Color::new(1.0, 1.0, 1.0));
    world.lights.push(light);

    let mut c = Camera::new(614, 614, PI / 3.0);

    c.transform = Transform::view_transformation(
        Point::new(0.0, 1.5, -4.0),
        Point::new(0.0, 1.0, 0.0),
        Vector::new(0.0, 1.0, 0.0),
    );

    c.render_to_file(world, AntiAlias::None, 5, "fractal.png");
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
