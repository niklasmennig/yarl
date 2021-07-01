use rltk::Point;
use specs::prelude::*;
use specs_derive::Component;

#[derive(Component, Debug)]
pub struct Position {
    pub x : i32,
    pub y : i32
}

#[derive(Component)]
pub struct Drawable {
    pub glyph : rltk::FontCharType,
    pub color : rltk::RGB
}

#[derive(Component)]
pub struct Player;

#[derive(Component)]
pub struct Viewshed {
    pub visible_tiles : Vec<Point>,
    pub range : i32
}

impl Viewshed {
    pub fn with_range(r : i32) -> Viewshed {
        Viewshed{range : r, visible_tiles : Vec::new()}
    }
}