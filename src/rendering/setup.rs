use bevy::prelude::*;
use std::time::{SystemTime, UNIX_EPOCH};

use super::chunk_mesher;
use crate::player::{Player, PlayerCamera, PlayerController, PlayerLook};
use crate::world::{BlockType, Chunk, IVec3, SingleChunkWorld, CHUNK_SIZE};

pub struct SetupPlugin;

impl Plugin for SetupPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<MapModifications>();
        app.add_message::<GenerateSeedEvent>();
        app.add_message::<SaveMapEvent>();
        app.add_message::<LoadMapEvent>();
        app.add_systems(Startup, setup);
        app.add_systems(Update, (trigger_random_seed, handle_generate_seed_event, interact_with_blocks, handle_save_load));
    }
}

#[derive(Component)]
pub struct WorldChunkMarker;

#[derive(Resource)]
pub struct CurrentSeed(pub u32);

fn setup(mut commands: Commands, mut meshes: ResMut<Assets<Mesh>>, mut materials: ResMut<Assets<StandardMaterial>>, asset_server: Res<AssetServer>) {
    let chunk_pos = IVec3 { x: 0, y: 0, z: 0 };
    let mut chunk = Chunk::new(chunk_pos, BlockType::Air)
        .expect("failed to create demo chunk");

    // Génération d'une seed aléatoire basée sur l'heure actuelle
    let current_seed = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_millis() as u32;
    /*
        // Seed aléatoire
        let current_seed = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_millis() as u32;
    
        // Seed fixe
        let current_seed = 12345;
    */

    println!("Génération initiale avec la seed : {}", current_seed);

    let map_seed = crate::proc_gen::bruit::MapSeed {
        seed: current_seed,
        width: CHUNK_SIZE as usize,
        height: CHUNK_SIZE as usize,
        scale: 15.0,
        octaves: 4,
        persistance: 0.5,
        lacunarity: 2.0,
        offset:(0,0),
    };

    let max_height = 16;
    let height_map = crate::proc_gen::bruit::generate_height_map(&map_seed, max_height);

    for x in 0..CHUNK_SIZE as i32 {
        for z in 0..CHUNK_SIZE as i32 {
            let h = height_map[z as usize][x as usize] as i32;

            for y in 0..=h {
                let block_type = if y == h {
                    BlockType::Grass
                } else if y >= h - 3 {
                    BlockType::Dirt
                } else {
                    BlockType::Stone
                };

                chunk
                    .set_local(IVec3 { x, y, z }, block_type)
                    .unwrap_or_else(|_| panic!("failed to set block at {},{},{}", x, y, z));
            }
        }
    }

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
        WorldChunkMarker,
    ));

    commands.insert_resource(SingleChunkWorld { chunk });
    commands.insert_resource(CurrentSeed(current_seed));

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
            Transform::from_xyz(16.0, 32.0, 16.0),
        ))
        .with_children(|parent| {
            parent.spawn((
                Camera3d::default(),
                Transform::from_xyz(0.0, 1.62, 0.0),
                PlayerCamera,
            ));
        });
}

#[derive(Message)]
pub struct GenerateSeedEvent(pub u32);

#[derive(Resource, Default)]
pub struct MapModifications(pub std::collections::HashMap<(i32, i32, i32), BlockType>);

fn generate_chunk_from_seed(seed: u32, chunk_pos: IVec3) -> Chunk {
    let mut new_chunk = Chunk::new(chunk_pos, BlockType::Air).unwrap();
    
    let map_seed = crate::proc_gen::bruit::MapSeed {
        seed,
        width: CHUNK_SIZE as usize,
        height: CHUNK_SIZE as usize,
        scale: 15.0,
        octaves: 4,
        persistance: 0.5,
        lacunarity: 2.0,
        offset:(0,0),
    };

    let max_height = 16;
    let height_map = crate::proc_gen::bruit::generate_height_map(&map_seed, max_height);

    for x in 0..CHUNK_SIZE as i32 {
        for z in 0..CHUNK_SIZE as i32 {
            let h = height_map[z as usize][x as usize] as i32;
            for y in 0..=h {
                let block_type = if y == h {
                    BlockType::Grass
                } else if y >= h - 3 {
                    BlockType::Dirt
                } else {
                    BlockType::Stone
                };
                new_chunk.set_local(IVec3 { x, y, z }, block_type).unwrap();
            }
        }
    }
    new_chunk
}
//idée pour générer le reste de la map générer les 8 chunks autour du chunk du 
//joueur et cacher le reste une fois générée juste le cacher 
//pour génerer utiliser fonc générer chunk
fn trigger_random_seed(keys: Res<ButtonInput<KeyCode>>, mut ev_writer: MessageWriter<GenerateSeedEvent>) {
    if keys.just_pressed(KeyCode::KeyR) {
        let new_seed = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_millis() as u32;
        ev_writer.write(GenerateSeedEvent(new_seed));
    }
}

