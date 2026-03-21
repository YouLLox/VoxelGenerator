use bevy::prelude::*;
use super::{camera, mesh_utils};

pub struct SetupPlugin;

impl Plugin for SetupPlugin
{
    fn build(&self, app: &mut App)
    {
        app.add_systems(Startup, setup);
    }
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
)
{
    // CHUNK en 16x16x16
    commands.spawn((
        Mesh3d(meshes.add(mesh_utils::simulate_chunk_mesh())),
        MeshMaterial3d(materials.add(Color::from(Srgba::new(0.2, 0.6, 0.9, 1.0)))),
        Transform::from_xyz(-8.0, 0.0, -8.0), // Centre du chunk
    ));

    // LUMIERE
    commands.spawn((
        PointLight {
            shadows_enabled: true,
            intensity: 10_000_000.0,
            range: 100.0,
            ..default()
        },
        Transform::from_xyz(4.0, 8.0, 4.0),
    ));

    // CAM
    commands.spawn((
        Camera3d::default(),
        Transform::from_xyz(-2.5, 4.5, 9.0).looking_at(Vec3::ZERO, Vec3::Y),
        camera::FlyCamera::default(),
    ));
}
