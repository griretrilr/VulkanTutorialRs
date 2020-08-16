use vulkano::device::Queue;
use vulkano::instance::debug::DebugCallback;
use vulkano::instance::Instance;
use vulkano::swapchain::Surface;

use winit::event::{Event, WindowEvent};
use winit::event_loop::{ControlFlow, EventLoop};
use winit::window::Window;

use std::sync::Arc;

const VALIDATION_LAYERS: &[&str] = &["VK_LAYER_LUNARG_standard_validation"];

#[cfg(all(debug_assertions))]
const ENABLE_VALIDATION_LAYERS: bool = true;
#[cfg(not(debug_assertions))]
const ENABLE_VALIDATION_LAYERS: bool = false;

pub struct App {
    _instance: Arc<Instance>,
    _debug_callback: Option<DebugCallback>,
    event_loop: EventLoop<()>,
    _surface: Arc<Surface<Window>>,
    _physical_device_index: usize,
    _graphics_queue: Arc<Queue>,
    _present_queue: Arc<Queue>,
}

mod app_setup {
    use crate::queue_family::QueueFamilyExt;

    use vulkano::device::{Device, DeviceExtensions, Features, Queue};
    use vulkano::instance::debug::{DebugCallback, MessageSeverity, MessageType};
    use vulkano::instance::{ApplicationInfo, QueueFamily};
    use vulkano::instance::{Instance, InstanceExtensions, PhysicalDevice, PhysicalDeviceType};
    use vulkano::swapchain::Surface;

    use vulkano_win::VkSurfaceBuild;

    use winit::event_loop::EventLoop;
    use winit::window::{Window, WindowBuilder};

    use std::collections::BTreeSet;
    use std::sync::Arc;

    struct QueueFamilies {
        graphics_family_id: Option<u32>,
        present_family_id: Option<u32>,
    }

