use std::{f64::consts::PI};

use rustic_ray::{Camera, Color, shapes::Cone, Point, PointLight, Transform, Vector, World, patterns::Checkers, ray_tracing::{camera::AntiAlias, color}, shapes::Cylinder, shapes::Plane};

fn main() {
    let mut world = World::new();

    let mut pattern = Checkers::new(color::WHITE, color::BLACK);
    pattern.transform = Transform::new()
        .scaling(0.1, 0.1, 0.1)
        .rotation_y(0.174)
        .translation(10.0, 0.0, 10.0)
        .build();

    let mut checkers = Checkers::new(color::WHITE, color::BLACK);
    checkers.transform = Transform::new().translation(10.0, 0.0, 10.0).build();

    let mut floor = Plane::new();
    floor.material.pattern = Some(Box::new(pattern));
    world.objects.push(Box::new(floor));

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

    let mut c1 = Cylinder::new();
    c1.maximum = 2.0;
    c1.minimum = 0.0;
    c1.closed = true;
    //c1.transform = Transform::new().translation(0, 0.0, 0).build();
    c1.material.transparency = 0.0;
    c1.material.reflective = 0.9;
    c1.material.refractive_index = 1.5;
    c1.material.ambient = 0.1;
    c1.material.diffuse = 0.05;
    // world.objects.push(Box::new(c1));

    let mut cone = Cone::new();
    cone.maximum = 1.0;
    cone.minimum = -1.0;
    cone.material.transparency = 0.0;
    cone.material.reflective = 0.9;
    cone.material.refractive_index = 1.5;
    cone.material.ambient = 0.1;
    cone.material.diffuse = 0.05;
    cone.material.color = Color::new(0.25, 0.0, 0.0);
    cone.transform = Transform::new()
        .rotation_z(PI / 2.0)
        .translation(0.0, 1.0, 0.0).build();
    world.objects.push(Box::new(cone));

    let light = PointLight::new(Point::new(10.0, 3.5, -10.0), Color::new(1.0, 1.0, 1.0));
    world.lights.push(light);

    let mut c = Camera::new(1024, 768, PI / 3.0);

    c.transform = Transform::view_transformation(
        Point::new(0.0, 1.5, -4.0),
        Point::new(0.0, 1.0, 0.0),
        Vector::new(0.0, 1.0, 0.0),
    );

    c.render_to_file(world, AntiAlias::None, 5, "cylinder.png");
}