use crate::{components, resources};
use rstar::RTree;
use specs::prelude::*;

pub struct Tree;

impl<'a> System<'a> for Tree {
    type SystemData = (
        Entities<'a>,
        ReadStorage<'a, components::Position>,
        Write<'a, resources::Tree>,
    );

    fn run(&mut self, (entities, positions, mut tree): Self::SystemData) {
        tree.tree = RTree::bulk_load(
            (&entities, &positions)
                .join()
                .into_iter()
                .map(|(entity, position)| resources::TreeItem {
                    0: Some(entity),
                    1: position.x,
                    2: position.y,
                })
                .collect(),
        )
    }
}
