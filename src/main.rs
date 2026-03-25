use bevy::prelude::*;

mod world;
mod rendering;
mod proc_gen;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(rendering::camera::CameraPlugin)
        .add_plugins(rendering::setup::SetupPlugin)
        .run();
}

#[cfg(test)]
mod tests {
    use super::world::{BlockType, IVec3, Octree};
    use std::path::Path;
    use std::sync::Mutex;

    const DEMO_JSON_PATH: &str = "octree_demo.json";
    static DEMO_TEST_LOCK: Mutex<()> = Mutex::new(());

    #[test]
    #[ignore = "manual"]
    //cargo test save_json -- --ignored
    fn save_json() {
        let _guard = DEMO_TEST_LOCK.lock().unwrap();

        let mut octree = Octree::new(8, BlockType::Air).unwrap();
        octree.set(IVec3::new(2, 3, 4), BlockType::Stone).unwrap();
        octree.set(IVec3::new(1, 1, 1), BlockType::Dirt).unwrap();

        octree.save().unwrap();

        let source = Path::new("octree.json");
        assert!(source.exists());
        std::fs::rename(source, DEMO_JSON_PATH).unwrap();
    }

    #[test]
    #[ignore = "manual"]
    //cargo test load_json -- --ignored
    fn load_json() {
        let _guard = DEMO_TEST_LOCK.lock().unwrap();
        assert!(Path::new(DEMO_JSON_PATH).exists());
        let loaded = Octree::load(DEMO_JSON_PATH, 8).unwrap();

        assert_eq!(
            loaded.get(IVec3::new(2, 3, 4)).unwrap(),
            BlockType::Stone
        );
        assert_eq!(loaded.get(IVec3::new(1, 1, 1)).unwrap(), BlockType::Dirt);
        assert_eq!(loaded.get(IVec3::new(0, 0, 0)).unwrap(), BlockType::Air);
    }
}


