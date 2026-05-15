use bevy::prelude::*;

use crate::world::{BlockType, Chunk, IVec3, OctreeError, CHUNK_SIZE};

#[derive(Resource)]
pub struct SingleChunkWorld {
    pub chunk: Chunk,
}

impl SingleChunkWorld {
    pub fn chunk_origin(&self) -> IVec3 {
        IVec3::new(
            self.chunk.chunk_pos.x * CHUNK_SIZE as i32,
            self.chunk.chunk_pos.y * CHUNK_SIZE as i32,
            self.chunk.chunk_pos.z * CHUNK_SIZE as i32,
        )
    }

    pub fn world_to_local(&self, world_pos: IVec3) -> Option<IVec3> {
        let origin = self.chunk_origin();

        let local = IVec3::new(
            world_pos.x - origin.x,
            world_pos.y - origin.y,
            world_pos.z - origin.z,
        );

        let size = CHUNK_SIZE as i32;

        if local.x < 0 || local.y < 0 || local.z < 0 {
            return None;
        }

        if local.x >= size || local.y >= size || local.z >= size {
            return None;
        }

        Some(local)
    }

    pub fn get_block_world(&self, world_pos: IVec3) -> Result<BlockType, OctreeError> {
        match self.world_to_local(world_pos) {
            Some(local) => self.chunk.get_local(local),
            None => Ok(BlockType::Air),
        }
    }

    pub fn is_solid_world(&self, world_pos: IVec3) -> bool {
        self.get_block_world(world_pos)
            .map(|block| block.is_solid())
            .unwrap_or(false)
    }
}