    fn create_app_info() -> ApplicationInfo<'static> {
        app_info_from_cargo_toml!()
    }

    fn supported_instance_extensions() -> InstanceExtensions {
        InstanceExtensions::supported_by_core().unwrap()
    }

    fn required_instance_extensions() -> InstanceExtensions {
        let mut extensions = vulkano_win::required_extensions();
        if super::ENABLE_VALIDATION_LAYERS {
            extensions.ext_debug_utils = true;
        }
        extensions
    }

    fn create_instance(
        app_info: &ApplicationInfo,
        required_instance_extensions: &InstanceExtensions,
    ) -> Arc<Instance> {
        if super::ENABLE_VALIDATION_LAYERS && check_validation_layer_support() {
            Instance::new(
                Some(&app_info),
                required_instance_extensions,
                super::VALIDATION_LAYERS.iter().cloned(),
            )
            .expect("failed to create Vulkan instance")
        } else {
            println!("Validation layers requested, but not available!");
            Instance::new(Some(&app_info), required_instance_extensions, None)
                .expect("failed to create Vulkan instance")
        }
    }

    fn check_validation_layer_support() -> bool {
        let layers: Vec<_> = vulkano::instance::layers_list()
            .unwrap()
            .map(|l| l.name().to_owned())
            .collect();
        super::VALIDATION_LAYERS
            .iter()
            .all(|layer_name| layers.contains(&layer_name.to_string()))
    }

    fn setup_debug_callback(instance: &Arc<Instance>) -> Option<DebugCallback> {
        if !super::ENABLE_VALIDATION_LAYERS {
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

    fn pick_physical_device<'a>(
        instance: &'a Arc<Instance>,
        surface: &'a Arc<Surface<Window>>,
    ) -> PhysicalDevice<'a> {
        let score_func = |d| {
            let s: Option<u32> = rate_physical_device(&d, surface);
            match s {
                Some(s) => Some((d, s)),
                None => None,
            }
        };
        PhysicalDevice::enumerate(instance)
            .filter_map(score_func)
            .max_by_key(|&(_, s)| s)
            .unwrap()
            .0
    }

    fn rate_physical_device(d: &PhysicalDevice, s: &Surface<Window>) -> Option<u32> {
        let queue_families = QueueFamilies::new(d, s);
        if !queue_families.is_complete() {
            return None;
        }

        let mut score = 0;

        // Discrete GPU is better
        if d.ty() == PhysicalDeviceType::DiscreteGpu {
            score += 1_000_000;
        }

        // Better image quality with bigger textures.
        score += d.limits().max_image_dimension_2d();

        Some(score)
    }

    fn required_device_extensions() -> DeviceExtensions {
        DeviceExtensions::none()
    }

    fn create_logical_device(
        physical_device: PhysicalDevice,
        families: &QueueFamilies,
        extensions: &DeviceExtensions,
    ) -> (Arc<Device>, Arc<Queue>, Arc<Queue>) {
        let features = Features {
            robust_buffer_access: false,
            full_draw_index_uint32: false,
            image_cube_array: false,
            independent_blend: false,
            geometry_shader: false,
            tessellation_shader: false,
            sample_rate_shading: false,
            dual_src_blend: false,
            logic_op: false,
            multi_draw_indirect: false,
            draw_indirect_first_instance: false,
            depth_clamp: false,
            depth_bias_clamp: false,
            fill_mode_non_solid: false,
            depth_bounds: false,
            wide_lines: false,
            large_points: false,
            alpha_to_one: false,
            multi_viewport: false,
            sampler_anisotropy: false,
            texture_compression_etc2: false,
            texture_compression_astc_ldr: false,
            texture_compression_bc: false,
            occlusion_query_precise: false,
            pipeline_statistics_query: false,
            vertex_pipeline_stores_and_atomics: false,
            fragment_stores_and_atomics: false,
            shader_tessellation_and_geometry_point_size: false,
            shader_image_gather_extended: false,
            shader_storage_image_extended_formats: false,
            shader_storage_image_multisample: false,
            shader_storage_image_read_without_format: false,
            shader_storage_image_write_without_format: false,
            shader_uniform_buffer_array_dynamic_indexing: false,
            shader_sampled_image_array_dynamic_indexing: false,
            shader_storage_buffer_array_dynamic_indexing: false,
            shader_storage_image_array_dynamic_indexing: false,
            shader_clip_distance: false,
            shader_cull_distance: false,
            shader_f3264: false,
            shader_int64: false,
            shader_int16: false,
            shader_resource_residency: false,
            shader_resource_min_lod: false,
            sparse_binding: false,
            sparse_residency_buffer: false,
            sparse_residency_image2d: false,
            sparse_residency_image3d: false,
            sparse_residency2_samples: false,
            sparse_residency4_samples: false,
            sparse_residency8_samples: false,
            sparse_residency16_samples: false,
            sparse_residency_aliased: false,
            variable_multisample_rate: false,
            inherited_queries: false,
            buffer_device_address: false,
            buffer_device_address_capture_replay: false,
            buffer_device_address_multi_device: false,
        };
        let graphics_family =
            QueueFamilyExt::new(families.graphics_family(&physical_device).unwrap());
        let present_family =
            QueueFamilyExt::new(families.present_family(&physical_device).unwrap());
        let queue_priority = 1.0f32;
        let mut families_set: BTreeSet<QueueFamilyExt> = BTreeSet::new();
        families_set.insert(graphics_family);
        families_set.insert(present_family);
        let families_and_priorities = families_set
            .into_iter()
            .map(|f| (f.inner(), queue_priority));

        let (device, mut queues) = vulkano::device::Device::new(
            physical_device,
            &features,
            extensions,
            families_and_priorities,
        )
        .unwrap();
        let graphics_queue = queues.next().unwrap();
        let present_queue = queues.next().unwrap_or_else(|| graphics_queue.clone());

        (device, graphics_queue, present_queue)
    }

    pub fn new_app() -> super::App {
        let app_info = create_app_info();
        let supported_instance_extensions = supported_instance_extensions();
        println!(
            "Supported instance extensions: {:?}",
            supported_instance_extensions
        );
        let required_instance_extensions = required_instance_extensions();
        println!(
            "Required instance extensions: {:?}",
            required_instance_extensions
        );

        let instance = create_instance(&app_info, &required_instance_extensions);
        let _debug_callback = setup_debug_callback(&instance);
        let event_loop = EventLoop::new();
        let surface = WindowBuilder::new()
            .build_vk_surface(&event_loop, instance.clone())
            .unwrap();
        let physical_device = pick_physical_device(&instance, &surface);
        println!("Physical device: {}", physical_device.name());
        let _physical_device_index = physical_device.index();
        let queue_families = QueueFamilies::new(&physical_device, surface.as_ref());
        let required_device_extensions = required_device_extensions();
        let (_device, _graphics_queue, _present_queue) = create_logical_device(
            physical_device,
            &queue_families,
            &required_device_extensions,
        );

        super::App {
            _instance: instance,
            _debug_callback,
            event_loop,
            _surface: surface,
            _physical_device_index,
            _graphics_queue,
            _present_queue,
        }
    }

    impl QueueFamilies {
        pub fn new(device: &PhysicalDevice, surface: &Surface<Window>) -> QueueFamilies {
            let mut families = QueueFamilies {
                graphics_family_id: None,
                present_family_id: None,
            };
            for family in device.queue_families() {
                if families.graphics_family_id == None && family.supports_graphics() {
                    families.graphics_family_id = Some(family.id())
                }
                if families.present_family_id == None && surface.is_supported(family).unwrap() {
                    families.present_family_id = Some(family.id())
                }
                if families.is_complete() {
                    break;
                }
            }
            families
        }

        fn graphics_family<'a>(&self, device: &PhysicalDevice<'a>) -> Option<QueueFamily<'a>> {
            self.graphics_family_id
                .and_then(|i| device.queue_family_by_id(i))
        }

        fn present_family<'a>(&self, device: &PhysicalDevice<'a>) -> Option<QueueFamily<'a>> {
            self.present_family_id
                .and_then(|i| device.queue_family_by_id(i))
        }

        fn is_complete(&self) -> bool {
            return self.graphics_family_id != None && self.present_family_id != None;
        }
    }
}

impl App {
    pub fn new() -> App {
        app_setup::new_app()
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
