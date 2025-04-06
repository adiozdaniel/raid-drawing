use super::{Drawable, Point};
use crate::geometrical_shapes::Displayable;
use rand::Rng;
use raster::{Color, Image};

pub struct Circle {
    circles: Vec<(Point, i32)>,
    color: Color,
}

impl Circle {
    pub fn random(width: i32, height: i32) -> Self {
        let mut rng = rand::thread_rng();
        let mut circles = Vec::new();

        let color = Color::rgb(
            rng.gen_range(50..200),
            rng.gen_range(50..200),
            rng.gen_range(50..200),
        );

        for _ in 0..rng.gen_range(1..2) {
            circles.push((Point::random(width, height), rng.gen_range(50..150)));
        }

        Circle { circles, color }
    }
}

impl Drawable for Circle {
    fn draw(&self, image: &mut Image) {
        for (center, radius) in &self.circles {
            let mut x = *radius;
            let mut y = 0;
            let mut err = 0;

            while x >= y {
                for (dx, dy) in &[
                    (x, y),
                    (y, x),
                    (-y, x),
                    (-x, y),
                    (-x, -y),
                    (-y, -x),
                    (y, -x),
                    (x, -y),
                ] {
                    image.display(center.x + dx, center.y + dy, self.color());
                }

                y += 1;
                err += 1 + 2 * y;
                if 2 * (err - x) + 1 > 0 {
                    x -= 1;
                    err += 1 - 2 * x;
                }
            }
        }
    }

    fn color(&self) -> Color {
        self.color.clone()
    }
}
