use crate::{chunk::Block, region::Region};
use glam::{IVec2, IVec3, Vec3, Vec3Swizzles};
use internment::Intern;
use std::{collections::HashMap, path::Path, time::Duration};

pub struct Renderer {
    regions: HashMap<IVec2, Region>,
}

impl Renderer {
    pub fn new(world_path: &Path) -> Self {
        Self {
            regions: Region::load_all(world_path),
        }
    }

    pub fn render(&self, options: &RenderOptions) {
        for z in 0..16 {
            print!("\x1b[2J\x1b[H");
            for y in (65..100).rev() {
                for x in 0..32 {
                    let block = self.block_at(IVec3 { x, y, z });
                    print!(
                        "{}\x1b[0m",
                        Self::block_appearance(&block).unwrap_or_else(|| &block.name[10..])
                    );
                }
                println!();
            }
            std::thread::sleep(Duration::from_millis(200));
        }
    }

    fn block_appearance(block: &Block) -> Option<&'static str> {
        Some(match &*block.name {
            "minecraft:air" => "  ",
            "minecraft:stone" => "\x1b[48;5;242m  ",
            "minecraft:dirt" => "\x1b[48;5;94m  ",
            "minecraft:coal_ore" => "\x1b[48;5;242m\x1b[38;5;0m::",
            "minecraft:copper_ore" => "\x1b[38;5;242m\x1b[48;5;35m::",
            "minecraft:iron_ore" => "\x1b[38;5;242m\x1b[48;5;209m::",
            "minecraft:lapis_ore" => "\x1b[48;5;242m\x1b[38;5;21m@@",
            "minecraft:diorite" => "\x1b[38;5;249m\x1b[48;5;254m##",
            "minecraft:granite" => "\x1b[38;5;240\x1b[48;5;238m##",
            "minecraft:grass_block" => "\x1b[38;5;34m\x1b[48;5;94m▀▀",
            "minecraft:podzol" => "\x1b[38;5;88m\x1b[48;5;94m▀▀",
            "minecraft:mossy_cobblestone" => "\x1b[48;5;242m\x1b[38;5;28m##",
            "minecraft:gravel" => "\x1b[48;5;252m\x1b[38;5;240m%%",
            "minecraft:spruce_leaves" => "\x1b[38;5;34m░░",
            "minecraft:fern" => "\x1b[38;5;40mww",
            "minecraft:large_fern" => "\x1b[38;5;40mWW",
            "minecraft:brown_mushroom" => "\x1b[38;5;130mmm",
            "minecraft:grass" => "\x1b[38;5;46mww",
            "minecraft:spruce_log" => "\x1b[48;5;58m\x1b[38;5;52m║║",
            "minecraft:sweet_berry_bush" => "\x1b[48;5;40m\x1b[38;5;196m::",
            _ => return None,
        })
    }

    fn block_at(&self, pos: IVec3) -> Intern<Block> {
        let region = &self.regions[&(pos.xz() >> 5)];
        let chunk = region.chunks[(pos.z >> 4).rem_euclid(32) as usize]
            [(pos.x >> 4).rem_euclid(32) as usize]
            .as_ref()
            .unwrap();
        let section = &chunk.sections[(pos.y + 64) as usize / 16];
        let offset_within_section =
            pos.y.rem_euclid(16) * 256 + pos.z.rem_euclid(16) * 16 + pos.x.rem_euclid(16);
        section.block_states.palette
            [section.block_states.data[offset_within_section as usize] as usize]
    }
}

pub struct RenderOptions {
    pub camera_position: Vec3,
    pub camera_target: Vec3,
}
