use rayutils::vec3::module::*;
use rayutils::ramp::module as ramp;
use rayutils::ppm::module as ppm;

fn chap1_output(path: &str) -> std::io::Result<()> {
    let img = ramp::color_ramp_test();
    ppm::write_ppm(&img, path)?;
    Ok(())
}

fn main() -> std::io::Result<()> {
    chap1_output("chap1.ppm")?;

    Ok(())
}