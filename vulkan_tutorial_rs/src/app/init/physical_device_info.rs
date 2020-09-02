use crate::app::config::required_device_extensions;
use crate::app::init::indented_printer::IndentedPrinter;
use crate::app::init::queue_families::QueueFamilies;
use std::cmp::Ordering;
use std::sync::Arc;
use vulkano::device::DeviceExtensions;
use vulkano::instance::{PhysicalDevice, PhysicalDeviceType, QueueFamily};
use vulkano::swapchain::{Capabilities, Surface};
use winit::window::Window;

pub struct PhysicalDeviceInfo<'a> {
    physical_device: PhysicalDevice<'a>,
    surface: &'a Arc<Surface<Window>>,
    supported_extensions: DeviceExtensions,
}

impl PhysicalDeviceInfo<'_> {
    pub fn new<'a>(
        physical_device: PhysicalDevice<'a>,
        surface: &'a Arc<Surface<Window>>,
    ) -> PhysicalDeviceInfo<'a> {
        let supported_extensions = DeviceExtensions::supported_by_device(physical_device);
        PhysicalDeviceInfo {
            physical_device,
            surface,
            supported_extensions,
        }
    }

    pub fn physical_device(&self) -> PhysicalDevice {
        self.physical_device
    }

    pub fn queue_families(&self) -> QueueFamilies {
        QueueFamilies::new(self.physical_device(), self.surface)
    }

    pub fn graphics_family(&self) -> Option<QueueFamily> {
        self.queue_families()
            .graphics_family(&self.physical_device())
    }

    pub fn present_family(&self) -> Option<QueueFamily> {
        self.queue_families()
            .present_family(&self.physical_device())
    }

    pub fn supported_extensions(&self) -> &DeviceExtensions {
        &self.supported_extensions
    }

    pub fn supports_required_extensions(&self) -> bool {
        required_device_extensions().intersection(&self.supported_extensions())
            == required_device_extensions()
    }

    pub fn surface_capabilities(&self) -> Capabilities {
        self.surface
            .capabilities(self.physical_device())
            .expect("failed to get surface capabilities")
    }

    pub fn is_surface_valid(&self) -> bool {
        let capabilities = self.surface_capabilities();
        !capabilities.supported_formats.is_empty()
            && capabilities.present_modes.iter().next().is_some()
    }

    pub fn is_discrete_gpu(&self) -> bool {
        self.physical_device.ty() == PhysicalDeviceType::DiscreteGpu
    }

    pub fn max_image_dimension_2d(&self) -> u32 {
        self.physical_device.limits().max_image_dimension_2d()
    }

    pub fn is_valid(&self) -> bool {
        self.queue_families()._is_complete()
            && self.supports_required_extensions()
            && self.is_surface_valid()
    }

    fn cmp_valid(&self, other: &Self) -> Ordering {
        self.is_valid().cmp(&other.is_valid()).reverse()
    }

    fn cmp_discrete_gpu(&self, other: &Self) -> Ordering {
        self.is_discrete_gpu().cmp(&other.is_discrete_gpu())
    }

    fn cmp_max_image_dimension_2d(&self, other: &Self) -> Ordering {
        self.max_image_dimension_2d()
            .cmp(&other.max_image_dimension_2d())
    }

    pub fn cmp(&self, other: &Self) -> Ordering {
        self.cmp_valid(other)
            .then_with(|| self.cmp_discrete_gpu(other))
            .then_with(|| self.cmp_max_image_dimension_2d(other))
    }

    pub fn print(&self, indent_base: &str, indent_step: &str) {
        let mut printer = IndentedPrinter::new_with_base(indent_base, indent_step);
        printer.print_line(&format!(
            "[{index}] {name}",
            index = self.physical_device().index(),
            name = self.physical_device.name()
        ));
        printer.indent();
        printer.print_key_value_debug("Is Valid", &self.is_valid());
        printer.print_key_value_debug("Type", &self.physical_device.ty());
        printer.print_key_value_debug("Graphics Queue Family", &self.graphics_family());
        printer.print_key_value_debug("Present Queue Family", &self.present_family());
        printer.print_key_value_debug("Max Image Dimensions 2D", &self.max_image_dimension_2d());
    }
}
