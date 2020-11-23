use std::{fs::File, io::Write, path::Path};

use rustic_ray::{Canvas, Color, Matrix, Point, Vector};

struct Projectile {
    position: Point,
    velocity: Vector,
}

struct Environment {
    gravity: Vector,
    wind: Vector,
}

fn main() {
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

    let path = Path::new("cannon.ppm");
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
