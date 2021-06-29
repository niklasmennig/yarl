use rltk::{Rltk};
use specs::prelude::*;
use crate::components;

use crate::State;

pub fn startup(state : &mut State) {
    state.ecs.create_entity()
    .with(components::Position {x: 5, y: 5})
    .with(components::Drawable {glyph : rltk::to_cp437('@'), color : rltk::RGB::named(rltk::YELLOW)})
    .with(components::Player)
    .build();
}

pub fn draw_drawables(state : &mut State, ctx : &mut Rltk) {
    let positions = state.ecs.read_storage::<components::Position>();
    let drawables = state.ecs.read_storage::<components::Drawable>();

    for (pos,ren) in (&positions, &drawables).join() {
        //println!("{:?}", pos);
        ctx.set(pos.x, pos.y, ren.color, rltk::RGB::named(rltk::RED), ren.glyph);
    }
}