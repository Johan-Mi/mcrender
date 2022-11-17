use crate::{chunk::Block, World};

pub struct Mesh {
    // TODO
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
        todo!()
    }
}

fn is_solid(block: &Block) -> bool {
    todo!()
}
