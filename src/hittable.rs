pub mod module {

use crate::types::module::*;
use crate::vec3::module::*;
use crate::ray::module::*;

#[derive(Clone, Copy)]
pub struct HitRecord {
    pub t: f32,      // t parameter
    pub p: Point,    // hit location
    pub normal: Vec3 // surface normal
}

impl HitRecord {
    pub fn new() -> HitRecord {
        HitRecord { t:0., p:Point::new(0.,0.,0.), normal:Vec3::new(0.,0.,0.) }
    }
}

pub trait Hittable {
    fn hit(&self, r: &Ray, t_min: f32, t_max: f32, record: &mut HitRecord) -> bool;
}

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

impl Hittable for Sphere {
    fn hit(&self, r: &Ray, t_min: f32, t_max: f32, record: &mut HitRecord) -> bool {
        let s = self;
        let oc = r.origin() - s.center();
        let rdir = r.dir();
        let a:f32 = Vec3::dot(&rdir, &rdir);
        let b:f32 = 2. * Vec3::dot(&oc, &rdir);
        let c:f32 = Vec3::dot(&oc, &oc) - s.radius()*s.radius();
        let discriminant:f32 = b*b - 4.*a*c;
        if discriminant > 0. {
            // check the first root
            let curr_t = (-b - discriminant.sqrt()) / (2. * a);
            if curr_t < t_max && curr_t > t_min { // check in range
                record.t = curr_t;
                record.p = r.point_at_parameter(curr_t);
                record.normal = (record.p - s.center()) / s.radius();
                return true;
            }
            // check the other root
            let curr_t = (-b + discriminant.sqrt()) / (2. * a);
            if curr_t < t_max && curr_t > t_min { // check in range
                record.t = curr_t;
                record.p = r.point_at_parameter(curr_t);
                record.normal = (record.p - s.center()) / s.radius();
                return true;
            }
        }
        // failed to hit
        false
    }
}

// a list of hittable objects
pub struct HittableList<'a> {
    list: Vec<&'a dyn Hittable>
}

impl HittableList<'_> {
    pub fn new<'a>(v: Vec<&'a dyn Hittable>) -> HittableList<'a> {
        HittableList { list:v }
    }
}

impl Hittable for HittableList<'_> {
    fn hit(&self, r: &Ray, t_min: f32, t_max: f32, record: &mut HitRecord) -> bool {
        let mut tmp_record = HitRecord::new();
        let mut hit_anything = false;
        let mut closest_so_far = t_max;
        for obj in self.list.iter() {
            if obj.hit(r, t_min, closest_so_far, &mut tmp_record) {
                hit_anything = true;
                closest_so_far = tmp_record.t;
                *record = tmp_record;
            }
        }
        hit_anything
    }
}

}