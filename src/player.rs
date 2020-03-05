use rltk::{VirtualKeyCode, Rltk};
use specs::prelude::*;
use crate::{State, Position};
use crate::map::{TileType, xy_idx};

#[derive(Component)]
pub struct Player;

pub fn try_move_player(delta_x: i32, delta_y: i32, ecs: &mut World) {
    let mut positions = ecs.write_storage::<Position>();
    let mut players = ecs.write_storage::<Player>();
    let map = ecs.fetch::<Vec<TileType>>();

    for (_player, pos) in (&mut players, &mut positions).join() {
        let dest_x = pos.x + delta_x;
        let dest_y = pos.y + delta_y;
        let destination_idx = xy_idx(dest_x, dest_y);

        if map[destination_idx] != TileType::Wall {
            pos.x = 0.max(dest_x).min(79);
            pos.y = 0.max(dest_y).min(49);
        }
    }
}

pub fn player_input(gs: &mut State, ctx: &mut Rltk) {
    // Player movement
    match ctx.key {
        None => {},
        Some(key) => match key {
            // Arrows
            VirtualKeyCode::Left => try_move_player(-1, 0, &mut gs.ecs),
            VirtualKeyCode::Right => try_move_player(1, 0, &mut gs.ecs),
            VirtualKeyCode::Up => try_move_player(0, -1, &mut gs.ecs),
            VirtualKeyCode::Down => try_move_player(0, 1, &mut gs.ecs),
            // Neo - nrtd
            VirtualKeyCode::N => try_move_player(-1, 0, &mut gs.ecs),
            VirtualKeyCode::D => try_move_player(1, 0, &mut gs.ecs),
            VirtualKeyCode::T => try_move_player(0, -1, &mut gs.ecs),
            VirtualKeyCode::R => try_move_player(0, 1, &mut gs.ecs),
            _ => {},
        }
    }
}
