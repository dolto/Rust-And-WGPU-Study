mod transform;
mod vertex_data;

use std:: {iter, mem};
use rand::{random, Rng};
use wgpu::util::DeviceExt;
use cgmath::*;
use winit::{
    event::*,
    event_loop::{ControlFlow, EventLoop},
    window::{Window, WindowBuilder},
};
use bytemuck:: {Pod, Zeroable, cast_slice};

const IS_PERSPECTIVE:bool = true;
const ANIMATION_SPEED:f32 = 1.0;

#[repr(C)]
#[derive(Copy, Clone, Debug, Pod, Zeroable)]
struct Vertex {
    position: [f32; 4],
    color: [f32; 4],
    random: [f32; 4]
}
impl Vertex {
    const ATTRIBUTES: [wgpu::VertexAttribute; 3] = wgpu::vertex_attr_array![0=>Float32x4, 1=>Float32x4, 2=>Float32x4];
    fn desc<'a>() -> wgpu::VertexBufferLayout<'a> {
        wgpu::VertexBufferLayout {
            array_stride: mem::size_of::<Vertex>() as wgpu::BufferAddress,
            step_mode: wgpu::VertexStepMode::Vertex,
            attributes: &Self::ATTRIBUTES,
        }
    }
}

fn vertex(p:[i8;3], c:[i8; 3]) -> Vertex {
    //포지션과 컬러는 본래 실수형이기 때문에 받은 데이터를 변환시켜줘야함
    let mut rng = rand::thread_rng();
    let random_vector:[f32; 4] = [rng.gen_range(-0.02..0.02), rng.gen_range(-0.02..0.02), rng.gen_range(-0.02..0.02),0.0];
    Vertex {
        position: [p[0] as f32, p[1] as f32, p[2] as f32, 1.0], 
        color: [c[0] as f32, c[1] as f32, c[2] as f32, 1.0],
        random: random_vector
    }
}

fn create_vertices() -> Vec<Vertex> {
    let pos = vertex_data::cube_positions();
    let col = vertex_data::cube_colors();
    let mut data:Vec<Vertex> = Vec::with_capacity(pos.len());
    for i in 0..pos.len() {
        data.push(vertex(pos[i], col[i]));
    }
    data.to_vec()
}

struct State {
    init: transform::InitWgpu, //그래픽 출력을 위한 초기화 부분
    pipeline: wgpu::RenderPipeline, //파이프라인 연결부분
    vertex_buffer: wgpu::Buffer, //버텍스 데이터 부분
    uniform_buffer: wgpu::Buffer, //유니폼 데이터 부분 (사실 이게 무슨 역할을 하는지 아직도 모르겠다...)
    uniform_bind_group:wgpu::BindGroup, //유니폼 그룹화 부분
    model_mat: Matrix4<f32>, //모델좌표
    view_mat: Matrix4<f32>, //뷰좌표
    project_mat: Matrix4<f32>, //
}

