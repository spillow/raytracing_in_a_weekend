use std::io::Write;
use std::fs::File;

fn main() -> std::io::Result<()> {
    println!("Hello, world!");
    let row1 = vec![1u8,2u8,3u8];
    let row2 = vec![3u8,4u8,5u8];
    let img = vec![row1, row2];
    write_ppm(&img, "my_output.txt")?;
    Ok(())
}

// img[row][col]
type Image = Vec<Vec<u8>>;

fn write_ppm(img: &Image, path: &str) -> std::io::Result<()> {
    let num_row  = img.len();
    let num_cols = img[0].len();

    let mut file = File::create(path)?;

    file.write(b"stuff")?;

    Ok(())
}