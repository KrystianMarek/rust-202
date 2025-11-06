//! # Composition over Inheritance
//!
//! Rust doesn't have classical inheritance. Instead, it uses composition
//! and traits to achieve polymorphism and code reuse.

use std::fmt;

/// A trait representing drawable objects
///
/// ## Why?
/// Traits provide polymorphism without inheritance, avoiding the fragile base class
/// problem common in Python/C++ OOP. Unlike Go interfaces which are implicit,
/// Rust traits must be explicitly implemented, providing better documentation.
///
/// ## Example
/// ```rust
/// use rust_202::oop::composition::{Drawable, Circle};
///
/// let circle = Circle::new(5.0);
/// circle.draw();
/// ```
pub trait Drawable {
    /// Draw the object
    fn draw(&self);

    /// Get a description of the object
    fn description(&self) -> String {
        "A drawable object".to_string()
    }
}

/// A trait for objects with area
pub trait HasArea {
    /// Calculate the area
    fn area(&self) -> f64;
}

/// A trait for objects with perimeter
pub trait HasPerimeter {
    /// Calculate the perimeter
    fn perimeter(&self) -> f64;
}

/// Circle implementation using composition
///
/// ## Example
/// ```rust
/// use rust_202::oop::composition::{Circle, Drawable, HasArea};
///
/// let circle = Circle::new(5.0);
/// assert_eq!(circle.area(), std::f64::consts::PI * 25.0);
/// circle.draw();
/// ```
#[derive(Debug, Clone)]
pub struct Circle {
    radius: f64,
}

impl Circle {
    /// Create a new circle
    pub fn new(radius: f64) -> Self {
        Self { radius }
    }
}

impl Drawable for Circle {
    fn draw(&self) {
        println!("Drawing a circle with radius {}", self.radius);
    }

    fn description(&self) -> String {
        format!("Circle(radius: {})", self.radius)
    }
}

impl HasArea for Circle {
    fn area(&self) -> f64 {
        std::f64::consts::PI * self.radius * self.radius
    }
}

impl HasPerimeter for Circle {
    fn perimeter(&self) -> f64 {
        2.0 * std::f64::consts::PI * self.radius
    }
}

/// Rectangle implementation
#[derive(Debug, Clone)]
pub struct Rectangle {
    width: f64,
    height: f64,
}

impl Rectangle {
    /// Create a new rectangle
    pub fn new(width: f64, height: f64) -> Self {
        Self { width, height }
    }
}

impl Drawable for Rectangle {
    fn draw(&self) {
        println!("Drawing a rectangle {}x{}", self.width, self.height);
    }

    fn description(&self) -> String {
        format!("Rectangle({}x{})", self.width, self.height)
    }
}

impl HasArea for Rectangle {
    fn area(&self) -> f64 {
        self.width * self.height
    }
}

impl HasPerimeter for Rectangle {
    fn perimeter(&self) -> f64 {
        2.0 * (self.width + self.height)
    }
}

/// Composition example: A shape with metadata
///
/// ## Why?
/// This demonstrates composition: embedding one struct in another.
/// Unlike inheritance, this makes relationships explicit and avoids
/// the diamond problem.
///
/// ## Example
/// ```rust
/// use rust_202::oop::composition::{CompositionExample, Circle};
///
/// let circle = Circle::new(5.0);
/// let example = CompositionExample::new(Box::new(circle), "Red");
/// assert_eq!(example.color(), "Red");
/// ```
pub struct CompositionExample {
    shape: Box<dyn Drawable>,
    color: String,
}

impl CompositionExample {
    /// Create a new composition example
    pub fn new(shape: Box<dyn Drawable>, color: &str) -> Self {
        Self {
            shape,
            color: color.to_string(),
        }
    }

    /// Get the color
    pub fn color(&self) -> &str {
        &self.color
    }

    /// Draw with color information
    pub fn draw_with_color(&self) {
        println!("Color: {}", self.color);
        self.shape.draw();
    }
}

impl fmt::Display for CompositionExample {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} (color: {})", self.shape.description(), self.color)
    }
}

/// Trait objects for runtime polymorphism
///
/// ## Why?
/// `dyn Trait` enables runtime polymorphism, similar to interfaces in Go
/// or virtual methods in C++. Unlike Python where everything is dynamic,
/// Rust's trait objects have predictable performance (one vtable indirection).
///
/// ## Example
/// ```rust
/// use rust_202::oop::composition::{ShapeCollection, Circle, Rectangle};
///
/// let mut collection = ShapeCollection::new();
/// collection.add(Box::new(Circle::new(5.0)));
/// collection.add(Box::new(Rectangle::new(10.0, 20.0)));
/// assert_eq!(collection.count(), 2);
/// ```
pub struct ShapeCollection {
    shapes: Vec<Box<dyn Drawable>>,
}

impl ShapeCollection {
    /// Create a new collection
    pub fn new() -> Self {
        Self { shapes: Vec::new() }
    }

    /// Add a shape to the collection
    pub fn add(&mut self, shape: Box<dyn Drawable>) {
        self.shapes.push(shape);
    }

    /// Draw all shapes
    pub fn draw_all(&self) {
        for shape in &self.shapes {
            shape.draw();
        }
    }

    /// Get the count of shapes
    pub fn count(&self) -> usize {
        self.shapes.len()
    }
}

impl Default for ShapeCollection {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_circle() {
        let circle = Circle::new(5.0);
        assert!((circle.area() - std::f64::consts::PI * 25.0).abs() < 0.001);
        circle.draw();
    }

    #[test]
    fn test_rectangle() {
        let rect = Rectangle::new(10.0, 20.0);
        assert_eq!(rect.area(), 200.0);
        assert_eq!(rect.perimeter(), 60.0);
    }

    #[test]
    fn test_composition() {
        let circle = Circle::new(5.0);
        let comp = CompositionExample::new(Box::new(circle), "Red");
        assert_eq!(comp.color(), "Red");
        comp.draw_with_color();
    }

    #[test]
    fn test_shape_collection() {
        let mut collection = ShapeCollection::new();
        collection.add(Box::new(Circle::new(5.0)));
        collection.add(Box::new(Rectangle::new(10.0, 20.0)));
        assert_eq!(collection.count(), 2);
        collection.draw_all();
    }
}
