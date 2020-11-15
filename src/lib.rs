pub mod ray_tracing;

pub use ray_tracing::camera::Camera;
pub use ray_tracing::canvas::Canvas;
pub use ray_tracing::color::Color;
pub use ray_tracing::intersection::Computations;
pub use ray_tracing::intersection::Intersection;
pub use ray_tracing::light::PointLight;
pub use ray_tracing::material::Material;
pub use ray_tracing::matrix::Matrix;
pub use ray_tracing::patterns;
pub use ray_tracing::point::Point;
pub use ray_tracing::ray::Ray;
pub use ray_tracing::shapes;
pub use ray_tracing::transform::Transform;
pub use ray_tracing::vector::Vector;
pub use ray_tracing::world::World;
pub use ray_tracing::xs::XS;

use std::cmp::Ordering;

pub const EPSILON: f64 = 0.0001;

pub fn float_eq(a: f64, b: f64) -> bool {
    /*
    let a1 = (a * 1000000.0).round() / 1000000.0;
    let b1 = (b * 1000000.0).round() / 1000000.0;
    // Calculate the difference.
    let diff = (a1 - b1).abs();

    let a2 = a1.abs();
    let b2 = b1.abs();
    // Find the largest
    let largest: f64;
    if b2 > a2 {
        largest = b2 * EPSILON;
    } else {
        largest = a2 * EPSILON;
    }

    let mut results:bool = false;
    if diff <= largest {
        results = true;
    }

    results
    */

    (a - b).abs() < EPSILON
}

pub fn float_cmp(a: f64, b: f64) -> Ordering {
    if float_eq(a, b) {
        Ordering::Equal
    } else if a < b {
        Ordering::Less
    } else {
        Ordering::Greater
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn equals() {
        assert!(float_eq(1.111113, 1.111115));
    }

    #[test]
    fn equals2() {
        assert!(float_eq(0.21804511278195488, 0.21804999999999999));
    }

    #[test]
    fn equals3() {
        assert!(float_eq(0.0, 0.00000000000000006123233995736766));
    }

    #[test]
    fn less_than() {
        assert_eq!(float_cmp(4.5, 6.0), Ordering::Less);
    }

    #[test]
    fn greater_than() {
        assert_eq!(float_cmp(6.0, 4.5), Ordering::Greater);
    }
}
