pub mod math {

use std::ops;

#[derive(Copy, Clone, Debug)]
pub struct Vec3 {
    e: [f32; 3]
}

impl Vec3 {
    pub fn new(x: f32, y: f32, z: f32) -> Vec3 {
        let vals = [x, y, z];
        Vec3 { e: vals }
    }

    // access individual elements
    // This struct is shared by colors and vectors
    fn x(&self) -> f32 { self.e[0] }
    fn y(&self) -> f32 { self.e[1] }
    fn z(&self) -> f32 { self.e[2] }
    pub fn r(&self) -> f32 { self.e[0] }
    pub fn g(&self) -> f32 { self.e[1] }
    pub fn b(&self) -> f32 { self.e[2] }

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

}