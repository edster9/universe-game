//! How many triangles a second can this machine draw? A window fills with
//! small triangles, more each round, with no waiting for the screen's
//! refresh, and the frame rate is measured at each load. Built once for
//! Linux (run inside WSL) and once for Windows (run natively), from this one
//! source, to compare the two. See docs/ideas/the-client.md.
//!
//! `gpu-bench` measures triangles a second, in one big draw; `gpu-bench
//! calls` measures draw calls a second, many small objects each drawn on
//! its own, which is how a game's frame is made.

use std::sync::Arc;
use std::time::{Duration, Instant};

use winit::application::ApplicationHandler;
use winit::event::WindowEvent;
use winit::event_loop::{ActiveEventLoop, EventLoop};
use winit::window::{Window, WindowId};

/// Triangles in the mesh drawn once per instance.
const MESH_TRIANGLES: u32 = 500_000;
/// Instances per frame at each round: 0.5 million triangles up to 64 million.
const ROUNDS: [u32; 8] = [1, 2, 4, 8, 16, 32, 64, 128];
/// With `calls`: separate draw calls per frame at each round, each a small
/// object of `CALL_TRIANGLES`, as a game draws many things.
const CALL_ROUNDS: [u32; 8] = [10, 100, 500, 1_000, 5_000, 10_000, 50_000, 100_000];
const CALL_TRIANGLES: u32 = 50;
/// How long each round is measured, after a short warm-up.
const ROUND: Duration = Duration::from_secs(3);
const WARM_UP: Duration = Duration::from_millis(500);

const SHADER: &str = r#"
struct Out {
    @builtin(position) position: vec4<f32>,
    @location(0) colour: vec3<f32>,
};

@vertex
fn vs(@location(0) at: vec2<f32>, @builtin(instance_index) instance: u32) -> Out {
    // Each instance is nudged a little, so copies don't sit exactly on top
    // of each other.
    let shift = vec2<f32>(f32(instance % 16u), f32(instance / 16u)) * 0.002;
    var out: Out;
    out.position = vec4<f32>(at + shift, 0.0, 1.0);
    out.colour = vec3<f32>(0.2 + 0.05 * f32(instance % 8u), 0.6, 0.9);
    return out;
}

@fragment
fn fs(in: Out) -> @location(0) vec4<f32> {
    return vec4<f32>(in.colour, 1.0);
}
"#;

/// A grid of small triangles covering most of the window, in clip space.
fn mesh() -> Vec<[f32; 2]> {
    let side = ((MESH_TRIANGLES / 2) as f32).sqrt().ceil() as u32;
    let cell = 1.8 / side as f32;
    let size = cell * 0.6;
    let mut vertices = Vec::with_capacity(MESH_TRIANGLES as usize * 3);
    'grid: for row in 0..side {
        for col in 0..side {
            let (x, y) = (-0.9 + col as f32 * cell, -0.9 + row as f32 * cell);
            for triangle in [
                [[x, y], [x + size, y], [x, y + size]],
                [[x + size, y], [x + size, y + size], [x, y + size]],
            ] {
                if vertices.len() as u32 >= MESH_TRIANGLES * 3 {
                    break 'grid;
                }
                vertices.extend(triangle);
            }
        }
    }
    vertices
}

struct Gpu {
    window: Arc<Window>,
    surface: wgpu::Surface<'static>,
    device: wgpu::Device,
    queue: wgpu::Queue,
    config: wgpu::SurfaceConfiguration,
    pipeline: wgpu::RenderPipeline,
    vertices: wgpu::Buffer,
    vertex_count: u32,
    adapter: String,
}

/// One round's measurement.
struct Result {
    triangles_per_frame: u64,
    frames: u64,
    seconds: f64,
}

struct Bench {
    /// Measuring draw calls rather than triangles.
    calls: bool,
    gpu: Option<Gpu>,
    round: usize,
    round_started: Option<Instant>,
    frames: u64,
    results: Vec<Result>,
}

