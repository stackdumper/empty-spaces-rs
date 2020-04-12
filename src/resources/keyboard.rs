use minifb::Key;
use std::collections::HashSet;

#[derive(Default)]
pub struct Keyboard {
    pub pressed: HashSet<Key>,
}
