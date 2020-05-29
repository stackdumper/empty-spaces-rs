use beryllium::*;
use pg::{components, create_dispatcher, resources};
use rand::{thread_rng, Rng};
use specs::prelude::*;

fn main() {
    // create dispatcher
    let mut dispatcher = create_dispatcher();

    // insert entities
    let mut rng = thread_rng();
    for _ in 0..100 {
        dispatcher
            .world_mut()
            .create_entity()
            .with(components::Position::new(
                rng.gen_range(750.0, 850.0),
                rng.gen_range(400.0, 500.0),
            ))
            .with(components::Velocity::new(
                rng.gen_range(-5.0, 5.0),
                rng.gen_range(-5.0, 5.0),
            ))
            .with(components::Texture::new(String::from(
                "src/assets/asteroid.png",
            )))
            .build();
    }

    // run game loop
    loop {
        // run dispatcher
        dispatcher.dispatch();

        // wait for dispatch to finish
        dispatcher.wait();

        // get world
        let world = dispatcher.world_mut();

        // maintain world (removes entities)
        world.maintain();

        // update clock, pause game loop
        world.get_mut::<resources::Clock>().unwrap().tick();

        // check exit
        match world
            .get_mut::<resources::SDL>()
            .unwrap()
            .sdl
            .lock()
            .unwrap()
            .poll_events()
            .and_then(Result::ok)
        {
            Some(Event::Quit(QuitEvent { .. })) => break,
            _ => {}
        }
    }
}
