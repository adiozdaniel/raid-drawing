use super::{Drawable, Line, Point};
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

impl Drawable for Triangle {
    fn draw(&self, image: &mut Image) {
        let thickness = 1;

        for (a, b, c, color) in &self.tris {
            let edge_ab = Line::from_points(a, b, thickness, color.clone());
            let edge_bc = Line::from_points(b, c, thickness, color.clone());
            let edge_ca = Line::from_points(c, a, thickness, color.clone());

            edge_ab.draw(image);
            edge_bc.draw(image);
            edge_ca.draw(image);
        }
    }

    fn color(&self) -> Color {
        Color::rgb(0, 0, 0)
    }
}
