pub mod components;
pub mod resources;
pub mod systems;

use specs::prelude::*;

pub fn create_dispatcher<'a>() -> AsyncDispatcher<'a, World> {
    // create world
    let mut world = World::new();

    // register components
    world.register::<components::Position>();
    world.register::<components::Velocity>();
    world.register::<components::Texture>();

    // insert resources
    world.insert(resources::Clock::new(30));
    world.insert(resources::Map::new(1600, 900));
    world.insert(resources::SDL::new(1600, 900));
    world.insert(resources::Assets::new("src/assets"));

    // create dispatcher
    return DispatcherBuilder::new()
        .with(systems::Position, "position", &[])
        .with_thread_local(systems::Render)
        .build_async(world);
}
