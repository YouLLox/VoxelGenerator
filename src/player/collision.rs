use bevy::prelude::*;

use crate::player::controller::PlayerController;
use crate::world::{IVec3, ChunkManager};

fn aabb_collides(world: &ChunkManager, pos: Vec3, controller: &PlayerController) -> bool {
    let min = Vec3::new(
        pos.x - controller.half_width,
        pos.y,
        pos.z - controller.half_depth,
    );

    let max = Vec3::new(
        pos.x + controller.half_width,
        pos.y + controller.height,
        pos.z + controller.half_depth,
    );

    let eps = 0.0001;

    let min_x = min.x.floor() as i32;
    let max_x = (max.x - eps).floor() as i32;

    let min_y = (min.y ).floor() as i32;
    let max_y = (max.y- eps).floor() as i32;

    let min_z = (min.z ).floor() as i32;
    let max_z = (max.z  - eps).floor() as i32;

    for x in min_x..=max_x {
        for y in min_y..=max_y {
            for z in min_z..=max_z {
                if world.is_solid_world(IVec3::new(x, y, z)) {
                    return true;
                }
            }
        }
    }

    false
}

pub fn move_and_collide(
    world: &ChunkManager,
    position: &mut Vec3,
    controller: &mut PlayerController,
    dt: f32,
) {
    // X
    let mut candidate = *position;
    candidate.x += controller.velocity.x * dt;

    if aabb_collides(world, candidate, controller) {
        controller.velocity.x = 0.0;
    } else {
        *position = candidate;
    }

    // Z
    let mut candidate = *position;
    candidate.z += controller.velocity.z * dt;

    if aabb_collides(world, candidate, controller) {
        controller.velocity.z = 0.0;
    } else {
        *position = candidate;
    }

    // Y
    controller.on_ground = false;

    let mut candidate = *position;
    candidate.y += controller.velocity.y * dt;

    if aabb_collides(world, candidate, controller) {
        if controller.velocity.y < 0.0 {
            controller.on_ground = true;
        }
        controller.velocity.y = 0.0;
    } else {
        *position = candidate;
    }
}
