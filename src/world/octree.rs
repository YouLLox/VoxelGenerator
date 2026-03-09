use crate::world::{BlockType, IVec3, OctreeError};

#[derive(Clone, Debug, PartialEq, Eq)]
enum Node {
    Leaf(BlockType),
    Children(Box<[Node; 8]>),
}

pub struct Octree {
    size: u32,
    root: Node,
}

//méthodes publiques
impl Octree {
    pub fn new(size: u32, fill: BlockType) -> Result<Self, OctreeError> {
        todo!()
    }

    pub fn size(&self) -> u32 {
        self.size
    }

    pub fn get(&self, pos: IVec3) -> Result<BlockType, OctreeError> {
        todo!()
    }

    pub fn set(&mut self, pos: IVec3, block: BlockType) -> Result<(), OctreeError> {
        todo!()
    }

    pub fn remove(&mut self, pos: IVec3) -> Result<(), OctreeError> {
        todo!()
    }

    pub fn clear(&mut self, fill: BlockType) {
        todo!()
    }

    pub fn fill_region(
        &mut self,
        min: IVec3,
        max: IVec3,
        block: BlockType,
    ) -> Result<(), OctreeError> {
        todo!()
    }

    pub fn visit_leaves<F>(&self, f: F)
    where
        F: FnMut(IVec3, u32, BlockType),
    {
        todo!()
    }
}

//méthodes privées
impl Octree {
    fn is_power_of_two(size: u32) -> bool {
        todo!()
    }

    fn contains(&self, pos: IVec3) -> bool {
        todo!()
    }

    fn child_index(origin: IVec3, half: u32, pos: IVec3) -> usize {
        todo!()
    }

    fn child_origin(origin: IVec3, half: u32, index: usize) -> IVec3 {
        todo!()
    }

    fn subdivide(node: &mut Node) {
        todo!()
    }

    fn try_merge(node: &mut Node) {
        todo!()
    }

    fn get_rec(node: &Node, origin: IVec3, size: u32, pos: IVec3) -> BlockType {
        todo!()
    }

    fn set_rec(node: &mut Node, origin: IVec3, size: u32, pos: IVec3, block: BlockType) {
        todo!()
    }

    fn visit_leaves_rec<F>(node: &Node, origin: IVec3, size: u32, f: &mut F)
    where
        F: FnMut(IVec3, u32, BlockType),
    {
        todo!()
    }
}