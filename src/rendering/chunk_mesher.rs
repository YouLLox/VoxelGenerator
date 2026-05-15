use bevy::prelude::*;

use crate::world::{BlockType, Chunk, IVec3, OctreeError, CHUNK_SIZE};

use super::mesh_utils;
//CHANGER EN GREEDY MESHER
pub fn mesh_from_chunk(chunk: &Chunk) -> Result<Mesh, OctreeError> {
    let mut positions = Vec::new();
    let mut normals = Vec::new();
    let mut uvs = Vec::new();
    let mut indices = Vec::new();

    let chunk_size = CHUNK_SIZE as i32;

    let is_solid = |x: i32, y: i32, z: i32| -> bool {
        if x < 0 || x >= chunk_size || y < 0 || y >= chunk_size || z < 0 || z >= chunk_size {
            return false; // air en dehors du chunk
        }

        let coords = IVec3 {x, y, z};
        match chunk.get_local(coords) {
            Ok(block) => block.is_solid(),
            Err(_)    => false,
        }
    };

    for x in 0..chunk_size {
        for y in 0..chunk_size {
            for z in 0..chunk_size {
                let pos = IVec3 { x, y, z };
                let block = chunk.get_local(pos)?;

                if block.is_solid() {
                    mesh_utils::add_voxel(
                        &mut positions,
                        &mut normals,
                        &mut uvs,
                        &mut indices,
                        x,
                        y,
                        z,
                        block,
                        &is_solid,
                    );
                }
            }
        }
    }

    Ok(mesh_utils::build_chunk_mesh(positions, normals, uvs, indices,))
}