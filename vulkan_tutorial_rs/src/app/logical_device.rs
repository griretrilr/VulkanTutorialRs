use crate::app::config::required_device_extensions;
use crate::app::PhysicalDeviceInfo;
use crate::vulkano_ext::QueueFamilyExt;
use std::collections::BTreeSet;
use std::sync::Arc;
use vulkano::device::{Device, Features, Queue};

pub struct LogicalDevice {
    device: Arc<Device>,
    graphics_queue: Arc<Queue>,
    present_queue: Arc<Queue>,
}

impl LogicalDevice {
    pub fn new(physical_device_info: &PhysicalDeviceInfo) -> LogicalDevice {
        let physical_device = physical_device_info.physical_device();

        let features = Features::none();

        let extensions = required_device_extensions();

        let families_and_priorities = {
            let families = physical_device_info.queue_families();
            let graphics_family =
                QueueFamilyExt::new(families.graphics_family(&physical_device).unwrap());
            let present_family =
                QueueFamilyExt::new(families.present_family(&physical_device).unwrap());
            let queue_priority = 1.0f32;
            let mut families_set: BTreeSet<QueueFamilyExt> = BTreeSet::new();
            families_set.insert(graphics_family);
            families_set.insert(present_family);
            families_set
                .into_iter()
                .map(move |f| (f.inner(), queue_priority))
        };

        let (device, mut queues) = vulkano::device::Device::new(
            physical_device,
            &features,
            &extensions,
            families_and_priorities,
        )
        .unwrap();

        let graphics_queue = queues.next().unwrap();
        let present_queue = queues.next().unwrap_or_else(|| graphics_queue.clone());

        LogicalDevice {
            device,
            graphics_queue,
            present_queue,
        }
    }

    pub fn device(&self) -> &Arc<Device> {
        &self.device
    }

    pub fn graphics_queue(&self) -> &Arc<Queue> {
        &self.graphics_queue
    }

    pub fn present_queue(&self) -> &Arc<Queue> {
        &self.present_queue
    }
}
