use crate::{
    chunk::{Block, AIR},
    Options, World,
};
use glam::{IVec3, Vec2, Vec3};
use internment::Intern;

const TOP_LIGHT_LEVEL: f32 = 1.0;
const FRONT_BACK_LIGHT_LEVEL: f32 = 0.85;
const SIDE_LIGHT_LEVEL: f32 = 0.75;
const BOTTOM_LIGHT_LEVEL: f32 = 0.6;

pub struct Mesh {
    pub vertices: Vec<Vertex>,
    pub indices: Vec<u32>,
}

impl Mesh {
    pub fn build(world: &World, options: &Options) -> Self {
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
        let mut vertices = Vec::new();
        let mut indices = Vec::new();

        for y in -64..320 {
            for z in options.area.start.y..options.area.end.y {
                for x in options.area.start.x..options.area.end.x {
                    let p = IVec3 { x, y, z };
                    let Some(block) = world.block_at(p) else {
                        continue;
                    };
                    if !is_solid(block) {
                        continue;
                    }

                    let texture_index = if &**block.name == "minecraft:stone" {
                        1.0
                    } else {
                        0.0
                    };

                    let v = |pos, u, v, light_level| Vertex {
                        pos,
                        uv: Vec2 { x: u, y: v },
                        light_level,
                        texture_index,
                    };

                    if !world.block_at(p - IVec3::X).map_or(false, is_solid) {
                        let p = p.as_vec3();
                        let vertex_count = vertices.len() as u32;
                        vertices.extend([
                            v(p, 0.0, 1.0, SIDE_LIGHT_LEVEL),
                            v(p + Vec3::Z, 1.0, 1.0, SIDE_LIGHT_LEVEL),
                            v(p + Vec3::Y, 0.0, 0.0, SIDE_LIGHT_LEVEL),
                            v(
                                p + Vec3::Z + Vec3::Y,
                                1.0,
                                0.0,
                                SIDE_LIGHT_LEVEL,
                            ),
                        ]);
                        indices.extend([
                            vertex_count,
                            vertex_count + 1,
                            vertex_count + 2,
                            vertex_count + 1,
                            vertex_count + 3,
                            vertex_count + 2,
                        ]);
                    }
                    if !world.block_at(p + IVec3::X).map_or(false, is_solid) {
                        let p = p.as_vec3() + Vec3::X;
                        let vertex_count = vertices.len() as u32;
                        vertices.extend([
                            v(p, 0.0, 1.0, SIDE_LIGHT_LEVEL),
                            v(p + Vec3::Z, 1.0, 1.0, SIDE_LIGHT_LEVEL),
                            v(p + Vec3::Y, 0.0, 0.0, SIDE_LIGHT_LEVEL),
                            v(
                                p + Vec3::Z + Vec3::Y,
                                1.0,
                                0.0,
                                SIDE_LIGHT_LEVEL,
                            ),
                        ]);
                        indices.extend([
                            vertex_count,
                            vertex_count + 2,
                            vertex_count + 1,
                            vertex_count + 1,
                            vertex_count + 2,
                            vertex_count + 3,
                        ]);
                    }
                    if !world.block_at(p - IVec3::Y).map_or(false, is_solid) {
                        let p = p.as_vec3();
                        let vertex_count = vertices.len() as u32;
                        vertices.extend([
                            v(p, 0.0, 1.0, BOTTOM_LIGHT_LEVEL),
                            v(p + Vec3::X, 1.0, 1.0, BOTTOM_LIGHT_LEVEL),
                            v(p + Vec3::Z, 0.0, 0.0, BOTTOM_LIGHT_LEVEL),
                            v(
                                p + Vec3::X + Vec3::Z,
                                1.0,
                                0.0,
                                BOTTOM_LIGHT_LEVEL,
                            ),
                        ]);
                        indices.extend([
                            vertex_count,
                            vertex_count + 1,
                            vertex_count + 2,
                            vertex_count + 1,
                            vertex_count + 3,
                            vertex_count + 2,
                        ]);
                    }
                    if !world.block_at(p + IVec3::Y).map_or(false, is_solid) {
                        let p = p.as_vec3() + Vec3::Y;
                        let vertex_count = vertices.len() as u32;
                        vertices.extend([
                            v(p, 0.0, 1.0, TOP_LIGHT_LEVEL),
                            v(p + Vec3::X, 1.0, 1.0, TOP_LIGHT_LEVEL),
                            v(p + Vec3::Z, 0.0, 0.0, TOP_LIGHT_LEVEL),
                            v(p + Vec3::X + Vec3::Z, 1.0, 0.0, TOP_LIGHT_LEVEL),
                        ]);
                        indices.extend([
                            vertex_count,
                            vertex_count + 2,
                            vertex_count + 1,
                            vertex_count + 1,
                            vertex_count + 2,
                            vertex_count + 3,
                        ]);
                    }
                    if !world.block_at(p - IVec3::Z).map_or(false, is_solid) {
                        let p = p.as_vec3();
                        let vertex_count = vertices.len() as u32;
                        vertices.extend([
                            v(p, 0.0, 1.0, FRONT_BACK_LIGHT_LEVEL),
                            v(p + Vec3::X, 1.0, 1.0, FRONT_BACK_LIGHT_LEVEL),
                            v(p + Vec3::Y, 0.0, 0.0, FRONT_BACK_LIGHT_LEVEL),
                            v(
                                p + Vec3::X + Vec3::Y,
                                1.0,
                                0.0,
                                FRONT_BACK_LIGHT_LEVEL,
                            ),
                        ]);
                        indices.extend([
                            vertex_count,
                            vertex_count + 2,
                            vertex_count + 1,
                            vertex_count + 1,
                            vertex_count + 2,
                            vertex_count + 3,
                        ]);
                    }
                    if !world.block_at(p + IVec3::Z).map_or(false, is_solid) {
                        let p = p.as_vec3() + Vec3::Z;
                        let vertex_count = vertices.len() as u32;
                        vertices.extend([
                            v(p, 0.0, 1.0, FRONT_BACK_LIGHT_LEVEL),
                            v(p + Vec3::X, 1.0, 1.0, FRONT_BACK_LIGHT_LEVEL),
                            v(p + Vec3::Y, 0.0, 0.0, FRONT_BACK_LIGHT_LEVEL),
                            v(
                                p + Vec3::X + Vec3::Y,
                                1.0,
                                0.0,
                                FRONT_BACK_LIGHT_LEVEL,
                            ),
                        ]);
                        indices.extend([
                            vertex_count,
                            vertex_count + 1,
                            vertex_count + 2,
                            vertex_count + 1,
                            vertex_count + 3,
                            vertex_count + 2,
                        ]);
                    }
                }
            }
        }

        Self { vertices, indices }
    }
}

fn is_solid(block: Intern<Block>) -> bool {
    block != *AIR
}

#[repr(C)]
pub struct Vertex {
    pub pos: Vec3,
    pub uv: Vec2,
    pub light_level: f32,
    pub texture_index: f32,
}
