use crate::{Lab, Rgb, Xyz};

#[derive(Debug, Clone, Copy, Default, PartialEq)]
pub struct Point3 {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

#[allow(non_snake_case)]
#[inline]
pub const fn Point3(x: f64, y: f64, z: f64) -> Point3 {
    Point3 { x, y, z }
}

impl Point3 {
    /// Computes the Euclidean distance of two points.
    pub fn dist(&self, p: Point3) -> f64 {
        ((self.x - p.x).powf(2.0) + (self.y - p.y).powf(2.0) + (self.z - p.z).powf(2.0)).sqrt()
    }
}

impl From<Rgb> for Point3 {
    fn from(c: Rgb) -> Self {
        Point3 {
            x: (c.r as f64) / 255.0,
            y: (c.g as f64) / 255.0,
            z: (c.b as f64) / 255.0,
        }
    }
}

impl const From<Xyz> for Point3 {
    #[inline]
    fn from(c: Xyz) -> Self {
        Point3(c.x, c.y, c.z)
    }
}

impl const From<Lab> for Point3 {
    #[inline]
    fn from(c: Lab) -> Self {
        Point3(c.l, c.a, c.b)
    }
}
