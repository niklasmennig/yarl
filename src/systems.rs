use std::usize;

use rltk::{Rltk};
use specs::prelude::*;
use crate::components;

use crate::State;
use crate::map;
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
    .with(components::Viewshed::with_range(5))
    .build();
}

pub fn draw_map(state : &mut State, ctx : &mut Rltk) {
    let map = state.ecs.read_resource::<StaticMap>();

    let players = state.ecs.read_component::<components::Player>();
    let mut viewsheds = state.ecs.write_component::<components::Viewshed>();

    let mut visible_cells : Vec<rltk::Point> = Vec::new();

    for (p, vs) in (&players, &mut viewsheds).join() {
        visible_cells.append(&mut vs.visible_tiles);
    }

    for x in 0..map.get_size().0 {
        for y in 0..map.get_size().1 {
            if visible_cells.contains(&rltk::Point{x : x as i32, y: y as i32}) {
                let tile = map.get_tile(x, y);
                let glyph = map_glyph(tile);
                let mut fg = rltk::GREEN;
                if tile == map::MapTileType::EMPTY {
                    fg = rltk::DARKGRAY;
                } 
                ctx.set(x, y, fg, rltk::BLACK, glyph);
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

pub fn calculate_visibility(state : &mut State) {
    let mut viewsheds = state.ecs.write_component::<components::Viewshed>();
    let positions = state.ecs.read_component::<components::Position>();
    let map = state.ecs.read_resource::<StaticMap>();

    for (pos, vs) in (&positions, &mut viewsheds).join() {
        vs.visible_tiles.clear();
        vs.visible_tiles = rltk::field_of_view(rltk::Point{x:pos.x, y:pos.y}, vs.range, &*map);
    }
}