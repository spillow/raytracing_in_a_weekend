use std::io::Write;
use std::fs::File;
use std::ops;

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

#[derive(Copy, Clone, Debug)]
struct Vec3 {
    e: [f32; 3]
}

impl Vec3 {
    fn new(x: f32, y: f32, z: f32) -> Vec3 {
        let vals = [x, y, z];
        Vec3 { e: vals }
    }

    // access individual elements
    // This struct is shared by colors and vectors
    fn x(&self) -> f32 { self.e[0] }
    fn y(&self) -> f32 { self.e[1] }
    fn z(&self) -> f32 { self.e[2] }
    fn r(&self) -> f32 { self.e[0] }
    fn g(&self) -> f32 { self.e[1] }
    fn b(&self) -> f32 { self.e[2] }

    fn length(&self) -> f32 {
        self.squared_length().sqrt()
    }
    fn squared_length(&self) -> f32 {
        self.e[0]*self.e[0] + self.e[1]*self.e[1] + self.e[2]*self.e[2]
    }
    fn make_unit_vector(&mut self) {
        let k = self.length();
        self.e[0] /= k;
        self.e[1] /= k;
        self.e[2] /= k;
    }
    fn unit_vector(v: Vec3) -> Vec3 {
        v / v.length()
    }
    fn dot(v1: &Vec3, v2: &Vec3) -> f32 {
        v1.e[0]*v2.e[0] + v1.e[1]*v2.e[1] + v1.e[2]*v2.e[2]
    }
    fn cross(v1: &Vec3, v2: &Vec3) -> Vec3 {
        Vec3 {
            e:
            [v1.e[1]*v2.e[2] - v1.e[2]*v2.e[1],
            -v1.e[0]*v2.e[2] + v1.e[2]*v2.e[0],
            v1.e[0]*v2.e[1] - v1.e[1]*v2.e[0]]
        }
    }
}

impl ops::AddAssign for Vec3 {
    fn add_assign(&mut self, rhs: Vec3) {
        self.e[0] += rhs.e[0];
        self.e[1] += rhs.e[1];
        self.e[2] += rhs.e[2];
    }
}

impl ops::Add for Vec3 {
    type Output = Vec3;
    fn add(self, rhs: Vec3) -> Vec3 {
        Vec3 { e:
            [self.e[0] + rhs.e[0],
             self.e[1] + rhs.e[1],
             self.e[2] + rhs.e[2]]
        }
    }
}

impl ops::SubAssign for Vec3 {
    fn sub_assign(&mut self, rhs: Vec3) {
        self.e[0] -= rhs.e[0];
        self.e[1] -= rhs.e[1];
        self.e[2] -= rhs.e[2];
    }
}

impl ops::Sub for Vec3 {
    type Output = Vec3;
    fn sub(self, rhs: Vec3) -> Vec3 {
        Vec3 { e:
            [self.e[0] - rhs.e[0],
             self.e[1] - rhs.e[1],
             self.e[2] - rhs.e[2]]
        }
    }
}

impl ops::MulAssign for Vec3 {
    fn mul_assign(&mut self, rhs: Vec3) {
        self.e[0] *= rhs.e[0];
        self.e[1] *= rhs.e[1];
        self.e[2] *= rhs.e[2];
    }
}

impl ops::Mul for Vec3 {
    type Output = Vec3;
    fn mul(self, rhs: Vec3) -> Vec3 {
        Vec3 { e:
            [self.e[0] * rhs.e[0],
             self.e[1] * rhs.e[1],
             self.e[2] * rhs.e[2]]
        }
    }
}

impl ops::MulAssign<f32> for Vec3 {
    fn mul_assign(&mut self, rhs: f32) {
        self.e[0] *= rhs;
        self.e[1] *= rhs;
        self.e[2] *= rhs;
    }
}

impl ops::DivAssign for Vec3 {
    fn div_assign(&mut self, rhs: Vec3) {
        self.e[0] /= rhs.e[0];
        self.e[1] /= rhs.e[1];
        self.e[2] /= rhs.e[2];
    }
}

impl ops::Div for Vec3 {
    type Output = Vec3;
    fn div(self, rhs: Vec3) -> Vec3 {
        Vec3 { e:
            [self.e[0] / rhs.e[0],
             self.e[1] / rhs.e[1],
             self.e[2] / rhs.e[2]]
        }
    }
}

impl ops::Div<f32> for Vec3 {
    type Output = Vec3;
    fn div(self, rhs: f32) -> Vec3 {
        Vec3 { e:
            [self.e[0] / rhs,
             self.e[1] / rhs,
             self.e[2] / rhs]
        }
    }
}

impl ops::DivAssign<f32> for Vec3 {
    fn div_assign(&mut self, rhs: f32) {
        self.e[0] /= rhs;
        self.e[1] /= rhs;
        self.e[2] /= rhs;
    }
}

impl ops::Neg for Vec3 {
    type Output = Vec3;
    fn neg(self) -> Vec3 {
        Vec3 { e:
            [-self.e[0],
             -self.e[1],
             -self.e[2]]
        }
    }
}

impl ops::Index<usize> for Vec3 {
    type Output = f32;
    fn index(&self, idx: usize) -> &Self::Output {
        &self.e[idx]
    }
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