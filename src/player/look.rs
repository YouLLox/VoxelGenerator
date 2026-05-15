use bevy::input::mouse::MouseMotion;
use bevy::prelude::*;
use bevy::window::{CursorGrabMode, PrimaryWindow};

use crate::player::controller::{Player, PlayerCamera, PlayerLook};

pub fn grab_mouse_system(
    mut cursor_options: Query<&mut bevy::window::CursorOptions, With<PrimaryWindow>>,
    mouse: Res<ButtonInput<MouseButton>>,
    keys: Res<ButtonInput<KeyCode>>,
) {
    let Some(mut cursor) = cursor_options.iter_mut().next() else {
        return;
    };

    if mouse.just_pressed(MouseButton::Left) {
        cursor.visible = false;
        cursor.grab_mode = CursorGrabMode::Locked;
    }

    if keys.just_pressed(KeyCode::Escape) {
        cursor.visible = true;
        cursor.grab_mode = CursorGrabMode::None;
    }
}

pub fn player_look_system(
    mut mouse_motion_events: MessageReader<MouseMotion>,
    cursor_options: Query<&bevy::window::CursorOptions, With<PrimaryWindow>>,
    mut player_query: Query<(&mut Transform, &mut PlayerLook), With<Player>>,
    mut camera_query: Query<&mut Transform, (With<PlayerCamera>, Without<Player>)>,
) {
    let Some(cursor) = cursor_options.iter().next() else {
        return;
    };

    if cursor.grab_mode != CursorGrabMode::Locked {
        mouse_motion_events.clear();
        return;
    }

    let Some((mut player_transform, mut look)) = player_query.iter_mut().next() else {
        return;
    };

    let Some(mut camera_transform) = camera_query.iter_mut().next() else {
        return;
    };

    let mut mouse_delta = Vec2::ZERO;
    for event in mouse_motion_events.read() {
        mouse_delta += event.delta;
    }

    if mouse_delta == Vec2::ZERO {
        return;
    }

    look.yaw -= mouse_delta.x * look.sensitivity;
    look.pitch -= mouse_delta.y * look.sensitivity;
    look.pitch = look.pitch.clamp(-2.0, 2.0);

    player_transform.rotation = Quat::from_axis_angle(Vec3::Y, look.yaw);
    camera_transform.rotation = Quat::from_axis_angle(Vec3::X, look.pitch);
}