pub mod module {

use crate::vec3::module::*;
use crate::types::module::*;
use crate::ray::module::*;
use crate::rand::module::*;

use std::f32::consts;

pub struct Camera {
    origin: Point,
    lower_left_corner: Point,
    horizontal: Vec3,
    vertical: Vec3,
    u: Vec3,
    v: Vec3,
    lens_radius: f32
}

impl Camera {
    // vfov: vertical field of view in degrees
    // aspect: width / height
    pub fn new(lookfrom: Point, lookat: Point, vup: Vec3,
               vfov: f32, aspect: f32,
               aperature: f32, focus_dist: f32) -> Camera {
        let theta = vfov * consts::PI / 180.;
        let half_height = (theta / 2.).tan();
        let half_width  = aspect * half_height;

        let lens_radius = aperature / 2.;

        let w = Vec3::unit_vector(lookfrom - lookat);
        let u = Vec3::unit_vector(Vec3::cross(&vup, &w));
        let v = Vec3::cross(&w, &u);
        Camera {
            origin: lookfrom,
            lower_left_corner: lookfrom - focus_dist * (half_width*u + half_height*v + w),
            horizontal: 2.*half_width*focus_dist*u,
            vertical: 2.*half_height*focus_dist*v,
            u, v,
            lens_radius
        }
    }

    pub fn get_ray(&self, s: f32, t: f32) -> Ray {
        let rd = self.lens_radius * random_in_unit_disk();
        let offset = self.u * rd.x() + self.v * rd.y();
        Ray::new(
            self.origin + offset,
            self.lower_left_corner +
                s*self.horizontal + t*self.vertical - self.origin - offset)
    }
}
    
}