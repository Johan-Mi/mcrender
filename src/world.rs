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
        let section =
            chunk.sections.get(usize::try_from(pos.y + 64).ok()? / 16)?;
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

    pub fn light_at(&self, pos: IVec3) -> u8 {
        (|| {
            let region = &self.regions.get(&(pos.xz() >> 9))?;
            let chunk = region.chunks[(pos.z >> 4).rem_euclid(32) as usize]
                [(pos.x >> 4).rem_euclid(32) as usize]
                .as_ref()?;
            let section =
                chunk.sections.get(usize::try_from(pos.y + 64).ok()? / 16)?;
            let offset_within_lightmap = (pos.y.rem_euclid(16) * 256
                + pos.z.rem_euclid(16) * 16
                + pos.x.rem_euclid(16))
                as usize;
            let block_light_byte = *section
                .block_light
                .get(offset_within_lightmap >> 1)
                .unwrap_or(&0) as u8;
            let block_light = if offset_within_lightmap % 2 == 0 {
                block_light_byte & 0xf
            } else {
                block_light_byte >> 4
            };
            let sky_light_byte = *section
                .sky_light
                .get(offset_within_lightmap >> 1)
                .unwrap_or(&0) as u8;
            let sky_light = if offset_within_lightmap % 2 == 0 {
                sky_light_byte & 0xf
            } else {
                sky_light_byte >> 4
            };
            Some(block_light + sky_light)
        })()
        .unwrap_or(0)
    }
}