impl Gpu {
    fn new(event_loop: &ActiveEventLoop) -> Gpu {
        let window = Arc::new(
            event_loop
                .create_window(
                    Window::default_attributes()
                        .with_title("gpu-bench")
                        .with_inner_size(winit::dpi::PhysicalSize::new(1280, 720)),
                )
                .expect("a window"),
        );
        // WGPU_BACKEND=gl, dx12, vulkan, ... chooses; otherwise the best there is.
        let instance =
            wgpu::Instance::new(wgpu::InstanceDescriptor::new_with_display_handle_from_env(
                Box::new(event_loop.owned_display_handle()),
            ));
        let surface = instance.create_surface(window.clone()).expect("a surface");
        let adapter = pollster::block_on(instance.request_adapter(&wgpu::RequestAdapterOptions {
            power_preference: wgpu::PowerPreference::HighPerformance,
            force_fallback_adapter: false,
            compatible_surface: Some(&surface),
            // Report the adapter's real limits, not rounded-down tiers.
            apply_limit_buckets: false,
        }))
        .expect("a graphics adapter");
        let info = adapter.get_info();
        let adapter_name = format!("{} ({:?}, {})", info.name, info.backend, info.driver);
        let (device, queue) = pollster::block_on(adapter.request_device(&wgpu::DeviceDescriptor {
            label: None,
            required_features: wgpu::Features::empty(),
            required_limits: adapter.limits(),
            experimental_features: wgpu::ExperimentalFeatures::default(),
            memory_hints: wgpu::MemoryHints::Performance,
            trace: wgpu::Trace::Off,
        }))
        .expect("a device");

        let size = window.inner_size();
        let mut config = surface
            .get_default_config(&adapter, size.width.max(1), size.height.max(1))
            .expect("a surface configuration");
        // Don't wait for the screen: draw as fast as the GPU can.
        config.present_mode = wgpu::PresentMode::AutoNoVsync;
        surface.configure(&device, &config);

        let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: None,
            source: wgpu::ShaderSource::Wgsl(SHADER.into()),
        });
        let layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: None,
            bind_group_layouts: &[],
            immediate_size: 0,
        });
        let pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: None,
            layout: Some(&layout),
            vertex: wgpu::VertexState {
                module: &shader,
                entry_point: Some("vs"),
                compilation_options: Default::default(),
                buffers: &[Some(wgpu::VertexBufferLayout {
                    array_stride: 8,
                    step_mode: wgpu::VertexStepMode::Vertex,
                    attributes: &wgpu::vertex_attr_array![0 => Float32x2],
                })],
            },
            primitive: wgpu::PrimitiveState::default(),
            depth_stencil: None,
            multisample: wgpu::MultisampleState::default(),
            fragment: Some(wgpu::FragmentState {
                module: &shader,
                entry_point: Some("fs"),
                compilation_options: Default::default(),
                targets: &[Some(config.format.into())],
            }),
            multiview_mask: None,
            cache: None,
        });
        let data = mesh();
        let vertices = wgpu::util::DeviceExt::create_buffer_init(
            &device,
            &wgpu::util::BufferInitDescriptor {
                label: None,
                contents: bytemuck::cast_slice(&data),
                usage: wgpu::BufferUsages::VERTEX,
            },
        );
        Gpu {
            window,
            surface,
            device,
            queue,
            config,
            pipeline,
            vertices,
            vertex_count: data.len() as u32,
            adapter: adapter_name,
        }
    }

    fn draw(&mut self, instances: u32, calls: bool) {
        let frame = match self.surface.get_current_texture() {
            wgpu::CurrentSurfaceTexture::Success(frame)
            | wgpu::CurrentSurfaceTexture::Suboptimal(frame) => frame,
            _ => {
                self.surface.configure(&self.device, &self.config);
                return;
            }
        };
        let view = frame
            .texture
            .create_view(&wgpu::TextureViewDescriptor::default());
        let mut encoder = self.device.create_command_encoder(&Default::default());
        {
            let mut pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: None,
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view: &view,
                    depth_slice: None,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(wgpu::Color::BLACK),
                        store: wgpu::StoreOp::Store,
                    },
                })],
                depth_stencil_attachment: None,
                timestamp_writes: None,
                occlusion_query_set: None,
                multiview_mask: None,
            });
            pass.set_pipeline(&self.pipeline);
            pass.set_vertex_buffer(0, self.vertices.slice(..));
            if calls {
                // Many small objects, each its own draw call.
                let each = CALL_TRIANGLES * 3;
                let slots = self.vertex_count / each;
                for i in 0..instances {
                    let start = (i % slots) * each;
                    pass.draw(start..start + each, 0..1);
                }
            } else {
                pass.draw(0..self.vertex_count, 0..instances);
            }
        }
        self.queue.submit([encoder.finish()]);
        self.queue.present(frame);
    }
}

