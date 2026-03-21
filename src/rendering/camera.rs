use bevy::prelude::*;
use bevy::input::mouse::MouseMotion;
use bevy::window::{CursorGrabMode, PrimaryWindow};

#[derive(Component)]
pub struct FlyCamera {
    pub speed: f32,
    pub sensitivity: f32,
}

impl Default for FlyCamera
{
    fn default() -> Self
    {
        Self {
            speed: 10.0,
            sensitivity: 0.0015,
        }
    }
}

pub struct CameraPlugin;

impl Plugin for CameraPlugin
{
    fn build(&self, app: &mut App)
    {
        app.add_systems(Update, (camera_look_system, camera_move_system, grab_mouse));
    }
}

fn grab_mouse(
    mut cursor_options: Query<&mut bevy::window::CursorOptions, With<PrimaryWindow>>,
    mouse: Res<ButtonInput<MouseButton>>,
    keys: Res<ButtonInput<KeyCode>>,
)
{
    let Some(mut cursor) = cursor_options.iter_mut().next() else { return };

    // Left click to grab cursor
    if mouse.just_pressed(MouseButton::Left) {
        cursor.visible = false;
        cursor.grab_mode = CursorGrabMode::Locked;
    }

    // Escape to un-grab cursor
    if keys.just_pressed(KeyCode::Escape) {
        cursor.visible = true;
        cursor.grab_mode = CursorGrabMode::None;
    }
}

fn camera_look_system(
    mut mouse_motion_events: MessageReader<MouseMotion>,
    mut query: Query<(&mut Transform, &FlyCamera)>,
    cursor_options: Query<&bevy::window::CursorOptions, With<PrimaryWindow>>,
){
    let Some(cursor) = cursor_options.iter().next() else { return };
    
    // Bouge la camera que si le curosor est locked
    if cursor.grab_mode != CursorGrabMode::Locked {
        mouse_motion_events.clear();
        return;
    }

    for (mut transform, camera) in query.iter_mut() {
        let mut mouse_delta = Vec2::ZERO;
        for event in mouse_motion_events.read() {
            mouse_delta += event.delta;
        }

        if mouse_delta != Vec2::ZERO {
            // rotation
            let (mut yaw, mut pitch, _) = transform.rotation.to_euler(EulerRot::YXZ);
            
            yaw -= mouse_delta.x * camera.sensitivity;
            pitch -= mouse_delta.y * camera.sensitivity;

            pitch = pitch.clamp(-1.54, 1.54); // clip l'orientation

            transform.rotation = Quat::from_axis_angle(Vec3::Y, yaw)
                * Quat::from_axis_angle(Vec3::X, pitch);
        }
    }
}

fn camera_move_system(
    time: Res<Time>,
    keys: Res<ButtonInput<KeyCode>>,
    mut query: Query<(&mut Transform, &FlyCamera)>,
    cursor_options: Query<&bevy::window::CursorOptions, With<PrimaryWindow>>,
)
{
    let Some(cursor) = cursor_options.iter().next() else { return };
    if cursor.grab_mode != CursorGrabMode::Locked { return; }

    for (mut transform, camera) in query.iter_mut() {
        let mut direction = Vec3::ZERO;

        let forward = -transform.local_z();
        let right = transform.local_x();

        if keys.pressed(KeyCode::KeyW) { direction += *forward; }
        if keys.pressed(KeyCode::KeyS) { direction -= *forward; }
        if keys.pressed(KeyCode::KeyD) { direction += *right; }
        if keys.pressed(KeyCode::KeyA) { direction -= *right; }
        if keys.pressed(KeyCode::Space) { direction += Vec3::Y; }
        if keys.pressed(KeyCode::ShiftLeft) { direction -= Vec3::Y; }

        if direction != Vec3::ZERO {
            transform.translation += direction.normalize() * camera.speed * time.delta_secs();
        }
    }
}