impl State {
    async fn new(window: &Window) -> Self {        
        let init =  transform::InitWgpu::init_wgpu(window).await;

        let shader = init.device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("Shader"),
            source: wgpu::ShaderSource::Wgsl(include_str!("shader.wgsl").into()),
        });

        // uniform data
        let camera_position = (3.0, 1.5, 3.0).into();
        let look_direction = (0.0,0.0,0.0).into();
        let up_direction = cgmath::Vector3::unit_y();
        
        let model_mat = transform::create_transforms([0.0,0.0,0.0], [0.0,0.0,0.0], [1.0,1.0,1.0]);
        let (view_mat, project_mat, view_project_mat) = 
            transform::create_view_projection(camera_position, look_direction, up_direction, 
            init.config.width as f32 / init.config.height as f32, IS_PERSPECTIVE);
        let mvp_mat = view_project_mat * model_mat;
        
        let mvp_ref:&[f32; 16] = mvp_mat.as_ref();
        let uniform_buffer = init.device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Uniform Buffer"),
            contents: bytemuck::cast_slice(mvp_ref),
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
        });

        let uniform_bind_group_layout = init.device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor{
            entries: &[wgpu::BindGroupLayoutEntry {
                binding: 0,
                visibility: wgpu::ShaderStages::VERTEX,
                ty: wgpu::BindingType::Buffer {
                    ty: wgpu::BufferBindingType::Uniform,
                    has_dynamic_offset: false,
                    min_binding_size: None,
                },
                count: None,
            }],
            label: Some("Uniform Bind Group Layout"),
        });

        let uniform_bind_group = init.device.create_bind_group(&wgpu::BindGroupDescriptor{
            layout: &uniform_bind_group_layout,
            entries: &[wgpu::BindGroupEntry {
                binding: 0,
                resource: uniform_buffer.as_entire_binding(),
            }],
            label: Some("Uniform Bind Group"),
        });

        let pipeline_layout = init.device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: Some("Render Pipeline Layout"),
            bind_group_layouts: &[&uniform_bind_group_layout],
            push_constant_ranges: &[],
        });

        let pipeline = init.device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("Render Pipeline"),
            layout: Some(&pipeline_layout),
            vertex: wgpu::VertexState {
                module: &shader,
                entry_point: "vs_main",
                buffers: &[Vertex::desc()],
            },
            fragment: Some(wgpu::FragmentState {
                module: &shader,
                entry_point: "fs_main",
                targets: &[Some(wgpu::ColorTargetState {
                    format: init.config.format,
                    blend: Some(wgpu::BlendState {
                        color: wgpu::BlendComponent::REPLACE,
                        alpha: wgpu::BlendComponent::REPLACE,
                    }),
                    write_mask: wgpu::ColorWrites::ALL,

                })],
            }),
            primitive: wgpu::PrimitiveState{
                //topology: wgpu::PrimitiveTopology::TriangleList,
                topology: wgpu::PrimitiveTopology::TriangleStrip,
                strip_index_format: None,
                //cull_mode: Some(wgpu::Face::Back),
                ..Default::default()
            },
            //depth_stencil: None,
            depth_stencil: Some(wgpu::DepthStencilState {
                format: wgpu::TextureFormat::Depth24Plus,
                depth_write_enabled: true,
                depth_compare: wgpu::CompareFunction::LessEqual,
                stencil: wgpu::StencilState::default(),
                bias: wgpu::DepthBiasState::default(),
            }), //3d를 출력하므로, 어느부분이 보이는 부분인지에 대한 설정이 필요함
            multisample: wgpu::MultisampleState::default(),
            multiview: None,
        });

        let vertex_buffer = init.device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Vertex Buffer"),
            contents: cast_slice(&create_vertices()),
            usage: wgpu::BufferUsages::VERTEX,
        });

        Self {
            init,
            pipeline,
            vertex_buffer,
            uniform_buffer,
            uniform_bind_group,
            model_mat,
            view_mat,
            project_mat,
        }
    }

    pub fn resize(&mut self, new_size: winit::dpi::PhysicalSize<u32>) {
        if new_size.width > 0 && new_size.height > 0 {
            self.init.instance.poll_all(true);
            self.init.size = new_size;
            self.init.config.width = new_size.width;
            self.init.config.height = new_size.height;
            self.init.surface.configure(&self.init.device, &self.init.config);

            self.project_mat = transform::create_projection(new_size.width as f32 / new_size.height as f32, IS_PERSPECTIVE);
            let mvp_mat = self.project_mat * self.view_mat * self.model_mat;        
            let mvp_ref:&[f32; 16] = mvp_mat.as_ref();
            self.init.queue.write_buffer(&self.uniform_buffer, 0, bytemuck::cast_slice(mvp_ref));
        }
    }

    #[allow(unused_variables)]
    fn input(&mut self, event: &WindowEvent) -> bool {
        false
    }

    fn update(&mut self, dt: std::time::Duration) {
        let dt = ANIMATION_SPEED * dt.as_secs_f32();

        let model_mat = transform::create_transforms(
            [0.0,0.0,0.0], [dt.sin(), dt.cos(), 0.0], [1.0,1.0,1.0]);
        let mvp_mat = self.project_mat * self.view_mat * model_mat;
        let mvp_ref:&[f32; 16] = mvp_mat.as_ref();

        if dt % 0.4 < 0.1{
            self.vertex_buffer = self.init.device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
                label: Some("Vertex Buffer"),
                contents: cast_slice(&create_vertices()),
                usage: wgpu::BufferUsages::VERTEX,
            });
        }
        
        self.init.queue.write_buffer(&self.uniform_buffer, 0, bytemuck::cast_slice(mvp_ref));
    }

    fn render(&mut self) -> Result<(), wgpu::SurfaceError> {
        //let output = self.init.surface.get_current_frame()?.output;
        let output = self.init.surface.get_current_texture()?;
        let view = output
            .texture
            .create_view(&wgpu::TextureViewDescriptor::default());  
        let depth_texture = self.init.device.create_texture(&wgpu::TextureDescriptor {
            size: wgpu::Extent3d {
                width: self.init.config.width,
                height: self.init.config.height,
                depth_or_array_layers: 1,
            },
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format:wgpu::TextureFormat::Depth24Plus,
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            label: None,
            view_formats: &[],
        });
        let depth_view = depth_texture.create_view(&wgpu::TextureViewDescriptor::default());
        
        let mut encoder = self
            .init.device
            .create_command_encoder(&wgpu::CommandEncoderDescriptor {
                label: Some("Render Encoder"),
            });

        {
            let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("Render Pass"),
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view: &view,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(wgpu::Color {
                            r: 0.2,
                            g: 0.247,
                            b: 0.314,
                            a: 1.0,
                        }),
                        store: true,
                    },
                })],
                //depth_stencil_attachment: None,
                depth_stencil_attachment: Some(wgpu::RenderPassDepthStencilAttachment {
                    view: &depth_view,
                    depth_ops: Some(wgpu::Operations {
                        load: wgpu::LoadOp::Clear(1.0),
                        store: false,
                    }),
                    stencil_ops: None,
                }),
            });

            render_pass.set_pipeline(&self.pipeline);
            render_pass.set_vertex_buffer(0, self.vertex_buffer.slice(..));           
            render_pass.set_bind_group(0, &self.uniform_bind_group, &[]);
            render_pass.draw(0..36, 0..1);
        }

        self.init.queue.submit(iter::once(encoder.finish()));
        output.present();

        Ok(())
    }
}

