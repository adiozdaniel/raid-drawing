use super::{Drawable, Displayable};
use rand::Rng;
use raster::{Color, Image};

#[derive(Debug, Clone)]
pub struct Point {
    pub x: i32,
    pub y: i32,
    color: Color,
}

impl Point {
    pub fn new(x: i32, y: i32) -> Self {
        Point {
            x,
            y,
            color: Color::rgb(
                rand::thread_rng().gen_range(50..200),
                rand::thread_rng().gen_range(50..200),
                rand::thread_rng().gen_range(50..200),
            ),
        }
    }

    // Add the distance method to calculate the Euclidean distance between two points
    pub fn distance(&self, other: &Point) -> i32 {
        let dx = self.x - other.x;
        let dy = self.y - other.y;
        ((dx * dx + dy * dy) as f64).sqrt() as i32
    }

    pub fn random(width: i32, height: i32) -> Self {
        let mut rng = rand::thread_rng();
        Point {
            x: rng.gen_range(0..width),
            y: rng.gen_range(0..height),
            color: Color::rgb(
                rng.gen_range(50..200),
                rng.gen_range(50..200),
                rng.gen_range(50..200),
            ),
        }
    }
}

impl Drawable for Point {
    fn draw(&self, image: &mut Image) {
        for dx in -1..=1 {
            for dy in -1..=1 {
                image.display(self.x + dx, self.y + dy, self.color());
            }
        }
    }

    fn color(&self) -> Color {
        self.color.clone()
    }
}
