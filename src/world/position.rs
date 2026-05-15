use std::ops::{Add, Sub, Mul, Neg};

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct IVec3 {
    pub x: i32,
    pub y: i32,
    pub z: i32,
}

impl IVec3 {
    pub const ZERO: Self = Self { x: 0, y: 0, z: 0 };

    pub fn new(x: i32, y: i32, z: i32) -> Self {
        Self { x, y, z }
    }
}

impl Add for IVec3 {
    type Output = Self;
    fn add(self, rhs: Self) -> Self {
        Self::new(self.x + rhs.x, self.y + rhs.y, self.z + rhs.z)
    }
}

impl Sub for IVec3 {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self {
        Self::new(self.x - rhs.x, self.y - rhs.y, self.z - rhs.z)
    }
}

impl Mul<i32> for IVec3 {
    type Output = Self;
    fn mul(self, rhs: i32) -> Self {
        Self::new(self.x * rhs, self.y * rhs, self.z * rhs)
    }
}

impl Neg for IVec3 {
    type Output = Self;
    fn neg(self) -> Self {
        Self::new(-self.x, -self.y, -self.z)
    }
}