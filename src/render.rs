use crate::{
    shader::{self, Uniforms},
    world::World,
    Options,
};
use glam::{Mat3, Mat4, Vec2, Vec3};
use miniquad::{
    conf::Conf, Bindings, Buffer, BufferLayout, BufferType, Context,
    EventHandler, FilterMode, KeyCode, PassAction, Pipeline, Shader, Texture,
    VertexAttribute, VertexFormat,
};

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
    #[allow(dead_code)]
    uv: Vec2,
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
            let v = |x, y, z, u, v| Vertex {
                pos: Vec3 { x, y, z },
                uv: Vec2 { x: u, y: v },
            };
            [
                v(-1.0, -1.0, 1.0, 0.0, 0.0),
                v(1.0, -1.0, 1.0, 1.0, 0.0),
                v(1.0, 1.0, 1.0, 1.0, 1.0),
                v(-1.0, 1.0, 1.0, 0.0, 1.0),
            ]
        };
        let vertex_buffer =
            Buffer::immutable(ctx, BufferType::VertexBuffer, &vertices);

        let indices = [0, 1, 2, 0, 2, 3];
        let index_buffer =
            Buffer::immutable(ctx, BufferType::IndexBuffer, &indices);

        let pixels: &[u32] = &[
            0xffffffff, 0xff0000ff, 0xffffffff, 0xff0000ff, 0xff0000ff,
            0xffffffff, 0xff0000ff, 0xffffffff, 0xffffffff, 0xff0000ff,
            0xffffffff, 0xff0000ff, 0xff0000ff, 0xffffffff, 0xff0000ff,
            0xffffffff,
        ];
        let texture =
            Texture::from_rgba8(ctx, 4, 4, bytemuck::cast_slice(pixels));
        texture.set_filter(ctx, FilterMode::Nearest);

        let bindings = Bindings {
            vertex_buffers: vec![vertex_buffer],
            index_buffer,
            images: vec![texture],
        };

        let shader =
            Shader::new(ctx, shader::VERTEX, shader::FRAGMENT, shader::meta())
                .unwrap();

        let pipeline = Pipeline::new(
            ctx,
            &[BufferLayout::default()],
            &[
                VertexAttribute::new("pos", VertexFormat::Float3),
                VertexAttribute::new("uv", VertexFormat::Float2),
            ],
            shader,
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
        self.camera_pitch = self
            .camera_pitch
            .clamp(-std::f32::consts::FRAC_PI_2, std::f32::consts::FRAC_PI_2);

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
        ctx.draw(0, 6, 1);
        ctx.end_render_pass();
        ctx.commit_frame();
    }
}
