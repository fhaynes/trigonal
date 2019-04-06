//! Top level module that contains the entire game world

use graphics_rendy::init;

/// A `World` consists of the game window and what is drawn on it. It is responsible for
/// the game loop.

#[cfg(any(feature = "dx12", feature = "metal", feature = "vulkan"))]
pub struct World {
    drawable_surface: init::GraphicsSurface
}

#[cfg(any(feature = "dx12", feature = "metal", feature = "vulkan"))]
impl World {
    pub fn new() -> World{
        World {
            drawable_surface: init::GraphicsSurface::new()
        }
    }
}