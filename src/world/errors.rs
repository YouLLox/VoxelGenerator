#[derive(Debug)]
pub enum OctreeError {
    OutOfBounds,
    InvalidSize,
    SaveFailed,
    LoadFailed
}