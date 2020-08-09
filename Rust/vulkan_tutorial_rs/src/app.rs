use vulkano::instance::Instance;
use vulkano::swapchain::Surface;

use vulkano_win::VkSurfaceBuild;

use winit::event::{Event, WindowEvent};
use winit::event_loop::{ControlFlow, EventLoop};
use winit::window::{Window, WindowBuilder};

use std::sync::Arc;

pub struct App {
    instance: Arc<Instance>,
    event_loop: EventLoop<()>,
    surface: Arc<Surface<Window>>,
}

impl App {
    pub fn new() -> App {
        let app_info = app_info_from_cargo_toml!();
        let supported_extensions =
            vulkano::instance::InstanceExtensions::supported_by_core().unwrap();
        println!("Supported extensions: {:?}", supported_extensions);
        let required_extensions = vulkano_win::required_extensions();
        println!("Required extensions: {:?}", required_extensions);
        let instance = Instance::new(Some(&app_info), &required_extensions, None).unwrap();
        let event_loop = EventLoop::new();
        let surface = WindowBuilder::new()
            .build_vk_surface(&event_loop, instance.clone())
            .unwrap();

        App {
            instance,
            event_loop,
            surface,
        }
    }

    pub fn run(self) {
        self.event_loop
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
