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
    world.insert(resources::Clock::new(60));
    world.insert(resources::Signals::default());
    world.insert(resources::Keyboard::default());
    world.insert(resources::Camera { x: 0.0, y: 0.0 });
    world.insert(resources::Textures::new());

    // insert entity

    world
        .create_entity()
        .with(components::Position { x: 0.0, y: 0.0 })
        .with(components::Velocity { x: 0.0, y: 0.0 })
        .with(components::Geometry::square(16, 16))
        .with(components::Texture(0))
        .with(components::CameraFollow)
        .with(components::Speed { x: 30.0, y: 30.0 })
        .build();

    for x in 0..1 {
        for y in 0..1 {
            world
                .create_entity()
                .with(components::Position {
                    x: (x * 8) as f32,
                    y: (y * 8) as f32,
                })
                .with(components::Velocity { x: 0.0, y: 0.0 })
                .with(components::Geometry::square(13, 13))
                .with(components::Texture(1))
                .build();
        }
    }

    // create dispatcher
    let mut dispatcher = DispatcherBuilder::new()
        .with(systems::Position, "position", &[])
        .with(systems::Camera, "camera", &[])
        .with(systems::Controls, "controls", &[])
        .with_thread_local(systems::Render::new("project-rts", 1000, 800))
        .build();

    // setup dispatcher
    dispatcher.setup(&mut world);

    // run game loop
    loop {
        // if closed, exit
        let signals = world.get_mut::<resources::Signals>().unwrap();
        if signals.close {
            break;
        }

        // run dispatcher
        dispatcher.dispatch(&mut world);

        // update clock, pause game loop
        world.get_mut::<resources::Clock>().unwrap().tick();
    }
}
