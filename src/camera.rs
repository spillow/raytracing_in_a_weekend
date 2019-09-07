pub mod module {

use crate::vec3::module::*;
use crate::types::module::*;
use crate::ray::module::*;
use std::f32::consts;

pub struct Camera {
    origin: Point,
    lower_left_corner: Point,
    horizontal: Vec3,
    vertical: Vec3
}

impl Camera {
    // vfov: vertical field of view in degrees
    // aspect: width / height
    pub fn new(lookfrom: Point, lookat: Point, vup: Vec3, vfov: f32, aspect: f32) -> Camera {
        let theta = vfov * consts::PI / 180.;
        let half_height = (theta / 2.).tan();
        let half_width  = aspect * half_height;

        let w = Vec3::unit_vector(lookfrom - lookat);
        let u = Vec3::unit_vector(Vec3::cross(&vup, &w));
        let v = Vec3::cross(&w, &u);
        Camera {
            origin: lookfrom,
            lower_left_corner: lookfrom - half_width*u - half_height*v - w,
            horizontal: 2.*half_width*u,
            vertical: 2.*half_height*v
        }
    }

    pub fn get_ray(&self, s: f32, t: f32) -> Ray {
        Ray::new(
            self.origin,
            self.lower_left_corner + s*self.horizontal + t*self.vertical - self.origin)
    }
}
    
}