pub mod module {

use crate::types::module::*;

pub struct Sphere {
    center: Point,
    radius: f32
}

impl Sphere {
    pub fn new(center: Point, radius: f32) -> Sphere {
        Sphere { center: center, radius: radius }
    }

    pub fn center(&self) -> Point {
        self.center
    }

    pub fn radius(&self) -> f32 {
        self.radius
    }
}

}