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