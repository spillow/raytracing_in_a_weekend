pub mod module {

use crate::objects::module::*;
use crate::ray::module::*;
use crate::vec3::module::*;

// use quadratic formula to solve for 't' value.  If the discriminant
// is positive then we have a hit.
// TODO: why don't we return true on discriminant = 0 as well?
pub fn hit_sphere(s: &Sphere, r: &Ray) -> bool {
    let oc = r.origin() - s.center();
    let rdir = r.dir();
    let a:f32 = Vec3::dot(&rdir, &rdir);
    let b:f32 = 2. * Vec3::dot(&oc, &rdir);
    let c:f32 = Vec3::dot(&oc, &oc) - s.radius()*s.radius();
    let discriminant:f32 = b*b - 4.*a*c;
    discriminant > 0.
}

}