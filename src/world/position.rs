#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct IVec3 {
    pub x: i32,
    pub y: i32,
    pub z: i32,
}

impl IVec3 {
    pub fn new(x:i32, y:i32, z:i32) -> Self {
        Self {x, y, z}
    }
}