use crate::{
    shader::{self, Uniforms},
    world::World,
    Options,
};
use glam::{Mat3, Mat4, Vec3};
use miniquad::{
    conf::Conf, Bindings, Buffer, BufferLayout, BufferType, Context,
    EventHandler, FilterMode, KeyCode, PassAction, Pipeline, PipelineParams,
    Shader, Texture, TextureFormat, TextureParams, VertexAttribute,
    VertexFormat,
};
use std::{fs::File, path::Path};

const MOVE_SPEED: f32 = 0.2;
const FLY_SPEED: f32 = 0.2;
const TURN_SPEED: f32 = 0.04;

pub fn render(world: &World, options: Options) {
    miniquad::start(Conf::default(), |ctx| {
        Box::new(Renderer::new(options, ctx))
    });
}

struct Vertex {
    #[allow(dead_code)]
    pos: Vec3,
}

struct Renderer {
    pipeline: Pipeline,
    bindings: Bindings,
    camera_position: Vec3,
    camera_pitch: f32,
    camera_yaw: f32,
    vfov: f32,
    key_w: bool,
    key_a: bool,
    key_s: bool,
    key_d: bool,
    key_h: bool,
    key_j: bool,
    key_k: bool,
    key_l: bool,
    key_space: bool,
    key_shift: bool,
}

impl Renderer {
    fn new(options: Options, ctx: &mut Context) -> Self {
        let vertices = {
            let v = |x, y, z| Vertex {
                pos: Vec3 { x, y, z },
            };
            [
                v(0.0, 0.0, 0.0),
                v(0.0, 0.0, 1.0),
                v(0.0, 1.0, 0.0),
                v(0.0, 1.0, 1.0),
                v(1.0, 0.0, 0.0),
                v(1.0, 0.0, 1.0),
                v(1.0, 1.0, 0.0),
                v(1.0, 1.0, 1.0),
            ]
        };
        let vertex_buffer =
            Buffer::immutable(ctx, BufferType::VertexBuffer, &vertices);

        let indices = [
            0, 2, 4, 2, 6, 4, 1, 5, 3, 3, 5, 7, 0, 1, 2, 2, 1, 3, 4, 6, 5, 6,
            7, 5, 0, 4, 1, 4, 5, 1, 2, 3, 6, 6, 3, 7,
        ];
        let index_buffer =
            Buffer::immutable(ctx, BufferType::IndexBuffer, &indices);

        let (pixels, width, height, format) = read_png_rgb(
            &options
                .resource_pack_path
                .join("assets/minecraft/textures/block/grass_block_side.png"),
        );
        let texture = Texture::from_data_and_format(
            ctx,
            &pixels,
            TextureParams {
                format,
                wrap: miniquad::TextureWrap::Repeat,
                filter: FilterMode::Nearest,
                width,
                height,
            },
        );

        let bindings = Bindings {
            vertex_buffers: vec![vertex_buffer],
            index_buffer,
            images: vec![texture],
        };

        let shader =
            Shader::new(ctx, shader::VERTEX, shader::FRAGMENT, shader::meta())
                .unwrap();

        let pipeline = Pipeline::with_params(
            ctx,
            &[BufferLayout::default()],
            &[VertexAttribute::new("pos", VertexFormat::Float3)],
            shader,
            PipelineParams {
                cull_face: miniquad::CullFace::Back,
                ..Default::default()
            },
        );

        Self {
            pipeline,
            bindings,
            camera_position: options.camera_position,
            camera_pitch: 0.0,
            camera_yaw: 0.0,
            vfov: options.vfov,
            key_w: false,
            key_a: false,
            key_s: false,
            key_d: false,
            key_h: false,
            key_j: false,
            key_k: false,
            key_l: false,
            key_space: false,
            key_shift: false,
        }
    }

    fn rotation_matrix(&self) -> Mat3 {
        Mat3::from_euler(
            glam::EulerRot::ZYX,
            0.0,
            self.camera_yaw,
            self.camera_pitch,
        )
    }
}

