use map::Map;
use rltk::{Rltk, GameState};
use specs::prelude::*;

mod components;
mod systems;

mod map;
mod generators;


pub struct State {
    ecs : World,
}

impl GameState for State {
    fn tick(&mut self, ctx: &mut Rltk) {
        ctx.cls();

        handle_input(self, ctx);

        ctx.print(1,1,"Test");

        systems::calculate_visibility(self);
        systems::draw_map(self, ctx);
        systems::draw_drawables(self, ctx);

        self.ecs.maintain();
    }
}

fn handle_input(gs: &mut State, ctx: &mut Rltk) {
    let ecs = &mut gs.ecs;
    match ctx.key {
        None => {}
        Some(key) => match key {
            rltk::VirtualKeyCode::W => try_move_player(0, -1, ecs),
            rltk::VirtualKeyCode::A => try_move_player(-1, 0, ecs),
            rltk::VirtualKeyCode::S => try_move_player(0, 1, ecs),
            rltk::VirtualKeyCode::D => try_move_player(1, 0, ecs),

            rltk::VirtualKeyCode::Numpad1 => try_move_player(-1, 1, ecs),
            rltk::VirtualKeyCode::Numpad2 => try_move_player(0, 1, ecs),
            rltk::VirtualKeyCode::Numpad3 => try_move_player(1, 1, ecs),
            rltk::VirtualKeyCode::Numpad4 => try_move_player(-1, 0, ecs),
            rltk::VirtualKeyCode::Numpad5 => try_move_player(0, 0, ecs),
            rltk::VirtualKeyCode::Numpad6 => try_move_player(1, 0, ecs),
            rltk::VirtualKeyCode::Numpad7 => try_move_player(-1, -1, ecs),
            rltk::VirtualKeyCode::Numpad8 => try_move_player(0, -1, ecs),
            rltk::VirtualKeyCode::Numpad9 => try_move_player(1, -1, ecs),

            _=>{}
        }
    }
}

fn try_move_player (dx: i32, dy: i32, ecs: &mut World) {
    let mut positions = ecs.write_storage::<components::Position>();
    let mut players = ecs.write_storage::<components::Player>(); 
    let mut map = ecs.read_resource::<map::StaticMap>();

    for (pl, pos) in (&mut players, &mut positions).join() {
        let end_x = pos.x + dx;
        let end_y = pos.y + dy;

        if end_x < 0 || end_x >= map.get_size().0 || end_y < 0 || end_y >= map.get_size().1 {
            continue;
        }
        
        match map.get_tile(end_x, end_y) {
            map::MapTileType::WALL => (),
            map::MapTileType::EMPTY => {pos.x = end_x; pos.y = end_y;}            
        };

    }
}


fn main() -> rltk::BError {
    use rltk::RltkBuilder;
    let ctx = RltkBuilder::simple80x50()
    .with_title("roguelike")
    .build()?;

    let mut gs = State{
        ecs : World::new(),
    };

    gs.ecs.register::<components::Position>();
    gs.ecs.register::<components::Drawable>();
    gs.ecs.register::<components::Player>();
    gs.ecs.register::<components::Viewshed>();

    systems::startup(&mut gs);

    rltk::main_loop(ctx, gs)
}