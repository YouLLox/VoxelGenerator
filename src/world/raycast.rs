use crate::world::IVec3;

pub struct RaycastResult {
    pub position: IVec3,
    pub normal: IVec3,
}

pub fn raycast_world(
    origin: bevy::prelude::Vec3,
    direction: bevy::prelude::Vec3,
    max_distance: f32,
    is_solid: &impl Fn(IVec3) -> bool,
) -> Option<RaycastResult> {
    let mut x = origin.x.floor() as i32;
    let mut y = origin.y.floor() as i32;
    let mut z = origin.z.floor() as i32;

    let dx = direction.x;
    let dy = direction.y;
    let dz = direction.z;

    let step_x = if dx > 0.0 { 1 } else { -1 };
    let step_y = if dy > 0.0 { 1 } else { -1 };
    let step_z = if dz > 0.0 { 1 } else { -1 };

    let t_delta_x = if dx != 0.0 { (1.0 / dx).abs() } else { f32::MAX };
    let t_delta_y = if dy != 0.0 { (1.0 / dy).abs() } else { f32::MAX };
    let t_delta_z = if dz != 0.0 { (1.0 / dz).abs() } else { f32::MAX };

    let mut t_max_x = if dx > 0.0 { (x as f32 + 1.0 - origin.x) * t_delta_x } else { (origin.x - x as f32) * t_delta_x };
    let mut t_max_y = if dy > 0.0 { (y as f32 + 1.0 - origin.y) * t_delta_y } else { (origin.y - y as f32) * t_delta_y };
    let mut t_max_z = if dz > 0.0 { (z as f32 + 1.0 - origin.z) * t_delta_z } else { (origin.z - z as f32) * t_delta_z };

    let mut normal = IVec3::ZERO;

    while t_max_x.min(t_max_y).min(t_max_z) <= max_distance {
        if t_max_x < t_max_y {
            if t_max_x < t_max_z {
                x += step_x;
                t_max_x += t_delta_x;
                normal = IVec3::new(-step_x, 0, 0);
            } else {
                z += step_z;
                t_max_z += t_delta_z;
                normal = IVec3::new(0, 0, -step_z);
            }
        } else {
            if t_max_y < t_max_z {
                y += step_y;
                t_max_y += t_delta_y;
                normal = IVec3::new(0, -step_y, 0);
            } else {
                z += step_z;
                t_max_z += t_delta_z;
                normal = IVec3::new(0, 0, -step_z);
            }
        }

        let pos = IVec3::new(x, y, z);
        if is_solid(pos) {
            return Some(RaycastResult {
                position: pos,
                normal,
            });
        }
    }

    None
}
