pub mod config;
mod init;

use std::sync::Arc;
use vulkano::device::{Device, Queue};
use vulkano::instance::debug::DebugCallback;
use vulkano::instance::Instance;
use vulkano::swapchain::Surface;
use winit::event::{Event, WindowEvent};
use winit::event_loop::{ControlFlow, EventLoop};
use winit::window::Window;

pub struct App {
    _instance: Arc<Instance>,
    _debug_callback: Option<DebugCallback>,
    _event_loop: EventLoop<()>,
    _surface: Arc<Surface<Window>>,
    _physical_device_index: usize,
    _device: Arc<Device>,
    _graphics_queue: Arc<Queue>,
    _present_queue: Arc<Queue>,
}

impl App {
    pub fn new() -> App {
        init::new_app()
    }

    pub fn run(self) {
        self._event_loop
            .run(move |event, _, control_flow| match event {
                Event::WindowEvent {
                    event: WindowEvent::CloseRequested,
                    ..
                } => {
                    *control_flow = ControlFlow::Exit;
                }
                _ => (),
            });
    }
}
