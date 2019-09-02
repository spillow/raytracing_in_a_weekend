use rayutils::vec3::math::*;
use rayutils::ramp::module as ramp;
use rayutils::ppm::module as ppm;

fn main() -> std::io::Result<()> {
    println!("Hello, world!");
    let img = ramp::color_ramp_test();
    ppm::write_ppm(&img, "my_output.ppm")?;

    let mut v = Vec3::new(1f32,2f32,3f32);

    v /= 5.0f32;
    println!("v = {:?}", v);

    let val = v[2];
    println!("val is = {}", val);

    Ok(())
}