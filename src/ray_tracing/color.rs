use crate::float_eq;
use rand::thread_rng;
use rand::Rng;
use std::ops::{Add, Div, Mul, Neg, Sub};

pub const BLACK: Color = Color {
    red: 0.0,
    green: 0.0,
    blue: 0.0,
};

pub const WHITE: Color = Color {
    red: 1.0,
    green: 1.0,
    blue: 1.0,
};

pub const RED: Color = Color {
    red: 1.0,
    green: 0.0,
    blue: 0.0,
};

#[derive(Debug, Copy, Clone)]
pub struct Color {
    pub red: f64,
    pub green: f64,
    pub blue: f64,
}

impl Color {
    pub fn new(red: f64, green: f64, blue: f64) -> Self {
        Color { red, green, blue }
    }

    pub fn new_rgb(red: i32, green: i32, blue: i32) -> Self {
        Color {
            red: red as f64 / 255.0,
            green: green as f64 / 255.0,
            blue: blue as f64 / 255.0,
        }
    }

    pub fn random() -> Self {
        let mut rng = thread_rng();
        let red: f64 = rng.gen_range(0.0, 1.0);
        let green: f64 = rng.gen_range(0.0, 1.0);
        let blue: f64 = rng.gen_range(0.0, 1.0);
        Color { red, green, blue }
    }

    pub fn from_tuple(color: (f64, f64, f64)) -> Self {
        Color {
            red: color.0,
            green: color.1,
            blue: color.2,
        }
    }

    pub fn rgb_string(color: f64) -> String {
        let mut rgb = color * 256.;
        if rgb < 0.0 {
            rgb = 0.0;
        }
        if rgb > 255.0 {
            rgb = 255.0;
        }
        let rgb = rgb as i64;
        format!("{}", rgb)
    }

    pub fn to_rgb(&self) -> (u8, u8, u8) {
        (
            Color::color_to_u8(self.red),
            Color::color_to_u8(self.green),
            Color::color_to_u8(self.blue),
        )
    }

    fn color_to_u8(c: f64) -> u8 {
        (255.0 * c) as u8
    }
}

impl Sub for Color {
    type Output = Color;

    fn sub(self, other: Color) -> Color {
        Color {
            red: self.red - other.red,
            green: self.green - other.green,
            blue: self.blue - other.blue,
        }
    }
}

impl Add for Color {
    type Output = Color;

    fn add(self, other: Color) -> Color {
        Color {
            red: self.red + other.red,
            green: self.green + other.green,
            blue: self.blue + other.blue,
        }
    }
}

impl Mul for Color {
    type Output = Color;

    fn mul(self, other: Color) -> Color {
        Color {
            red: self.red * other.red,
            green: self.green * other.green,
            blue: self.blue * other.blue,
        }
    }
}

impl Mul<f64> for Color {
    type Output = Color;

    fn mul(self, other: f64) -> Color {
        Color {
            red: self.red * other,
            green: self.green * other,
            blue: self.blue * other,
        }
    }
}

impl Mul<Color> for f64 {
    type Output = Color;

    fn mul(self, other: Color) -> Color {
        Color {
            red: other.red * self,
            green: other.green * self,
            blue: other.blue * self,
        }
    }
}

impl Div<f64> for Color {
    type Output = Color;

    fn div(self, other: f64) -> Color {
        Color {
            red: self.red / other,
            green: self.green / other,
            blue: self.blue / other,
        }
    }
}

impl Neg for Color {
    type Output = Color;

    fn neg(self) -> Color {
        Color {
            red: -self.red,
            green: -self.green,
            blue: -self.blue,
        }
    }
}

impl PartialEq for Color {
    fn eq(&self, other: &Color) -> bool {
        float_eq(self.red, other.red)
            && float_eq(self.green, other.green)
            && float_eq(self.blue, other.blue)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn colors_are_n0_5_0_4_1_7() {
        let c = Color::new(-0.5, 0.4, 1.7);

        assert_eq!(-0.5, c.red);
        assert_eq!(0.4, c.green);
        assert_eq!(1.7, c.blue);
    }

    #[test]
    fn add_two_colors() {
        let c1 = Color::new(0.9, 0.6, 0.75);
        let c2 = Color::new(0.7, 0.1, 0.25);
        let actual = c1 + c2;
        let expected = Color::new(1.6, 0.7, 1.0);
        assert_eq!(expected, actual);
    }

    #[test]
    fn subtract_tw_colors() {
        let c1 = Color::new(0.9, 0.6, 0.75);
        let c2 = Color::new(0.7, 0.1, 0.25);
        let actual = c1 - c2;
        let expected = Color::new(0.2, 0.5, 0.5);
        assert_eq!(expected, actual);
    }

    #[test]
    fn mutiple_color_by_float() {
        let c = Color::new(0.2, 0.3, 0.4);
        let actual = c * 2.0;
        let expected = Color::new(0.4, 0.6, 0.8);
        assert_eq!(expected, actual);
    }

    #[test]
    fn mutiple_float_by_color() {
        let c = Color::new(0.2, 0.3, 0.4);
        let actual = 2.0 * c;
        let expected = Color::new(0.4, 0.6, 0.8);
        assert_eq!(expected, actual);
    }

    #[test]
    fn mutiple_two_colors() {
        let c1 = Color::new(1.0, 0.2, 0.4);
        let c2 = Color::new(0.9, 1.0, 0.1);
        let actual = c1 * c2;
        let expected = Color::new(0.9, 0.2, 0.04);
        assert_eq!(expected, actual);
    }
}
