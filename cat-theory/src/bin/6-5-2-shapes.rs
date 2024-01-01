use std::arch::x86_64;

fn main() {

}

trait Shape {
    fn area(&self) -> f64; 
    fn circumference(&self) -> f64;
}

// oop version
// easier to add variants, harder to add functions
struct Circle {
    radius: f64
}

impl Shape for Circle {
    fn area(&self) -> f64 {
        std::f64::consts::PI * self.radius.powf(2 as f64)
    }

    fn circumference(&self) -> f64 {
        2.0 * std::f64::consts::PI * self.radius
    }
}

struct Rect {
    width: f64,
    height: f64
}

impl Shape for Rect {
    fn area(&self) -> f64 {
        self.width * self.height
    }

    fn circumference(&self) -> f64 {
        2.0 * self.width + 2.0 * self.height
    }
}

struct Square {
    side: f64
}

impl Shape for Square {
    fn area(&self) -> f64 {
        self.side.powf(2.0)
    }

    fn circumference(&self) -> f64 {
        4.0 * self.side
    }
}




// enum version
// easier to add functions when no variants are needed
enum ShapeEnum {
    CIRCLE { radius: f64 },
    RECT { width: f64, height: f64 },
    SQUARE { side: f64 }
}

impl ShapeEnum {
    fn area(&self) -> f64 {
        match self {
            Self::CIRCLE { radius: x } => std::f64::consts::PI * x.powi(2),
            Self::RECT { width: a, height: b } => a * b,
            Self::SQUARE { side: x } => x.powi(2)
        }
    }

    fn circumference(&self) -> f64 {
        match self {
            Self::CIRCLE { radius: r } => 2.0 * std::f64::consts::PI * r,
            Self::RECT { width: a, height: b } => 2.0 * a + 2.0 * b,
            Self::SQUARE { side: x } => 4.0 * x
        }
    }
}