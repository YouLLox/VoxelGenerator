use bevy::prelude::*;

use super::chunk_mesher;
use crate::player::{Player, PlayerCamera, PlayerController, PlayerLook};
use crate::world::{BlockType, Chunk, IVec3, SingleChunkWorld, CHUNK_SIZE};

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
    asset_server: Res<AssetServer>,
) {
    let chunk_pos = IVec3 { x: 0, y: 0, z: 0 };
    let mut chunk = Chunk::new(chunk_pos, BlockType::Air)
        .expect("failed to create demo chunk");

    for x in 0..CHUNK_SIZE as i32 {
        for z in 0..CHUNK_SIZE as i32 {
            for y in 0..2 {
                chunk
                    .set_local(IVec3 { x, y, z }, BlockType::Stone)
                    .expect("failed to set stone block");
            }

            chunk
                .set_local(IVec3 { x, y: 2, z }, BlockType::Dirt)
                .expect("failed to set dirt block");

            chunk
                .set_local(IVec3 { x, y: 3, z }, BlockType::Grass)
                .expect("failed to set grass block");
        }
    }

    for y in 4..10 {
        chunk
            .set_local(IVec3 { x: 8, y, z: 8 }, BlockType::Dirt)
            .expect("failed to set pillar block");
    }

    chunk
        .remove_local(IVec3 { x: 4, y: 3, z: 4 })
        .expect("failed to remove test block");

    let chunk_mesh = chunk_mesher::mesh_from_chunk(&chunk)
        .expect("failed to generate mesh from chunk");

    let world_x = (chunk.chunk_pos.x * CHUNK_SIZE as i32) as f32;
    let world_y = (chunk.chunk_pos.y * CHUNK_SIZE as i32) as f32;
    let world_z = (chunk.chunk_pos.z * CHUNK_SIZE as i32) as f32;

    let atlas_texture: Handle<Image> = asset_server.load("textures/blocks_atlas_32.png");

    commands.spawn((
        Mesh3d(meshes.add(chunk_mesh)),
        MeshMaterial3d(materials.add(StandardMaterial {
            base_color_texture: Some(atlas_texture),
            perceptual_roughness: 1.0,
            metallic: 0.0,
            ..default()
        })),
        Transform::from_xyz(world_x, world_y, world_z),
    ));

    commands.insert_resource(SingleChunkWorld { chunk });

    commands.insert_resource(AmbientLight {
        color: Color::WHITE,
        brightness: 250.0,
        ..default()
    });

    commands.spawn((
        DirectionalLight {
            shadows_enabled: true,
            illuminance: 20_000.0,
            ..default()
        },
        Transform::from_rotation(
            Quat::from_euler(EulerRot::XYZ, -1.0, -0.9, 0.0)
        ),
    ));

    commands
        .spawn((
            Player,
            PlayerController::default(),
            PlayerLook::default(),
            Transform::from_xyz(8.0, 6.0, 12.0),
        ))
        .with_children(|parent| {
            parent.spawn((
                Camera3d::default(),
                Transform::from_xyz(0.0, 1.62, 0.0),
                PlayerCamera,
            ));
        });
}