use bevy::prelude::*;

use crate::world::{BlockFace, BlockType, Chunk, IVec3, OctreeError, CHUNK_SIZE};
use super::mesh_utils;

pub fn mesh_from_chunk(chunk: &Chunk) -> Result<Mesh, OctreeError> {
    let mut positions = Vec::new();
    let mut normals = Vec::new();
    let mut uvs = Vec::new();
    let mut colors = Vec::new();
    let mut indices = Vec::new();

    let size = CHUNK_SIZE as i32;

    // 1. Aplatissement du chunk
    let mut voxels = vec![BlockType::Air; (size * size * size) as usize];
    for x in 0..size {
        for y in 0..size {
            for z in 0..size {
                voxels[(x + y * size + z * size * size) as usize] = chunk.get_local(IVec3 { x, y, z })?;
            }
        }
    }

    let get_voxel = |x: i32, y: i32, z: i32| -> BlockType {
        if x < 0 || x >= size || y < 0 || y >= size || z < 0 || z >= size {
            return BlockType::Air;
        }
        voxels[(x + y * size + z * size * size) as usize]
    };

    // 2. Greedy Meshing par face
    for face in [
        BlockFace::Top, BlockFace::Bottom,
        BlockFace::Side, 
    ] {
        let directions = match face {
            BlockFace::Top => vec![(IVec3::new(0, 1, 0), [0.0, 1.0, 0.0])],
            BlockFace::Bottom => vec![(IVec3::new(0, -1, 0), [0.0, -1.0, 0.0])],
            BlockFace::Side => vec![
                (IVec3::new(1, 0, 0), [1.0, 0.0, 0.0]),
                (IVec3::new(-1, 0, 0), [-1.0, 0.0, 0.0]),
                (IVec3::new(0, 0, 1), [0.0, 0.0, 1.0]),
                (IVec3::new(0, 0, -1), [0.0, 0.0, -1.0]),
            ],
        };

        for (dir, n) in directions {
            let (u_axis, v_axis, d_axis) = if (n[1] as f32).abs() > 0.5 {
                (0, 2, 1) // Y normal: u=X, v=Z
            } else if (n[0] as f32).abs() > 0.5 {
                (2, 1, 0) // X normal: u=Z, v=Y
            } else {
                (0, 1, 2) // Z normal: u=X, v=Y
            };

            for i in 0..size {
                let mut mask = vec![None; (size * size) as usize];
                for j in 0..size {
                    for k in 0..size {
                        let mut pos = [0; 3];
                        pos[d_axis] = i;
                        pos[u_axis] = j;
                        pos[v_axis] = k;

                        let block = get_voxel(pos[0], pos[1], pos[2]);
                        let neighbor = get_voxel(pos[0] + dir.x, pos[1] + dir.y, pos[2] + dir.z);

                        if block.is_solid() && !neighbor.is_solid() {
                            mask[(j + k * size) as usize] = Some(block);
                        }
                    }
                }

                let mut n_idx = 0;
                for k in 0..size {
                    for j in 0..size {
                        if let Some(block) = mask[n_idx] {
                            let mut width = 1;
                            while j + width < size && mask[n_idx + width as usize] == Some(block) {
                                width += 1;
                            }

                            let mut height = 1;
                            'outer: while k + height < size {
                                for w in 0..width {
                                    if mask[n_idx + w as usize + (height * size) as usize] != Some(block) {
                                        break 'outer;
                                    }
                                }
                                height += 1;
                            }

                            let mut quad_pos = [0; 3];
                            quad_pos[d_axis] = i;
                            quad_pos[u_axis] = j;
                            quad_pos[v_axis] = k;

                            add_greedy_face(
                                &mut positions, &mut normals, &mut uvs, &mut colors, &mut indices,
                                IVec3::new(quad_pos[0], quad_pos[1], quad_pos[2]),
                                n, width, height, u_axis, v_axis, block, &get_voxel
                            );

                            for h in 0..height {
                                for w in 0..width {
                                    mask[n_idx + w as usize + (h * size) as usize] = None;
                                }
                            }
                        }
                        n_idx += 1;
                    }
                }
            }
        }
    }

    Ok(mesh_utils::build_chunk_mesh(positions, normals, uvs, colors, indices))
}

