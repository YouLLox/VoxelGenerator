use bevy::prelude::*;

mod world;
mod rendering;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(rendering::camera::CameraPlugin)
        .add_plugins(rendering::setup::SetupPlugin)
        .run();
}


