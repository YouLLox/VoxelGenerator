use crate::world::{BlockType, IVec3, Octree, OctreeError};

pub const CHUNK_SIZE: u32 = 32;

pub struct Chunk {
    pub chunk_pos: IVec3,
    pub octree: Octree,
    pub dirty: bool,
}

impl Chunk {
    pub fn new(chunk_pos: IVec3, fill: BlockType) -> Result<Self, OctreeError> {
        match Octree::new(CHUNK_SIZE, fill) {
            Err(error)  => Err(error),
            Ok(tree)    => Ok(Self { chunk_pos, octree: tree, dirty: false }),
        }
    }

    pub fn get_local(&self, pos: IVec3) -> Result<BlockType, OctreeError> {
        self.octree.get(pos)
    }

    pub fn set_local(&mut self, pos: IVec3, block: BlockType) -> Result<(), OctreeError> {
        self.octree.set(pos, block)?;
        self.dirty = true;
        Ok(())
    }

    pub fn remove_local(&mut self, pos: IVec3) -> Result<(), OctreeError> {
        self.octree.remove(pos)?;
        self.dirty = true;
        Ok(())
    }

    pub fn is_dirty(&self) -> bool {
        self.dirty
    }

    pub fn clear_dirty(&mut self) {
        self.dirty = false;
    }
}
