// line.rs
use super::{Drawable, Point, Displayable};
use rand::Rng;
use raster::{Color, Image};

fn draw_line_with_color(
    start: &Point,
    end: &Point,
    thickness: i32,
    image: &mut Image,
    color: &Color,
) {
    let dx = (end.x - start.x).abs();
    let dy = (end.y - start.y).abs();
    let sx = if start.x < end.x { 1 } else { -1 };
    let sy = if start.y < end.y { 1 } else { -1 };
    let err = dx - dy;

    for t in 0..thickness {
        let offset = t - thickness / 2;
        let mut x = start.x;
        let mut y = start.y;
        let mut local_err = err;
        loop {
            if dx > dy {
                image.display(x, y + offset, color.clone());
            } else {
                image.display(x + offset, y, color.clone());
            }
            if x == end.x && y == end.y {
                break;
            }
            let e2 = 2 * local_err;
            if e2 > -dy {
                local_err -= dy;
                x += sx;
            }
            if e2 < dx {
                local_err += dx;
                y += sy;
            }
        }
    }
}

pub struct Line {
    start: Point,
    end: Point,
    thickness: i32,
    color: Color,
}

#[allow(dead_code)]
impl Line {
    pub fn new(p1: &Point, p2: &Point) -> Self {
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

    pub fn from_points(p1: &Point, p2: &Point, thickness: i32, color: Color) -> Self {
        Line {
            start: p1.clone(),
            end: p2.clone(),
            thickness,
            color,
        }
    }
}

impl Drawable for Line {
    fn draw(&self, image: &mut Image) {
        draw_line_with_color(&self.start, &self.end, self.thickness, image, &self.color);
    }

    fn color(&self) -> Color {
        self.color.clone()
    }
}
