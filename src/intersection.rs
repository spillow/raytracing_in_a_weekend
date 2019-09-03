pub mod module {

use crate::objects::module::*;
use crate::ray::module::*;
use crate::vec3::module::*;

// use quadratic formula to solve for 't' value.  If the discriminant
// is positive then we have a hit.
pub fn hit_sphere(s: &Sphere, r: &Ray) -> Option<f32> {
    let oc = r.origin() - s.center();
    let rdir = r.dir();
    let a:f32 = Vec3::dot(&rdir, &rdir);
    let b:f32 = 2. * Vec3::dot(&oc, &rdir);
    let c:f32 = Vec3::dot(&oc, &oc) - s.radius()*s.radius();
    let discriminant:f32 = b*b - 4.*a*c;
    if discriminant < 0. {
        None
    }
    else {
        // Note: we take the smaller of the two roots as that corresponds
        // to the entry hit rather than the exit
        Some((-b - discriminant.sqrt()) / (2. * a))
    }
}

}