fn handle_generate_seed_event(
    mut events: MessageReader<GenerateSeedEvent>,
    mut world: ResMut<SingleChunkWorld>,
    mut current_seed_res: ResMut<CurrentSeed>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut modifications: ResMut<MapModifications>,
    query: Query<&Mesh3d, With<WorldChunkMarker>>
) {
    for event in events.read() {
        let new_seed = event.0;
        current_seed_res.0 = new_seed;
        modifications.0.clear();
            
        println!("Régénération de la carte avec la seed : {}", new_seed);

        world.chunk = generate_chunk_from_seed(new_seed, world.chunk.chunk_pos);
        
        for mesh3d in &query {
            let new_mesh = crate::rendering::chunk_mesher::mesh_from_chunk(&world.chunk).unwrap();
            if let Some(mesh) = meshes.get_mut(mesh3d.0.id()) {
                *mesh = new_mesh;
            }
        }
    }
}

#[derive(Message)]
pub struct SaveMapEvent;

#[derive(Message)]
pub struct LoadMapEvent;

fn handle_save_load(mut save_events: MessageReader<SaveMapEvent>, mut load_events: MessageReader<LoadMapEvent>, mut world: ResMut<SingleChunkWorld>, mut current_seed: ResMut<CurrentSeed>, mut meshes: ResMut<Assets<Mesh>>, mut modifications: ResMut<MapModifications>, query: Query<&Mesh3d, With<WorldChunkMarker>>) {
    for _ in save_events.read() {
        let mut mods_json = Vec::new();
        for ((x, y, z), block) in &modifications.0 {
            let block_str = match block {
                BlockType::Stone => "Stone",
                BlockType::Dirt => "Dirt",
                BlockType::Grass => "Grass",
                BlockType::Air => "Air",
                _ => "Air",
            };
            mods_json.push(serde_json::json!({
                "x": x,
                "y": y,
                "z": z,
                "type": block_str
            }));
        }

        let json_data = serde_json::json!({
            "seed": current_seed.0,
            "modifications": mods_json
        });
        if std::fs::write("map.json", json_data.to_string()).is_ok() {
            println!("Map sauvegardée avec la seed {} dans map.json", current_seed.0);
        }
    }

    for _ in load_events.read() {
        if let Ok(content) = std::fs::read_to_string("map.json") {
            if let Ok(v) = serde_json::from_str::<serde_json::Value>(&content) {
                if let Some(seed) = v.get("seed").and_then(|s| s.as_u64()) {
                    current_seed.0 = seed as u32;
                    world.chunk = generate_chunk_from_seed(seed as u32, world.chunk.chunk_pos);
                    modifications.0.clear();

                    if let Some(mods) = v.get("modifications").and_then(|m| m.as_array()) {
                        for m in mods {
                            if let (Some(x), Some(y), Some(z), Some(t_str)) = (
                                m.get("x").and_then(|x| x.as_i64()),
                                m.get("y").and_then(|y| y.as_i64()),
                                m.get("z").and_then(|z| z.as_i64()),
                                m.get("type").and_then(|t| t.as_str())
                            ) {
                                let block = match t_str {
                                    "Stone" => BlockType::Stone,
                                    "Dirt" => BlockType::Dirt,
                                    "Grass" => BlockType::Grass,
                                    _ => BlockType::Air,
                                };
                                let pos = IVec3 { x: x as i32, y: y as i32, z: z as i32 };
                                modifications.0.insert((pos.x, pos.y, pos.z), block);
                                let _ = world.chunk.set_local(pos, block);
                            }
                        }
                    }

                    // Re-mesh
                    for mesh3d in &query {
                        if let Ok(new_mesh) = crate::rendering::chunk_mesher::mesh_from_chunk(&world.chunk) {
                            if let Some(mesh) = meshes.get_mut(mesh3d.0.id()) {
                                *mesh = new_mesh;
                            }
                        }
                    }
                    println!("Map chargée depuis map.json (Seed: {})", current_seed.0);
                }
            }
        }
    }
}

fn interact_with_blocks(mouse: Res<ButtonInput<MouseButton>>, mut world: ResMut<SingleChunkWorld>, mut meshes: ResMut<Assets<Mesh>>, mut modifications: ResMut<MapModifications>, camera_query: Query<&GlobalTransform, With<PlayerCamera>>, chunk_query: Query<&Mesh3d, With<WorldChunkMarker>>) {
    if mouse.just_pressed(MouseButton::Right) {
        for camera_transform in &camera_query {
            let origin = camera_transform.translation();
            let forward = camera_transform.forward();
            let is_solid = |p: IVec3| -> bool {
                if p.x < 0 || p.x >= CHUNK_SIZE as i32 || p.y < 0 || p.y >= CHUNK_SIZE as i32 || p.z < 0 || p.z >= CHUNK_SIZE as i32 {
                    return false;
                }
                world.chunk.get_local(p).map(|b| b.is_solid()).unwrap_or(false)
            };

            if let Some(result) = crate::world::raycast_world(origin, forward.into(), 10.0, &is_solid) {
                let voxel_pos = result.position;
                world.chunk.set_local(voxel_pos, BlockType::Air).unwrap();
                modifications.0.insert((voxel_pos.x, voxel_pos.y, voxel_pos.z), BlockType::Air);
                
                for mesh3d in &chunk_query {
                    let new_mesh = crate::rendering::chunk_mesher::mesh_from_chunk(&world.chunk).unwrap();
                    if let Some(mesh) = meshes.get_mut(mesh3d.0.id()) {
                        *mesh = new_mesh;
                    }
                }
            }
        }
    }
}
