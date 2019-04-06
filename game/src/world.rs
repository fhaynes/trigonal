//! Top level module that contains the entire game world

use graphics::init;

/// A `World` consists of the game window and what is drawn on it. It is responsible for
/// the game loop.
pub struct World<'a> {
    drawable_surface: init::GraphicsSurface<'a>
}

impl<'a> World<'a> {
    pub fn new() -> World<'a> {
        World {
            drawable_surface: init::GraphicsSurface::new()
        }
    }
}