impl EventHandler for Renderer {
    fn update(&mut self, _ctx: &mut miniquad::Context) {
        if self.key_h {
            self.camera_yaw += TURN_SPEED;
        }
        if self.key_l {
            self.camera_yaw -= TURN_SPEED;
        }
        if self.key_j {
            self.camera_pitch += TURN_SPEED;
        }
        if self.key_k {
            self.camera_pitch -= TURN_SPEED;
        }
        self.camera_pitch = self.camera_pitch.clamp(
            -std::f32::consts::FRAC_PI_2 + 1e-5,
            std::f32::consts::FRAC_PI_2 - 1e-5,
        );

        if self.key_space {
            self.camera_position.y += FLY_SPEED;
        }
        if self.key_shift {
            self.camera_position.y -= FLY_SPEED;
        }

        let f32_from_bool = |b| if b { 1.0 } else { 0.0 };
        self.camera_position += Mat3::from_rotation_y(self.camera_yaw)
            * Vec3 {
                x: f32_from_bool(self.key_a) - f32_from_bool(self.key_d),
                y: 0.0,
                z: f32_from_bool(self.key_w) - f32_from_bool(self.key_s),
            }
            * MOVE_SPEED;
    }

    fn key_down_event(
        &mut self,
        _ctx: &mut Context,
        keycode: KeyCode,
        _keymods: miniquad::KeyMods,
        _repeat: bool,
    ) {
        match keycode {
            KeyCode::W => self.key_w = true,
            KeyCode::A => self.key_a = true,
            KeyCode::S => self.key_s = true,
            KeyCode::D => self.key_d = true,
            KeyCode::H => self.key_h = true,
            KeyCode::J => self.key_j = true,
            KeyCode::K => self.key_k = true,
            KeyCode::L => self.key_l = true,
            KeyCode::Space => self.key_space = true,
            KeyCode::LeftShift => self.key_shift = true,
            _ => {}
        }
    }

    fn key_up_event(
        &mut self,
        _ctx: &mut Context,
        keycode: KeyCode,
        _keymods: miniquad::KeyMods,
    ) {
        match keycode {
            KeyCode::W => self.key_w = false,
            KeyCode::A => self.key_a = false,
            KeyCode::S => self.key_s = false,
            KeyCode::D => self.key_d = false,
            KeyCode::H => self.key_h = false,
            KeyCode::J => self.key_j = false,
            KeyCode::K => self.key_k = false,
            KeyCode::L => self.key_l = false,
            KeyCode::Space => self.key_space = false,
            KeyCode::LeftShift => self.key_shift = false,
            _ => {}
        }
    }

    fn draw(&mut self, ctx: &mut miniquad::Context) {
        let (width, height) = ctx.screen_size();
        let proj =
            Mat4::perspective_rh_gl(self.vfov, width / height, 0.01, f32::MAX);
        let view = Mat4::look_to_rh(
            self.camera_position,
            self.rotation_matrix() * Vec3::Z,
            Vec3::Y,
        );
        let view = proj * view;

        let vs_params = Uniforms { mvp: view };

        ctx.begin_default_pass(PassAction::clear_color(0.5, 0.7, 1.0, 1.0));
        ctx.apply_pipeline(&self.pipeline);
        ctx.apply_bindings(&self.bindings);
        ctx.apply_uniforms(&vs_params);
        ctx.draw(0, 36, 1);
        ctx.end_render_pass();
        ctx.commit_frame();
    }
}

fn read_png_rgb(path: &Path) -> (Box<[u8]>, u32, u32, TextureFormat) {
    let raster = png_pong::Decoder::new(File::open(path).unwrap())
        .unwrap()
        .into_steps()
        .next()
        .unwrap()
        .unwrap()
        .raster;
    match raster {
        png_pong::PngRaster::Rgb8(raster) => {
            let width = raster.width();
            let height = raster.height();
            (raster.into(), width, height, TextureFormat::RGB8)
        }
        png_pong::PngRaster::Rgba8(raster) => {
            let width = raster.width();
            let height = raster.height();
            (raster.into(), width, height, TextureFormat::RGBA8)
        }
        _ => todo!(),
    }
}
