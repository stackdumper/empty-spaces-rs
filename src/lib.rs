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

    // insert resources
    world.insert(resources::Clock::new(30));
    world.insert(resources::Map::new(32, 32));
    world.insert(resources::SDL::default());

    // create dispatcher
    return DispatcherBuilder::new()
        .with(systems::Position, "position", &[])
        .build_async(world);
}
