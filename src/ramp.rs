pub mod module {

use crate::types::module::*;
use crate::vec3::module::*;
use crate::ray::module::*;

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

}