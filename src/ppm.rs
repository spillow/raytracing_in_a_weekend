pub mod module {

use std::io::Write;
use std::fs::File;

use crate::types::module::*;

pub fn write_ppm(img: &Image, path: &str) -> std::io::Result<()> {
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

}
