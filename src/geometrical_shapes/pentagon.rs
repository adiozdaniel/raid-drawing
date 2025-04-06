use super::{Drawable, Point};
use crate::geometrical_shapes::Displayable;
use rand::Rng;
use raster::{Color, Image};
use std::f64::consts::PI;

#[derive(Debug)]
pub struct Pentagon {
    pentagons: Vec<(Point, i32, Color)>,
}

impl Pentagon {
    pub fn random(width: i32, height: i32) -> Self {
        let mut rng = rand::thread_rng();
        let mut pentagons = Vec::new();

        let count = rng.gen_range(3..6);
        for _ in 0..count {
            let center = Point::random(width, height);
            let radius = rng.gen_range(30..80);
            let color = Color::rgb(
                rng.gen_range(100..255),
                rng.gen_range(100..255),
                rng.gen_range(100..255),
            );
            pentagons.push((center, radius, color));
        }

        Pentagon { pentagons }
    }

    fn get_vertices(center: &Point, radius: i32) -> Vec<Point> {
        (0..5)
            .map(|i| {
                let angle = 2.0 * PI * i as f64 / 5.0;
                Point::new(
                    center.x + (radius as f64 * angle.cos()) as i32,
                    center.y + (radius as f64 * angle.sin()) as i32,
                )
            })
            .collect()
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

impl Drawable for Pentagon {
    fn draw(&self, image: &mut Image) {
        let thickness = 2;

        for (center, radius, color) in &self.pentagons {
            let vertices = Pentagon::get_vertices(center, *radius);

            for i in 0..5 {
                let start = &vertices[i];
                let end = &vertices[(i + 1) % 5];

                draw_line_with_color(start, end, thickness, image, color);
            }
        }
    }

    fn color(&self) -> Color {
        Color::rgb(0, 0, 0) 
    }
}
