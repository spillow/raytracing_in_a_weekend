pub mod module {

use crate::types::module::*;
use crate::vec3::module::*;
use crate::ray::module::*;
use crate::hittable::module::*;
use crate::camera::module::*;
use crate::material::module::*;
use crate::rand::module::*;

use std::f32;

fn get_color(
    r: &Ray, world: &dyn Hittable, materials: &Vec<Box<dyn Material>>, depth: u32) -> Color {
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
    let nx = 1200;
    let ny = 800;
    let ns = 30; // num samples / pixel

    let mut mat_idx = 0;

    let ground_mat = Lambertian::new(Color::new(0.5,0.5,0.5), mat_idx);
    mat_idx += 1;
    let dielectric = Dielectric::new(1.5, mat_idx);
    mat_idx += 1;
    let lambertian = Lambertian::new(Color::new(0.4,0.2,0.1), mat_idx);
    mat_idx += 1;
    let metal = Metal::new(Color::new(0.7,0.6,0.5), 0.0, mat_idx);
    mat_idx += 1;

    let mut materials:Vec<Box<dyn Material>> = vec![
        Box::new(ground_mat),
        Box::new(dielectric),
        Box::new(lambertian),
        Box::new(metal)];

    let ground_sphere = Sphere::new(Point::new(0.,-1000.,0.), 1000., ground_mat.get_idx());
    let sphere1 = Sphere::new(Point::new(0.,1.,0.), 1., dielectric.get_idx());
    let sphere2 = Sphere::new(Point::new(-4.,1.,0.), 1., lambertian.get_idx());
    let sphere3 = Sphere::new(Point::new(4.,1.,0.), 1., metal.get_idx());

    let mut list:Vec<Box<dyn Hittable>> = vec![
        Box::new(ground_sphere),
        Box::new(sphere1),
        Box::new(sphere2),
        Box::new(sphere3)];

    let num_spheres = 11;

    for a in -num_spheres..num_spheres {
        for b in -num_spheres..num_spheres {
            let choose_mat = rand_unit();
            let center = Point::new(a as f32+0.9*rand_unit(),0.2,b as f32+0.9*rand_unit());

            if (center - Point::new(4.,0.2,0.)).length() > 0.9 {
                if choose_mat < 0.8 { // diffuse
                    let lambertian = Lambertian::new(
                        Color::new(
                            rand_unit()*rand_unit(),
                            rand_unit()*rand_unit(),
                            rand_unit()*rand_unit()), mat_idx);
                    materials.push(Box::new(lambertian));
                    let sphere = Sphere::new(center, 0.2, lambertian.get_idx());
                    list.push(Box::new(sphere));
                }
                else if choose_mat < 0.95 { // metal
                    let metal = Metal::new(
                        Color::new(
                            0.5*(1. + rand_unit()),
                            0.5*(1. + rand_unit()),
                            0.5*(1. + rand_unit())), 0., mat_idx);
                    materials.push(Box::new(metal));
                    let sphere = Sphere::new(center, 0.2, metal.get_idx());
                    list.push(Box::new(sphere));
                }
                else { // glass
                    let dielectric = Dielectric::new(1.5, mat_idx);
                    materials.push(Box::new(dielectric));
                    let sphere = Sphere::new(center, 0.2, dielectric.get_idx());
                    list.push(Box::new(sphere));
                }
                mat_idx += 1;
            }
        }
    }

    let world = HittableList::new(list);

    let mut rows = Vec::new();

    let lookfrom   = Point::new(13.,2.,3.);
    let lookat     = Point::new(0.,0.,0.);
    let vup        = Vec3::new(0.,1.,0.);
    let vfov       = 20.;
    let aspect     = nx as f32 / ny as f32;
    let aperature  = 0.1;
    let focus_dist = 10.;

    let cam = Camera::new(
        lookfrom,
        lookat,
        vup,
        vfov,
        aspect,
        aperature,
        focus_dist);

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