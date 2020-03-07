use specs::prelude::*;
use crate::components::Viewshed;
use crate::{map::Map, Position, player::Player};
use rltk::{field_of_view, Point};

pub struct Visibility;

impl<'a> System<'a> for Visibility {
    type SystemData = (
        WriteExpect<'a, Map>,
        Entities<'a>,
        WriteStorage<'a, Viewshed>,
        WriteStorage<'a, Position>,
        ReadStorage<'a, Player>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (mut map, entities, mut viewshed, pos, player) = data;

        for (ent, viewshed, pos) in (&entities, &mut viewshed, &pos).join() {
            if !viewshed.dirty {
                continue;
            }
            
            viewshed.dirty = false;
            
            let center = Point::new(pos.x, pos.y);
            let range = viewshed.range;

            viewshed.visible_tiles = field_of_view(center, range, &*map);

            if player.get(ent).is_some() {
                for visible in &mut map.visible_tiles {
                    *visible = false;
                }

                for visible in &viewshed.visible_tiles {
                    let idx = map.xy_idx(visible.x, visible.y);

                    map.revealed_tiles[idx] = true;
                    map.visible_tiles[idx] = true;
                }
            }
        }
    }    
}