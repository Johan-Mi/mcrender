use crate::chunk::Chunk;
use glam::IVec2;
use std::{
    collections::HashMap,
    fs::{self, DirEntry},
    path::Path,
};

pub struct Region {
    pub chunks: [[Option<Chunk>; 32]; 32],
}

impl Region {
    pub fn load_all(world_path: &Path) -> HashMap<IVec2, Self> {
        fs::read_dir(world_path.join("region"))
            .unwrap()
            .map(Result::unwrap)
            // Only load one region to keep things fast
            // TODO: let the user specify a specific part of the world to render
            .filter(|entry| entry.file_name() == "r.0.0.mca")
            .map(Self::load)
            .collect()
    }

    fn load(entry: DirEntry) -> (IVec2, Self) {
        let file_name = entry.file_name();
        let mut coordinates = file_name
            .as_os_str()
            .to_str()
            .unwrap()
            .split('.')
            .skip(1)
            .map(str::parse)
            .map(Result::unwrap);
        let coordinates = IVec2 {
            x: coordinates.next().unwrap(),
            y: coordinates.next().unwrap(),
        };
        let file = fs::read(entry.path()).unwrap();

        // This should really use `array::from_fn`, but that would overflow
        // the stack
        let mut chunks: [[Option<Chunk>; 32]; 32] = Default::default();
        for (chunk_z, column) in chunks.iter_mut().enumerate() {
            for (chunk_x, chunk) in column.iter_mut().enumerate() {
                *chunk = Chunk::load(&file, chunk_x, chunk_z);
            }
        }

        (coordinates, Self { chunks })
    }
}
