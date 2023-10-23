mod data;
mod common;
use common::{start_window, set_device};
use winit::{event_loop::ControlFlow, event::{Event, WindowEvent}};

use self::data::create_data;

pub fn study_appendix_main(){
    let event_loop = winit::event_loop::EventLoop::new();
    //윈도우 생성
    let (window, instance, surface) = 
    start_window(&event_loop);

    //디바이스 설정 후, 설정과 큐 받음
    let (config, device ,queue) = 
    set_device(&window, &instance, &surface);
    env_logger::init();
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
}