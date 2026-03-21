use bevy::prelude::*;
use bevy::mesh::{Indices, PrimitiveTopology};
use bevy::asset::RenderAssetUsages;

pub fn create_quad_mesh() -> Mesh
{

    let mut mesh = Mesh::new(
        PrimitiveTopology::TriangleList,
        RenderAssetUsages::default(),
    );

    // positions of the vertices
    let positions = vec![
        [-1.0, 1.0, 0.0], // Top Left
        [1.0, 1.0, 0.0],  // Top Right
        [-1.0, 0.0, 0.0], // Bottom Left
        [1.0, 0.0, 0.0],  // Bottom Right
    ];
    
    // Les Normals vont vers +Z
    let normals = vec![
        [0.0, 0.0, 1.0], // Top Left
        [0.0, 0.0, 1.0], // Top Right
        [0.0, 0.0, 1.0], // Bottom Left
        [0.0, 0.0, 1.0], // Bottom Right
    ];
    
    // Les UV
    let uvs = vec![
        [0.0, 1.0],  // Top Left
        [1.0, 1.0],  // Top Right
        [0.0, 0.0],  // Bottom Left
        [1.0, 0.0],  // Bottom Right
    ];
    
    // Indices list dans le sens contraire des aiguilles d'une montre
    let indices = Indices::U32(vec![
        0, 2, 3,
        0, 3, 1,
    ]);

    //Finish the job
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

    // positions of the vertices
    let positions = vec![
        [0.0, 1.0, 0.0],  // Top vertex
        [-1.0, 0.0, 0.0], // Bottom Left vertex
        [1.0, 0.0, 0.0],  // Bottom Right vertex
    ];
    
    // Les Normals vont vers +Z
    let normals = vec![
        [0.0, 0.0, 1.0], // Top Left
        [0.0, 0.0, 1.0], // Top Right
        [0.0, 0.0, 1.0], // Bottom Right
    ];
    
    // Les UV
    let uvs = vec![
        [0.5, 1.0],   // Top
        [0.0, 0.0],  // Bottom Left
        [1.0, 0.0],   // Bottom Right
    ];
    
    // Indices list dans le sens contraire des aiguilles d'une montre
    let indices = Indices::U32(vec![
        0, 1, 2,
    ]);

    //Finish the job
    mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, positions);
    mesh.insert_attribute(Mesh::ATTRIBUTE_NORMAL, normals);
    mesh.insert_attribute(Mesh::ATTRIBUTE_UV_0, uvs);
    mesh.insert_indices(indices);

    mesh
}




// --------------------------------------------------------------------------------
// CHUNK BATCHING 
// --------------------------------------------------------------------------------

/// Append the 4 vertices and 6 indices of a quad to the lists
pub fn add_quad(
    positions: &mut Vec<[f32; 3]>,
    normals: &mut Vec<[f32; 3]>,
    uvs: &mut Vec<[f32; 2]>,
    indices: &mut Vec<u32>,
    offset: Vec3,
    normal: [f32; 3],
    tl: [f32; 3], tr: [f32; 3],
    bl: [f32; 3], br: [f32; 3],
)
{
    // The current number of vertices
    let start_index = positions.len() as u32;

    // Ajout d'un offset pour faire apparaitre le carre a la bonne position
    positions.push([offset.x + tl[0], offset.y + tl[1], offset.z + tl[2]]);
    positions.push([offset.x + tr[0], offset.y + tr[1], offset.z + tr[2]]);
    positions.push([offset.x + bl[0], offset.y + bl[1], offset.z + bl[2]]);
    positions.push([offset.x + br[0], offset.y + br[1], offset.z + br[2]]);

    normals.extend_from_slice(&[normal; 4]);
    uvs.extend_from_slice(&[[0.0, 1.0], [1.0, 1.0], [0.0, 0.0], [1.0, 0.0]]);

    // Les indices doivent etre decales par le start index
    indices.extend_from_slice(&[
        start_index + 0, start_index + 2, start_index + 3,
        start_index + 0, start_index + 3, start_index + 1,
    ]);
}

