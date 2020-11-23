use std::f64::consts::PI;

use rustic_ray::{
    patterns::Checkers, patterns::Stripe, ray_tracing::camera::AntiAlias, ray_tracing::color,
    shapes::Cube, Camera, Color, Point, PointLight, Transform, Vector, World,
};

fn main() {
    let mut world = World::new();

    let mut checkers = Checkers::new(color::WHITE, color::BLACK);
    checkers.transform = Transform::new().scaling(0.15, 0.15, 0.15).build();

    let mut stripes1 = Stripe::new(Color::new_rgb(161, 64, 5), Color::new_rgb(145, 41, 3));
    stripes1.transform = Transform::new()
        .scaling(0.05, 0.05, 0.05)
        .rotation_y(PI / 2.0)
        .build();

    let mut stripes2 = Stripe::new(Color::new_rgb(161, 64, 5), Color::new_rgb(145, 41, 3));
    stripes2.transform = Transform::new().scaling(0.05, 0.05, 0.05).build();

    let mut floor = Cube::new();
    floor.transform = Transform::new()
        .scaling(5.0, 0.1, 5.0)
        .translation(0.0, -0.1, 0.0)
        .build();
    floor.material.pattern = Some(Box::new(checkers));
    world.add_shape(Box::new(floor));

    let mut right_wall = Cube::new();
    right_wall.transform = Transform::new()
        .scaling(0.1, 4.0, 5.0)
        .translation(-5.1, 4.0, 0.0)
        .build();
    right_wall.material.pattern = Some(Box::new(stripes1));
    world.add_shape(Box::new(right_wall));

    let mut left_wall = Cube::new();
    left_wall.transform = Transform::new()
        .scaling(0.1, 4.0, 5.0)
        .translation(5.1, 4.0, 0.0)
        .build();
    left_wall.material.pattern = Some(Box::new(stripes1));
    world.add_shape(Box::new(left_wall));

    let mut back_wall = Cube::new();
    back_wall.transform = Transform::new()
        .scaling(5.0, 4.0, 0.1)
        .translation(0.0, 4.0, 5.1)
        .build();
    back_wall.material.pattern = Some(Box::new(stripes2));
    world.add_shape(Box::new(back_wall));

    let mut painting = Cube::new();
    painting.transform = Transform::new()
        .scaling(1.0, 2.0, 0.1)
        .translation(-1.5, 4.0, 4.9)
        .build();
    painting.material.color = Color::new(0.1, 1.0, 0.1);
    world.add_shape(Box::new(painting));

    let mut painting2 = Cube::new();
    painting2.transform = Transform::new()
        .scaling(1.75, 0.5, 0.1)
        .translation(1.5, 4.0, 4.9)
        .build();
    painting2.material.color = Color::new(1.0, 0.3, 0.3);
    world.add_shape(Box::new(painting2));

    let mut painting3 = Cube::new();
    painting3.transform = Transform::new()
        .scaling(1.75, 0.5, 0.1)
        .translation(1.5, 2.75, 4.9)
        .build();
    painting3.material.color = Color::new(0.0, 0.3, 1.0);
    world.add_shape(Box::new(painting3));

    let mut mirror = Cube::new();
    mirror.transform = Transform::new()
        .scaling(0.01, 2.0, 4.0)
        .translation(5.0, 3.0, 0.0)
        .build();
    mirror.material.reflective = 1.0;
    mirror.material.refractive_index = 1.458;
    world.add_shape(Box::new(mirror));

    let mut table_top = Cube::new();
    table_top.transform = Transform::new()
        .scaling(2.5, 0.1, 3.0)
        .translation(0.5, 1.25, 0.0)
        .build();
    table_top.material.pattern = Some(Box::new(stripes1));
    table_top.material.reflective = 0.02;
    table_top.material.refractive_index = 3.45;
    world.add_shape(Box::new(table_top));

    let mut leg1 = Cube::new();
    leg1.transform = Transform::new()
        .scaling(0.1, 0.65, 0.1)
        .translation(-1.9, 0.65, -2.9)
        .build();
    leg1.material.color = Color::new_rgb(161, 64, 5);
    world.add_shape(Box::new(leg1));

    let mut leg2 = Cube::new();
    leg2.transform = Transform::new()
        .scaling(0.1, 0.65, 0.1)
        .translation(2.9, 0.65, -2.9)
        .build();
    leg2.material.color = Color::new_rgb(161, 64, 5);
    world.add_shape(Box::new(leg2));

    let mut leg3 = Cube::new();
    leg3.transform = Transform::new()
        .scaling(0.1, 0.65, 0.1)
        .translation(2.9, 0.65, 2.9)
        .build();
    leg3.material.color = Color::new_rgb(161, 64, 5);
    world.add_shape(Box::new(leg3));

    let mut leg4 = Cube::new();
    leg4.transform = Transform::new()
        .scaling(0.1, 0.65, 0.1)
        .translation(-1.9, 0.65, 2.9)
        .build();
    leg4.material.color = Color::new_rgb(161, 64, 5);
    world.add_shape(Box::new(leg4));

    let mut block1 = Cube::new();
    block1.transform = Transform::new()
        .scaling(0.1, 1.0, 0.1)
        .translation(-0.75, 2.35, -0.75)
        .build();
    block1.material.color = Color::new_rgb(211, 102, 151);
    block1.material.refractive_index = 2.417;
    block1.material.reflective = 0.45;
    world.add_shape(Box::new(block1));

    let mut block1 = Cube::new();
    block1.transform = Transform::new()
        .scaling(0.1, 0.1, 0.1)
        .translation(0.5, 1.45, -2.0)
        .build();
    block1.material.color = Color::new_rgb(213, 14, 151);
    world.add_shape(Box::new(block1));

    let mut block3 = Cube::new();
    block3.transform = Transform::new()
        .scaling(0.2, 0.2, 0.2)
        .translation(1.75, 1.55, -1.0)
        .build();
    block3.material.color = Color::new_rgb(10, 234, 36);
    world.add_shape(Box::new(block3));

    let mut block3 = Cube::new();
    block3.transform = Transform::new()
        .scaling(0.55, 0.5, 1.75)
        .translation(0.2, 1.55, 0.05)
        .build();
    block3.material.reflective = 0.6;
    block3.material.refractive_index = 1.31;
    block3.material.ambient = 0.025;
    block3.material.diffuse = 0.25;
    block3.material.color = Color::new_rgb(237, 234, 36);
    world.add_shape(Box::new(block3));

    let light = PointLight::new(Point::new(3.0, 11.0, -10.0), Color::new(1.0, 1.0, 1.0));
    world.lights.push(light);

    let mut c = Camera::new(614, 614, PI / 3.0);

    // Point::new(-3.0, 2.5, -6.0),
    // Point::new(0.0, 5.5, -11.80),
    c.transform = Transform::view_transformation(
        Point::new(-4.0, 2.5, -4.8),
        Point::new(0.90, 1.25, 0.0),
        Vector::new(0.0, 1.0, 0.0),
    );

    c.render_to_file(&world, AntiAlias::None, 5, "room.png");
}
