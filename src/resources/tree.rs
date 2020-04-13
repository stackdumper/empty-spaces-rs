use rstar::{Point, RTree};
use specs::prelude::*;

pub struct Tree {
    pub tree: RTree<TreeItem>,
}

impl Default for Tree {
    fn default() -> Self {
        Self { tree: RTree::new() }
    }
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct TreeItem(pub Option<Entity>, pub f32, pub f32);

impl<'a> Point for TreeItem {
    type Scalar = f32;

    const DIMENSIONS: usize = 2;

    fn generate(generator: impl Fn(usize) -> Self::Scalar) -> Self {
        Self {
            0: Option::None,
            1: generator(0),
            2: generator(1),
        }
    }

    fn nth(&self, index: usize) -> Self::Scalar {
        if index == 0 {
            return self.1;
        }

        if index == 1 {
            return self.2;
        }

        panic!("unknown scalar");
    }

    fn nth_mut(&mut self, index: usize) -> &mut Self::Scalar {
        if index == 0 {
            return &mut self.1;
        }

        if index == 1 {
            return &mut self.2;
        }

        panic!("unknown scalar");
    }
}