fn add_greedy_face(
    positions: &mut Vec<[f32; 3]>,
    normals: &mut Vec<[f32; 3]>,
    uvs: &mut Vec<[f32; 2]>,
    colors: &mut Vec<[f32; 4]>,
    indices: &mut Vec<u32>,
    pos: IVec3,
    n: [f32; 3],
    w: i32,
    h: i32,
    u_axis: usize,
    v_axis: usize,
    block: BlockType,
    get_voxel: &impl Fn(i32, i32, i32) -> BlockType,
) {
    let face_type = if n[1] > 0.5 { BlockFace::Top } 
                    else if n[1] < -0.5 { BlockFace::Bottom } 
                    else { BlockFace::Side };
    
    let tile = block.atlas_tile(face_type).unwrap_or((0, 0));
    let atlas_uvs = mesh_utils::add_quad_uvs(tile.0, tile.1);
    
    let start_index = positions.len() as u32;

    let mut du = [0.0; 3]; du[u_axis] = w as f32;
    let mut dv = [0.0; 3]; dv[v_axis] = h as f32;

    let (v0, v1, v2, v3) = if n[1] > 0.5 { // TOP (+Y)
        ([0.0, 1.0, dv[2]], [du[0], 1.0, dv[2]], [du[0], 1.0, 0.0], [0.0, 1.0, 0.0])
    } else if n[1] < -0.5 { // BOTTOM (-Y)
        ([0.0, 0.0, 0.0], [du[0], 0.0, 0.0], [du[0], 0.0, dv[2]], [0.0, 0.0, dv[2]])
    } else if n[0] > 0.5 { // RIGHT (+X)
        ([1.0, 0.0, du[2]], [1.0, 0.0, 0.0], [1.0, dv[1], 0.0], [1.0, dv[1], du[2]])
    } else if n[0] < -0.5 { // LEFT (-X)
        ([0.0, 0.0, 0.0], [0.0, 0.0, du[2]], [0.0, dv[1], du[2]], [0.0, dv[1], 0.0])
    } else if n[2] > 0.5 { // FRONT (+Z)
        ([0.0, 0.0, 1.0], [du[0], 0.0, 1.0], [du[0], dv[1], 1.0], [0.0, dv[1], 1.0])
    } else { // BACK (-Z)
        ([du[0], 0.0, 0.0], [0.0, 0.0, 0.0], [0.0, dv[1], 0.0], [du[0], dv[1], 0.0])
    };

    let p = [pos.x as f32, pos.y as f32, pos.z as f32];
    positions.push([p[0] + v0[0], p[1] + v0[1], p[2] + v0[2]]);
    positions.push([p[0] + v1[0], p[1] + v1[1], p[2] + v1[2]]);
    positions.push([p[0] + v2[0], p[1] + v2[1], p[2] + v2[2]]);
    positions.push([p[0] + v3[0], p[1] + v3[1], p[2] + v3[2]]);

    normals.extend_from_slice(&[n; 4]);
    uvs.extend_from_slice(&[atlas_uvs[2], atlas_uvs[3], atlas_uvs[1], atlas_uvs[0]]);

    let normal_ivec = IVec3::new(n[0] as i32, n[1] as i32, n[2] as i32);
    for vertex_offset in [v0, v1, v2, v3] {
        let du_sign = if vertex_offset[u_axis] > 0.1 { 1 } else { -1 };
        let dv_sign = if vertex_offset[v_axis] > 0.1 { 1 } else { -1 };
        let mut d_u = IVec3::ZERO; 
        match u_axis {
            0 => d_u.x = du_sign,
            1 => d_u.y = du_sign,
            2 => d_u.z = du_sign,
            _ => {}
        }
        let mut d_v = IVec3::ZERO;
        match v_axis {
            0 => d_v.x = dv_sign,
            1 => d_v.y = dv_sign,
            2 => d_v.z = dv_sign,
            _ => {}
        }

        let ao = calculate_vertex_ao(pos, normal_ivec, d_u, d_v, get_voxel);
        colors.push([ao, ao, ao, 1.0]);
    }

    indices.extend_from_slice(&[
        start_index + 0, start_index + 1, start_index + 2,
        start_index + 0, start_index + 2, start_index + 3,
    ]);
}

fn calculate_vertex_ao(p: IVec3, n: IVec3, du: IVec3, dv: IVec3, get_voxel: &impl Fn(i32, i32, i32) -> BlockType) -> f32 {
    let side1 = get_voxel(p.x + n.x + du.x, p.y + n.y + du.y, p.z + n.z + du.z).is_solid();
    let side2 = get_voxel(p.x + n.x + dv.x, p.y + n.y + dv.y, p.z + n.z + dv.z).is_solid();
    let corner = get_voxel(p.x + n.x + du.x + dv.x, p.y + n.y + du.y + dv.y, p.z + n.z + du.z + dv.z).is_solid();

    if side1 && side2 { return 0.5; }
    let mut score = 0.0;
    if side1 { score += 1.0; }
    if side2 { score += 1.0; }
    if corner { score += 1.0; }
    1.0 - (score * 0.15)
}