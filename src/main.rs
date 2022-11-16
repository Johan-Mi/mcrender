mod chunk;
mod region;
mod render;

use glam::{IVec2, Vec3};
use render::{RenderOptions, Renderer};
use std::path::Path;

fn main() {
    let Some(world_path) = std::env::args().nth(1) else {
        eprintln!("Error: no world path provided");
        return;
    };
    let renderer = Renderer::new(
        Path::new(&world_path),
        RenderOptions {
            camera_position: Vec3 {
                x: 0.0,
                y: 50.0,
                z: -50.0,
            },
            camera_target: Vec3 {
                x: 0.0,
                y: 3.0,
                z: 0.0,
            },
            area: IVec2::new(0, 0)..IVec2::new(32, 16),
        },
    );
    renderer.render();
}
