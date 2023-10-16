use winit::{event_loop::{EventLoop, ControlFlow}, window::{Window, CursorIcon}, event::{WindowEvent, Event}};
/***
 * 내 그래픽카드에서 지원하는 백엔드출력과 간단한 창 생성
 */
pub fn study1_main(){
    let instances = wgpu::Instance::new(wgpu::InstanceDescriptor{
        backends: wgpu::Backends::all(),
        dx12_shader_compiler: Default::default(),
    });
    for adapter in instances.enumerate_adapters(wgpu::Backends::all()) {
        println!("{:?}", adapter.get_info());
    }

    let event_loop = EventLoop::new();
    let window = Window::new(&event_loop).unwrap();
    window.set_title("dolto's window!");
    window.set_cursor_icon(CursorIcon::AllScroll);
    window.set_resizable(false);
    env_logger::init();

    event_loop.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Wait;
        match event {
            Event::WindowEvent {
                event:WindowEvent::CloseRequested,
                ..
            } => *control_flow = ControlFlow::Exit,
            _ => {}
        }
    });
}