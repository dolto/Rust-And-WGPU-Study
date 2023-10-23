use std::default;

use wgpu::{Surface, SurfaceConfiguration, Queue, Device};
use winit::{window::{WindowBuilder, Window}, event_loop::EventLoop};

pub fn start_window(event_loop: &EventLoop<()>)
-> (Window, wgpu::Instance, Surface){
    //윈도우 생성하는 기본적인 함수
    let window = WindowBuilder::new().build(event_loop).unwrap();
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


fn uniformData(device: &wgpu::Device, data: Vec<Vec<i32>>, size: usize)
-> wgpu::Buffer{
    let buffer = device.create_buffer(
        &wgpu::BufferDescriptor{
            label: Some("Data Uniform Buffer"),
            size: size as u64,
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false,
        }
    );
    buffer
}
pub fn set_pipeline(){

}