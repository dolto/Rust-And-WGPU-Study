use std::default;

use wgpu::Surface;
use winit::{window::{WindowBuilder, Window}, event_loop::ControlFlow, event::{Event, WindowEvent}};

pub fn start_window<'a>()
-> (&'a Window, &'a wgpu::Instance, &'a Surface){
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

    return (&window, &instance, &surface);
}

pub async fn set_device(window: &Window, instance: &wgpu::Instance, surface: &Surface){
    let adapter = futures::executor::block_on(
        instance.request_adapter(
            &wgpu::RequestAdapterOptions{
                power_preference: wgpu::PowerPreference::default(),
                compatible_surface: Some(surface),
                force_fallback_adapter: false,
            },
        )
    )
    .unwrap();
}

struct State {
    
}