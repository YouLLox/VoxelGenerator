use bevy::prelude::*;
use bevy::mesh::{Indices, PrimitiveTopology};
use bevy::asset::RenderAssetUsages;

use crate::world::{BlockFace, BlockType};

pub fn create_quad_mesh() -> Mesh
{
    let mut mesh = Mesh::new(
        PrimitiveTopology::TriangleList,
        RenderAssetUsages::default(),
    );

    let positions = vec![
        [-1.0, 1.0, 0.0],
        [1.0, 1.0, 0.0],
        [-1.0, 0.0, 0.0],
        [1.0, 0.0, 0.0],
    ];

    let normals = vec![
        [0.0, 0.0, 1.0],
        [0.0, 0.0, 1.0],
        [0.0, 0.0, 1.0],
        [0.0, 0.0, 1.0],
    ];

    let uvs = vec![
        [0.0, 1.0],
        [1.0, 1.0],
        [0.0, 0.0],
        [1.0, 0.0],
    ];

    let indices = Indices::U32(vec![
        0, 2, 3,
        0, 3, 1,
    ]);

    mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, positions);
    mesh.insert_attribute(Mesh::ATTRIBUTE_NORMAL, normals);
    mesh.insert_attribute(Mesh::ATTRIBUTE_UV_0, uvs);
    mesh.insert_indices(indices);

    mesh
}

pub fn create_triangle_mesh() -> Mesh
{
    let mut mesh = Mesh::new(
        PrimitiveTopology::TriangleList,
        RenderAssetUsages::default(),
    );

    let positions = vec![
        [0.0, 1.0, 0.0],
        [-1.0, 0.0, 0.0],
        [1.0, 0.0, 0.0],
    ];

    let normals = vec![
        [0.0, 0.0, 1.0],
        [0.0, 0.0, 1.0],
        [0.0, 0.0, 1.0],
    ];

    let uvs = vec![
        [0.5, 1.0],
        [0.0, 0.0],
        [1.0, 0.0],
    ];

    let indices = Indices::U32(vec![
        0, 1, 2,
    ]);

    mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, positions);
    mesh.insert_attribute(Mesh::ATTRIBUTE_NORMAL, normals);
    mesh.insert_attribute(Mesh::ATTRIBUTE_UV_0, uvs);
    mesh.insert_indices(indices);

    mesh
}

// --------------------------------------------------------------------------------
// CHUNK BATCHING
// --------------------------------------------------------------------------------

pub fn atlas_uv_rect(col: u32, row: u32, atlas_cols: u32, atlas_rows: u32) -> [[f32; 2]; 4]
{
    let tile_w = 1.0 / atlas_cols as f32;
    let tile_h = 1.0 / atlas_rows as f32;

    let u_min = col as f32 * tile_w;
    let u_max = (col + 1) as f32 * tile_w;

    let v_min = row as f32 * tile_h;
    let v_max = (row + 1) as f32 * tile_h;

    [
        [u_min, v_min],
        [u_max, v_min],
        [u_min, v_max],
        [u_max, v_max],
    ]
}

pub fn add_quad_uvs(col: u32, row: u32) -> [[f32; 2]; 4] {
    atlas_uv_rect(col, row, 3, 2)
}

/// Ajoute les 4 vertex et 6 indices d'un quad aux listes
pub fn add_quad(
    positions: &mut Vec<[f32; 3]>,
    normals: &mut Vec<[f32; 3]>,
    uvs: &mut Vec<[f32; 2]>,
    indices: &mut Vec<u32>,
    offset: Vec3,
    normal: [f32; 3],
    tl: [f32; 3], tr: [f32; 3],
    bl: [f32; 3], br: [f32; 3],
    face_uvs: [[f32; 2]; 4],
)
{
    let start_index = positions.len() as u32;

    positions.push([offset.x + tl[0], offset.y + tl[1], offset.z + tl[2]]);
    positions.push([offset.x + tr[0], offset.y + tr[1], offset.z + tr[2]]);
    positions.push([offset.x + bl[0], offset.y + bl[1], offset.z + bl[2]]);
    positions.push([offset.x + br[0], offset.y + br[1], offset.z + br[2]]);

    normals.extend_from_slice(&[normal; 4]);
    uvs.extend_from_slice(&face_uvs);

    indices.extend_from_slice(&[
        start_index + 0, start_index + 2, start_index + 3,
        start_index + 0, start_index + 3, start_index + 1,
    ]);
}

