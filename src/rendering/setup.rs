use bevy::prelude::*;

use super::{camera, chunk_mesher};
use crate::world::{BlockType, Chunk, IVec3, CHUNK_SIZE};

pub struct SetupPlugin;

impl Plugin for SetupPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup);
    }
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // 1) Création d'un vrai chunk vide
    let chunk_pos = IVec3 { x: 0, y: 0, z: 0 };
    let mut chunk = Chunk::new(chunk_pos, BlockType::Air)
        .expect("failed to create demo chunk");

    // 2) Remplissage du chunk avec une petite scène de test

    // Sol plat : y = 0..3
    for x in 0..CHUNK_SIZE as i32 {
        for z in 0..CHUNK_SIZE as i32 {
            for y in 0..4 {
                chunk
                    .set_local(IVec3 { x, y, z }, BlockType::Stone)
                    .expect("failed to set floor block");
            }
        }
    }

    // Colonne au centre
    for y in 4..10 {
        chunk
            .set_local(IVec3 { x: 8, y, z: 8 }, BlockType::Dirt)
            .expect("failed to set pillar block");
    }

    // Petit trou pour vérifier visuellement que le mesh vient bien du chunk
    chunk
        .remove_local(IVec3 { x: 4, y: 3, z: 4 })
        .expect("failed to remove test block");

    // 3) Génération du vrai mesh depuis le vrai chunk
    let chunk_mesh = chunk_mesher::mesh_from_chunk(&chunk)
        .expect("failed to generate mesh from chunk");

    // 4) Position monde du chunk
    // Ton mesher travaille en local, donc le placement global se fait ici.
    let world_x = (chunk.chunk_pos.x * CHUNK_SIZE as i32) as f32;
    let world_y = (chunk.chunk_pos.y * CHUNK_SIZE as i32) as f32;
    let world_z = (chunk.chunk_pos.z * CHUNK_SIZE as i32) as f32;

    // 5) Spawn du chunk
    commands.spawn((
        Mesh3d(meshes.add(chunk_mesh)),
        MeshMaterial3d(materials.add(Color::from(Srgba::new(0.2, 0.6, 0.9, 1.0)))),
        Transform::from_xyz(world_x, world_y, world_z),
    ));

    // Lumière
    commands.spawn((
        PointLight {
            shadows_enabled: true,
            intensity: 10_000_000.0,
            range: 200.0,
            ..default()
        },
        Transform::from_xyz(20.0, 35.0, 20.0),
    ));

    // Caméra
    commands.spawn((
        Camera3d::default(),
        Transform::from_xyz(16.0, 18.0, 42.0)
            .looking_at(Vec3::new(16.0, 4.0, 16.0), Vec3::Y),
        camera::FlyCamera::default(),
    ));
}