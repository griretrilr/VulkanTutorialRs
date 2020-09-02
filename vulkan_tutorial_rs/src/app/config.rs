use vulkano::device::DeviceExtensions;

pub const VALIDATION_LAYERS: &[&str] = &["VK_LAYER_LUNARG_standard_validation"];

pub const _DEFAULT_WIDTH: u32 = 1024;
pub const _DEFAULT_HEIGHT: u32 = 768;

#[cfg(all(debug_assertions))]
pub const ENABLE_VALIDATION_LAYERS: bool = true;
#[cfg(not(debug_assertions))]
pub const ENABLE_VALIDATION_LAYERS: bool = false;

pub fn required_device_extensions() -> DeviceExtensions {
    DeviceExtensions {
        khr_swapchain: true,
        ..DeviceExtensions::none()
    }
}
