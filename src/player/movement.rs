use bevy::prelude::*;

use crate::player::collision;
use crate::player::controller::{Player, PlayerController, PlayerLook};
use crate::world::SingleChunkWorld;

pub fn player_move_system(
    time: Res<Time>,
    keys: Res<ButtonInput<KeyCode>>,
    world: Res<SingleChunkWorld>,
    mut query: Query<(&mut Transform, &mut PlayerController, &PlayerLook), With<Player>>,
) {
    let Some((mut transform, mut controller, look)) = query.iter_mut().next() else {
        return;
    };

    let dt = time.delta_secs();

    let yaw_rotation = Quat::from_axis_angle(Vec3::Y, look.yaw);
    let forward = yaw_rotation * -Vec3::Z;
    let right = yaw_rotation * Vec3::X;

    let mut move_dir = Vec3::ZERO;

    if keys.pressed(KeyCode::KeyW) {
        move_dir += forward;
    }
    if keys.pressed(KeyCode::KeyS) {
        move_dir -= forward;
    }
    if keys.pressed(KeyCode::KeyD) {
        move_dir += right;
    }
    if keys.pressed(KeyCode::KeyA) {
        move_dir -= right;
    }

    move_dir.y = 0.0;

    if move_dir != Vec3::ZERO {
        move_dir = move_dir.normalize();
    }

    controller.velocity.x = move_dir.x * controller.speed;
    controller.velocity.z = move_dir.z * controller.speed;

    if controller.on_ground && keys.just_pressed(KeyCode::Space) {
        controller.velocity.y = controller.jump_speed;
        controller.on_ground = false;
    }

    controller.velocity.y -= controller.gravity * dt;

    collision::move_and_collide(&world, &mut transform.translation, &mut controller, dt);
}