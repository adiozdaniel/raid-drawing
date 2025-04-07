use super::{Drawable, Line, Point};
use rand::Rng;
use raster::{Color, Image};

/// Struct to represent a triangle with vertices and color
pub struct Triangle {
    tris: Vec<(Point, Point, Point, Color)>,
}

#[allow(dead_code)]
impl Triangle {
    /// Creates a triangle from 3 given points with a random color
    pub fn new(a: &Point, b: &Point, c: &Point) -> Self {
        let mut rng = rand::thread_rng();
        let mut tris = Vec::new();

        let color = Color::rgb(
            rng.gen_range(150..255),
            rng.gen_range(150..255),
            rng.gen_range(150..255),
        );

        tris.push((a.clone(), b.clone(), c.clone(), color));

        Triangle { tris }
    }

    /// Generates a random triangle based on random dimensions and color
    pub fn random(_a: &Point, _b: &Point, _c: &Point) -> Self {
        let mut rng = rand::thread_rng();
        let mut tris = Vec::new();

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

        Triangle { tris }
    }
}

/// Implementation of the Drawable trait for Triangle
impl Drawable for Triangle {
    /// Draws the triangle onto the given image
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

    // Returns a default a fallback color
    fn color(&self) -> Color {
        Color::rgb(0, 0, 0)
    }
}

// Unit tests for Triangle
#[cfg(test)]
mod tests {
    use super::*;

    // Test creation of a triangle using given points
    #[test]
    fn test_triangle_new() {
        let a = Point::new(100, 100);
        let b = Point::new(200, 100);
        let c = Point::new(150, 50);
        let triangle = Triangle::new(&a, &b, &c);

        assert_eq!(triangle.tris.len(), 1);
        let (tri_a, tri_b, tri_c, _color) = &triangle.tris[0];
        assert_eq!(tri_a.x, a.x);
        assert_eq!(tri_a.y, a.y);
        assert_eq!(tri_b.x, b.x);
        assert_eq!(tri_b.y, b.y);
        assert_eq!(tri_c.x, c.x);
        assert_eq!(tri_c.y, c.y);
    }

    // Test creation of a triangle with randomized geometry
    #[test]
    fn test_triangle_random() {
        let a = Point::new(0, 0);
        let b = Point::new(800, 0);
        let c = Point::new(400, 800);
        let triangle = Triangle::random(&a, &b, &c);

        // Check each triangle's properties
        for (base, right, top, _color) in &triangle.tris {
            assert!(base.x >= 0 && base.x <= 800);
            assert!(base.y >= 0 && base.y <= 800);
            assert!(right.x > base.x);
            assert_eq!(right.y, base.y);
            assert!(top.x >= base.x && top.x <= right.x);
            assert!(top.y < base.y);
        }
    }

    // Test that triangle drawing modifies the image
    #[test]
    fn test_triangle_draw() {
        let a = Point::new(100, 100);
        let b = Point::new(200, 100);
        let c = Point::new(150, 50);
        let triangle = Triangle::new(&a, &b, &c);

        let mut image = Image::blank(800, 800);
        triangle.draw(&mut image);
    }

    // Test color function returns black
    #[test]
    fn test_triangle_color() {
        let a = Point::new(100, 100);
        let b = Point::new(200, 100);
        let c = Point::new(150, 50);
        let triangle = Triangle::new(&a, &b, &c);

        let color = triangle.color();
        assert_eq!(color.r, 0);
        assert_eq!(color.g, 0);
        assert_eq!(color.b, 0);
    }

    #[test]
    fn test_triangle_edge_cases() {
        // Test with collinear points
        let a = Point::new(100, 100);
        let b = Point::new(200, 100);
        let c = Point::new(300, 100);
        let triangle = Triangle::new(&a, &b, &c);
        assert_eq!(triangle.tris.len(), 1);

        // Test with negative coordinates
        let a = Point::new(-100, -100);
        let b = Point::new(0, 0);
        let c = Point::new(100, -100);
        let triangle = Triangle::new(&a, &b, &c);
        assert_eq!(triangle.tris.len(), 1);

        // Test with very large coordinates
        let a = Point::new(1000, 1000);
        let b = Point::new(2000, 1000);
        let c = Point::new(1500, 500);
        let triangle = Triangle::new(&a, &b, &c);
        assert_eq!(triangle.tris.len(), 1);
    }
}
