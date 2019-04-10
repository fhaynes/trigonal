use std::marker::PhantomData;

use {
    rendy::{
        command::{Families},
        factory::{Config, Factory},
        graph::{
            present::PresentNode, GraphBuilder, Graph
        }
    },
};

use winit::{EventsLoop, WindowBuilder, Window};


#[derive(Debug)]
#[cfg(any(feature = "dx12", feature = "metal", feature = "vulkan"))]
pub struct Aux<B: gfx_hal::Backend> {
    frames: usize,
    align: u64,
    phantom: PhantomData<B>,
}

#[cfg(feature = "dx12")]
type Backend = rendy::dx12::Backend;

#[cfg(feature = "metal")]
type Backend = rendy::metal::Backend;

#[cfg(feature = "vulkan")]
type Backend = rendy::vulkan::Backend;

#[cfg(any(feature = "dx12", feature = "metal", feature = "vulkan"))]
pub struct Context {
    pub factory: Factory<Backend>,
    pub graph: Box<Graph<Backend, ()>>,
    pub event_loop: EventsLoop,
    pub window: Window,
    pub queue_families: Families<Backend>
    
}

#[cfg(any(feature = "dx12", feature = "metal", feature = "vulkan"))]
impl Context {
    pub fn new(title: &str) -> Context {
        let config: Config = Default::default();
        let (mut factory, mut families): (Factory<Backend>, _) = rendy::factory::init(config).unwrap();
        let event_loop = EventsLoop::new();
        let window = WindowBuilder::new()
            .with_title(title)
            .build(&event_loop)
            .unwrap();
        let graph_builder = GraphBuilder::<Backend, ()>::new();
        let graph = graph_builder.build(&mut factory, &mut families, &mut ()).unwrap();
        Context{
            factory: factory,
            queue_families: families,
            event_loop: event_loop,
            window: window,
            graph: Box::new(graph)
        }
    }
}


#[cfg(any(feature = "dx12", feature = "metal", feature = "vulkan"))]
#[cfg(test)]
mod tests {
    use super::*;
    use std::thread::sleep;
    use std::time::Duration;
    #[test]
    fn create_context() {
        let _new_context = Context::new("Test");
    }
}