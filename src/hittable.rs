pub mod module {

use crate::types::module::*;
use crate::vec3::module::*;
use crate::ray::module::*;

#[derive(Clone, Copy, Default)]
pub struct HitRecord {
    pub t: f32,       // t parameter
    pub p: Point,     // hit location
    pub normal: Vec3, // surface normal
    pub mat: Option<u32>
}

pub trait Hittable {
    fn hit(&self, r: &Ray, t_min: f32, t_max: f32, record: &mut HitRecord) -> bool;
}

pub struct Sphere {
    center: Point,
    radius: f32,
    material: u32
}

impl Sphere {
    pub fn new(center: Point, radius: f32, material: u32) -> Sphere {
        Sphere { center, radius, material}
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
                // this is normalized
                record.normal = (record.p - s.center()) / s.radius();
                record.mat = Some(self.material);
                return true;
            }
            // check the other root
            let curr_t = (-b + discriminant.sqrt()) / (2. * a);
            if curr_t < t_max && curr_t > t_min { // check in range
                record.t = curr_t;
                record.p = r.point_at_parameter(curr_t);
                // this is normalized
                record.normal = (record.p - s.center()) / s.radius();
                record.mat = Some(self.material);
                return true;
            }
        }
        // failed to hit
        false
    }
}

// a list of hittable objects
pub struct HittableList {
    list: Vec<Box<dyn Hittable>>
}

impl HittableList {
    pub fn new(v: Vec<Box<dyn Hittable>>) -> HittableList {
        HittableList { list:v }
    }
}

impl Hittable for HittableList {
    fn hit(&self, r: &Ray, t_min: f32, t_max: f32, record: &mut HitRecord) -> bool {
        let mut tmp_record = HitRecord::default();
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