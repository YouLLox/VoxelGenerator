#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BlockType {
    Air,
    Dirt,
    Stone,
}

impl BlockType {
    pub fn is_empty(self) -> bool {
        matches!(self, BlockType::Air)
    }

    pub fn is_solid(self) -> bool {
        !matches!(self, BlockType::Air)
    }
}