use vulkano::instance::{PhysicalDevice, QueueFamily};
use vulkano::swapchain::Surface;

use winit::window::Window;

pub struct QueueFamilies {
    graphics_family_id: Option<u32>,
    present_family_id: Option<u32>,
}

impl QueueFamilies {
    pub fn new(device: PhysicalDevice, surface: &Surface<Window>) -> QueueFamilies {
        let mut families = QueueFamilies {
            graphics_family_id: None,
            present_family_id: None,
        };
        for family in device.queue_families() {
            if families.graphics_family_id.is_none() && family.supports_graphics() {
                families.graphics_family_id = Some(family.id())
            }
            if families.present_family_id.is_none() && surface.is_supported(family).unwrap() {
                families.present_family_id = Some(family.id())
            }
            if families._is_complete() {
                break;
            }
        }
        families
    }

    pub fn graphics_family<'a>(&self, device: &PhysicalDevice<'a>) -> Option<QueueFamily<'a>> {
        self.graphics_family_id
            .and_then(|i| device.queue_family_by_id(i))
    }

    pub fn present_family<'a>(&self, device: &PhysicalDevice<'a>) -> Option<QueueFamily<'a>> {
        self.present_family_id
            .and_then(|i| device.queue_family_by_id(i))
    }

    pub fn _is_complete(&self) -> bool {
        return self.graphics_family_id.is_some() && self.present_family_id.is_some();
    }
}
