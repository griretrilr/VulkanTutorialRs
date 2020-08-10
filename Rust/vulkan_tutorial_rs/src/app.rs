use vulkano::instance::debug::{DebugCallback, MessageSeverity, MessageType};
use vulkano::instance::ApplicationInfo;
use vulkano::instance::{Instance, InstanceExtensions};
use vulkano::swapchain::Surface;

use vulkano_win::VkSurfaceBuild;

use winit::event::{Event, WindowEvent};
use winit::event_loop::{ControlFlow, EventLoop};
use winit::window::{Window, WindowBuilder};

use std::sync::Arc;

const VALIDATION_LAYERS: &[&str] = &["VK_LAYER_LUNARG_standard_validation"];

#[cfg(all(debug_assertions))]
const ENABLE_VALIDATION_LAYERS: bool = true;
#[cfg(not(debug_assertions))]
const ENABLE_VALIDATION_LAYERS: bool = false;

pub struct App {
    instance: Arc<Instance>,
    debug_callback: Option<DebugCallback>,
    event_loop: EventLoop<()>,
    surface: Arc<Surface<Window>>,
}

impl App {
    pub fn new() -> App {
        let app_info = Self::create_app_info();
        let supported_extensions = Self::supported_extensions();
        println!("Supported extensions: {:?}", supported_extensions);
        let required_extensions = Self::required_extensions();
        println!("Required extensions: {:?}", required_extensions);

        let instance = if ENABLE_VALIDATION_LAYERS && Self::check_validation_layer_support() {
            Instance::new(
                Some(&app_info),
                &required_extensions,
                VALIDATION_LAYERS.iter().cloned(),
            )
            .expect("failed to create Vulkan instance")
        } else {
            println!("Validation layers requested, but not available!");
            Instance::new(Some(&app_info), &required_extensions, None)
                .expect("failed to create Vulkan instance")
        };
        let debug_callback = Self::setup_debug_callback(&instance);
        let event_loop = EventLoop::new();
        let surface = WindowBuilder::new()
            .build_vk_surface(&event_loop, instance.clone())
            .unwrap();

        App {
            instance,
            debug_callback,
            event_loop,
            surface,
        }
    }

    fn create_app_info() -> ApplicationInfo<'static> {
        app_info_from_cargo_toml!()
    }

    fn supported_extensions() -> InstanceExtensions {
        InstanceExtensions::supported_by_core().unwrap()
    }

    fn required_extensions() -> InstanceExtensions {
        let mut extensions = vulkano_win::required_extensions();
        if ENABLE_VALIDATION_LAYERS {
            extensions.ext_debug_utils = true;
        }
        extensions
    }

    fn setup_debug_callback(instance: &Arc<Instance>) -> Option<DebugCallback> {
        if !ENABLE_VALIDATION_LAYERS {
            return None;
        }

        let msg_types = MessageType {
            general: true,
            validation: true,
            performance: true,
        }; // TODO - add a MessageType::all()

        let msg_severities = MessageSeverity {
            error: true,
            warning: true,
            information: true,
            verbose: true,
        }; // TODO - add a MessageSeverity::all()

        DebugCallback::new(instance, msg_severities, msg_types, |msg| {
            println!("debug callback: {:?}", msg.description);
        })
        .ok()
    }

    fn check_validation_layer_support() -> bool {
        let layers: Vec<_> = vulkano::instance::layers_list()
            .unwrap()
            .map(|l| l.name().to_owned())
            .collect();
        VALIDATION_LAYERS
            .iter()
            .all(|layer_name| layers.contains(&layer_name.to_string()))
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
