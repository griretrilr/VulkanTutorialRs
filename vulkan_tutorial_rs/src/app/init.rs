pub mod indented_printer;

pub use indented_printer::IndentedPrinter;

use crate::app::{LogicalDevice, PhysicalDeviceInfo, SwapchainInfo};
use crate::vulkano_ext::{message_severity_to_string, message_type_to_string};

use vulkano::instance::debug::{DebugCallback, MessageSeverity, MessageType};
use vulkano::instance::ApplicationInfo;
use vulkano::instance::{Instance, InstanceExtensions, PhysicalDevice};
use vulkano::swapchain::Surface;

use vulkano_win::VkSurfaceBuild;

use winit::event_loop::EventLoop;
use winit::window::{Window, WindowBuilder};

use std::sync::Arc;

pub fn new_app() -> crate::App {
    println!("Initialising app...");

    let app_info = create_app_info();
    println!("App info:\n\t{:?}", app_info);

    let supported_instance_extensions = supported_instance_extensions();
    println!(
        "Supported instance extensions:\n\t{:?}",
        supported_instance_extensions
    );

    let required_instance_extensions = required_instance_extensions();
    println!(
        "Required instance extensions:\n\t{:?}",
        required_instance_extensions
    );

    let instance = create_instance(&app_info, &required_instance_extensions);
    println!("Instance created");

    let debug_callback = setup_debug_callback(&instance);
    println!("Debug callback set up");

    let event_loop = EventLoop::new();
    println!("Event loop created");

    let surface = create_surface(&instance, &event_loop);
    println!("Surface created");

    let physical_device_infos = get_physical_device_infos(&instance, &surface);
    println!("Physical devices:");
    for d in physical_device_infos.iter() {
        d.print("    ", "    ");
    }

    let physical_device_info = pick_physical_device_info(physical_device_infos);
    println!("Best device:");
    physical_device_info.print("    ", "    ");

    let logical_device = LogicalDevice::new(&physical_device_info);

    let swapchain_info = SwapchainInfo::new(
        &surface,
        &physical_device_info.physical_device(),
        &logical_device,
    );

    crate::App {
        _instance: instance,
        _debug_callback: debug_callback,
        _event_loop: event_loop,
        _surface: surface,
        _physical_device_info: physical_device_info,
        _logical_device: logical_device,
        _swapchain_info: swapchain_info,
    }
}

fn supported_instance_extensions() -> InstanceExtensions {
    InstanceExtensions::supported_by_core().unwrap()
}

fn create_app_info() -> ApplicationInfo<'static> {
    vulkano::app_info_from_cargo_toml!()
}

fn required_instance_extensions() -> InstanceExtensions {
    let mut extensions = vulkano_win::required_extensions();
    if crate::app::config::ENABLE_VALIDATION_LAYERS {
        extensions.ext_debug_utils = true;
    }
    extensions
}

fn create_instance(
    app_info: &ApplicationInfo,
    required_instance_extensions: &InstanceExtensions,
) -> Arc<Instance> {
    if crate::app::config::ENABLE_VALIDATION_LAYERS && check_validation_layer_support() {
        Instance::new(
            Some(&app_info),
            required_instance_extensions,
            crate::app::config::VALIDATION_LAYERS.iter().cloned(),
        )
        .expect("failed to create Vulkan instance")
    } else {
        println!("WARNING: Validation layers requested, but not available!");
        Instance::new(Some(&app_info), required_instance_extensions, None)
            .expect("failed to create Vulkan instance")
    }
}

fn create_surface(instance: &Arc<Instance>, event_loop: &EventLoop<()>) -> Arc<Surface<Window>> {
    WindowBuilder::new()
        .build_vk_surface(event_loop, instance.clone())
        .unwrap()
}

fn get_physical_device_infos(
    instance: &Arc<Instance>,
    surface: &Arc<Surface<Window>>,
) -> Vec<PhysicalDeviceInfo> {
    PhysicalDevice::enumerate(instance)
        .map(|d| PhysicalDeviceInfo::new(&d, surface))
        .collect()
}

fn pick_physical_device_info(devices: Vec<PhysicalDeviceInfo>) -> PhysicalDeviceInfo {
    devices
        .into_iter()
        .filter(|d| d.is_valid())
        .max_by(|a, b| a.cmp(&b))
        .expect("no valid physical devices found")
}

fn check_validation_layer_support() -> bool {
    let layers: Vec<_> = vulkano::instance::layers_list()
        .unwrap()
        .map(|l| l.name().to_owned())
        .collect();
    crate::app::config::VALIDATION_LAYERS
        .iter()
        .all(|layer_name| layers.contains(&layer_name.to_string()))
}

fn setup_debug_callback(instance: &Arc<Instance>) -> Option<DebugCallback> {
    if !crate::app::config::ENABLE_VALIDATION_LAYERS {
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
        println!(
            "VK-{ty}-{severity}: {layer}: {description}",
            severity = message_severity_to_string(msg.severity),
            ty = message_type_to_string(msg.ty),
            layer = msg.layer_prefix,
            description = msg.description
        );
    })
    .ok()
}
