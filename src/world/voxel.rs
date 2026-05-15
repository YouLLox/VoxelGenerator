#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BlockType {
    Air,
    Grass,
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

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BlockFace {
    Top,
    Bottom,
    Side,
}

impl BlockType {
    pub fn atlas_tile(self, face: BlockFace) -> Option<(u32, u32)> {
        match self {
            BlockType::Air => None,
            BlockType::Stone => Some((0, 0)),
            BlockType::Dirt => Some((1, 0)),
            BlockType::Grass => match face {
                BlockFace::Top => Some((2, 0)),
                BlockFace::Side => Some((0, 1)),
                BlockFace::Bottom => Some((1, 1)),
            },
        }
    }
}