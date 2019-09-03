pub mod module {

use crate::types::module::*;
use crate::vec3::module::*;
use crate::ray::module::*;
use crate::hittable::module::*;
use crate::camera::module::*;
use crate::material::module::*;

use std::f32;

use rand::Rng;

// chap1
pub fn color_ramp_test() -> Image {
    let nx = 200;
    let ny = 100;

    let mut rows = Vec::new();

    for j in (0..ny).rev() {
        let mut cols = Vec::new();
        for i in 0..nx {
            let r = i as f32 / nx as f32;
            let g = j as f32 / ny as f32;
            let b = 0.2f32;
            let mut color = Vec3::new(r,g,b);
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

fn color(r: &Ray) -> Color {
    // make it so -1 < y < 1
    let unit_direction = Vec3::unit_vector(r.dir());
    // shift and scale so 0 < t < 1
    // so y = 1  => t = 1
    //    y = -1 => t = 0
    let t = 0.5f32 * (unit_direction.y() + 1.);
    let blue  = Color::new(0.5, 0.7, 1.0);
    let white = Color::new(1.,1.,1.);
    (1.-t)*white + t*blue
}

// chap3
pub fn color_ray_test() -> Image {
    let nx = 200;
    let ny = 100;

    let mut rows = Vec::new();

    let lower_left_corner = Point::new(-2., -1., -1.);
    let horizontal = Vec3::new(4., 0., 0.);
    let vertical   = Vec3::new(0., 2., 0.);
    let origin     = Point::new(0., 0., 0.);

    for j in (0..ny).rev() {
        let mut cols = Vec::new();
        for i in 0..nx {
            let u = i as f32 / nx as f32;
            let v = j as f32 / ny as f32;

            let r = Ray::new(origin, lower_left_corner + u*horizontal + v*vertical);

            let mut color = color(&r);
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

fn rand_unit() -> f32 {
    rand::thread_rng().gen_range(0.0f32, 1.0f32)
}

fn color_world(r: &Ray, world: &dyn Hittable, depth: u32) -> Color {
    let mut record = HitRecord::default();
    // use a small t_min value here to avoid "shadow acne"
    if world.hit(r, 0.001, f32::MAX, &mut record) {
        let mut scattered   = Ray::default();
        let mut attenuation = Vec3::default();
        if depth < 50 &&
           record.mat.unwrap().scatter(r, &record, &mut attenuation, &mut scattered) {
            return attenuation * color_world(&scattered, world, depth + 1);
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
pub fn sphere_hit_ray_test() -> Image {
    let nx = 200;
    let ny = 100;
    let ns = 100; // num samples / pixel

    let mut rows = Vec::new();

    let red_diffuse    = Lambertian::new(Color::new(0.8, 0.3, 0.3));
    let yellow_diffuse = Lambertian::new(Color::new(0.8, 0.3, 0.3));

    let sphere1 = Sphere::new(Point::new(0.,0.,-1.), 0.5, &red_diffuse);
    let sphere2 = Sphere::new(Point::new(0.,-100.5,-1.), 100., &yellow_diffuse);

    let spheres:Vec<&dyn Hittable> = vec![&sphere1, &sphere2];

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
                color += color_world(&r, &world, 0);
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