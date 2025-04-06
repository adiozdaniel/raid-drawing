use super::{Drawable, Point, Line};
use rand::Rng;
use raster::{Color, Image};

#[derive(Debug)]
pub struct Rectangle {
    rects: Vec<(Point, Point, Color)>,
}

#[allow(dead_code)]
impl Rectangle {
    pub fn new(p1: &Point, p2: &Point) -> Self {
        let mut rng = rand::thread_rng();
        let mut rects = Vec::new();

        let rect_height = p1.y + p2.y;
        let rect_width = p2.x + p2.x;

        let top_left = Point::new(p2.x, p2.y);
        let bottom_right = Point::new(top_left.x + rect_width, top_left.y + rect_height);
        let color = Color::rgb(
            rng.gen_range(100..255),
            rng.gen_range(100..255),
            rng.gen_range(100..255),
        );

        rects.push((top_left, bottom_right, color));
        Rectangle { rects }
    }


    pub fn random(_p1: &Point, _p2: &Point) -> Self {
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

impl Drawable for Rectangle {
    fn draw(&self, image: &mut Image) {
        let thickness = 2;

        for (p1, p2, color) in &self.rects {
            let top_right = Point::new(p2.x, p1.y);
            let bottom_left = Point::new(p1.x, p2.y);

            let top_edge = Line::from_points(p1, &top_right, thickness, color.clone());
            let right_edge = Line::from_points(&top_right, p2, thickness, color.clone());
            let bottom_edge = Line::from_points(p2, &bottom_left, thickness, color.clone());
            let left_edge = Line::from_points(&bottom_left, p1, thickness, color.clone());

            top_edge.draw(image);
            right_edge.draw(image);
            bottom_edge.draw(image);
            left_edge.draw(image);
        }
    }

    fn color(&self) -> Color {
        Color::black()
    }
}
