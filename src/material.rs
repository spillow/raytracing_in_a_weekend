pub mod module {

use crate::ray::module::*;
use crate::hittable::module::*;
use crate::vec3::module::*;
use crate::types::module::*;

use rand::Rng;

pub trait Material {
    fn scatter(
        &self,
        r_in: &Ray,
        record: &HitRecord,
        attenuation: &mut Vec3,
        scattered: &mut Ray) -> bool;

    fn get_idx(&self) -> u32;
}

#[derive(Copy, Clone)]
pub struct Lambertian {
    albedo: Vec3,
    idx: u32 // index in material table
}

#[derive(Copy, Clone)]
pub struct Metal {
    albedo: Vec3,
    fuzz: f32,
    idx: u32 // index in material table
}

impl Lambertian {
    pub fn new(albedo: Vec3, idx: u32) -> Lambertian {
        Lambertian { albedo, idx }
    }
}

impl Metal {
    pub fn new(albedo: Vec3, fuzz:f32, idx: u32) -> Metal {
        let clamped_fuzz = if fuzz < 1. { fuzz } else { 1. };
        Metal { albedo, fuzz:clamped_fuzz, idx }
    }
}

fn rand_unit() -> f32 {
    rand::thread_rng().gen_range(0.0f32, 1.0f32)
}

fn random_in_unit_sphere() -> Vec3 {
    let mut p:Point;
    loop {
        p = 2.0 * Vec3::new(rand_unit(), rand_unit(), rand_unit()) - Vec3::new(1.,1.,1.);
        if p.squared_length() < 1.0 {
            break;
        }
    }
    p
}

impl Material for Lambertian {
    fn scatter(
        &self,
        _: &Ray,
        record: &HitRecord,
        attenuation: &mut Vec3,
        scattered: &mut Ray) -> bool {

        let target = record.p + record.normal + random_in_unit_sphere();
        *scattered = Ray::new(record.p, target - record.p);
        *attenuation = self.albedo;

        true
    }

    fn get_idx(&self) -> u32 {
        self.idx
    }
}

fn reflect(v: Vec3, n: Vec3) -> Vec3 {
    v - 2. * Vec3::dot(&v, &n) * n
}

impl Material for Metal {
    fn scatter(
        &self,
        r_in: &Ray,
        record: &HitRecord,
        attenuation: &mut Vec3,
        scattered: &mut Ray) -> bool {

        let reflected = reflect(Vec3::unit_vector(r_in.dir()), record.normal);
        *scattered = Ray::new(record.p, reflected + self.fuzz * random_in_unit_sphere());
        *attenuation = self.albedo;

        Vec3::dot(&scattered.dir(), &record.normal) > 0.
    }

    fn get_idx(&self) -> u32 {
        self.idx
    }
}

}