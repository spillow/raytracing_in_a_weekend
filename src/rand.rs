pub mod module {

use crate::vec3::module::*;
use crate::types::module::*;
use rand::Rng;

pub fn random_in_unit_disk() -> Point {
    let mut p: Point;

    loop {
        p = 2.0 * Point::new(rand_unit(), rand_unit(), 0.) - Vec3::new(1.,1.,0.);
        if  Vec3::dot(&p, &p) < 1.0 {
            break;
        }
    }
    p
}

pub fn random_in_unit_sphere() -> Vec3 {
    let mut p:Point;
    loop {
        p = 2.0 * Point::new(rand_unit(), rand_unit(), rand_unit()) - Vec3::new(1.,1.,1.);
        if p.squared_length() < 1.0 {
            break;
        }
    }
    p
}

pub fn rand_unit() -> f32 {
    rand::thread_rng().gen_range(0.0f32, 1.0f32)
}

}