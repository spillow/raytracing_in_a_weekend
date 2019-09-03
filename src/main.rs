use rayutils::ramp::module as ramp;
use rayutils::ppm::module as ppm;

fn generate_image(path: &str) -> std::io::Result<()> {
    let img = ramp::raytrace();
    ppm::write_ppm(&img, path)?;
    Ok(())
}

fn main() -> std::io::Result<()> {
    generate_image("output.ppm")?;

    println!("Finished tracing!");
    Ok(())
}