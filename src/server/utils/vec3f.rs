use std::ops::{Add, Div, Mul, Sub};

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct Vec3f {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl Vec3f {
    pub fn new_empty() -> Vec3f {
        Vec3f {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        }
    }
    
    pub fn from_x(x: f64) -> Vec3f {
        Vec3f {
            x,
            y: 0.0,
            z: 0.0,
        }
    }
    
    pub fn from_y(y: f64) -> Vec3f {
        Vec3f {
            x: 0.0,
            y,
            z: 0.0,
        }
    }
    
    pub fn from_z(z: f64) -> Vec3f {
        Vec3f {
            x: 0.0,
            y: 0.0,
            z,
        }
    }

    pub fn distance_to(&self, other: &Vec3f) -> f64 {
        self.distance_squared(other).sqrt()
    }

    pub fn distance_squared(&self, other: &Vec3f) -> f64 {
        let x = self.x - other.x;
        let y = self.y - other.y;
        let z = self.z - other.z;
        z.mul_add(z, x.mul_add(x, y * y))
    }
}

impl Add for Vec3f {
    type Output = Vec3f;
    
    fn add(self, rhs: Self) -> Self {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}

impl Sub for Vec3f {
    type Output = Vec3f;

    fn sub(self, rhs: Self) -> Self {
        Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
        }
    }
}

impl Div for Vec3f {
    type Output = Vec3f;
    
    fn div(self, rhs: Self) -> Self {
        Self {
            x: self.x / rhs.x,
            y: self.y / rhs.y,
            z: self.z / rhs.z,
        }
    }
}

impl Mul for Vec3f {
    type Output = Vec3f;
    
    fn mul(self, rhs: Self) -> Self {
        Self {
            x: self.x * rhs.x,
            y: self.y * rhs.y,
            z: self.z * rhs.z,
        }
    }
}