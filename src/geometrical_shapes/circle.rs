use super::{Drawable, Point, Displayable};
use rand::Rng;
use raster::{Color, Image};

pub struct Circle {
    circles: Vec<(Point, i32)>,
    color: Color,
}

#[allow(dead_code)]
impl Circle {
    pub fn new(center: &Point, radius_point: &Point) -> Self {
        let mut rng = rand::thread_rng();
        let mut circles = Vec::new();
        let radius = center.distance(radius_point);
        let color = Color::rgb(
            rng.gen_range(50..200),
            rng.gen_range(50..200),
            rng.gen_range(50..200),
        );

        circles.push((center.clone(), radius));

        Circle { circles, color }
    }

    pub fn random(width: i32, height: i32) -> Self {
        let mut rng = rand::thread_rng();
        let mut circles = Vec::new();

        let color = Color::rgb(
            rng.gen_range(50..200),
            rng.gen_range(50..200),
            rng.gen_range(50..200),
        );

        if rng.gen_bool(0.7) {
            let radius = if rng.gen_bool(0.3) {
                rng.gen_range(150..300)
            } else {
                rng.gen_range(50..150)
            };
            circles.push((Point::random(width, height), radius));
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
                    let px = center.x + dx;
                    let py = center.y + dy;

                    image.display(px, py, self.color());
                    image.display(px + 1, py, self.color());
                    image.display(px, py + 1, self.color());
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
