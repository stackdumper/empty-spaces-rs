use pg::{components, resources, systems};
use specs::prelude::*;

fn main() {
    // create world
    let mut world = World::new();

    // register components
    world.register::<components::Position>();
    world.register::<components::Velocity>();
    world.register::<components::Geometry>();
    world.register::<components::Texture>();
    world.register::<components::CameraFollow>();
    world.register::<components::Speed>();

    // insert resources
    world.insert(resources::Clock::new(30));
    world.insert(resources::Signals::default());
    world.insert(resources::Keyboard::default());
    world.insert(resources::Camera { x: 0.0, y: 0.0 });
    world.insert(resources::Textures::new());
    world.insert(resources::Tree::default());
    world.insert(resources::Map::default());

    world
        .create_entity()
        .with(components::Position { x: 0.0, y: 0.0 })
        .with(components::Velocity { x: 0.0, y: 0.0 })
        .with(components::Geometry::square(16, 16))
        .with(components::Texture(5))
        .with(components::CameraFollow)
        .with(components::Speed { x: 30.0, y: 30.0 })
        .build();

    for x in 0..20 {
        for y in 0..20 {
            world
                .create_entity()
                .with(components::Position {
                    x: (x * 32) as f32,
                    y: (y * 32) as f32,
                })
                .with(components::Velocity { x: 0.0, y: 0.0 })
                .with(components::Geometry::square(13, 13))
                .with(components::Texture(6))
                .build();
        }
    }

    // create dispatcher
    let mut dispatcher = DispatcherBuilder::new()
        .with(systems::Controls, "controls", &[])
        .with(systems::Camera, "camera", &["controls"])
        .with(systems::Position, "position", &[])
        .with(systems::Tree, "tree", &["position"])
        .with_thread_local(systems::Render::new("project-rts", 500, 400))
        .build_async(world);

    // // setup dispatcher
    // dispatcher.setup();

    // run game loop
    loop {
        // run dispatcher
        dispatcher.dispatch();
        dispatcher.wait();

        // get world
        let world = dispatcher.world_mut();

        world.maintain();

        // update clock, pause game loop
        world.get_mut::<resources::Clock>().unwrap().tick();

        // if closed, exit
        let signals = world.get_mut::<resources::Signals>().unwrap();
        if signals.close {
            break;
        }
    }
}
