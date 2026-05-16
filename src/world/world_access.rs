use bevy::prelude::*;
use std::collections::HashMap;

use crate::world::{BlockType, Chunk, IVec3, OctreeError, CHUNK_SIZE};

#[derive(Resource,Default)]
pub struct ChunkManager {
    pub loaded_chunks: HashMap<IVec3, Chunk>,

    pub chunk_entities: HashMap<IVec3, Entity>,
}

impl ChunkManager {
    /*
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
    */ 
    pub fn world_to_chunk_and_local(world_pos: IVec3) -> (IVec3, IVec3) {
        let size = CHUNK_SIZE as i32;

        let chunk_x = world_pos.x.div_euclid(size);
        let chunk_y = world_pos.y.div_euclid(size);
        let chunk_z = world_pos.z.div_euclid(size);

        let local_x = world_pos.x.rem_euclid(size);
        let local_y = world_pos.y.rem_euclid(size);
        let local_z = world_pos.z.rem_euclid(size);

        let chunk_pos = IVec3::new(chunk_x, chunk_y, chunk_z);
        let local_pos = IVec3::new(local_x, local_y, local_z);

        (chunk_pos, local_pos)
    }

    pub fn get_block_world(&self, world_pos: IVec3) -> Result<BlockType, OctreeError> {
        let (chunk_pos, local_pos)=Self::world_to_chunk_and_local(world_pos);

        match self.loaded_chunks.get(&chunk_pos) {
            Some(chunk) => chunk.get_local(local_pos),
            None => Ok(BlockType::Air),
        }
    }

    pub fn set_block_world(&mut self, world_pos: IVec3, 
        block: BlockType) -> Result<(), OctreeError> {
        
        let (chunk_pos, local_pos) = Self::world_to_chunk_and_local(world_pos);

        if let Some(chunk) = self.loaded_chunks.get_mut(&chunk_pos) {
            chunk.set_local(local_pos, block)
        } else {
            Ok(())
        }
    }

    pub fn is_solid_world(&self, world_pos: IVec3) -> bool {
        self.get_block_world(world_pos)
            .map(|block| block.is_solid())
            .unwrap_or(false)
    }
}
