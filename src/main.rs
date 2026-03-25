use bevy::prelude::*;

mod world;
mod rendering;
mod proc_gen;
fn main() {
    //proc_gen::bruit_main::noise_main();
    //test génération procédurale.
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(rendering::camera::CameraPlugin)
        .add_plugins(rendering::setup::SetupPlugin)
        .run();


}


