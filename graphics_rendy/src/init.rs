use rendy;

#[cfg(feature = "dx12")]
type Backend = rendy::dx12::Backend;

#[cfg(feature = "metal")]
type Backend = rendy::metal::Backend;

#[cfg(feature = "vulkan")]
type Backend = rendy::vulkan::Backend;

#[cfg(any(feature = "dx12", feature = "metal", feature = "vulkan"))]
pub struct GraphicsSurface {
    factory: rendy::factory::Factory
}

#[cfg(any(feature = "dx12", feature = "metal", feature = "vulkan"))]
impl GraphicsSurface {
    pub fn new() -> GraphicsSurface {
        let config: Config = Default::default();
        let (factory, families): (Factory<Backend>, _) = rendy::factory::init(config).unwrap();

        GraphicsSurface{
            factory: factory,
            queue_families: families
        }
    }
}