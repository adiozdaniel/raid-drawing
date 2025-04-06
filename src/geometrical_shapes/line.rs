use super::{Drawable, Point};
use crate::geometrical_shapes::Displayable;
use rand::Rng;
use raster::{Color, Image};

pub struct Line {
    start: Point,
    end: Point,
    thickness: i32,
    color: Color,
}

impl Line {
    pub fn random(width: i32, height: i32) -> Self {
        let mut rng = rand::thread_rng();
        Line {
            start: Point::random(width, height),
            end: Point::random(width, height),
            thickness: rng.gen_range(2..5),
            color: Color::rgb(
                rng.gen_range(50..200),
                rng.gen_range(50..200),
                rng.gen_range(50..200),
            ),
        }
    }

    pub fn from_points(p1: &Point, p2: &Point) -> Self {
        let mut rng = rand::thread_rng();
        Line {
            start: p1.clone(),
            end: p2.clone(),
            thickness: rng.gen_range(2..5),
            color: Color::rgb(
                rng.gen_range(50..200),
                rng.gen_range(50..200),
                rng.gen_range(50..200),
            ),
        }
    }
}

impl Drawable for Line {
    fn draw(&self, image: &mut Image) {
        for t in 0..self.thickness {
            let offset = t - self.thickness / 2;
            let dx = (self.end.x - self.start.x).abs();
            let dy = (self.end.y - self.start.y).abs();
            let sx = if self.start.x < self.end.x { 1 } else { -1 };
            let sy = if self.start.y < self.end.y { 1 } else { -1 };
            let mut err = dx - dy;
            let (mut x, mut y) = (self.start.x, self.start.y);

            loop {
                if dx > dy {
                    image.display(x, y + offset, self.color.clone());
                } else {
                    image.display(x + offset, y, self.color.clone());
                }

                if x == self.end.x && y == self.end.y {
                    break;
                }
                let e2 = 2 * err;
                if e2 > -dy {
                    err -= dy;
                    x += sx;
                }
                if e2 < dx {
                    err += dx;
                    y += sy;
                }
            }
        }
    }

    fn color(&self) -> Color {
        self.color.clone()
    }
}
