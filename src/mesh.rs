use crate::{chunk::Block, World};
use glam::Vec3;

pub struct Mesh {
    pub vertices: Vec<Vertex>,
    pub indices: Vec<u32>,
}

impl Mesh {
    pub fn build(world: &World) -> Self {
        // # The plan:
        // - Loop through each pair of opposite faces (e.g. top then bottom,
        //   front then back, left then right)
        // - Place squares for each solid block
        // - When placing a bottom face, if a top face already exists in the
        //   same location, delete both of them
        // - Place triangles for other block types like cross-shaped plants
        // - Perform greedy meshing
        // - ???
        // - Profit!

        // Temporary cube model
        let v = |x, y, z| Vertex {
            pos: Vec3 { x, y, z },
        };
        Self {
            vertices: vec![
                v(0.0, 0.0, 0.0),
                v(0.0, 0.0, 1.0),
                v(0.0, 1.0, 0.0),
                v(0.0, 1.0, 1.0),
                v(1.0, 0.0, 0.0),
                v(1.0, 0.0, 1.0),
                v(1.0, 1.0, 0.0),
                v(1.0, 1.0, 1.0),
            ],
            indices: vec![
                0, 4, 2, 2, 4, 6, 1, 3, 5, 3, 7, 5, 0, 2, 1, 2, 3, 1, 4, 5, 6,
                6, 5, 7, 0, 1, 4, 4, 1, 5, 2, 6, 3, 6, 7, 3,
            ],
        }
    }
}

fn is_solid(block: &Block) -> bool {
    todo!()
}

pub struct Vertex {
    #[allow(dead_code)]
    pub pos: Vec3,
}
