pub mod module {

use crate::vec3::module::*;
use crate::types::module::*;
use crate::ray::module::*;

pub struct Camera {
    origin: Point,
    lower_left_corner: Point,
    horizontal: Vec3,
    vertical: Vec3
}

impl Camera {
    pub fn new() -> Camera {
        Camera {
            origin: Point::new(0., 0., 0.),
            lower_left_corner: Point::new(-2., -1., -1.),
            horizontal: Vec3::new(4., 0., 0.),
            vertical: Vec3::new(0., 2., 0.)
        }
    }

    pub fn get_ray(&self, u: f32, v: f32) -> Ray {
        Ray::new(
            self.origin,
            self.lower_left_corner + u*self.horizontal + v*self.vertical - self.origin)
    }
}
    
}