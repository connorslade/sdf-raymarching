use std::{borrow::Cow, mem, time::Instant};

use anyhow::Result;

use egui_snarl::{
    Snarl,
    ui::{SnarlStyle, SnarlWidget},
};
use encase::ShaderType;
use tufa::{
    bindings::buffer::UniformBuffer,
    export::{
        egui::{Context, Frame, Key, Pos2, Slider, Window},
        nalgebra::{Vector2, Vector3},
        wgpu::{PowerPreference, RenderPass, ShaderModuleDescriptor, ShaderSource, ShaderStages},
        winit::{
            event::{DeviceEvent, DeviceId},
            window::{CursorGrabMode, WindowAttributes},
        },
    },
    gpu::Gpu,
    interactive::{GraphicsCtx, Interactive},
    misc::camera::PerspectiveCamera,
    pipeline::render::RenderPipeline,
};

use crate::nodes::{Node, NodeViewer};
mod nodes;

struct App {
    pipeline: RenderPipeline,
    uniform: UniformBuffer<Uniform>,

    camera: PerspectiveCamera,
    snarl: Snarl<Node>,
    cursor_locked: bool,

    startup: Instant,
    last_frame: Instant,
    steps: u32,
}

#[derive(ShaderType)]
struct Uniform {
    window: Vector2<u32>,
    steps: u32,

    camera: Camera,
    t: f32,
}

#[derive(ShaderType)]
struct Camera {
    pos: Vector3<f32>,
    pitch: f32,
    yaw: f32,

    fov: f32,
    aspect: f32,
}

fn main() -> Result<()> {
    let gpu = Gpu::builder()
        .power_preference(PowerPreference::HighPerformance)
        .build()?;

    let uniform = gpu.create_uniform(&unsafe { mem::zeroed() });
    let pipeline = gpu
        .render_pipeline(ShaderModuleDescriptor {
            label: None,
            source: ShaderSource::Wgsl(Cow::Borrowed(concat!(
                include_str!("../shaders/main.wgsl"),
                include_str!("../shaders/sdf.wgsl"),
                include_str!("../shaders/types.wgsl"),
                include_str!("../shaders/vertex.wgsl")
            ))),
        })
        .bind(&uniform, ShaderStages::FRAGMENT)
        .finish();

    let window = gpu.create_window(
        WindowAttributes::default().with_title("SDF Raymarching"),
        App {
            pipeline,
            uniform,

            camera: PerspectiveCamera::default(),
            snarl: Snarl::new(),
            cursor_locked: true,

            startup: Instant::now(),
            last_frame: Instant::now(),
            steps: 100,
        },
    );

    window.run()?;
    Ok(())
}

impl Interactive for App {
    fn init(&mut self, _gcx: GraphicsCtx) {
        // self.snarl.insert_node(Pos2::new(0.0, 0.0), Node::Output);
    }

    fn render(&mut self, gcx: GraphicsCtx, render_pass: &mut RenderPass) {
        let grab_mode = [CursorGrabMode::None, CursorGrabMode::Locked][self.cursor_locked as usize];
        gcx.window.set_cursor_grab(grab_mode).unwrap();
        gcx.window.set_cursor_visible(!self.cursor_locked);

        let size = gcx.window.inner_size();
        self.uniform.upload(&Uniform {
            window: Vector2::new(size.width, size.height),
            steps: self.steps,

            t: self.startup.elapsed().as_secs_f32(),
            camera: Camera {
                pos: self.camera.position,
                pitch: self.camera.pitch,
                yaw: self.camera.yaw,
                fov: self.camera.fov,
                aspect: size.width as f32 / size.height as f32,
            },
        });

        self.pipeline.draw_quad(render_pass, 0..1);
    }

    fn device_event(&mut self, _gcx: GraphicsCtx, _device_id: DeviceId, event: &DeviceEvent) {
        self.cursor_locked.then(|| self.camera.device_event(event));
    }

    fn ui(&mut self, _gcx: GraphicsCtx, ctx: &Context) {
        ctx.input(|i| self.cursor_locked ^= i.key_pressed(Key::Escape));
        self.camera.update(ctx);

        Window::new("SDF Raymarching").show(ctx, |ui| {
            let fps = self.last_frame.elapsed().as_secs_f32().recip();
            ui.label(format!("FPS: {fps:.1}",));
            ui.add(Slider::new(&mut self.steps, 0..=1000));
            self.camera.ui(ui, "Camera");

            let mut style = SnarlStyle::new();
            style.wire_smoothness = Some(0.1);
            style.wire_width = Some(3.0);

            SnarlWidget::new()
                .style(style)
                .show(&mut self.snarl, &mut NodeViewer, ui);
        });

        self.last_frame = Instant::now();
    }
}
