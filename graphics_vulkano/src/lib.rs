// The `vulkano` crate is the main crate that you must use to use Vulkan.
extern crate vulkano;
// Provides the `shader!` macro that is used to generate code for using shaders.
extern crate vulkano_shaders;
// The Vulkan library doesn't provide any functionality to create and handle windows, as
// this would be out of scope. In order to open a window, we are going to use the `winit` crate.
extern crate winit;
// The `vulkano_win` crate is the link between `vulkano` and `winit`. Vulkano doesn't know about
// winit, and winit doesn't know about vulkano, so import a crate that will provide a link between
// the two.
extern crate vulkano_win;

#[macro_use] 
extern crate log;
extern crate env_logger;

pub mod init;