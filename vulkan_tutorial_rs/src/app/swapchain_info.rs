use crate::app::LogicalDevice;
use std::sync::Arc;
use vulkano::format::Format;
use vulkano::image::{ImageUsage, SwapchainImage};
use vulkano::instance::PhysicalDevice;
use vulkano::swapchain::{
    Capabilities, ColorSpace, CompositeAlpha, FullscreenExclusive, PresentMode,
    SupportedPresentModes, Surface, Swapchain,
};
use vulkano::sync::SharingMode;
use winit::window::Window;

pub struct SwapchainInfo {
    _swapchain: Arc<Swapchain<Window>>,
    _images: Vec<Arc<SwapchainImage<Window>>>,
    _format: Format,
    _dimensions: [u32; 2],
}

impl SwapchainInfo {
    pub fn new(
        surface: &Arc<Surface<Window>>,
        physical_device: &PhysicalDevice,
        logical_device: &LogicalDevice,
    ) -> SwapchainInfo {
        let surface_capabilities = surface
            .capabilities(*physical_device)
            .expect("failed to get surface capabilities");
        let (format, color_space) = *choose_format(&surface_capabilities.supported_formats);
        let dimensions = choose_dimensions(&surface_capabilities);
        let num_images = choose_image_count(&surface_capabilities);
        let layers = 1u32;
        let usage = ImageUsage::color_attachment();
        let sharing = choose_sharing_mode(logical_device);
        let transform = surface_capabilities.current_transform;
        let alpha = CompositeAlpha::Opaque;
        let mode = choose_present_mode(&surface_capabilities.present_modes);
        let fullscreen_exclusive = FullscreenExclusive::Default;
        let clipped = true;
        let (swapchain, images) = vulkano::swapchain::Swapchain::new(
            logical_device.device().clone(),
            surface.clone(),
            num_images,
            format,
            dimensions,
            layers,
            usage,
            sharing,
            transform,
            alpha,
            mode,
            fullscreen_exclusive,
            clipped,
            color_space,
        )
        .expect("failed to create swapchain");
        SwapchainInfo {
            _swapchain: swapchain,
            _images: images,
            _format: format,
            _dimensions: dimensions,
        }
    }

    pub fn _swapchain(&self) -> &Arc<Swapchain<Window>> {
        &self._swapchain
    }

    pub fn _images(&self) -> &Vec<Arc<SwapchainImage<Window>>> {
        &self._images
    }
}

fn choose_format(formats: &Vec<(Format, ColorSpace)>) -> &(Format, ColorSpace) {
    let ideal_format = formats
        .iter()
        .filter(|&(f, c)| *f == Format::B8G8R8A8Srgb && *c == ColorSpace::SrgbNonLinear)
        .next();
    if ideal_format.is_some() {
        return ideal_format.unwrap();
    }
    &formats[0]
}

fn choose_present_mode(modes: &SupportedPresentModes) -> PresentMode {
    if modes.mailbox {
        PresentMode::Mailbox
    } else {
        assert!(
            modes.fifo,
            "Fifo support is required by the Vulkan standard"
        );
        PresentMode::Fifo
    }
}

fn choose_dimensions(c: &Capabilities) -> [u32; 2] {
    if c.current_extent.is_some() {
        return c.current_extent.unwrap();
    }
    let width = num::clamp(
        crate::app::config::DEFAULT_WIDTH,
        c.min_image_extent[0],
        c.max_image_extent[0],
    );
    let height = num::clamp(
        crate::app::config::DEFAULT_HEIGHT,
        c.min_image_extent[1],
        c.max_image_extent[1],
    );
    [width, height]
}

fn choose_image_count(c: &Capabilities) -> u32 {
    // ideally we get at least one more image than the minimum so we aren't waiting on the driver
    let ideal = c.min_image_count + 1;
    match c.max_image_count {
        Some(max) if ideal > max => max,
        _ => ideal,
    }
}

fn choose_sharing_mode(ld: &LogicalDevice) -> SharingMode {
    if ld.graphics_queue().family() == ld.present_queue().family() {
        ld.graphics_queue().into()
    } else {
        vec![ld.graphics_queue(), ld.present_queue()]
            .as_slice()
            .into()
    }
}