impl Bench {
    fn report(&self) -> String {
        let gpu = self.gpu.as_ref().expect("measured");
        let mut text = format!(
            "gpu-bench on {} ({}), {}x{}\n",
            gpu.adapter,
            std::env::consts::OS,
            gpu.config.width,
            gpu.config.height
        );
        let mut best = 0.0f64;
        for r in &self.results {
            let fps = r.frames as f64 / r.seconds;
            let per_second = r.triangles_per_frame as f64 * fps;
            best = best.max(per_second);
            if self.calls {
                text.push_str(&format!(
                    "{:>7} draw calls a frame: {:>7.1} frames a second, {:>8.2} M draw calls a second\n",
                    r.triangles_per_frame,
                    fps,
                    per_second / 1e6
                ));
            } else {
                text.push_str(&format!(
                    "{:>6.1} M triangles a frame: {:>7.1} frames a second, {:>8.1} M triangles a second\n",
                    r.triangles_per_frame as f64 / 1e6,
                    fps,
                    per_second / 1e6
                ));
            }
        }
        let unit = if self.calls {
            "draw calls"
        } else {
            "triangles"
        };
        text.push_str(&format!("best: {:.2} M {unit} a second\n", best / 1e6));
        text
    }
}

impl ApplicationHandler for Bench {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        if self.gpu.is_none() {
            let gpu = Gpu::new(event_loop);
            println!("drawing on {}", gpu.adapter);
            self.gpu = Some(gpu);
        }
    }

    fn window_event(&mut self, event_loop: &ActiveEventLoop, _: WindowId, event: WindowEvent) {
        match event {
            WindowEvent::CloseRequested => event_loop.exit(),
            WindowEvent::Resized(size) => {
                if let Some(gpu) = &mut self.gpu {
                    gpu.config.width = size.width.max(1);
                    gpu.config.height = size.height.max(1);
                    gpu.surface.configure(&gpu.device, &gpu.config);
                }
            }
            WindowEvent::RedrawRequested => {
                let Some(gpu) = &mut self.gpu else { return };
                let rounds: &[u32] = if self.calls { &CALL_ROUNDS } else { &ROUNDS };
                let instances = rounds[self.round];
                let t = Instant::now();
                gpu.draw(instances, self.calls);
                if std::env::var("GPU_BENCH_TRACE").is_ok() && self.frames < 5 {
                    eprintln!("frame of {instances}: {:?}", t.elapsed());
                }
                let started = *self.round_started.get_or_insert_with(Instant::now);
                let elapsed = started.elapsed();
                if elapsed > WARM_UP {
                    self.frames += 1;
                }
                if elapsed > WARM_UP + ROUND {
                    let seconds = (elapsed - WARM_UP).as_secs_f64();
                    // In `calls`, what's counted is draw calls.
                    let triangles = if self.calls {
                        instances as u64
                    } else {
                        gpu.vertex_count as u64 / 3 * instances as u64
                    };
                    self.results.push(Result {
                        triangles_per_frame: triangles,
                        frames: self.frames,
                        seconds,
                    });
                    println!(
                        "{} a frame: {:.1} frames a second",
                        triangles,
                        self.frames as f64 / seconds
                    );
                    let too_slow = (self.frames as f64 / seconds) < 3.0;
                    self.round += 1;
                    self.round_started = None;
                    self.frames = 0;
                    if self.round == rounds.len() || too_slow {
                        let report = self.report();
                        print!("{report}");
                        let file = format!(
                            "gpu-bench-{}{}.txt",
                            std::env::consts::OS,
                            if self.calls { "-calls" } else { "" }
                        );
                        let _ = std::fs::write(&file, &report);
                        println!("written to {file}");
                        event_loop.exit();
                        return;
                    }
                }
                gpu.window.request_redraw();
            }
            _ => {}
        }
    }
}

fn main() {
    let event_loop = EventLoop::new().expect("an event loop");
    let mut bench = Bench {
        calls: std::env::args().any(|a| a == "calls"),
        gpu: None,
        round: 0,
        round_started: None,
        frames: 0,
        results: Vec::new(),
    };
    event_loop.run_app(&mut bench).expect("the event loop");
}
