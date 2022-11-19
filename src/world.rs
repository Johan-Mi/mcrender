use crate::{chunk::Block, region::Region, Options};
use glam::{IVec2, IVec3, Vec3Swizzles};
use internment::Intern;
use std::{collections::HashMap, path::Path};

pub struct World {
    regions: HashMap<IVec2, Region>,
}

impl World {
    pub fn new(world_path: &Path, options: &Options) -> Self {
        Self {
            regions: Region::load_all(world_path, options),
        }
    }

    pub fn block_at(&self, pos: IVec3) -> Option<Intern<Block>> {
        let region = &self.regions.get(&(pos.xz() >> 9))?;
        let chunk = region.chunks[(pos.z >> 4).rem_euclid(32) as usize]
            [(pos.x >> 4).rem_euclid(32) as usize]
            .as_ref()?;
        let section = &chunk.sections[(pos.y + 64) as usize / 16];
        let offset_within_section = pos.y.rem_euclid(16) * 256
            + pos.z.rem_euclid(16) * 16
            + pos.x.rem_euclid(16);
        Some(
            section.block_states.palette[*section
                .block_states
                .data
                .get(offset_within_section as usize)?
                as usize],
        )
    }
}
