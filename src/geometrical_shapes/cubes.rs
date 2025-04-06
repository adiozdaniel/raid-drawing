use super::{Drawable, Line, Point};
use rand::Rng;
use raster::{Color, Image};

#[derive(Debug)]
pub struct Cubes {
    cubes: Vec<(Point, i32, Color)>,
}

#[allow(dead_code)]
impl Cubes {
    // `new` function to create a cube with a specific center, size, and color
    pub fn new(center: &Point, size: i32) -> Self {
        let mut rng = rand::thread_rng();
        let mut cubes = Vec::new();
        
        let color = Color::rgb(
            rng.gen_range(50..200),
            rng.gen_range(50..200),
            rng.gen_range(50..200),
        );

        cubes.push((center.clone(), size as i32, color));

        Cubes { cubes }
    }

    pub fn random(width: i32, height: i32) -> Self {
        let mut rng = rand::thread_rng();
        let mut cubes = Vec::new();

        for _ in 0..rng.gen_range(3..6) {
            let center = Point::random(width, height);
            let size = rng.gen_range(30..80);
            let color = Color::rgb(
                rng.gen_range(150..255),
                rng.gen_range(150..255),
                rng.gen_range(150..255),
            );
            cubes.push((center, size, color));
        }

        Cubes { cubes }
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

impl Drawable for Cubes {
    fn draw(&self, image: &mut Image) {
        for (center, size, color) in &self.cubes {
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
                let line = Line::from_points(start, end, edge_thickness, color.clone());

                line.draw(image);
            }
        }
    }

    fn color(&self) -> Color {
        Color::rgb(0, 0, 0)
    }
}
