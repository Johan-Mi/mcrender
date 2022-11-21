use crate::{chunk::Chunk, Options};
use glam::IVec2;
use std::{
    collections::HashMap,
    ffi::OsStr,
    fs::{self, DirEntry},
    ops::Range,
    path::Path,
};

pub struct Region {
    pub chunks: [[Option<Chunk>; 32]; 32],
}

impl Region {
    pub fn load_all(
        world_path: &Path,
        options: &Options,
    ) -> HashMap<IVec2, Self> {
        fs::read_dir(world_path.join("region"))
            .unwrap()
            .map(Result::unwrap)
            .map(|entry| {
                let location = Self::parse_file_name(&entry.file_name());
                (entry, location)
            })
            .filter(|(_entry, location)| {
                (options.area.start.x.rem_euclid(512)
                    ..options.area.end.x.rem_euclid(512))
                    .contains(&location.x)
                    && (options.area.start.y.rem_euclid(512)
                        ..options.area.end.y.rem_euclid(512))
                        .contains(&location.y)
            })
            .map(|(entry, location)| {
                Self::load(entry, options.area.clone(), location)
            })
            .collect()
    }

    fn parse_file_name(file_name: &OsStr) -> IVec2 {
        let mut coordinates = file_name
            .to_str()
            .unwrap()
            .split('.')
            .skip(1)
            .map(str::parse)
            .map(Result::unwrap);
        IVec2 {
            x: coordinates.next().unwrap(),
            y: coordinates.next().unwrap(),
        }
    }

    fn load(
        entry: DirEntry,
        area: Range<IVec2>,
        location: IVec2,
    ) -> (IVec2, Self) {
        let file = fs::read(entry.path()).unwrap();

        // This should really use `array::from_fn`, but that would overflow
        // the stack
        let mut chunks: [[Option<Chunk>; 32]; 32] = Default::default();
        for (chunk_z, column) in chunks.iter_mut().enumerate() {
            for (chunk_x, chunk) in column.iter_mut().enumerate() {
                *chunk = Chunk::load(
                    &file,
                    area.clone(),
                    chunk_x as i32 + location.x * 32,
                    chunk_z as i32 + location.y * 32,
                );
            }
        }

        (location, Self { chunks })
    }
}
