use std::{borrow::Cow, mem, time::Instant};

use anyhow::Result;
use encase::ShaderType;
use tufa::{
    bindings::buffer::UniformBuffer,
    export::{
        egui::{Context, Window},
        nalgebra::{Vector2, Vector3},
        wgpu::{PowerPreference, RenderPass, ShaderModuleDescriptor, ShaderSource, ShaderStages},
        winit::{
            event::{DeviceEvent, DeviceId},
            window::WindowAttributes,
        },
    },
    gpu::Gpu,
    interactive::{GraphicsCtx, Interactive},
    misc::camera::PerspectiveCamera,
    pipeline::render::RenderPipeline,
};

struct App {
    pipeline: RenderPipeline,
    uniform: UniformBuffer<Uniform>,

    camera: PerspectiveCamera,
    startup: Instant,
    last_frame: Instant,
}

#[derive(ShaderType)]
struct Uniform {
    window: Vector2<u32>,
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
            startup: Instant::now(),
            last_frame: Instant::now(),
        },
    );

    window.run()?;
    Ok(())
}

impl Interactive for App {
    fn render(&mut self, gcx: GraphicsCtx, render_pass: &mut RenderPass) {
        let size = gcx.window.inner_size();
        self.uniform.upload(&Uniform {
            window: Vector2::new(size.width, size.height),

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
        self.camera.device_event(event);
    }

    fn ui(&mut self, _gcx: GraphicsCtx, ctx: &Context) {
        self.camera.update(ctx);

        Window::new("SDF Raymarching").show(ctx, |ui| {
            let fps = self.last_frame.elapsed().as_secs_f32().recip();
            ui.label(format!("FPS: {fps:.1}",));
            self.camera.ui(ui, "Camera");
        });

        self.last_frame = Instant::now();
    }
}
