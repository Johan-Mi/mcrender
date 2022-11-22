use crate::{
    chunk::{Block, AIR},
    Options, World,
};
use glam::{IVec3, Vec2, Vec3, Vec4, Vec4Swizzles};
use indexmap::IndexSet;
use internment::Intern;

const TOP_LIGHT_LEVEL: f32 = 1.0;
const FRONT_BACK_LIGHT_LEVEL: f32 = 0.85;
const SIDE_LIGHT_LEVEL: f32 = 0.75;
const BOTTOM_LIGHT_LEVEL: f32 = 0.6;

pub struct Mesh {
    pub vertices: Vec<Vertex>,
    pub indices: Vec<u32>,
    pub texture_names: IndexSet<&'static str>,
}

impl Mesh {
    pub fn build(world: &World, options: &Options) -> Self {
        Self {
            vertices: Vec::new(),
            indices: Vec::new(),
            texture_names: IndexSet::new(),
        }
        .inner_build_impl(world, options)
    }

    fn inner_build_impl(mut self, world: &World, options: &Options) -> Self {
        // TODO: perform greedy meshing
        for y in -64..320 {
            for z in options.area.start.y..options.area.end.y {
                for x in options.area.start.x..options.area.end.x {
                    let p = IVec3 { x, y, z };
                    let Some(block) = world.block_at(p) else {
                        continue;
                    };
                    match BlockModel::of(block) {
                        BlockModel::None => {}
                        BlockModel::SolidBlock
                        | BlockModel::TransparentBlock => {
                            let texture_index =
                                self.block_front_side_texture_id(block);

                            let v = |pos, u, v, light_level| Vertex {
                                pos,
                                uv: Vec2 { x: u, y: v },
                                light_level,
                                texture_index,
                            };

                            if !world
                                .block_at(p - IVec3::X)
                                .map_or(false, is_solid)
                            {
                                let p = p.as_vec3();
                                let vertex_count = self.vertices.len() as u32;
                                self.vertices.extend([
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
                                self.indices.extend([
                                    vertex_count,
                                    vertex_count + 1,
                                    vertex_count + 2,
                                    vertex_count + 1,
                                    vertex_count + 3,
                                    vertex_count + 2,
                                ]);
                            }
                            if !world
                                .block_at(p + IVec3::X)
                                .map_or(false, is_solid)
                            {
                                let p = p.as_vec3() + Vec3::X;
                                let vertex_count = self.vertices.len() as u32;
                                self.vertices.extend([
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
                                self.indices.extend([
                                    vertex_count,
                                    vertex_count + 2,
                                    vertex_count + 1,
                                    vertex_count + 1,
                                    vertex_count + 2,
                                    vertex_count + 3,
                                ]);
                            }
                            if !world
                                .block_at(p - IVec3::Y)
                                .map_or(false, is_solid)
                            {
                                let p = p.as_vec3();
                                let vertex_count = self.vertices.len() as u32;
                                self.vertices.extend([
                                    v(p, 0.0, 1.0, BOTTOM_LIGHT_LEVEL),
                                    v(
                                        p + Vec3::X,
                                        1.0,
                                        1.0,
                                        BOTTOM_LIGHT_LEVEL,
                                    ),
                                    v(
                                        p + Vec3::Z,
                                        0.0,
                                        0.0,
                                        BOTTOM_LIGHT_LEVEL,
                                    ),
                                    v(
                                        p + Vec3::X + Vec3::Z,
                                        1.0,
                                        0.0,
                                        BOTTOM_LIGHT_LEVEL,
                                    ),
                                ]);
                                self.indices.extend([
                                    vertex_count,
                                    vertex_count + 1,
                                    vertex_count + 2,
                                    vertex_count + 1,
                                    vertex_count + 3,
                                    vertex_count + 2,
                                ]);
                            }
                            if !world
                                .block_at(p + IVec3::Y)
                                .map_or(false, is_solid)
                            {
                                let texture_index =
                                    self.block_top_texture_id(block);

                                let v = |pos, u, v, light_level| Vertex {
                                    pos,
                                    uv: Vec2 { x: u, y: v },
                                    light_level,
                                    texture_index,
                                };

                                let p = p.as_vec3() + Vec3::Y;
                                let vertex_count = self.vertices.len() as u32;
                                self.vertices.extend([
                                    v(p, 0.0, 1.0, TOP_LIGHT_LEVEL),
                                    v(p + Vec3::X, 1.0, 1.0, TOP_LIGHT_LEVEL),
                                    v(p + Vec3::Z, 0.0, 0.0, TOP_LIGHT_LEVEL),
                                    v(
                                        p + Vec3::X + Vec3::Z,
                                        1.0,
                                        0.0,
                                        TOP_LIGHT_LEVEL,
                                    ),
                                ]);
                                self.indices.extend([
                                    vertex_count,
                                    vertex_count + 2,
                                    vertex_count + 1,
                                    vertex_count + 1,
                                    vertex_count + 2,
                                    vertex_count + 3,
                                ]);
                            }
                            if !world
                                .block_at(p - IVec3::Z)
                                .map_or(false, is_solid)
                            {
                                let p = p.as_vec3();
                                let vertex_count = self.vertices.len() as u32;
                                self.vertices.extend([
                                    v(p, 0.0, 1.0, FRONT_BACK_LIGHT_LEVEL),
                                    v(
                                        p + Vec3::X,
                                        1.0,
                                        1.0,
                                        FRONT_BACK_LIGHT_LEVEL,
                                    ),
                                    v(
                                        p + Vec3::Y,
                                        0.0,
                                        0.0,
                                        FRONT_BACK_LIGHT_LEVEL,
                                    ),
                                    v(
                                        p + Vec3::X + Vec3::Y,
                                        1.0,
                                        0.0,
                                        FRONT_BACK_LIGHT_LEVEL,
                                    ),
                                ]);
                                self.indices.extend([
                                    vertex_count,
                                    vertex_count + 2,
                                    vertex_count + 1,
                                    vertex_count + 1,
                                    vertex_count + 2,
                                    vertex_count + 3,
                                ]);
                            }
                            if !world
                                .block_at(p + IVec3::Z)
                                .map_or(false, is_solid)
                            {
                                let p = p.as_vec3() + Vec3::Z;
                                let vertex_count = self.vertices.len() as u32;
                                self.vertices.extend([
                                    v(p, 0.0, 1.0, FRONT_BACK_LIGHT_LEVEL),
                                    v(
                                        p + Vec3::X,
                                        1.0,
                                        1.0,
                                        FRONT_BACK_LIGHT_LEVEL,
                                    ),
                                    v(
                                        p + Vec3::Y,
                                        0.0,
                                        0.0,
                                        FRONT_BACK_LIGHT_LEVEL,
                                    ),
                                    v(
                                        p + Vec3::X + Vec3::Y,
                                        1.0,
                                        0.0,
                                        FRONT_BACK_LIGHT_LEVEL,
                                    ),
                                ]);
                                self.indices.extend([
                                    vertex_count,
                                    vertex_count + 1,
                                    vertex_count + 2,
                                    vertex_count + 1,
                                    vertex_count + 3,
                                    vertex_count + 2,
                                ]);
                            }
                        }
                        BlockModel::Cross(texture_name) => {
                            let texture_index =
                                self.allocate_texture(texture_name) as f32;

                            let v = |pos, u, v, light_level| Vertex {
                                pos,
                                uv: Vec2 { x: u, y: v },
                                light_level,
                                texture_index,
                            };

                            let p = p.as_vec3();
                            let vertex_count = self.vertices.len() as u32;
                            self.vertices.extend([
                                v(p, 1.0, 1.0, FRONT_BACK_LIGHT_LEVEL),
                                v(
                                    p + Vec3::X + Vec3::Z,
                                    0.0,
                                    1.0,
                                    FRONT_BACK_LIGHT_LEVEL,
                                ),
                                v(
                                    p + Vec3::Y,
                                    1.0,
                                    0.0,
                                    FRONT_BACK_LIGHT_LEVEL,
                                ),
                                v(
                                    p + Vec3::X + Vec3::Y + Vec3::Z,
                                    0.0,
                                    0.0,
                                    FRONT_BACK_LIGHT_LEVEL,
                                ),
                                v(
                                    p + Vec3::X,
                                    1.0,
                                    1.0,
                                    FRONT_BACK_LIGHT_LEVEL,
                                ),
                                v(
                                    p + Vec3::Z,
                                    0.0,
                                    1.0,
                                    FRONT_BACK_LIGHT_LEVEL,
                                ),
                                v(
                                    p + Vec3::X + Vec3::Y,
                                    1.0,
                                    0.0,
                                    FRONT_BACK_LIGHT_LEVEL,
                                ),
                                v(
                                    p + Vec3::Y + Vec3::Z,
                                    0.0,
                                    0.0,
                                    FRONT_BACK_LIGHT_LEVEL,
                                ),
                            ]);
                            self.indices.extend([
                                vertex_count,
                                vertex_count + 1,
                                vertex_count + 3,
                                vertex_count,
                                vertex_count + 3,
                                vertex_count + 2,
                                vertex_count,
                                vertex_count + 3,
                                vertex_count + 1,
                                vertex_count,
                                vertex_count + 2,
                                vertex_count + 3,
                                vertex_count + 4,
                                vertex_count + 5,
                                vertex_count + 7,
                                vertex_count + 4,
                                vertex_count + 7,
                                vertex_count + 6,
                                vertex_count + 4,
                                vertex_count + 7,
                                vertex_count + 5,
                                vertex_count + 4,
                                vertex_count + 6,
                                vertex_count + 7,
                            ]);
                        }
                        BlockModel::FlatDirectional {
                            texture_name,
                            north,
                            south,
                            east,
                            west,
                            up,
                            down,
                        } => {
                            let texture_index =
                                self.allocate_texture(texture_name) as f32;

                            let v = |pos, u, v, light_level| Vertex {
                                pos,
                                uv: Vec2 { x: u, y: v },
                                light_level,
                                texture_index,
                            };

                            let p = p.as_vec3();

                            if north {
                                let p = p + Vec3::Z * 0.0625;
                                let vertex_count = self.vertices.len() as u32;
                                self.vertices.extend([
                                    v(p, 0.0, 1.0, FRONT_BACK_LIGHT_LEVEL),
                                    v(
                                        p + Vec3::X,
                                        1.0,
                                        1.0,
                                        FRONT_BACK_LIGHT_LEVEL,
                                    ),
                                    v(
                                        p + Vec3::Y,
                                        0.0,
                                        0.0,
                                        FRONT_BACK_LIGHT_LEVEL,
                                    ),
                                    v(
                                        p + Vec3::X + Vec3::Y,
                                        1.0,
                                        0.0,
                                        FRONT_BACK_LIGHT_LEVEL,
                                    ),
                                ]);
                                self.indices.extend([
                                    vertex_count,
                                    vertex_count + 1,
                                    vertex_count + 2,
                                    vertex_count + 1,
                                    vertex_count + 3,
                                    vertex_count + 2,
                                ]);
                            }
                            if south {
                                let p = p + Vec3::Z * 0.9375;
                                let vertex_count = self.vertices.len() as u32;
                                self.vertices.extend([
                                    v(p, 0.0, 1.0, FRONT_BACK_LIGHT_LEVEL),
                                    v(
                                        p + Vec3::X,
                                        1.0,
                                        1.0,
                                        FRONT_BACK_LIGHT_LEVEL,
                                    ),
                                    v(
                                        p + Vec3::Y,
                                        0.0,
                                        0.0,
                                        FRONT_BACK_LIGHT_LEVEL,
                                    ),
                                    v(
                                        p + Vec3::X + Vec3::Y,
                                        1.0,
                                        0.0,
                                        FRONT_BACK_LIGHT_LEVEL,
                                    ),
                                ]);
                                self.indices.extend([
                                    vertex_count,
                                    vertex_count + 2,
                                    vertex_count + 1,
                                    vertex_count + 1,
                                    vertex_count + 2,
                                    vertex_count + 3,
                                ]);
                            }
                            if east {
                                let p = p + Vec3::X * 0.9375;
                                let vertex_count = self.vertices.len() as u32;
                                self.vertices.extend([
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
                                self.indices.extend([
                                    vertex_count,
                                    vertex_count + 1,
                                    vertex_count + 2,
                                    vertex_count + 1,
                                    vertex_count + 3,
                                    vertex_count + 2,
                                ]);
                            }
                            if west {
                                let p = p + Vec3::X * 0.0625;
                                let vertex_count = self.vertices.len() as u32;
                                self.vertices.extend([
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
                                self.indices.extend([
                                    vertex_count,
                                    vertex_count + 2,
                                    vertex_count + 1,
                                    vertex_count + 1,
                                    vertex_count + 2,
                                    vertex_count + 3,
                                ]);
                            }
                            if up {
                                let p = p + Vec3::Y * 0.9375;
                                let vertex_count = self.vertices.len() as u32;
                                self.vertices.extend([
                                    v(p, 0.0, 1.0, BOTTOM_LIGHT_LEVEL),
                                    v(
                                        p + Vec3::X,
                                        1.0,
                                        1.0,
                                        BOTTOM_LIGHT_LEVEL,
                                    ),
                                    v(
                                        p + Vec3::Z,
                                        0.0,
                                        0.0,
                                        BOTTOM_LIGHT_LEVEL,
                                    ),
                                    v(
                                        p + Vec3::X + Vec3::Z,
                                        1.0,
                                        0.0,
                                        TOP_LIGHT_LEVEL,
                                    ),
                                ]);
                                self.indices.extend([
                                    vertex_count,
                                    vertex_count + 1,
                                    vertex_count + 2,
                                    vertex_count + 1,
                                    vertex_count + 3,
                                    vertex_count + 2,
                                ]);
                            }
                            if down {
                                let p = p + Vec3::Y * 0.0625;
                                let vertex_count = self.vertices.len() as u32;
                                self.vertices.extend([
                                    v(p, 0.0, 1.0, TOP_LIGHT_LEVEL),
                                    v(p + Vec3::X, 1.0, 1.0, TOP_LIGHT_LEVEL),
                                    v(p + Vec3::Z, 0.0, 0.0, TOP_LIGHT_LEVEL),
                                    v(
                                        p + Vec3::X + Vec3::Z,
                                        1.0,
                                        0.0,
                                        TOP_LIGHT_LEVEL,
                                    ),
                                ]);
                                self.indices.extend([
                                    vertex_count,
                                    vertex_count + 2,
                                    vertex_count + 1,
                                    vertex_count + 1,
                                    vertex_count + 2,
                                    vertex_count + 3,
                                ]);
                            }
                        }
                    }
                }
            }
        }

        let light_at =
            |pos: IVec3| (f32::from(world.light_at(pos)) + 5.0) / 20.0;

        for vertex in &mut self.vertices {
            let pos = vertex.pos.as_ivec3();
            let x_lerped = Vec4::new(
                light_at(pos - IVec3::new(1, 1, 1)),
                light_at(pos - IVec3::new(1, 1, 0)),
                light_at(pos - IVec3::new(1, 0, 1)),
                light_at(pos - IVec3::new(1, 0, 0)),
            )
            .lerp(
                Vec4::new(
                    light_at(pos - IVec3::new(0, 1, 1)),
                    light_at(pos - IVec3::new(0, 1, 0)),
                    light_at(pos - IVec3::new(0, 0, 1)),
                    light_at(pos),
                ),
                (vertex.pos.x - 0.5).rem_euclid(1.0),
            );
            let z_lerped = x_lerped
                .xz()
                .lerp(x_lerped.yw(), (vertex.pos.z - 0.5).rem_euclid(1.0));
            vertex.light_level *= lerp(
                z_lerped.x,
                z_lerped.y,
                (vertex.pos.y - 0.5).rem_euclid(1.0),
            );
        }

        self
    }

    fn block_top_texture_id(&mut self, block: Intern<Block>) -> f32 {
        self.allocate_texture(block_top_texture_name(block)) as f32
    }

    fn block_front_side_texture_id(&mut self, block: Intern<Block>) -> f32 {
        self.allocate_texture(block_front_side_texture_name(block)) as f32
    }

    fn allocate_texture(&mut self, texture_name: &'static str) -> usize {
        self.texture_names.insert_full(texture_name).0
    }
}

fn is_solid(block: Intern<Block>) -> bool {
    BlockModel::of(block) == BlockModel::SolidBlock
}

fn block_texture_name(block: Intern<Block>) -> &'static str {
    let name = block.name.as_ref().strip_prefix("minecraft:").unwrap();
    match name {
        "infested_stone" => "stone",
        "infested_deepslate" => "deepslate",
        "snow_block" => "snow",
        "water" => "water_still",
        "lava" => "lava_still",
        _ => name,
    }
}

fn block_top_texture_name(block: Intern<Block>) -> &'static str {
    match &**block.name {
        "minecraft:podzol" => "podzol_top",
        "minecraft:grass_block" => "grass_block_top",
        _ => block_texture_name(block),
    }
}

fn block_front_side_texture_name(block: Intern<Block>) -> &'static str {
    match &**block.name {
        "minecraft:podzol" => "podzol_side",
        "minecraft:grass_block" => "grass_block_side",
        _ => block_texture_name(block),
    }
}

#[derive(Clone, Copy, PartialEq)]
enum BlockModel {
    None,
    SolidBlock,
    TransparentBlock,
    Cross(&'static str),
    FlatDirectional {
        texture_name: &'static str,
        north: bool,
        south: bool,
        east: bool,
        west: bool,
        up: bool,
        down: bool,
    },
}

impl BlockModel {
    fn of(block: Intern<Block>) -> Self {
        if block == *AIR {
            Self::None
        } else {
            match &**block.name {
                "minecraft:cave_air" => Self::None,
                "minecraft:spruce_leaves" => Self::TransparentBlock,
                "minecraft:grass"
                | "minecraft:fern"
                | "minecraft:dead_bush"
                | "minecraft:brown_mushroom"
                | "minecraft:red_mushroom"
                | "minecraft:small_dripleaf_stem_bottom"
                | "minecraft:small_dripleaf_stem_top"
                | "minecraft:big_dripleaf_stem"
                | "minecraft:cave_vines"
                | "minecraft:cave_vines_lit"
                | "minecraft:cave_vines_plant"
                | "minecraft:cave_vines_plant_lit" => {
                    Self::Cross(block_texture_name(block))
                }
                "minecraft:glow_lichen"
                | "minecraft:vine"
                | "minecraft:sculk_vein" => Self::FlatDirectional {
                    texture_name: block_texture_name(block),
                    north: block.properties.get("north").map(String::as_str)
                        == Some("true"),
                    south: block.properties.get("south").map(String::as_str)
                        == Some("true"),
                    east: block.properties.get("east").map(String::as_str)
                        == Some("true"),
                    west: block.properties.get("west").map(String::as_str)
                        == Some("true"),
                    up: block.properties.get("up").map(String::as_str)
                        == Some("true"),
                    down: block.properties.get("down").map(String::as_str)
                        == Some("true"),
                },
                _ => Self::SolidBlock,
            }
        }
    }
}

fn lerp(from: f32, to: f32, amount: f32) -> f32 {
    to * amount + from * (1.0 - amount)
}

#[repr(C)]
pub struct Vertex {
    pub pos: Vec3,
    pub uv: Vec2,
    pub light_level: f32,
    pub texture_index: f32,
}
