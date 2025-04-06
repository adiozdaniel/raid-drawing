use super::{Drawable, Point};
use crate::geometrical_shapes::Displayable;
use rand::Rng;
use raster::{Color, Image};

pub struct Triangle {
    tris: Vec<(Point, Point, Point, Color)>,
}

impl Triangle {
    pub fn new(_a: &Point, _b: &Point, _c: &Point) -> Self {
        let mut rng = rand::thread_rng();
        let mut tris = Vec::new();

        for _ in 0..rng.gen_range(2..4) {
            let base = Point::random(800, 800);
            let height = rng.gen_range(80..180);
            let width = rng.gen_range(60..150);
            let color = Color::rgb(
                rng.gen_range(150..255),
                rng.gen_range(150..255),
                rng.gen_range(150..255),
            );

            tris.push((
                base.clone(),
                Point::new(base.x + width, base.y),
                Point::new(base.x + width / 2, base.y - height),
                color,
            ));
        }

        Triangle { tris }
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

impl Drawable for Triangle {
    fn draw(&self, image: &mut Image) {
        let thickness = 1;

        for (a, b, c, color) in &self.tris {
            draw_line_with_color(a, b, thickness, image, color);
            draw_line_with_color(b, c, thickness, image, color);
            draw_line_with_color(c, a, thickness, image, color);
        }
    }

    fn color(&self) -> Color {
        Color::rgb(0, 0, 0)
    }
}
