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
    let world_path = std::env::args()
        .nth(1)
        .unwrap_or_else(|| "world".to_owned());
    let resource_pack_path = PathBuf::from(
        std::env::args()
            .nth(2)
            .unwrap_or_else(|| "resource-pack".to_owned()),
    );

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
        area: IVec2::new(0, 0)..IVec2::new(32, 16),
    };

    let world = World::new(Path::new(&world_path), &options);

    render::render(world, options);
}
