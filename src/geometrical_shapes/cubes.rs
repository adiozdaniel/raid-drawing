use super::{Drawable, Point};
use crate::geometrical_shapes::Displayable;
use rand::Rng;
use raster::{Color, Image};

#[derive(Debug)]
pub struct Cubes {
    cubes: Vec<(Point, i32)>,
    color: Color,
}

impl Cubes {
    pub fn random(width: i32, height: i32) -> Self {
        let mut rng = rand::thread_rng();
        let mut cubes = Vec::new();

        let color = Color::rgb(
            rng.gen_range(150..255),
            rng.gen_range(150..255),
            rng.gen_range(150..255),
        );

        for _ in 0..rng.gen_range(3..6) {
            cubes.push((Point::random(width, height), rng.gen_range(30..80)));
        }

        Cubes { cubes, color }
    }

    fn get_isometric_projection(center: &Point, size: i32) -> [Point; 8] {
        let x = center.x;
        let y = center.y;
        let s = size;
        [
            Point::new(x - s, y - s / 2),
            Point::new(x + s, y - s / 2),
            Point::new(x - s, y + s / 2),
            Point::new(x + s, y + s / 2),
            Point::new(x - s / 2, y - s),
            Point::new(x + s / 2, y - s),
            Point::new(x - s / 2, y),
            Point::new(x + s / 2, y),
        ]
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

impl Drawable for Cubes {
    fn draw(&self, image: &mut Image) {
        for (center, size) in &self.cubes {
            let vertices = Cubes::get_isometric_projection(center, *size);

            let edges = [
                // Front face
                (0, 1), (1, 3), (3, 2), (2, 0),
                // Back face
                (4, 5), (5, 7), (7, 6), (6, 4),
                // Connecting edges
                (0, 4), (1, 5), (2, 6), (3, 7),
            ];

            let edge_thickness = 2;

            for (i, j) in edges.iter() {
                let start = &vertices[*i];
                let end = &vertices[*j];
                draw_line_with_color(start, end, edge_thickness, image, &self.color);
            }
        }
    }

    fn color(&self) -> Color {
        self.color.clone()
    }
}
