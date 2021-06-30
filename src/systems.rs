use rltk::{Rltk};
use specs::prelude::*;
use crate::components;

use crate::State;
use crate::map::Map;
use crate::map::StaticMap;
use crate::map::map_glyph;

pub fn startup(state : &mut State) {
    // generate map
    let mut map = crate::generators::generate_random_map(50, 50);
    state.ecs.insert(map);

    // create player
    state.ecs.create_entity()
    .with(components::Position {x: 5, y: 5})
    .with(components::Drawable {glyph : rltk::to_cp437('@'), color : rltk::RGB::named(rltk::YELLOW)})
    .with(components::Player)
    .build();
}

pub fn draw_map(state : &mut State, ctx : &mut Rltk) {
    let map = state.ecs.read_resource::<StaticMap>();

    for x in 0..map.get_size().0 {
        for y in 0..map.get_size().1 {
            let tile = map.get_tile(x, y);
            match map_glyph(tile) {
                Some(g) => ctx.set(x, y, rltk::GREEN, rltk::BLACK, g),
                _ => ()
            }
        }
    }
}

pub fn draw_drawables(state : &mut State, ctx : &mut Rltk) {
    let positions = state.ecs.read_storage::<components::Position>();
    let drawables = state.ecs.read_storage::<components::Drawable>();

    for (pos,ren) in (&positions, &drawables).join() {
        //println!("{:?}", pos);
        ctx.set(pos.x, pos.y, ren.color, rltk::RGB::named(rltk::RED), ren.glyph);
    }
}