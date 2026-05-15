use bevy::prelude::*;

#[derive(Component)]
pub struct Player;

#[derive(Component)]
pub struct PlayerCamera;

#[derive(Component)]
pub struct PlayerController {
    pub velocity: Vec3,
    pub speed: f32,
    pub jump_speed: f32,
    pub gravity: f32,
    pub on_ground: bool,
    pub half_width: f32,
    pub half_depth: f32,
    pub height: f32,
    pub eye_height: f32,
}

impl Default for PlayerController {
    fn default() -> Self {
        Self {
            velocity: Vec3::ZERO,
            speed: 6.0,
            jump_speed: 8.5,
            gravity: 22.0,
            on_ground: false,
            half_width: 0.3,
            half_depth: 0.3,
            height: 1.8,
            eye_height: 1.62,
        }
    }
}

#[derive(Component)]
pub struct PlayerLook {
    pub yaw: f32,
    pub pitch: f32,
    pub sensitivity: f32,
}

impl Default for PlayerLook {
    fn default() -> Self {
        Self {
            yaw: 0.0,
            pitch: 0.0,
            sensitivity: 0.0015,
        }
    }
}