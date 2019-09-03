use rayutils::ramp::module as ramp;
use rayutils::ppm::module as ppm;

fn chap1_output(path: &str) -> std::io::Result<()> {
    let img = ramp::color_ramp_test();
    ppm::write_ppm(&img, path)?;
    Ok(())
}

fn chap3_output(path: &str) -> std::io::Result<()> {
    let img = ramp::color_ray_test();
    ppm::write_ppm(&img, path)?;
    Ok(())
}

fn chap7_output(path: &str) -> std::io::Result<()> {
    let img = ramp::sphere_hit_ray_test();
    ppm::write_ppm(&img, path)?;
    Ok(())
}

fn main() -> std::io::Result<()> {
    chap1_output("chap1.ppm")?;
    chap3_output("chap3.ppm")?;
    chap7_output("chap7.ppm")?;

    println!("Finished tracing!");
    Ok(())
}