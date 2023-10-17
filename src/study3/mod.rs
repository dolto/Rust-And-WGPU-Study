mod common;

use winit::{
    event_loop::{EventLoop},
};
use std::borrow::Cow;

pub fn study3_main(){
    let mut primitive_type = "point-list"; //primitive_type의 기본값
    let args: Vec<String> = std::env::args().collect(); //이 러스트 프로그램을 콘솔로 수행할때 매개변수를 받아옴
    if args.len() > 1 {
        primitive_type = &args[1]; //primitive_type 변수를 매개변수 변환
    }

    let mut topology = wgpu::PrimitiveTopology::PointList;
    let mut index_format = None;
    if  primitive_type == "line-list" {
        topology = wgpu::PrimitiveTopology::LineList; //만약 primitive_type가 line-list라면 점 두개씩 선을 그리도록 토폴로지를 바꿈
        index_format = None;
    } else if  primitive_type == "line-strip" {
        topology = wgpu::PrimitiveTopology::LineStrip; //만약 primitive_type가 line-strip라면 점 순서대로 전부 선을 그리도록 토폴로지를 바꿈
        index_format = Some(wgpu::IndexFormat::Uint32);
    }

    let inputs = common::Inputs{
        source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(include_str!("shader.wgsl"))),
        topology: topology,
        strip_index_format: index_format
    };
    let event_loop = EventLoop::new();    
    let window = winit::window::Window::new(&event_loop).unwrap(); 
    
    window.set_title(&*format!("점선그리기| {}: {}", "Primitive", primitive_type));
    env_logger::init();   
    pollster::block_on( common::run(event_loop, &window, inputs, 6));    
}