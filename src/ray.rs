
pub mod module {

use crate::vec3::module::*;
use crate::types::module::*;

pub struct Ray {
    orig: Vec3,
    dir: Vec3
}

impl Ray {
    pub fn new(orig: Point, dir: Vec3) -> Ray {
        Ray { orig: orig, dir: dir }
    }

    pub fn origin(&self) -> Point {
        self.orig
    }

    pub fn dir(&self) -> Vec3 {
        self.dir
    }

    pub fn point_at_parameter(&self, t: f32) -> Point {
        self.orig + t * self.dir
    }
}

}