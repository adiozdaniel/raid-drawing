// line.rs
use super::{Displayable, Drawable, Point};
use rand::Rng;
use raster::{Color, Image};

/// Implementation of PartialEq for Point to enable equality comparisons
impl PartialEq for Point {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y
    }
}

/// Draws a line with specified color and thickness using Bresenham's algorithm
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

/// Represents a line segment with start/end points, thickness and color
pub struct Line {
    start: Point,
    end: Point,
    thickness: i32,
    color: Color,
}

#[allow(dead_code)]
impl Line {
    /// Creates a new line between two points with random thickness and color
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

    /// Creates a random line within specified bounds
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

    /// Creates a line with explicit parameters
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
    /// Draws the line on the specified image
    fn draw(&self, image: &mut Image) {
        draw_line_with_color(&self.start, &self.end, self.thickness, image, &self.color);
    }

    /// Returns the color of the line
    fn color(&self) -> Color {
        self.color.clone()
    }
}

// Unit tests for Line
#[cfg(test)]
mod tests {
    use super::*;
    use raster::Image;

    // Test basic line creation with expected properties
    #[test]
    fn test_line_new() {
        let p1 = Point::new(0, 0);
        let p2 = Point::new(10, 10);
        let line = Line::new(&p1, &p2);

        assert_eq!(line.start, p1);
        assert_eq!(line.end, p2);
        assert!(line.thickness >= 2 && line.thickness < 5);
        assert!(line.color.r >= 50 && line.color.r < 200);
        assert!(line.color.g >= 50 && line.color.g < 200);
        assert!(line.color.b >= 50 && line.color.b < 200);
    }

    // Test random line generation stays within bounds
    #[test]
    fn test_line_random() {
        let width = 100;
        let height = 100;
        let line = Line::random(width, height);

        assert!(line.start.x >= 0 && line.start.x < width);
        assert!(line.start.y >= 0 && line.start.y < height);
        assert!(line.end.x >= 0 && line.end.x < width);
        assert!(line.end.y >= 0 && line.end.y < height);
        assert!(line.thickness >= 2 && line.thickness < 5);
        assert!(line.color.r >= 50 && line.color.r < 200);
        assert!(line.color.g >= 50 && line.color.g < 200);
        assert!(line.color.b >= 50 && line.color.b < 200);
    }

    // Test line creation with explicit parameters
    #[test]
    fn test_line_from_points() {
        let p1 = Point::new(0, 0);
        let p2 = Point::new(10, 10);
        let thickness = 3;
        let color = Color::rgb(100, 100, 100);
        let line = Line::from_points(&p1, &p2, thickness, color.clone());

        assert_eq!(line.start, p1);
        assert_eq!(line.end, p2);
        assert_eq!(line.thickness, thickness);
        assert_eq!(line.color.r, color.r);
        assert_eq!(line.color.g, color.g);
        assert_eq!(line.color.b, color.b);
    }

    // Test color getter returns correct color
    #[test]
    fn test_line_color() {
        let p1 = Point::new(0, 0);
        let p2 = Point::new(10, 10);
        let thickness = 3;
        let color = Color::rgb(100, 100, 100);
        let line = Line::from_points(&p1, &p2, thickness, color.clone());

        let line_color = line.color();
        assert_eq!(line_color.r, color.r);
        assert_eq!(line_color.g, color.g);
        assert_eq!(line_color.b, color.b);
    }

    // Test horizontal line drawing
    #[test]
    fn test_draw_horizontal_line() {
        let mut image = Image::blank(20, 20);
        let p1 = Point::new(5, 10);
        let p2 = Point::new(15, 10);
        let thickness = 1;
        let color = Color::rgb(255, 255, 255);
        let line = Line::from_points(&p1, &p2, thickness, color.clone());

        line.draw(&mut image);

        // Verify all pixels along the line were drawn
        for x in 5..=15 {
            let pixel = image.get_pixel(x, 10).unwrap();
            assert_eq!(pixel.r, color.r);
            assert_eq!(pixel.g, color.g);
            assert_eq!(pixel.b, color.b);
        }
    }

    // Test vertical line drawing
    #[test]
    fn test_draw_vertical_line() {
        let mut image = Image::blank(20, 20);
        let p1 = Point::new(10, 5);
        let p2 = Point::new(10, 15);
        let thickness = 1;
        let color = Color::rgb(255, 255, 255);
        let line = Line::from_points(&p1, &p2, thickness, color.clone());

        line.draw(&mut image);

        // Verify all pixels along the line were drawn
        for y in 5..=15 {
            let pixel = image.get_pixel(10, y).unwrap();
            assert_eq!(pixel.r, color.r);
            assert_eq!(pixel.g, color.g);
            assert_eq!(pixel.b, color.b);
        }
    }

    // Test diagonal line drawing
    #[test]
    fn test_draw_diagonal_line() {
        let mut image = Image::blank(20, 20);
        let p1 = Point::new(5, 5);
        let p2 = Point::new(15, 15);
        let thickness = 1;
        let color = Color::rgb(255, 255, 255);
        let line = Line::from_points(&p1, &p2, thickness, color.clone());

        line.draw(&mut image);

        // Verify all pixels along the line were drawn
        for i in 0..=10 {
            let pixel = image.get_pixel(5 + i, 5 + i).unwrap();
            assert_eq!(pixel.r, color.r);
            assert_eq!(pixel.g, color.g);
            assert_eq!(pixel.b, color.b);
        }
    }

    // Test thick line drawing (3 pixels wide)
    #[test]
    fn test_thick_line() {
        let mut image = Image::blank(20, 20);
        let p1 = Point::new(5, 10);
        let p2 = Point::new(15, 10);
        let thickness = 3;
        let color = Color::rgb(255, 255, 255);
        let line = Line::from_points(&p1, &p2, thickness, color.clone());

        line.draw(&mut image);

        // Verify the line has thickness by checking multiple rows
        for x in 5..=15 {
            for y in 9..=11 {
                let pixel = image.get_pixel(x, y).unwrap();
                assert_eq!(pixel.r, color.r);
                assert_eq!(pixel.g, color.g);
                assert_eq!(pixel.b, color.b);
            }
        }
    }

    // Test line drawing at image boundaries
    #[test]
    fn test_line_boundaries() {
        let mut image = Image::blank(10, 10);
        let p1 = Point::new(0, 0);
        let p2 = Point::new(9, 9);
        let thickness = 1;
        let color = Color::rgb(255, 255, 255);
        let line = Line::from_points(&p1, &p2, thickness, color.clone());

        line.draw(&mut image);

        // Verify both endpoints were drawn
        let pixel1 = image.get_pixel(0, 0).unwrap();
        let pixel2 = image.get_pixel(9, 9).unwrap();
        assert_eq!(pixel1.r, color.r);
        assert_eq!(pixel1.g, color.g);
        assert_eq!(pixel1.b, color.b);
        assert_eq!(pixel2.r, color.r);
        assert_eq!(pixel2.g, color.g);
        assert_eq!(pixel2.b, color.b);
    }
}
