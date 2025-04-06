use super::{Drawable, Point, Line};
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

impl Drawable for Pentagon {
    fn draw(&self, image: &mut Image) {
        let thickness = 2;

        for (center, radius, color) in &self.pentagons {
            let vertices = Pentagon::get_vertices(center, *radius);

            for i in 0..5 {
                let start = &vertices[i];
                let end = &vertices[(i + 1) % 5];

                let line = Line::from_points(start, end, thickness, color.clone());
                line.draw(image);
            }
        }
    }

    fn color(&self) -> Color {
        Color::rgb(0, 0, 0) 
    }
}
