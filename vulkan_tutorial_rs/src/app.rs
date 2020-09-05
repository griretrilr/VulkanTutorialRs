pub mod config;

mod init;
mod logical_device;
mod physical_device_info;
mod queue_families;
mod swapchain_info;

pub use logical_device::LogicalDevice;
pub use physical_device_info::PhysicalDeviceInfo;
pub use queue_families::QueueFamilies;
pub use swapchain_info::SwapchainInfo;

use vulkano::instance::debug::DebugCallback;
use vulkano::instance::Instance;

use winit::event::{Event, WindowEvent};
use winit::event_loop::{ControlFlow, EventLoop};

use std::sync::Arc;
use vulkano::swapchain::Surface;
use winit::window::Window;

pub struct App {
    _instance: Arc<Instance>,
    _debug_callback: Option<DebugCallback>,
    _event_loop: EventLoop<()>,
    _surface: Arc<Surface<Window>>,
    _physical_device_info: PhysicalDeviceInfo,
    _logical_device: LogicalDevice,
    _swapchain_info: SwapchainInfo,
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
