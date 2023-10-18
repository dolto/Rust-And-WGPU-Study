use std:: {iter, mem};
use wgpu::util::DeviceExt;
use cgmath::*;
use winit::{
    event::*,
    event_loop::{ControlFlow, EventLoop},
    window::{Window, WindowBuilder},
};
use bytemuck:: {Pod, Zeroable, cast_slice};
use common::*;

mod vertex_data;
mod transforms;
mod common;

const IS_PERSPECTIVE:bool = true;
const ANIMATION_SPEED:f32 = 1.0;

pub fn study8_main() {
    run( 
        light([1.0,1.0,1.0], [0.0,1.0,0.0], 0.2,0.1,0.1,1.0), 
        "Test Light"
    );
}