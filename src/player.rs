use std::ops;
use rltk::{VirtualKeyCode, Rltk, Point};
use specs::prelude::*;
use crate::{State, Position};
use crate::map::{Map, TileType};
use crate::{RunState, components::Viewshed};

#[derive(Component)]
pub struct Player;

pub fn try_move_player(delta_x: i32, delta_y: i32, ecs: &mut World) {
    let mut positions = ecs.write_storage::<Position>();
    let mut players = ecs.write_storage::<Player>();
    let mut viewsheds = ecs.write_storage::<Viewshed>();
    let map = ecs.fetch::<Map>();

    for (_player, pos, viewshed) in (&mut players, &mut positions, &mut viewsheds).join() {
        let dest_x = pos.x + delta_x;
        let dest_y = pos.y + delta_y;
        let destination_idx = map.xy_idx(dest_x, dest_y);

        if map.tiles[destination_idx] != TileType::Wall {
            pos.x = 0.max(dest_x).min(79);
            pos.y = 0.max(dest_y).min(49);

            let mut player_position = ecs.write_resource::<PlayerPosition>();
            player_position.x = pos.x;
            player_position.y = pos.y;

            viewshed.dirty = true;
        }
    }
}

pub fn player_input(gs: &mut State, ctx: &mut Rltk) -> RunState {
    // Player movement
    let key = match ctx.key {
        Some(key) => key,
        None => return RunState::Paused,
    };

    match key {
        // Arrows
        VirtualKeyCode::Left => try_move_player(-1, 0, &mut gs.ecs),
        VirtualKeyCode::Down => try_move_player(0, 1, &mut gs.ecs),
        VirtualKeyCode::Up => try_move_player(0, -1, &mut gs.ecs),
        VirtualKeyCode::Right => try_move_player(1, 0, &mut gs.ecs),
        // Neo - nrtd
        VirtualKeyCode::N => try_move_player(-1, 0, &mut gs.ecs),
        VirtualKeyCode::R => try_move_player(0, 1, &mut gs.ecs),
        VirtualKeyCode::T => try_move_player(0, -1, &mut gs.ecs),
        VirtualKeyCode::D => try_move_player(1, 0, &mut gs.ecs),

        VirtualKeyCode::H => try_move_player(-1, -1, &mut gs.ecs),
        VirtualKeyCode::G => try_move_player(1, -1, &mut gs.ecs),
        VirtualKeyCode::B => try_move_player(-1, 1, &mut gs.ecs),
        VirtualKeyCode::M => try_move_player(1, 1, &mut gs.ecs),
        _ => return RunState::Paused,
    }

    RunState::Running
}

#[derive(Copy, Clone, PartialEq)]
pub struct PlayerPosition(pub Point);

impl PlayerPosition {
    pub fn new(x: i32, y: i32) -> Self {
        Self(Point::new(x, y))
    }
}

impl ops::Deref for PlayerPosition {
    type Target = Point;

    fn deref(&self) -> &Point {
        &self.0
    }
}

impl ops::DerefMut for PlayerPosition {
    fn deref_mut(&mut self) -> &mut Point {
        &mut self.0
    }
}
