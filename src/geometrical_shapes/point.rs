use super::{Displayable, Drawable};
use rand::Rng;
use raster::{Color, Image};

/// Represents a 2D point with coordinates and color
/// Can be rendered as a small 3x3 pixel area for visibility
#[derive(Debug, Clone)]
pub struct Point {
    pub x: i32,
    pub y: i32,
    pub color: Color,
}

impl Point {
    /// Creates a new point at specified coordinates
    /// Generates a random moderate color (RGB 50-200)
    pub fn new(x: i32, y: i32) -> Self {
        Point {
            x,
            y,
            color: Color::rgb(
                rand::thread_rng().gen_range(50..200),
                rand::thread_rng().gen_range(50..200),
                rand::thread_rng().gen_range(50..200),
            ),
        }
    }

    /// Generates a random point within specified bounds
    /// Creates point with coordinates within width/height limits
    /// Uses random moderate color (RGB 50-200)
    pub fn random(width: i32, height: i32) -> Self {
        let mut rng = rand::thread_rng();
        Point {
            x: rng.gen_range(0..width),
            y: rng.gen_range(0..height),
            color: Color::rgb(
                rng.gen_range(50..200),
                rng.gen_range(50..200),
                rng.gen_range(50..200),
            ),
        }
    }
}

impl Drawable for Point {
    /// Renders the point as a 3x3 pixel area
    /// Implements Drawable trait requirement
    fn draw(&self, image: &mut Image) {
        for dx in -1..=1 {
            for dy in -1..=1 {
                image.display(self.x + dx, self.y + dy, self.color());
            }
        }
    }

    /// Returns the color of the point
    /// Implements Drawable trait requirement
    fn color(&self) -> Color {
        self.color.clone()
    }
}

// Unit tests for Point
#[cfg(test)]
mod tests {
    use super::*;

    // Test basic point creation with expected properties
    #[test]
    fn test_point_new() {
        let point = Point::new(100, 100);

        assert_eq!(point.x, 100);
        assert_eq!(point.y, 100);
        assert!(point.color.r >= 50 && point.color.r <= 200);
        assert!(point.color.g >= 50 && point.color.g <= 200);
        assert!(point.color.b >= 50 && point.color.b <= 200);
    }

    // Test random point generation stays within bounds
    #[test]
    fn test_point_random() {
        let point = Point::random(800, 800);

        assert!(point.x >= 0 && point.x < 800);
        assert!(point.y >= 0 && point.y < 800);
        assert!(point.color.r >= 50 && point.color.r <= 200);
        assert!(point.color.g >= 50 && point.color.g <= 200);
        assert!(point.color.b >= 50 && point.color.b <= 200);
    }

    // Test point drawing affects a 3x3 pixel area
    #[test]
    fn test_point_draw() {
        let point = Point::new(100, 100);

        let mut image = Image::blank(800, 800);
        point.draw(&mut image);

        // Verify that the image was modified in a 3x3 area around the point
        for dx in -1..=1 {
            for dy in -1..=1 {
                let pixel = image
                    .get_pixel((100 + dx) as i32, (100 + dy) as i32)
                    .unwrap();
                assert!(pixel.r >= 50 && pixel.r <= 200);
                assert!(pixel.g >= 50 && pixel.g <= 200);
                assert!(pixel.b >= 50 && pixel.b <= 200);
            }
        }
    }

    // Test color getter returns correct color
    #[test]
    fn test_point_color() {
        let point = Point::new(100, 100);
        let color = point.color();

        assert!(color.r >= 50 && color.r <= 200);
        assert!(color.g >= 50 && color.g <= 200);
        assert!(color.b >= 50 && color.b <= 200);
    }

    // Test edge cases including negative and large coordinates
    #[test]
    fn test_point_edge_cases() {
        // Test with negative coordinates
        let point = Point::new(-100, -100);
        assert_eq!(point.x, -100);
        assert_eq!(point.y, -100);

        // Test with very large coordinates
        let point = Point::new(1000, 1000);
        assert_eq!(point.x, 1000);
        assert_eq!(point.y, 1000);

        // Test with zero coordinates
        let point = Point::new(0, 0);
        assert_eq!(point.x, 0);
        assert_eq!(point.y, 0);
    }
}
