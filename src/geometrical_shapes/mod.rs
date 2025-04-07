//! Geometric shape module containing all primitive shapes and traits
//! 
//! Provides implementations of various 2D and 3D geometric shapes
//! with rendering capabilities through the Drawable trait.

/// Point primitive module
mod point;

/// Line segment primitive module
mod line;

/// Rectangle primitive module
mod rectangle;

/// Triangle primitive module
mod triangle;

/// Circle primitive module
mod circle;

/// Pentagon primitive module
mod pentagon;

/// 3D Cube primitive module (isometric projection)
mod cubes;

// Primary exports
pub use self::point::Point;
pub use self::line::Line;
pub use self::rectangle::Rectangle;
pub use self::triangle::Triangle;
pub use self::circle::Circle;
use raster::{Color, Image};

// Temporary exports (marked as bonus implementations)
#[allow(unused_imports)]
pub use self::pentagon::Pentagon;
#[allow(unused_imports)]
pub use self::cubes::Cubes;

/// Trait for renderable objects
/// 
/// # Required Methods
/// - `draw`: Renders the object to an image
/// - `color`: Returns the base color of the object
pub trait Drawable {
    fn draw(&self, image: &mut Image);
    fn color(&self) -> Color;
}

/// Trait for displayable surfaces
/// 
/// # With one Required method `display`
/// Provides pixel-level manipulation capabilities
/// for types that can display individual pixels
pub trait Displayable {
    /// Sets the color of a specific pixel
    fn display(&mut self, x: i32, y: i32, color: Color);
}
