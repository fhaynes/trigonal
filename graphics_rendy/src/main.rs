
use graphics_rendy;
use env_logger;
use log;

#[cfg(any(feature = "dx12", feature = "metal", feature = "vulkan"))]
fn main() {
    env_logger::Builder::from_default_env()
        .filter_module("basic", log::LevelFilter::Trace)
        .init();

    let mut surface = graphics_rendy::init::Context::new("Test");
    let mut frames = 0u64..;

    for _ in &mut frames {
        surface.factory.maintain(&mut surface.queue_families);
        surface.event_loop.poll_events(|_| ());
        surface.graph.run(&mut surface.factory, &mut surface.queue_families, &mut ());
    }

    println!("Done!");
}