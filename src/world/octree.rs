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

//méthodes privées
impl Octree {
    fn is_power_of_two(size: u32) -> bool {
        size != 0 && (size & (size - 1)) == 0
    }

    fn contains(&self, pos: IVec3) -> bool {
        let size = self.size as i32;

        pos.x >= 0
            && pos.y >= 0
            && pos.z >= 0
            && pos.x < size
            && pos.y < size
            && pos.z < size
    }

    fn child_index(origin: IVec3, half: u32, pos: IVec3) -> usize {
        let half = half as i32;

        let x_bit = if pos.x < origin.x + half { 0 } else { 1 };
        let y_bit = if pos.y < origin.y + half { 0 } else { 1 };
        let z_bit = if pos.z < origin.z + half { 0 } else { 1 };

        (x_bit + y_bit * 2 + z_bit * 4) as usize
    }

    fn child_origin(origin: IVec3, half: u32, index: usize) -> IVec3 {
        let half = half as i32;

        let x = if (index & 1) == 0 {
            origin.x
        } else {
            origin.x + half
        };

        let y = if (index & 2) == 0 {
            origin.y
        } else {
            origin.y + half
        };

        let z = if (index & 4) == 0 {
            origin.z
        } else {
            origin.z + half
        };

        IVec3 { x, y, z }
    }

    fn subdivide(node: &mut Node) {
        match node {
            Node::Leaf(block) => {
                let fill = *block;

                *node = Node::Children(Box::new([
                    Node::Leaf(fill),
                    Node::Leaf(fill),
                    Node::Leaf(fill),
                    Node::Leaf(fill),
                    Node::Leaf(fill),
                    Node::Leaf(fill),
                    Node::Leaf(fill),
                    Node::Leaf(fill),
                ]));
            }
            Node::Children(_) => {}
        }
    }

    fn try_merge(node: &mut Node) {
        let first_block = match node {
            Node::Leaf(_) => return,
            Node::Children(children) => match &children[0] {
                Node::Leaf(block) => *block,
                Node::Children(_) => return,
            },
        };

        match node {
            Node::Leaf(_) => {}
            Node::Children(children) => {
                let mut i = 1;

                while i < 8 {
                    match &children[i] {
                        Node::Leaf(block) if *block == first_block => {}
                        _ => return,
                    }
                    i += 1;
                }

                *node = Node::Leaf(first_block);
            }
        }
    }

    fn get_rec(node: &Node, origin: IVec3, size: u32, pos: IVec3) -> BlockType {
        match node {
            Node::Leaf(block) => *block,
            Node::Children(children) => {
                let half = size / 2;
                let index = Self::child_index(origin, half, pos);
                let next_origin = Self::child_origin(origin, half, index);

                Self::get_rec(&children[index], next_origin, half, pos)
            }
        }
    }

    fn set_rec(node: &mut Node, origin: IVec3, size: u32, pos: IVec3, block: BlockType) {
        if size == 1 {
            *node = Node::Leaf(block);
            return;
        }

        match node {
            Node::Leaf(current) => {
                if *current == block {
                    return;
                }

                Self::subdivide(node);
            }
            Node::Children(_) => {}
        }

        let half = size / 2;
        let index = Self::child_index(origin, half, pos);
        let next_origin = Self::child_origin(origin, half, index);

        match node {
            Node::Leaf(_) => unreachable!(),
            Node::Children(children) => {
                Self::set_rec(&mut children[index], next_origin, half, pos, block);
            }
        }

        Self::try_merge(node);
    }

    fn visit_leaves_rec<F>(node: &Node, origin: IVec3, size: u32, f: &mut F)
    where
        F: FnMut(IVec3, u32, BlockType),
    {
        match node {
            Node::Leaf(block) => {
                f(origin, size, *block);
            }
            Node::Children(children) => {
                let half = size / 2;

                let mut i = 0;
                while i < 8 {
                    let next_origin = Self::child_origin(origin, half, i);
                    Self::visit_leaves_rec(&children[i], next_origin, half, f);
                    i += 1;
                }
            }
        }
    }
}

// méthodes publiques
impl Octree {
    pub fn new(size: u32, fill: BlockType) -> Result<Self, OctreeError> {
        if !Self::is_power_of_two(size) {
            return Err(OctreeError::InvalidSize);
        }

        Ok(Self { size: size, root: Node::Leaf(fill) })
    }

    pub fn size(&self) -> u32 {
        self.size
    }

    pub fn get(&self, pos: IVec3) -> Result<BlockType, OctreeError> {
        let root_origin = IVec3 { x: 0, y: 0, z: 0 };

        if !self.contains(pos) {
            return Err(OctreeError::OutOfBounds);
        }

        Ok(Self::get_rec(&self.root, root_origin, self.size, pos))
    }

    pub fn set(&mut self, pos: IVec3, block: BlockType) -> Result<(), OctreeError> {
        let root_origin = IVec3 { x: 0, y: 0, z: 0 };

        if !self.contains(pos) {
            return Err(OctreeError::OutOfBounds);
        }

        Self::set_rec(&mut self.root, root_origin, self.size, pos, block);

        Ok(())
    }

    pub fn remove(&mut self, pos: IVec3) -> Result<(), OctreeError> {
        self.set(pos, BlockType::Air)
    }

    pub fn clear(&mut self, fill: BlockType) {
        self.root = Node::Leaf(fill);
    }

    pub fn fill_region(&mut self, min: IVec3, max: IVec3, block: BlockType,) -> Result<(), OctreeError> {
        if min.x > max.x || min.y > max.y || min.z > max.z {
            return Err(OctreeError::OutOfBounds);
        }

        if !self.contains(min) || !self.contains(max) {
            return Err(OctreeError::OutOfBounds);
        }

        for z in min.z..=max.z {
            for y in min.y..=max.y {
                for x in min.x..=max.x {
                    self.set(IVec3 { x, y, z }, block)?;
                }
            }
        }

        Ok(())
    }

    pub fn visit_leaves<F>(&self, mut f: F)
    where
        F: FnMut(IVec3, u32, BlockType),
    {
        let root_origin = IVec3 { x: 0, y: 0, z: 0 };
        Self::visit_leaves_rec(&self.root, root_origin, self.size, &mut f);
    }
}