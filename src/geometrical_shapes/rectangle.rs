use super::{Drawable, Point};
use crate::geometrical_shapes::Displayable;
use rand::Rng;
use raster::{Color, Image};

#[derive(Debug)]
pub struct Rectangle {
    rects: Vec<(Point, Point, Color)>,
}

impl Rectangle {
    pub fn new(_p1: &Point, _p2: &Point) -> Self {
        let mut rng = rand::thread_rng();
        let mut rects = Vec::new();

        for _ in 0..rng.gen_range(2..4) {
            let width = rng.gen_range(100..250);
            let height = rng.gen_range(80..180);
            let pos = Point::random(800, 800);
            let color = Color::rgb(
                rng.gen_range(100..255),
                rng.gen_range(100..255),
                rng.gen_range(100..255),
            );
            rects.push((pos.clone(), Point::new(pos.x + width, pos.y + height), color));
        }

        Rectangle { rects }
    }
}

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

impl Drawable for Rectangle {
    fn draw(&self, image: &mut Image) {
        let thickness = 2;

        for (p1, p2, color) in &self.rects {
            let top_right = Point::new(p2.x, p1.y);
            let bottom_left = Point::new(p1.x, p2.y);

            draw_line_with_color(p1, &top_right, thickness, image, color);
            draw_line_with_color(&top_right, p2, thickness, image, color);
            draw_line_with_color(p2, &bottom_left, thickness, image, color);
            draw_line_with_color(&bottom_left, p1, thickness, image, color);
        }
    }

    fn color(&self) -> Color {
        Color::black()
    }
}
