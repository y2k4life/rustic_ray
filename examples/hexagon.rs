use std::f64::consts::PI;

use rustic_ray::{
    patterns::Checkers, ray_tracing::camera::AntiAlias, ray_tracing::color, shapes::Cylinder,
    shapes::Group, shapes::Plane, shapes::Sphere, Camera, Color, Point, PointLight, Transform,
    Vector, World,
};

fn main() {
    let w = &mut World::new();

    let mut pattern = Checkers::new(color::WHITE, color::BLACK);
    pattern.transform = Transform::new()
        .scaling(0.1, 0.1, 0.1)
        .rotation_y(0.174)
        .translation(10.0, 0.0, 10.0)
        .build();

    let mut floor = Plane::new();
    floor.material.pattern = Some(Box::new(pattern));
    w.add_shape(Box::new(floor));

    let mut hex = hexagon();
    hex.transform = Transform::new()
        .rotation_x(-PI / 6.0)
        .translation(1.0, 1.0, 0.0)
        .build();

    w.add_shape(Box::new(hex));

    let mut hex2 = hexagon();
    hex2.transform = Transform::new()
        .scaling(0.25, 0.25, 0.25)
        .rotation_x(-PI / 6.0)
        .translation(1.0, 1.0, 0.0)
        .build();
    hex2.material.color = Color::new(0.0, 1.0, 0.0);

    w.add_shape(Box::new(hex2));

    let light = PointLight::new(Point::new(10.0, 3.5, -10.0), Color::new(1.0, 1.0, 1.0));
    w.lights.push(light);

    let w = &*w;

    let mut c = Camera::new(614, 614, PI / 3.0);

    c.transform = Transform::view_transformation(
        Point::new(0.0, 1.5, -4.0),
        Point::new(0.0, 1.0, 0.0),
        Vector::new(0.0, 1.0, 0.0),
    );

    c.render_to_file(w, AntiAlias::None, 5, "hexagon.png");
}

fn hexagon_corner() -> Sphere {
    let mut corner = Sphere::new();
    corner.transform = Transform::new()
        .scaling(0.25, 0.25, 0.25)
        .translation(0.0, 0.0, -1.0)
        .build();

    corner.material.color = Color::new(1.0, 0.0, 0.0);

    corner
}

pub fn hexagon_edge() -> Cylinder {
    let mut edge = Cylinder::new();
    edge.minimum = 0.0;
    edge.maximum = 1.0;
    edge.transform = Transform::new()
        .scaling(0.25, 1.0, 0.25)
        .rotation_z(-PI / 2.0)
        .rotation_y(-PI / 6.0)
        .translation(0.0, 0.0, -1.0)
        .build();

    edge.material.color = Color::new(1.0, 0.0, 0.0);

    edge
}

pub fn hexagon_side() -> Group {
    let mut side = Group::new();

    let mut corner = hexagon_corner();
    corner.parent_id = Some(side.id);

    let mut hexagon_edge = hexagon_edge();
    hexagon_edge.parent_id = Some(side.id);

    side.add_shape(Box::new(corner));
    side.add_shape(Box::new(hexagon_edge));

    side
}

pub fn hexagon() -> Group {
    let mut hex = Group::new();

    for n in 0..6 {
        let mut side = hexagon_side();
        side.transform = Transform::new().rotation_y(n as f64 * PI / 3.0).build();
        side.parent_id = Some(hex.id);

        hex.add_shape(Box::new(side));
    }

    hex
}
