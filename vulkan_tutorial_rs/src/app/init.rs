mod indented_printer;
mod logical_device;
mod physical_device_info;
mod queue_families;

use crate::vulkano_ext::{message_severity_to_string, message_type_to_string};
use logical_device::LogicalDevice;
use physical_device_info::PhysicalDeviceInfo;
use std::sync::Arc;
use vulkano::instance::debug::{DebugCallback, MessageSeverity, MessageType};
use vulkano::instance::ApplicationInfo;
use vulkano::instance::{Instance, InstanceExtensions, PhysicalDevice};
use vulkano::swapchain::Surface;
use vulkano_win::VkSurfaceBuild;
use winit::event_loop::EventLoop;
use winit::window::{Window, WindowBuilder};

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

    let physical_devices = get_physical_devices(&instance, &surface);
    println!("Physical devices:");
    for d in physical_devices.iter() {
        d.print("    ", "    ");
    }

    let physical_device = pick_physical_device(&physical_devices);
    println!("Best device:");
    physical_device.print("    ", "    ");

    let physical_device_index = physical_device.physical_device().index();

    let logical_device = LogicalDevice::new(physical_device);

    crate::App {
        _instance: instance,
        _debug_callback: debug_callback,
        _event_loop: event_loop,
        _surface: surface,
        _physical_device_index: physical_device_index,
        _device: logical_device.device().clone(),
        _graphics_queue: logical_device.graphics_queue().clone(),
        _present_queue: logical_device.present_queue().clone(),
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

fn get_physical_devices<'a>(
    instance: &'a Arc<Instance>,
    surface: &'a Arc<Surface<Window>>,
) -> Vec<PhysicalDeviceInfo<'a>> {
    PhysicalDevice::enumerate(instance)
        .map(|d| PhysicalDeviceInfo::new(d, surface))
        .collect()
}

fn pick_physical_device<'a>(
    devices: &'a Vec<PhysicalDeviceInfo<'a>>,
) -> &'a PhysicalDeviceInfo<'a> {
    devices
        .iter()
        .filter(|&d| d.is_valid())
        .max_by(|&a, &b| a.cmp(b))
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
//
//    fn choose_swap_surface_format(formats: &Vec<(Format, ColorSpace)>) -> &(Format, ColorSpace) {
//        let ideal_format = formats
//            .iter()
//            .filter(|&(f, c)| *f == Format::B8G8R8A8Srgb && *c == ColorSpace::SrgbNonLinear)
//            .next();
//        if ideal_format.is_some() {
//            return ideal_format.unwrap();
//        }
//        &formats[0]
//    }
//
//    fn choose_swap_present_mode(modes: &SupportedPresentModes) -> PresentMode {
//        if modes.mailbox {
//            PresentMode::Mailbox
//        } else {
//            assert!(
//                modes.fifo,
//                "Fifo support is required by the Vulkan standard"
//            );
//            PresentMode::Fifo
//        }
//    }
//
//    fn choose_swap_extents(c: &Capabilities) -> [u32; 2] {
//        if c.current_extent.is_some() {
//            return c.current_extent.unwrap();
//        }
//
//        let width = num::clamp(
//            super::DEFAULT_WIDTH,
//            c.min_image_extent[0],
//            c.max_image_extent[0],
//        );
//        let height = num::clamp(
//            super::DEFAULT_HEIGHT,
//            c.min_image_extent[1],
//            c.max_image_extent[1],
//        );
//
//        [width, height]
//    }
//
//    fn choose_swap_image_count(c: &Capabilities) -> u32 {
//        // ideally we get at least one more image than the minimum so we aren't waiting on the driver
//        let ideal = c.min_image_count + 1;
//
//        match c.max_image_count {
//            Some(max) if ideal > max => max,
//            _ => ideal,
//        }
//    }
//
//    fn create_swap_chain(
//        instance: &Arc<Instance>,
//        surface: &Arc<Surface<Window>>,
//        physical_device: &PhysicalDevice,
//        device: &Arc<Device>,
//        graphics_queue: &Arc<Queue>,
//        present_queue: &Arc<Queue>,
//    ) -> Result<(Arc<Swapchain<Window>>, Vec<Arc<SwapchainImage<Window>>>), SwapchainCreationError>
//    {
//        let surface_capabilities = surface
//            .capabilities(*physical_device)
//            .expect("failed to get surface capabilities");
//
//        let surface_format = choose_swap_surface_format(&surface_capabilities.supported_formats);
//        let present_mode = choose_swap_present_mode(&surface_capabilities.present_modes);
//        let extents = choose_swap_extents(&surface_capabilities);
//        let num_images = choose_swap_image_count(&surface_capabilities);
//        let image_array_layers = 1u32;
//
//
//        vulkano::swapchain::Swapchain::new(
//            device,
//            surface,
//            num_images,
//            surface_format,
//            extents,
//            1,
//            ImageUsage::color_attachment(),
//            vulkano::sync::SharingMode::
//        )
//    }
//
//
//    pub fn new_app() -> super::App {
//
//    }
