pub mod module {

use crate::types::module::*;
use crate::vec3::module::*;
use crate::ray::module::*;
use crate::hittable::module::*;
use crate::camera::module::*;
use crate::material::module::*;

use std::f32;

use rand::Rng;

fn rand_unit() -> f32 {
    rand::thread_rng().gen_range(0.0f32, 1.0f32)
}

fn get_color(
    r: &Ray, world: &dyn Hittable, materials: &Vec<&dyn Material>, depth: u32) -> Color {
    let mut record = HitRecord::default();
    // use a small t_min value here to avoid "shadow acne"
    if world.hit(r, 0.001, f32::MAX, &mut record) {
        let mut scattered   = Ray::default();
        let mut attenuation = Vec3::default();
        if depth < 50 &&
           materials[record.mat.unwrap() as usize].scatter(
               r, &record, &mut attenuation, &mut scattered) {
            return attenuation * get_color(&scattered, world, materials, depth + 1);
        }
        let black = Color::new(0.,0.,0.);
        return black;
    }

    // make it so -1 < y < 1
    let unit_direction = Vec3::unit_vector(r.dir());
    // shift and scale so 0 < t < 1
    // so y = 1  => t = 1
    //    y = -1 => t = 0
    let t = 0.5f32 * (unit_direction.y() + 1.);
    let blue  = Color::new(0.5, 0.7, 1.0); // a light blue
    let white = Color::new(1.,1.,1.);
    (1.-t)*white + t*blue
}

// chap8
pub fn raytrace() -> Image {
    let nx = 200;
    let ny = 100;
    let ns = 100; // num samples / pixel

    let mut rows = Vec::new();

    let red_diffuse    = Lambertian::new(Color::new(0.1, 0.2, 0.5), 0);
    let yellow_diffuse = Lambertian::new(Color::new(0.8, 0.8, 0.0), 1);
    let metal1         = Metal::new(Color::new(0.8, 0.6, 0.2), 0.2, 2);
    let dielectric     = Dielectric::new(1.5, 3);

    let materials:Vec<&dyn Material> =
        vec![&red_diffuse, &yellow_diffuse, &metal1, &dielectric];

    let sphere1 = Sphere::new(Point::new(0.,0.,-1.),     0.5,  &red_diffuse);
    let sphere2 = Sphere::new(Point::new(0.,-100.5,-1.), 100., &yellow_diffuse);
    let sphere3 = Sphere::new(Point::new(1.,0.,-1.),     0.5,  &metal1);
    let sphere4 = Sphere::new(Point::new(-1.,0.,-1.),    0.5,  &dielectric);
    let sphere5 = Sphere::new(Point::new(-1.,0.,-1.),    -0.45,  &dielectric);

    let spheres:Vec<&dyn Hittable> = vec![&sphere1, &sphere2, &sphere3, &sphere4, &sphere5];

    let world = HittableList::new(spheres);
    let cam = Camera::new();

    for j in (0..ny).rev() {
        let mut cols = Vec::new();
        for i in 0..nx {
            let mut color = Color::init();
            for _ in 0..ns {
                let u = ((i as f32) + rand_unit()) / nx as f32;
                let v = ((j as f32) + rand_unit()) / ny as f32;
                let r = cam.get_ray(u, v);
                color += get_color(&r, &world, &materials, 0);
            }

            color /= ns as f32;
            // gamma 2 correction
            color = Color::new(color.r().sqrt(), color.g().sqrt(), color.b().sqrt());

            color *= 255.99f32;
            let ir = color.r() as u8;
            let ig = color.g() as u8;
            let ib = color.b() as u8;
            cols.push((ir,ig,ib));
        }
        rows.push(cols);
    }

    rows
}

}