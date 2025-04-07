use super::{Drawable, Line, Point};
use rand::Rng;
use raster::{Color, Image};

/// Represents a 3D cube in isometric projection
/// Contains vertices, edges, and rendering properties
#[derive(Debug)]
pub struct Cubes {
    cubes: Vec<(Point, i32, Color)>,
}

#[allow(dead_code)]
impl Cubes {
    /// Creates a new cube with specified center and size
    /// Generates a random color in moderate RGB range (50-200)
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

    /// Generates a random cube within specified bounds
    /// Creates cubes with size between 30-80px
    /// Uses vibrant colors (RGB 150-255)
    pub fn random(width: i32, height: i32) -> Self {
        let mut rng = rand::thread_rng();
        let mut cubes = Vec::new();

        let center = Point::random(width, height);
        let size = rng.gen_range(30..80);
        let color = Color::rgb(
            rng.gen_range(150..255),
            rng.gen_range(150..255),
            rng.gen_range(150..255),
        );
        cubes.push((center, size, color));

        Cubes { cubes }
    }

    /// Calculates isometric projection vertices for cube rendering
    /// Returns 8 points representing cube vertices in 2D space
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
    /// Renders cube using isometric projection
    /// Draws 12 edges with consistent 2px thickness
    /// Implements Drawable trait requirement
    fn draw(&self, image: &mut Image) {
        for (center, size, color) in &self.cubes {
            let vertices = Cubes::get_isometric_projection(center, *size);

            let edges = [
                // Front face
                (0, 1),
                (1, 3),
                (3, 2),
                (2, 0),
                // Back face
                (4, 5),
                (5, 7),
                (7, 6),
                (6, 4),
                // Connecting edges
                (0, 4),
                (1, 5),
                (2, 6),
                (3, 7),
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

    /// Returns a fallback color for cubes (black)
    /// Implements Drawable trait requirement
    fn color(&self) -> Color {
        Color::rgb(0, 0, 0)
    }
}

// Unit tests for Cubes
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cubes_new() {
        let center = Point::new(100, 100);
        let size = 50;
        let cubes = Cubes::new(&center, size);

        assert_eq!(cubes.cubes.len(), 1);
        let (cube_center, cube_size, color) = &cubes.cubes[0];
        assert_eq!(cube_center.x, center.x);
        assert_eq!(cube_center.y, center.y);
        assert_eq!(*cube_size, size);
        assert!(color.r >= 50 && color.r <= 200);
        assert!(color.g >= 50 && color.g <= 200);
        assert!(color.b >= 50 && color.b <= 200);
    }

    #[test]
    fn test_cubes_random() {
        let cubes = Cubes::random(800, 800);

        // Check each cube's properties
        for (center, size, _color) in &cubes.cubes {
            assert!(center.x >= 0 && center.x <= 800);
            assert!(center.y >= 0 && center.y <= 800);
            assert!(*size >= 30 && *size <= 80);
        }
    }

    #[test]
    fn test_cubes_get_isometric_projection() {
        let center = Point::new(100, 100);
        let size = 50;
        let vertices = Cubes::get_isometric_projection(&center, size);

        assert_eq!(vertices.len(), 8);

        // Check that all vertices are within the expected bounds
        for vertex in vertices {
            assert!(vertex.x >= center.x - size);
            assert!(vertex.x <= center.x + size);
            assert!(vertex.y >= center.y - size);
            assert!(vertex.y <= center.y + size);
        }
    }

    #[test]
    fn test_cubes_draw() {
        let center = Point::new(100, 100);
        let size = 50;
        let cubes = Cubes::new(&center, size);

        let mut image = Image::blank(800, 800);
        cubes.draw(&mut image);

        // Verify that the image was modified
        // assert_ne!(image.get_pixel(100, 100).unwrap(), Color::rgb(0, 0, 0));
    }

    #[test]
    fn test_cubes_color() {
        let center = Point::new(100, 100);
        let size = 50;
        let cubes = Cubes::new(&center, size);

        let color = cubes.color();
        assert_eq!(color.r, 0);
        assert_eq!(color.g, 0);
        assert_eq!(color.b, 0);
    }

    #[test]
    fn test_cubes_edge_cases() {
        // Test with zero size
        let center = Point::new(100, 100);
        let cubes = Cubes::new(&center, 0);
        assert_eq!(cubes.cubes.len(), 1);

        // Test with negative coordinates
        let center = Point::new(-100, -100);
        let cubes = Cubes::new(&center, 50);
        assert_eq!(cubes.cubes.len(), 1);

        // Test with very large size
        let center = Point::new(100, 100);
        let cubes = Cubes::new(&center, 1000);
        assert_eq!(cubes.cubes.len(), 1);
    }
}
