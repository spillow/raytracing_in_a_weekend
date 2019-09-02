pub mod module {

use crate::vec3::module::*;

// img[row][col]
pub type PPMColor = (u8, u8, u8);
pub type Color = Vec3;
pub type Image = Vec<Vec<PPMColor>>;
pub type Point = Vec3;

}