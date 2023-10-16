/***
 * 컬러풀한 삼각형 그리기
 */

 use std::{default, cmp::max};

use winit::{
    event::{Event, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::{Window, WindowId},
};

pub async fn run(event_loop: EventLoop<()>, window: &Window){
    let size = window.inner_size();
    let instance = wgpu::Instance::new(wgpu::InstanceDescriptor{
        backends: wgpu::Backends::VULKAN,
        dx12_shader_compiler: Default::default()
    });

    let surface = unsafe {
        /// 윈도우를 역참조 하기 때문에
        /// 그동안 윈도우가 없어질 수도 있으므로
        /// 그러나 윈도우가 생성되자마자 수행하므로 괜찮음
        instance.create_surface(&window)
    }.unwrap();
    let adapter = instance
    .request_adapter(&wgpu::RequestAdapterOptions{
        power_preference: wgpu::PowerPreference::default(),
        compatible_surface: Some(&surface),
        force_fallback_adapter: false,
    })
    .await
    .expect("그래픽 카드가 지원하지 않는등의 문제로 실패!");

    let (device, queue) = adapter
    .request_device(
        &wgpu::DeviceDescriptor{
            label: None,
            features: wgpu::Features::empty(),
            limits: wgpu::Limits::default(),
        }, None
    )
    .await
    .expect("그래픽 엔진 디바이스를 생성하는데 문제가 있음!");

    //여기까지가 기본적인 세팅

    let surface_caps = surface.get_capabilities(&adapter);
    let format = surface_caps.formats[0];

    let mut config = wgpu::SurfaceConfiguration {
        usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
        format,
        width: max(size.width, 1),
        height: max(size.height, 1),
        present_mode: wgpu::PresentMode::Fifo,
        alpha_mode:surface_caps.alpha_modes[0],
        view_formats: vec![]
    };
    surface.configure(&device, &config);

    //여기까지가 기본적인 초기화 단계

    let shader = device.create_shader_module(
        wgpu::ShaderModuleDescriptor {
            label: None,
            source: wgpu::ShaderSource::Wgsl(include_str!("shader.wgsl").into()),
        }
    );
    let pipline_layout = device.create_pipeline_layout( //레이아웃 생성은 gpu버퍼를 이용할때 매우 유용함
        &wgpu::PipelineLayoutDescriptor {
            label: None,
            bind_group_layouts:&[], //그래픽 카드에 전달할 버퍼 데이터 (현재의 경우 셰이더 코드에서 직접 만들기 때문에 비어있음)
            push_constant_ranges:&[] //비어있더라도 자리표시자로 이건 반드시 있어야함
        }
    );
    let render_pipline = device.create_render_pipeline( 
        &wgpu::RenderPipelineDescriptor{
            label: None,
            layout: Some(&pipline_layout),
            vertex: wgpu::VertexState {
                module: &shader,
                entry_point: "vs_main",
                buffers: &[]
            },
            fragment: Some(wgpu::FragmentState {
                module: &shader,
                entry_point:"fs_main",
                targets:&[Some(wgpu::ColorTargetState {
                    format,
                    blend:Some(wgpu::BlendState {
                        color: wgpu::BlendComponent::REPLACE,
                        alpha: wgpu::BlendComponent::REPLACE,
                    }),
                    write_mask: wgpu::ColorWrites::ALL,
                })],
            }),
            primitive: wgpu::PrimitiveState::default(),
            depth_stencil: None,
            multisample: wgpu::MultisampleState::default(),
            multiview: None,
        }
    );

    event_loop.run(
        move |event, _, control_flow|{
            let _ = (&instance, &adapter, &shader, &pipline_layout);
            *control_flow = ControlFlow::Wait;

            match event {
                Event::WindowEvent { 
                    event: WindowEvent::Resized(size),
                    .. 
                } => {
                    instance.poll_all(true);
                    config.width = max(size.width,1);
                    config.height = max(size.height,1);
                    surface.configure(&device, &config);
                },
                Event::RedrawRequested(_) => { //리렌더링 이벤트가 나올 경우
                    let frame = surface.get_current_texture().unwrap(); //현재 프레임
                    let view = frame.texture.create_view(
                        &wgpu::TextureViewDescriptor::default()
                    );
                    let mut encoder = device.create_command_encoder(
                        &wgpu::CommandEncoderDescriptor {label: None}
                    );
                    {
                        let mut rpass = encoder.begin_render_pass(
                            &wgpu::RenderPassDescriptor {
                                label:None,
                                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                                    view: &view,
                                    resolve_target: None,
                                    ops: wgpu::Operations {
                                        load: wgpu::LoadOp::Clear(wgpu::Color {r:0.05, g:0.062, b: 0.08, a: 1.0 }), //창의 배경 색 (도형색이 아님!)
                                        store: true,
                                    },
                                })],
                                depth_stencil_attachment:None,
                            }
                        );
                        rpass.set_pipeline(&render_pipline);
                        rpass.draw(0..3, 0..1); //정점 개수와 객체 개수?
                    }

                    queue.submit(Some(encoder.finish())); //그리기가 완료 되었다고 알림
                    frame.present(); //그린 프레임 보여줌
                },
                Event::WindowEvent { 
                    event: WindowEvent::CloseRequested,
                    ..
                } => *control_flow = ControlFlow::Exit,
                _=>{}
            }
        }
    );
}

pub fn study2_main(){
    let event_loop = EventLoop::new();
    let window = winit::window::Window::new(&event_loop).unwrap();
    window.set_title("웹 GPU 컬러풀 삼각형");
    env_logger::init();
    pollster::block_on(run(event_loop, &window));
}