/// Ajoute les 6 faces d'un cube, UNIQUEMENT si le voxel voisin est VIDE
pub fn add_voxel(
    positions: &mut Vec<[f32; 3]>,
    normals: &mut Vec<[f32; 3]>,
    uvs: &mut Vec<[f32; 2]>,
    indices: &mut Vec<u32>,
    x: i32, y: i32, z: i32,
    block: BlockType,
    is_solid: &impl Fn(i32, i32, i32) -> bool,
)
{
    let offset = Vec3::new(x as f32, y as f32, z as f32);

    let top_tile = block.atlas_tile(BlockFace::Top).unwrap();
    let bottom_tile = block.atlas_tile(BlockFace::Bottom).unwrap();
    let side_tile = block.atlas_tile(BlockFace::Side).unwrap();

    let top_uvs = add_quad_uvs(top_tile.0, top_tile.1);
    let bottom_uvs = add_quad_uvs(bottom_tile.0, bottom_tile.1);
    let side_uvs = add_quad_uvs(side_tile.0, side_tile.1);

    // Top (+Y)
    if !is_solid(x, y + 1, z) {
        add_quad(
            positions, normals, uvs, indices, offset, [0.0, 1.0, 0.0],
            [-0.5, 0.5, -0.5], [0.5, 0.5, -0.5], [-0.5, 0.5, 0.5], [0.5, 0.5, 0.5],
            top_uvs,
        );
    }

    // Bottom (-Y)
    if !is_solid(x, y - 1, z) {
        add_quad(
            positions, normals, uvs, indices, offset, [0.0, -1.0, 0.0],
            [-0.5, -0.5, 0.5], [0.5, -0.5, 0.5], [-0.5, -0.5, -0.5], [0.5, -0.5, -0.5],
            bottom_uvs,
        );
    }

    // Right (+X)
    if !is_solid(x + 1, y, z) {
        add_quad(
            positions, normals, uvs, indices, offset, [1.0, 0.0, 0.0],
            [0.5, 0.5, 0.5], [0.5, 0.5, -0.5], [0.5, -0.5, 0.5], [0.5, -0.5, -0.5],
            side_uvs,
        );
    }

    // Left (-X)
    if !is_solid(x - 1, y, z) {
        add_quad(
            positions, normals, uvs, indices, offset, [-1.0, 0.0, 0.0],
            [-0.5, 0.5, -0.5], [-0.5, 0.5, 0.5], [-0.5, -0.5, -0.5], [-0.5, -0.5, 0.5],
            side_uvs,
        );
    }

    // Front (+Z)
    if !is_solid(x, y, z + 1) {
        add_quad(
            positions, normals, uvs, indices, offset, [0.0, 0.0, 1.0],
            [-0.5, 0.5, 0.5], [0.5, 0.5, 0.5], [-0.5, -0.5, 0.5], [0.5, -0.5, 0.5],
            side_uvs,
        );
    }

    // Back (-Z)
    if !is_solid(x, y, z - 1) {
        add_quad(
            positions, normals, uvs, indices, offset, [0.0, 0.0, -1.0],
            [0.5, 0.5, -0.5], [-0.5, 0.5, -0.5], [0.5, -0.5, -0.5], [-0.5, -0.5, -0.5],
            side_uvs,
        );
    }
}

pub fn build_chunk_mesh(
    positions: Vec<[f32; 3]>,
    normals: Vec<[f32; 3]>,
    uvs: Vec<[f32; 2]>,
    colors: Vec<[f32; 4]>,
    indices: Vec<u32>,
) -> Mesh
{
    let mut mesh = Mesh::new(
        PrimitiveTopology::TriangleList,
        RenderAssetUsages::default(),
    );

    mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, positions);
    mesh.insert_attribute(Mesh::ATTRIBUTE_NORMAL, normals);
    mesh.insert_attribute(Mesh::ATTRIBUTE_UV_0, uvs);
    mesh.insert_attribute(Mesh::ATTRIBUTE_COLOR, colors);
    mesh.insert_indices(Indices::U32(indices));

    mesh
}

pub fn simulate_chunk_mesh() -> Mesh
{
    let mut positions = Vec::new();
    let mut normals = Vec::new();
    let mut uvs = Vec::new();
    let colors = Vec::new(); // Placeholder
    let mut indices = Vec::new();

    let chunk_size = 16;

    let is_solid = |x: i32, y: i32, z: i32| -> bool
    {
        if y < 0 { return true; }
        if x < 0 || x >= chunk_size || y >= chunk_size || z < 0 || z >= chunk_size {
            return false;
        }
        y < 4 || (x == 8 && z == 8 && y < 10)
    };

    for x in 0..chunk_size {
        for y in 0..chunk_size {
            for z in 0..chunk_size {
                if is_solid(x, y, z) {
                    add_voxel(
                        &mut positions,
                        &mut normals,
                        &mut uvs,
                        &mut indices,
                        x, y, z,
                        BlockType::Stone,
                        &is_solid,
                    );
                }
            }
        }
    }

    build_chunk_mesh(positions, normals, uvs, colors, indices)
}