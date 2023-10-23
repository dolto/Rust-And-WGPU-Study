mod common;
use common::{start_window, set_device};

pub fn study_appendix_main(){
    //윈도우 생성
    let (window, instance, surface) = 
    start_window();

    //디바이스 설정 후, 설정과 큐 받음
    let (config, device ,queue) = 
    set_device(&window, &instance, &surface);

    
}