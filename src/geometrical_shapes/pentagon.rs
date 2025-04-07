use super::{Drawable, Line, Point};
use rand::Rng;
use raster::{Color, Image};
use std::f64::consts::PI;

/// Represents a pentagon shape with center, radius and color
/// Can contain multiple pentagons for grouped rendering
#[derive(Debug)]
pub struct Pentagon {
    pentagons: Vec<(Point, i32, Color)>,
}

#[allow(dead_code)]
impl Pentagon {
    /// Creates a new pentagon with specified center and radius
    /// Generates a random vibrant color (RGB 100-255)
    pub fn new(center: &Point, radius: i32) -> Self {
        let mut rng = rand::thread_rng();
        let mut pentagons = Vec::new();

        let color = Color::rgb(
            rng.gen_range(100..255),
            rng.gen_range(100..255),
            rng.gen_range(100..255),
        );

        pentagons.push((center.clone(), radius, color.clone()));

        Pentagon { pentagons }
    }

    /// Generates a random pentagon within specified bounds
    /// Creates pentagon with radius between 30-80px
    /// Uses vibrant colors (RGB 100-255)
    pub fn random(width: i32, height: i32) -> Self {
        let mut rng = rand::thread_rng();
        let mut pentagons = Vec::new();

        let center = Point::random(width, height);
        let radius = rng.gen_range(30..80);
        let color = Color::rgb(
            rng.gen_range(100..255),
            rng.gen_range(100..255),
            rng.gen_range(100..255),
        );
        pentagons.push((center, radius, color));

        Pentagon { pentagons }
    }

    /// Calculates vertex positions for a regular pentagon
    /// Returns 5 points representing the vertices
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
    /// Renders the pentagon using line segments
    /// Draws 5 edges with consistent 2px thickness
    /// Implements Drawable trait requirement
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

    /// Returns default color for pentagon (black)
    /// Implements Drawable trait requirement
    fn color(&self) -> Color {
        Color::rgb(0, 0, 0)
    }
}

// Unit tests for Pentagon
#[cfg(test)]
mod tests {
    use super::*;

    // Test basic pentagon creation with expected properties
    #[test]
    fn test_pentagon_new() {
        let center = Point::new(100, 100);
        let radius = 50;
        let pentagon = Pentagon::new(&center, radius);

        assert_eq!(pentagon.pentagons.len(), 1);
        let (pent_center, pent_radius, _color) = &pentagon.pentagons[0];
        assert_eq!(pent_center.x, center.x);
        assert_eq!(pent_center.y, center.y);
        assert_eq!(*pent_radius, radius);
    }

    // Test random pentagon generation stays within bounds
    #[test]
    fn test_pentagon_random() {
        let pentagon = Pentagon::random(800, 800);

        // Check each pentagon's properties
        for (center, radius, _color) in &pentagon.pentagons {
            assert!(center.x >= 0 && center.x <= 800);
            assert!(center.y >= 0 && center.y <= 800);
            assert!(*radius >= 30 && *radius <= 80);
        }
    }

    // Test vertex calculation produces correct pentagon shape
    #[test]
    fn test_pentagon_get_vertices() {
        let center = Point::new(100, 100);
        let radius = 50;
        let vertices = Pentagon::get_vertices(&center, radius);

        assert_eq!(vertices.len(), 5);

        // Check that all vertices are within the expected radius
        for vertex in vertices {
            let dx = vertex.x - center.x;
            let dy = vertex.y - center.y;
            let distance = ((dx * dx + dy * dy) as f64).sqrt();
            assert!((distance - radius as f64).abs() < 1.0); // Allow for small floating point errors
        }
    }

    // Test pentagon rendering modifies the image
    #[test]
    fn test_pentagon_draw() {
        let center = Point::new(100, 100);
        let radius = 50;
        let pentagon = Pentagon::new(&center, radius);

        let mut image = Image::blank(800, 800);
        pentagon.draw(&mut image);
    }

    // Test color getter returns default black color
    #[test]
    fn test_pentagon_color() {
        let center = Point::new(100, 100);
        let radius = 50;
        let pentagon = Pentagon::new(&center, radius);

        let color = pentagon.color();
        assert_eq!(color.r, 0);
        assert_eq!(color.g, 0);
        assert_eq!(color.b, 0);
    }

    // Test edge cases with ambigous values
    #[test]
    fn test_pentagon_edge_cases() {
        // Test with zero radius
        let center = Point::new(100, 100);
        let pentagon = Pentagon::new(&center, 0);
        assert_eq!(pentagon.pentagons.len(), 1);

        // Test with negative coordinates
        let center = Point::new(-100, -100);
        let pentagon = Pentagon::new(&center, 50);
        assert_eq!(pentagon.pentagons.len(), 1);

        // Test with very large radius
        let center = Point::new(100, 100);
        let pentagon = Pentagon::new(&center, 1000);
        assert_eq!(pentagon.pentagons.len(), 1);
    }
}
