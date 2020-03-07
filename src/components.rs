use std::ops;
use specs::prelude::*;
use rltk::Point;

#[derive(Component)]
pub struct Viewshed {
    pub visible_tiles: Vec<Point>,
    pub range: i32,
    pub dirty: bool,
}

#[derive(Component)]
pub struct Monster;

#[derive(Component)]
pub struct Name(pub String);

impl ops::Deref for Name {
    type Target = str;

    fn deref(&self) -> &str {
        &self.0
    }
}

impl ops::DerefMut for Name {
    fn deref_mut(&mut self) -> &mut str {
        &mut self.0
    }
}

