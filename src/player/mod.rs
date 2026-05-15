use bevy::prelude::*;

pub mod controller;
pub mod look;
pub mod movement;
pub mod collision;

pub use controller::{Player, PlayerCamera, PlayerController, PlayerLook};

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (
            look::grab_mouse_system,
            look::player_look_system,
            movement::player_move_system,
        ));
    }
}