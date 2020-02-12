use specs::prelude::*;
use rltk::{RGB};

#[derive(Component)]
pub struct Position {
    pub x: i32,
    pub y: i32,
}

#[derive(Component)]
pub struct Renderable {
    pub glyph: u8,
    pub fg: RGB,
    pub bg: RGB,
}

#[derive(Component, Debug)]
pub struct Player {}

#[derive(Component, Debug)]
pub struct Monster {}

#[derive(Component)]
pub struct Viewshed {
	pub visible_tiles: Vec<rltk::Point>,
	pub range: i32,
	pub dirty: bool,
}

#[derive(PartialEq, Copy, Clone)]
pub enum RunState {
	Paused,
	Running,
}

#[derive(Component, Debug)]
pub struct Name {
	pub name: String
}
