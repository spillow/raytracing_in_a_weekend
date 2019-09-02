use std::io::Write;
use std::fs::File;

fn mk_color(x: u8) -> Color {
    (x,x,x)
}

fn main() -> std::io::Result<()> {
    println!("Hello, world!");
    let row1 = vec![mk_color(1u8),mk_color(2u8),mk_color(3u8)];
    let row2 = vec![mk_color(3u8),mk_color(4u8),mk_color(5u8)];
    let img = vec![row1, row2];
    write_ppm(&img, "my_output.txt")?;
    Ok(())
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