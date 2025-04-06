use super::{Drawable, Line, Point};
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
            rects.push((
                pos.clone(),
                Point::new(pos.x + width, pos.y + height),
                color,
            ));
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rectangle_new() {
        let p1 = Point::new(0, 0);
        let p2 = Point::new(100, 100);
        let rect = Rectangle::new(&p1, &p2);

        assert_eq!(rect.rects.len(), 1);
        let (top_left, bottom_right, _) = &rect.rects[0];
        assert_eq!(top_left.x, 100);
        assert_eq!(top_left.y, 100);
        assert_eq!(bottom_right.x, 300); // 100 + (100 + 100)
        assert_eq!(bottom_right.y, 200); // 100 + (0 + 100)
    }

    #[test]
    fn test_rectangle_random() {
        let p1 = Point::new(0, 0);
        let p2 = Point::new(800, 800);
        let rect = Rectangle::random(&p1, &p2);

        // Random rectangles should be between 2 and 4
        assert!(rect.rects.len() >= 2 && rect.rects.len() <= 4);

        // Check each rectangle's properties
        for (top_left, bottom_right, _) in &rect.rects {
            assert!(top_left.x >= 0 && top_left.x <= 800);
            assert!(top_left.y >= 0 && top_left.y <= 800);
            assert!(bottom_right.x > top_left.x);
            assert!(bottom_right.y > top_left.y);
            assert!(bottom_right.x - top_left.x >= 100 && bottom_right.x - top_left.x <= 250);
            assert!(bottom_right.y - top_left.y >= 80 && bottom_right.y - top_left.y <= 180);
        }
    }

    #[test]
    fn test_rectangle_draw() {
        let p1 = Point::new(0, 0);
        let p2 = Point::new(100, 100);
        let rect = Rectangle::new(&p1, &p2);

        let mut image = Image::blank(800, 800);
        rect.draw(&mut image);
    }

    #[test]
    fn test_rectangle_color() {
        let p1 = Point::new(0, 0);
        let p2 = Point::new(100, 100);
        let rect = Rectangle::new(&p1, &p2);

        let black = Color::black();
        let rect_color = rect.color();
        assert_eq!(rect_color.r, black.r);
        assert_eq!(rect_color.g, black.g);
        assert_eq!(rect_color.b, black.b);
    }

    #[test]
    fn test_rectangle_edge_cases() {
        // Test with zero dimensions
        let p1 = Point::new(0, 0);
        let p2 = Point::new(0, 0);
        let rect = Rectangle::new(&p1, &p2);
        assert_eq!(rect.rects.len(), 1);

        // Test with negative coordinates
        let p1 = Point::new(-100, -100);
        let p2 = Point::new(100, 100);
        let rect = Rectangle::new(&p1, &p2);
        assert_eq!(rect.rects.len(), 1);

        // Test with very large coordinates
        let p1 = Point::new(0, 0);
        let p2 = Point::new(1000, 1000);
        let rect = Rectangle::new(&p1, &p2);
        assert_eq!(rect.rects.len(), 1);
    }
}
