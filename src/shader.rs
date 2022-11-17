use glam::Mat4;
use miniquad::{ShaderMeta, UniformBlockLayout, UniformDesc};

pub const VERTEX: &str = include_str!("shader/shader.vert");
pub const FRAGMENT: &str = include_str!("shader/shader.frag");

#[repr(C)]
pub struct Uniforms {
    pub mvp: Mat4,
}

pub fn meta() -> ShaderMeta {
    ShaderMeta {
        uniforms: UniformBlockLayout {
            uniforms: vec![UniformDesc::new(
                "view",
                miniquad::UniformType::Mat4,
            )],
        },
        images: vec!["tex".to_owned()],
    }
}
