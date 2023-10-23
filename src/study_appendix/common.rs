use std::default;

use wgpu::{Surface, TextureFormat, SurfaceConfiguration, Queue, Device};
use winit::{window::{WindowBuilder, Window}, event_loop::ControlFlow, event::{Event, WindowEvent}};

pub fn start_window()
-> (Window, wgpu::Instance, Surface){
    //윈도우 생성하는 기본적인 함수
    let event_loop = winit::event_loop::EventLoop::new();
    let window = WindowBuilder::new().build(&event_loop).unwrap();
    window.set_title("그래픽 카드로 연산 수행 테스트");
    //윈도우 생성

    let instance = wgpu::Instance::new(
        wgpu::InstanceDescriptor { 
            backends: wgpu::Backends::VULKAN, 
            dx12_shader_compiler: Default::default()
        });
    let surface = unsafe {
        instance.create_surface(&window)
    }.unwrap();
    //그래픽 카드 연동

    event_loop.run(move |event, _, control_flow|{
        *control_flow = ControlFlow::Poll;
        match event {
            Event::WindowEvent {
                window_id: _,
                event: WindowEvent::CloseRequested,
            } => {*control_flow = ControlFlow::Exit},

            _ => {},
        }
    });

    return (window, instance, surface);
}

pub fn set_device(window: &Window, instance: &wgpu::Instance, surface: &Surface)
-> (SurfaceConfiguration, Device, Queue){
    let adapter = futures::executor::block_on(
        instance.request_adapter(
            &wgpu::RequestAdapterOptions{
                power_preference: wgpu::PowerPreference::default(),
                compatible_surface: Some(surface),
                force_fallback_adapter: false,
            },
        )
    ).unwrap();
    // device: 그래픽이나 컴퓨팅 장치에 대한 연결
    // queue: 장치의 명령 대기열을 처리함
    let (device, queue) = futures::executor::block_on(
        adapter.request_device(
            &wgpu::DeviceDescriptor {
                label: None,
                features: wgpu::Features::empty(),
                limits: wgpu::Limits::default(),
            }, 
        None)
    ).unwrap();
    
    let surface_caps = surface.get_capabilities(&adapter);
    let format = surface_caps.formats[0]; //기본적으로 0번째가 맞음

    let size = window.inner_size();
    let config = wgpu::SurfaceConfiguration {
        usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
        format,
        width: size.width,
        height: size.height,
        present_mode: wgpu::PresentMode::Fifo,
        alpha_mode: surface_caps.alpha_modes[0],
        view_formats: vec![]
    };
    surface.configure(&device, &config);

    (config, device, queue)
}