pub fn study7_main(){
    env_logger::init();
    let event_loop = EventLoop::new();
    let window = WindowBuilder::new().build(&event_loop).unwrap();
    window.set_title(&*format!("{}", "cube with distinct face colors"));
    let mut state = pollster::block_on(State::new(&window));

    let start_time = std::time::Instant::now();

    event_loop.run(move |event, _, control_flow| {
        match event {
            Event::WindowEvent {
                ref event,
                window_id,
            } if window_id == window.id() => {
                if !state.input(event) {
                    match event {
                        WindowEvent::CloseRequested
                        | WindowEvent::KeyboardInput {
                            input:
                                KeyboardInput {
                                    state: ElementState::Pressed,
                                    virtual_keycode: Some(VirtualKeyCode::Escape),
                                    ..
                                },
                            ..
                        } => *control_flow = ControlFlow::Exit, //ecs키를 입력할 경우
                        WindowEvent::Resized(physical_size) => {
                            state.resize(*physical_size);
                        }
                        WindowEvent::ScaleFactorChanged { new_inner_size, .. } => {
                            state.resize(**new_inner_size);
                        }
                        _ => {}
                    }
                }
            }
            Event::RedrawRequested(_) => {
                let now = std::time::Instant::now();
                let dt = now - start_time;
                state.update(dt);
                match state.render() {
                    Ok(_) => {}
                    Err(wgpu::SurfaceError::Lost) => state.resize(state.init.size),
                    Err(wgpu::SurfaceError::OutOfMemory) => *control_flow = ControlFlow::Exit,
                    Err(e) => eprintln!("{:?}", e),
                }
            }
            Event::MainEventsCleared => {
                window.request_redraw();
            }
            _ => {}
        }
    });
}