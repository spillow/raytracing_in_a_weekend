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

#[derive(Copy, Clone)]
pub struct Dielectric {
    ref_idx: f32,
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

impl Dielectric {
    pub fn new(ref_idx: f32, idx: u32) -> Dielectric {
        Dielectric { ref_idx, idx }
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

impl Material for Dielectric {
    fn scatter(
        &self,
        r_in: &Ray,
        record: &HitRecord,
        attenuation: &mut Vec3,
        scattered: &mut Ray) -> bool {

        *attenuation = Vec3::new(1., 1., 1.);

        let outward_normal: Vec3;
        let ni_over_nt: f32;
        let cosine: f32;
        let mut refracted = Vec3::default();

        if Vec3::dot(&r_in.dir(), &record.normal) > 0. {
            outward_normal = -record.normal;
            ni_over_nt     = self.ref_idx;
            cosine = self.ref_idx *
                Vec3::dot(&r_in.dir(), &record.normal) / r_in.dir().length();
        }
        else {
            outward_normal = record.normal;
            ni_over_nt     = 1. / self.ref_idx;
            cosine = -Vec3::dot(&r_in.dir(), &record.normal) / r_in.dir().length();
        }

        let reflect_prob: f32;

        if let Some(refrac) = refract(r_in.dir(), outward_normal, ni_over_nt) {
            refracted = refrac;
            reflect_prob = schlick(cosine, self.ref_idx);
        }
        else {
            reflect_prob = 1.;
        }

        if rand_unit() < reflect_prob {
            let reflected = reflect(r_in.dir(), record.normal);
            *scattered = Ray::new(record.p, reflected);
        }
        else {
            *scattered = Ray::new(record.p, refracted);
        }

        true
    }

    fn get_idx(&self) -> u32 {
        self.idx
    }
}

// schlick approximation for varying angle
fn schlick(cosine: f32, ref_idx: f32) -> f32 {
    let mut r0 = (1. - ref_idx) / (1. + ref_idx);
    r0 = r0 * r0;
    r0 + (1. - r0) * (1. - cosine).powf(5.)
}

// total internal reflection can happen when a ray moves from a slower
// media to a faster media (e.g., from water to air).  We return an Option
// to indicate that refraction may not happen if the incident ray is
// beyond the critical angle.
fn refract(v: Vec3, n: Vec3, ni_over_nt: f32) -> Option<Vec3>
{
    let uv = Vec3::unit_vector(v);
    let dt = Vec3::dot(&uv, &n);
    let discriminant = 1. - ni_over_nt * ni_over_nt * (1. - dt * dt);
    if discriminant > 0. {
        let refracted = ni_over_nt * (uv - n * dt) - n * discriminant.sqrt();
        Some(refracted)
    }
    else {
        None
    }
}

}