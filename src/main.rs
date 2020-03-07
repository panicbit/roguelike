use rltk::{Rltk, GameState, Console, RGB};
use specs::prelude::*;

#[macro_use]
extern crate specs_derive;

mod components;
use components::*;

mod map;
use map::*;

mod player;
use player::*;

mod rect;
use rect::*;

mod systems;
use systems::*;

fn main() {
    use rltk::RltkBuilder;

    let context = RltkBuilder::simple80x50()
        .with_title("Roguelike Tutorial")
        .build();
    
    let mut gs = State {
        ecs: World::new(),
        run_state: RunState::Running,
    };

    gs.ecs.register::<Position>();
    gs.ecs.register::<Renderable>();
    gs.ecs.register::<Player>();
    gs.ecs.register::<Viewshed>();
    gs.ecs.register::<Monster>();
    gs.ecs.register::<Name>();

    let map = Map::new_map_rooms_and_corridoors();
    // let map = Map::new_map_test();
    let (player_x, player_y) = map.rooms[0].center();
    
    gs.ecs.insert(PlayerPosition::new(player_x, player_y));

    // Spawn player
    gs.ecs
        .create_entity()
        .with(Player)
        .with(Name("You".into()))
        .with(Position { x: player_x, y: player_y })
        .with(Renderable {
            glyph: rltk::to_cp437('@'),
            fg: RGB::named(rltk::YELLOW),
            bg: RGB::named(rltk::BLACK),
        })
        .with(Viewshed {
            visible_tiles: vec![],
            range: 8,
            dirty: true,
        })
        .build();

    // Spawn enemies
    let mut rng = rltk::RandomNumberGenerator::new();
    for (i, room) in map.rooms.iter().skip(1).enumerate() {
        let (x, y) = room.center();
        let roll = rng.roll_dice(1, 2);

        let (glyph, name) = match roll {
            1 => ('g', "Goblin"),
            _ => ('o', "Orc"),
        };
        let glyph = rltk::to_cp437(glyph);
        let name = format!("{} #{}", name, i);

        gs.ecs.create_entity()
            .with(Monster)
            .with(Name(name))
            .with(Position { x, y })
            .with(Renderable {
                glyph,
                fg: RGB::named(rltk::RED),
                bg: RGB::named(rltk::BLACK),
            })
            .with(Viewshed {
                visible_tiles: vec![],
                range: 8,
                dirty: true,
            })
            .build();
    }

    gs.ecs.insert(map);

    rltk::main_loop(context, gs);
}

pub struct State {
    ecs: World,
    run_state: RunState,
}

impl GameState for State {
    fn tick(&mut self, ctx: &mut Rltk) {
        ctx.cls();

        if self.run_state == RunState::Running {
            self.run_systems();
            self.run_state = RunState::Paused;
        } else {
            self.run_state = player_input(self, ctx);
        }

        draw_map(&self.ecs, ctx);

        let positions = self.ecs.read_storage::<Position>();
        let renderables = self.ecs.read_storage::<Renderable>();
        let map = self.ecs.fetch::<Map>();

        // Render entities
        for (pos, render) in (&positions, &renderables).join() {
            let idx = map.xy_idx(pos.x, pos.y);

            if map.visible_tiles[idx] {
                ctx.set(pos.x, pos.y, render.fg, render.bg, render.glyph);
            }
        }
    }
}

impl State {
    fn run_systems(&mut self) {
        Visibility.run_now(&self.ecs);
        MonsterAI.run_now(&self.ecs);

        self.ecs.maintain();
    }
}

#[derive(Copy, Clone, PartialEq)]
pub enum RunState {
    Paused,
    Running,
}

#[derive(Component)]
pub struct Position {
    x: i32,
    y: i32,
}

#[derive(Component)]
pub struct Renderable {
    glyph: u8,
    fg: RGB,
    bg: RGB,
}
