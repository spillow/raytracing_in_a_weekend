pub mod module {

use crate::vec3::module::*;

// img[row][col]
pub type Color = (u8, u8, u8);
pub type Image = Vec<Vec<Color>>;
pub type Point = Vec3;

}