use std::f64::consts::PI;

pub trait Shape {
    fn area(&self) -> f64;
    fn perimeter(&self) -> f64;
}

pub struct Circle {
    radius: f64,
}

impl Circle {
    pub fn new(radius: f64) -> Self {
        Self { radius }
    }
}

impl Shape for Circle {
    fn area(&self) -> f64 {
        PI * self.radius * self.radius
    }

    fn perimeter(&self) -> f64 {
        2 as f64 * PI * self.radius
    }
}

pub struct Polygon<T: Shape> {
    shape: T,
}

impl<T> Polygon<T>
where
    T: Shape,
{
    pub fn get_area(&self) -> f64 {
        self.shape.area()
    }

    pub fn new(shape: T) -> Self {
        Self { shape }
    }
}
