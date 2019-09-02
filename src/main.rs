use std::io::Write;
use std::fs::File;

use vec3::math::*;

fn main() -> std::io::Result<()> {
    println!("Hello, world!");
    let img = color_ramp_test();
    write_ppm(&img, "my_output.ppm")?;

    let mut v = Vec3::new(1f32,2f32,3f32);

    v /= 5.0f32;
    println!("v = {:?}", v);

    let val = v[2];
    println!("val is = {}", val);

    Ok(())
}

fn color_ramp_test() -> Image {
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

// img[row][col]
type Color = (u8, u8, u8);
type Image = Vec<Vec<Color>>;

fn write_ppm(img: &Image, path: &str) -> std::io::Result<()> {
    let num_rows = img.len();
    let num_cols = img[0].len();

    let mut file = File::create(path)?;

    file.write(b"P3\n")?;
    file.write(format!("{} {}\n", num_cols, num_rows).as_bytes())?;
    file.write(b"255\n")?;

    for row in 0..num_rows {
        for col in 0..num_cols {
            let (r,g,b) = img[row][col];
            file.write(format!("{} {} {}\n", r, g, b).as_bytes())?;
        }
    }

    Ok(())
}