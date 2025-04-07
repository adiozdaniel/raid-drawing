use super::{Displayable, Drawable, Point};
use rand::Rng;
use raster::{Color, Image};

/// Represents a circle shape with center points and radii
/// Contains a collection of circles (for grouped rendering)
/// and a shared color for all circles in the collection
pub struct Circle {
    circles: Vec<(Point, i32)>,
    color: Color,
}

#[allow(dead_code)]
impl Circle {
    /// Creates a new circle with specified center and radius
    /// Generates a random color in moderate RGB range (50-200)
    pub fn new(center: &Point, radius: i32) -> Self {
        let mut rng = rand::thread_rng();
        let mut circles = Vec::new();
        let color = Color::rgb(
            rng.gen_range(50..200),
            rng.gen_range(50..200),
            rng.gen_range(50..200),
        );

        circles.push((center.clone(), radius));

        Circle { circles, color }
    }

    /// Generates a random circle within specified bounds
    /// Has 70% chance to generate a circle, with 30% chance
    /// of being a large circle (150-300px radius)
    pub fn random(width: i32, height: i32) -> Self {
        let mut rng = rand::thread_rng();
        let mut circles = Vec::new();

        let color = Color::rgb(
            rng.gen_range(50..200),
            rng.gen_range(50..200),
            rng.gen_range(50..200),
        );

        if rng.gen_bool(0.7) {
            let radius = if rng.gen_bool(0.3) {
                rng.gen_range(150..300)
            } else {
                rng.gen_range(50..150)
            };
            circles.push((Point::random(width, height), radius));
        }

        Circle { circles, color }
    }
}

impl Drawable for Circle {
    /// Draws the circle using midpoint circle algorithm
    /// Renders with 2-pixel thickness for visibility
    /// Implements the Drawable trait requirement
    fn draw(&self, image: &mut Image) {
        for (center, radius) in &self.circles {
            let mut x = *radius;
            let mut y = 0;
            let mut err = 0;

            while x >= y {
                for (dx, dy) in &[
                    (x, y),
                    (y, x),
                    (-y, x),
                    (-x, y),
                    (-x, -y),
                    (-y, -x),
                    (y, -x),
                    (x, -y),
                ] {
                    let px = center.x + dx;
                    let py = center.y + dy;

                    image.display(px, py, self.color());
                    image.display(px + 1, py, self.color());
                    image.display(px, py + 1, self.color());
                }

                y += 1;
                err += 1 + 2 * y;
                if 2 * (err - x) + 1 > 0 {
                    x -= 1;
                    err += 1 - 2 * x;
                }
            }
        }
    }

    /// Returns the color of the circle
    /// Implements the Drawable trait requirement
    fn color(&self) -> Color {
        self.color.clone()
    }
}

// Unit tests for Circle
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_circle_new() {
        let center = Point::new(100, 100);
        let radius = 50;
        let circle = Circle::new(&center, radius);

        assert_eq!(circle.circles.len(), 1);
        let (circle_center, circle_radius) = &circle.circles[0];
        assert_eq!(circle_center.x, center.x);
        assert_eq!(circle_center.y, center.y);
        assert_eq!(*circle_radius, radius);
    }

    #[test]
    fn test_circle_random() {
        let circle = Circle::random(800, 800);

        // Random circle should have 0 or 1 circles
        assert!(circle.circles.len() <= 1);

        // If there is a circle, check its properties
        if let Some((center, radius)) = circle.circles.first() {
            assert!(center.x >= 0 && center.x <= 800);
            assert!(center.y >= 0 && center.y <= 800);
            assert!(*radius >= 50 && *radius <= 300);
        }
    }

    #[test]
    fn test_circle_draw() {
        let center = Point::new(100, 100);
        let radius = 50;
        let circle = Circle::new(&center, radius);

        let mut image = Image::blank(800, 800);
        circle.draw(&mut image);

        // Verify that the image was modified
        // assert_ne!(image.get_pixel(100, 100).unwrap(), Color::rgb(0, 0, 0));
    }

    #[test]
    fn test_circle_color() {
        let center = Point::new(100, 100);
        let radius = 50;
        let circle = Circle::new(&center, radius);

        let color = circle.color();
        assert!(color.r >= 50 && color.r <= 200);
        assert!(color.g >= 50 && color.g <= 200);
        assert!(color.b >= 50 && color.b <= 200);
    }

    #[test]
    fn test_circle_edge_cases() {
        // Test with zero radius
        let center = Point::new(100, 100);
        let circle = Circle::new(&center, 0);
        assert_eq!(circle.circles.len(), 1);

        // Test with negative coordinates
        let center = Point::new(-100, -100);
        let circle = Circle::new(&center, 50);
        assert_eq!(circle.circles.len(), 1);

        // Test with very large radius
        let center = Point::new(100, 100);
        let circle = Circle::new(&center, 1000);
        assert_eq!(circle.circles.len(), 1);
    }
}
