mod chunk;
mod mesh;
mod region;
mod render;
mod shader;
mod world;

use glam::{IVec2, Vec3};
use std::{
    ops::Range,
    path::{Path, PathBuf},
};
use world::World;

pub struct Options {
    pub resource_pack_path: PathBuf,
    pub camera_position: Vec3,
    pub camera_pitch: f32,
    pub camera_yaw: f32,
    pub vfov: f32,
    pub area: Range<IVec2>,
}

fn main() {
    let mut args = std::env::args().skip(1);
    let world_path = args.next().unwrap_or_else(|| "world".to_owned());
    let resource_pack_path = PathBuf::from(
        args.next().unwrap_or_else(|| "resource-pack".to_owned()),
    );
    let area = match args.next() {
        None => IVec2::new(0, 0)..IVec2::new(32, 16),
        Some(s) => {
            let [x1, z1, x2, z2]: [i32; 4] = s
                .split(',')
                .map(|coord| coord.parse().unwrap())
                .collect::<Vec<_>>()
                .try_into()
                .unwrap();
            IVec2::new(x1, z1)..IVec2::new(x2, z2)
        }
    };

    let options = Options {
        resource_pack_path,
        camera_position: Vec3 {
            x: 0.0,
            y: 0.0,
            z: -5.0,
        },
        camera_pitch: 0.0,
        camera_yaw: 0.0,
        vfov: 1.0,
        area,
    };

    let world = World::new(Path::new(&world_path), &options);

    render::render(world, options);
}
