use crate::Color;

pub struct Canvas {
    pub width: usize,
    pub height: usize,
    pub pixels: Vec<Vec<Color>>,
}

impl Canvas {
    pub fn new(width: usize, height: usize) -> Self {
        Canvas {
            width,
            height,
            pixels: vec![vec![Color::new(0.0, 0.0, 0.0); height]; width],
        }
    }

    pub fn pixel_at(&self, x: usize, y: usize) -> Color {
        self.pixels[x][y]
    }

    pub fn canvas_to_ppm(&self) -> String {
        let mut buffer = ["P3", &format!("{} {}", self.width, self.height), "255"].join("\n");
        buffer.push_str("\n");

        let mut col_counter = 0;
        for y in 0..self.height {
            for x in 0..self.width {
                let p = &self.pixels[x][y];
                let red = Color::rgb_string(p.red);
                let green = Color::rgb_string(p.green);
                let blue = Color::rgb_string(p.blue);

                buffer.push_str(&Canvas::write_color(red, &mut col_counter));
                buffer.push_str(&Canvas::write_color(green, &mut col_counter));
                if x == self.width - 1 {
                    buffer.push_str(&Canvas::write_color(blue, &mut col_counter).trim());
                } else {
                    buffer.push_str(&Canvas::write_color(blue, &mut col_counter));
                }
            }
            buffer.push_str("\n");
            col_counter = 0;
        }
        buffer.push_str("\n");
        buffer
    }

    fn write_color(color: String, col_count: &mut usize) -> String {
        let mut color_buffer = String::new();
        if *col_count + color.len() > 70 {
            color_buffer.push_str("\n");
            *col_count = 0;
        }
        color_buffer.push_str(&color);
        *col_count += color.len();

        if *col_count + 4 > 70 {
            color_buffer.push_str("\n");
            *col_count = 0;
        } else {
            color_buffer.push_str(" ");
            *col_count += 1;
        }
        color_buffer
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Color;

    #[test]
    fn create_canvas() {
        let c = Canvas::new(10, 20);
        assert_eq!(10, c.width);
        assert_eq!(20, c.height);
        let expected = Color::new(0.0, 0.0, 0.0);
        for x in 0..10 {
            for y in 0..20 {
                let color = c.pixels[x][y];
                assert_eq!(expected, color);
            }
        }
    }

    #[test]
    fn write_canvas() {
        let mut c = Canvas::new(10, 20);
        c.pixels[2][3] = Color::new(1.0, 0.0, 0.0);
        let expected = Color::new(1.0, 0.0, 0.0);
        assert_eq!(expected, c.pixels[2][3]);
    }

    #[test]
    fn write_to_ppm() {
        let c = Canvas::new(5, 3);
        let actual = c.canvas_to_ppm();
        let split = actual.split("\n").collect::<Vec<_>>();
        assert_eq!("P3", split[0]);
        assert_eq!("5 3", split[1]);
        assert_eq!("255", split[2]);
    }

    // "0 0 0 0 0 0 0 0 0 0 0 0 0 0 0"
    #[test]
    fn create_ppm_pixal_data() {
        let mut c = Canvas::new(5, 3);
        let c1 = Color::new(1.5, 0.0, 0.0);
        let c2 = Color::new(0.0, 0.5, 0.0);
        let c3 = Color::new(-0.5, 0.0, 1.0);
        c.pixels[0][0] = c1;
        c.pixels[2][1] = c2;
        c.pixels[4][2] = c3;
        let actual = c.canvas_to_ppm();
        let split = actual.split("\n").collect::<Vec<_>>();
        assert_eq!("255 0 0 0 0 0 0 0 0 0 0 0 0 0 0", split[3]);
        assert_eq!("0 0 0 0 0 0 0 128 0 0 0 0 0 0 0", split[4]);
        assert_eq!("0 0 0 0 0 0 0 0 0 0 0 0 0 0 255", split[5]);
    }

    #[test]
    fn create_ppm_pixel_data_no_70() {
        let mut c = Canvas::new(10, 2);
        for x in 0..10 {
            for y in 0..2 {
                c.pixels[x][y] = Color::new(1.0, 0.8, 0.6);
            }
        }
        let actual = c.canvas_to_ppm();

        let split = actual.split("\n").collect::<Vec<_>>();
        assert_eq!(
            "255 204 153 255 204 153 255 204 153 255 204 153 255 204 153 255 204",
            split[3]
        );
        assert_eq!(
            "153 255 204 153 255 204 153 255 204 153 255 204 153",
            split[4]
        );
        assert_eq!(
            "255 204 153 255 204 153 255 204 153 255 204 153 255 204 153 255 204",
            split[5]
        );
        assert_eq!(
            "153 255 204 153 255 204 153 255 204 153 255 204 153",
            split[6]
        );
    }
}
