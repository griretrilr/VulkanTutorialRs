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
    _physical_device_index: usize,
    _event_loop: EventLoop<()>,
    _surface: Arc<Surface<Window>>,
}

mod app_setup {
    use vulkano::instance::debug::{DebugCallback, MessageSeverity, MessageType};
    use vulkano::instance::{ApplicationInfo, QueueFamily};
    use vulkano::instance::{Instance, InstanceExtensions, PhysicalDevice, PhysicalDeviceType};

    use vulkano_win::VkSurfaceBuild;

    use winit::event_loop::EventLoop;
    use winit::window::WindowBuilder;

    use std::sync::Arc;
    use vulkano::device::{Device, DeviceCreationError, DeviceExtensions, Features, QueuesIter};

    struct QueueFamilies {
        graphics_family_id: Option<u32>,
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

    fn pick_physical_device(instance: &Arc<Instance>) -> PhysicalDevice {
        PhysicalDevice::enumerate(&instance)
            .max_by_key(rate_physical_device)
            .unwrap()
    }

    fn rate_physical_device(d: &PhysicalDevice) -> u32 {
        let mut score = 0;

        // Discrete GPU is better
        if d.ty() == PhysicalDeviceType::DiscreteGpu {
            score += 1_000_000;
        }

        // Better image quality with bigger textures.
        score += d.limits().max_image_dimension_2d();

        score
    }

    fn required_device_extensions() -> DeviceExtensions {
        DeviceExtensions::none()
    }

    fn create_logical_device(
        physical_device: PhysicalDevice,
        families: &QueueFamilies,
        extensions: &DeviceExtensions,
    ) -> Result<(Arc<Device>, QueuesIter), DeviceCreationError> {
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
        let graphics_family = families.graphics_family(&physical_device);
        let queue_priority = 1.0f32;
        let queue_families = [(graphics_family.unwrap(), queue_priority)];
        let v = Vec::from(queue_families);
        let i = v.into_iter();

        vulkano::device::Device::new(physical_device, &features, extensions, i)
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
        let physical_device = pick_physical_device(&instance);
        println!("Physical device: {}", physical_device.name());
        let _physical_device_index = physical_device.index();
        let queue_families = QueueFamilies::new(&physical_device);
        let required_device_extensions = required_device_extensions();
        let _queue = create_logical_device(
            physical_device,
            &queue_families,
            &required_device_extensions,
        );

        let event_loop = EventLoop::new();
        let _surface = WindowBuilder::new()
            .build_vk_surface(&event_loop, instance.clone())
            .unwrap();

        super::App {
            _instance: instance,
            _debug_callback,
            _physical_device_index,
            _event_loop: event_loop,
            _surface,
        }
    }

    impl QueueFamilies {
        pub fn new(device: &PhysicalDevice) -> QueueFamilies {
            let mut families = QueueFamilies {
                graphics_family_id: None,
            };
            for family in device.queue_families() {
                if families.graphics_family_id == None && family.supports_graphics() {
                    families.graphics_family_id = Some(family.id())
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

        fn is_complete(&self) -> bool {
            return self.graphics_family_id != None;
        }
    }
}

impl App {
    pub fn new() -> App {
        app_setup::new_app()
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