/// Ajoute les 6 faces d'un cube, UNIQUEMENT si le voxel voisin est VIDE ! (Face Culling) donc sinon on ajoute que les faces necessaires
pub fn add_voxel(
    positions: &mut Vec<[f32; 3]>,
    normals: &mut Vec<[f32; 3]>,
    uvs: &mut Vec<[f32; 2]>,
    indices: &mut Vec<u32>,
    x: i32, y: i32, z: i32,
    is_solid: &impl Fn(i32, i32, i32) -> bool,
)
{
    let offset = Vec3::new(x as f32, y as f32, z as f32);

    // Fuck Rust Fuck Bevy et Fuck tout les tutos plus a jours
    // Mais bon, on continue

    // Top (+Y)
    if !is_solid(x, y + 1, z) {
        add_quad(positions, normals, uvs, indices, offset, [0.0, 1.0, 0.0],
            [-0.5, 0.5, -0.5], [0.5, 0.5, -0.5], [-0.5, 0.5, 0.5], [0.5, 0.5, 0.5]
        );
    }
    // Bottom (-Y)
    if !is_solid(x, y - 1, z) {
        add_quad(positions, normals, uvs, indices, offset, [0.0, -1.0, 0.0],
            [-0.5, -0.5, 0.5], [0.5, -0.5, 0.5], [-0.5, -0.5, -0.5], [0.5, -0.5, -0.5]
        );
    }
    // Right (+X)
    if !is_solid(x + 1, y, z) {
        add_quad(positions, normals, uvs, indices, offset, [1.0, 0.0, 0.0],
            [0.5, 0.5, 0.5], [0.5, 0.5, -0.5], [0.5, -0.5, 0.5], [0.5, -0.5, -0.5]
        );
    }
    // Left (-X)
    if !is_solid(x - 1, y, z) {
        add_quad(positions, normals, uvs, indices, offset, [-1.0, 0.0, 0.0],
            [-0.5, 0.5, -0.5], [-0.5, 0.5, 0.5], [-0.5, -0.5, -0.5], [-0.5, -0.5, 0.5]
        );
    }
    // Front (+Z)
    if !is_solid(x, y, z + 1) {
        add_quad(positions, normals, uvs, indices, offset, [0.0, 0.0, 1.0],
            [-0.5, 0.5, 0.5], [0.5, 0.5, 0.5], [-0.5, -0.5, 0.5], [0.5, -0.5, 0.5]
        );
    }
    // Back (-Z)
    if !is_solid(x, y, z - 1) {
        add_quad(positions, normals, uvs, indices, offset, [0.0, 0.0, -1.0],
            [0.5, 0.5, -0.5], [-0.5, 0.5, -0.5], [0.5, -0.5, -0.5], [-0.5, -0.5, -0.5]
        );
    }   


}

/// Quand tous les voxels du Chunk sont remplies, on construit UN Mesh pour le chunk 
pub fn build_chunk_mesh(
    positions: Vec<[f32; 3]>,
    normals: Vec<[f32; 3]>,
    uvs: Vec<[f32; 2]>,
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
    mesh.insert_indices(Indices::U32(indices));
    mesh
}

/// Exemple de simulation de chunk pour illustrer le batching
pub fn simulate_chunk_mesh() -> Mesh
{
    let mut positions = Vec::new();
    let mut normals = Vec::new();
    let mut uvs = Vec::new();
    let mut indices = Vec::new();

    let chunk_size = 16;

    // Methode degueulasse mais c'est juste pour debug le system
    let is_solid = |x: i32, y: i32, z: i32| -> bool
    {
        if y < 0 { return true; } // sol infini
        if x < 0 || x >= chunk_size || y >= chunk_size || z < 0 || z >= chunk_size {
            return false; // air en dehors du chunk
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
                        &is_solid,
                    );
                }
            }
        }
    }

    build_chunk_mesh(positions, normals, uvs, indices)
}
