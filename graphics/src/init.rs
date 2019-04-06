use std::sync::Arc;

// use vulkano::instance::{Instance, PhysicalDevice, QueueFamily};
// use vulkano::swapchain::{Surface};
// use vulkano::device::{Device, DeviceExtensions, Queue};
// use vulkano::swapchain::{AcquireError, PresentMode, SurfaceTransform, Swapchain, SwapchainCreationError};
// use vulkano::swapchain;

use vulkano::buffer::{BufferUsage, CpuAccessibleBuffer};
use vulkano::command_buffer::{AutoCommandBufferBuilder, DynamicState};
use vulkano::device::{Device, DeviceExtensions, Queue};
use vulkano::framebuffer::{Framebuffer, FramebufferAbstract, Subpass, RenderPassAbstract};
use vulkano::image::SwapchainImage;
use vulkano::instance::{Instance, PhysicalDevice, QueueFamily};
use vulkano::pipeline::GraphicsPipeline;
use vulkano::pipeline::viewport::Viewport;
use vulkano::swapchain::{Surface, AcquireError, PresentMode, SurfaceTransform, Swapchain, SwapchainCreationError};
use vulkano::swapchain;
use vulkano::sync::{GpuFuture, FlushError};
use vulkano::sync;

use winit::{EventsLoop, WindowBuilder};

use vulkano_win::VkSurfaceBuild;

pub struct GraphicsSurface<'a> {
    instance: Option<Arc<Instance>>,
    physical: Option<PhysicalDevice<'a>>,
    surface: Option<Arc<Surface<winit::Window>>>,
    queue_family: Option<QueueFamily<'a>>,
    queue: Option<Arc<Queue>>,
    device: Option<Arc<Device>>
}

impl<'a> GraphicsSurface<'a> {
    pub fn new() -> GraphicsSurface<'a> {
        let mut gs = GraphicsSurface{
            instance: None,
            physical: None,
            surface: None,
            queue_family: None,
            queue: None,
            device: None
        };

        gs.instance = Some(GraphicsSurface::new_instance());
        gs = GraphicsSurface::new_physical(gs);
        gs = GraphicsSurface::new_surface(gs);
        gs = GraphicsSurface::new_queue_family(gs);
        gs = GraphicsSurface::new_device(gs);
        gs
    }

    pub fn get_swapchain(&mut self) -> Option<(Arc<Swapchain<winit::Window>>, Vec<Arc<SwapchainImage<winit::Window>>>)> {
        let (swapchain, images) = {
            if let Some(ref surface) = self.surface {
                if let Some(physical) = self.physical {
                    let caps = surface.capabilities(physical).unwrap();
                    let usage = caps.supported_usage_flags;
                    let alpha = caps.supported_composite_alpha.iter().next().unwrap();
                    let format = caps.supported_formats[0].0;
                    
                    let window = surface.window();

                    let initial_dimensions = 
                        if let Some(dimensions) = window.get_inner_size() {
                            let dimensions: (u32, u32) = dimensions.to_physical(window.get_hidpi_factor()).into();
                            [dimensions.0, dimensions.1]
                        } else {
                            return None;
                        };

                    let device = self.device.clone().unwrap();
                    let queue = self.queue.clone().unwrap();
                    Swapchain::new(device, surface.clone(), caps.min_image_count, format,
                        initial_dimensions, 1, usage, &queue, SurfaceTransform::Identity, alpha,
                        PresentMode::Fifo, true, None).unwrap()
                } else {
                    return None;
                }
            } else {
                return None;
            }
        };
        return Some((swapchain, images));
    }

    /// Creates a new Vulkan Instance, which is the first thing a Vulkan application
    /// must do. It stores all per-application state.
    fn new_instance() -> Arc<Instance> {
        let instance = {
            let extensions = vulkano_win::required_extensions();
            Instance::new(None, &extensions, None).expect("Failed to create Vulkan instance")
        };
        instance
    }

    /// Chooses a physical device to use. We opt to use the first one we find.
    fn new_physical(self) -> GraphicsSurface<'a> {
        if let Some(ref instance) = self.instance {
            let physical = PhysicalDevice::enumerate(instance).next().unwrap();
            debug!("Using device: {} (type: {:?})", physical.name(), physical.ty());
        }
        return self;
    }

    /// Creates a drawable surface using `winit`.
    fn new_surface(mut self) -> GraphicsSurface<'a> {
        if let Some(ref instance) = self.instance {
            let events_loop = EventsLoop::new();
            self.surface = Some(WindowBuilder::new().build_vk_surface(&events_loop, instance.clone()).unwrap());
            return self;
        }
        return self;
        
    }

    /// This returns a queue family that supports drawing to our `Surface`. 
    fn new_queue_family(mut self) -> GraphicsSurface<'a> {
        if let Some(ref physical) = self.physical {
            if let Some(ref surface) = self.surface {            
                let queue_family: QueueFamily<'a> = physical.queue_families().find(|&q| {
                    // We take the first queue that supports drawing to our window.
                    q.supports_graphics() && surface.is_supported(q).unwrap_or(false)
                }).unwrap();
                self.queue_family = Some(queue_family);
                return self;
            }
        }
        return self;
    }

    /// This initializes the device and returns the queue from the queue family
    //fn new_device(physical: PhysicalDevice, queue_family: QueueFamily<'a>) -> (Arc<Queue>, Arc<Device>) {
    fn new_device(mut self) -> GraphicsSurface<'a> {
        if let Some(physical) = self.physical {
            if let Some(queue_family) = self.queue_family {
                let device_ext = DeviceExtensions { khr_swapchain: true, .. DeviceExtensions::none() };
                let (device, mut queues) = Device::new(physical, physical.supported_features(), &device_ext, [(queue_family, 0.5)].iter().cloned()).unwrap();
                let queue = queues.next().unwrap();
                self.queue = Some(queue);
                self.device = Some(device);
                return self;
            }
        }
        return self;
    }
}