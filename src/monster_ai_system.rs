extern crate specs;
use specs::prelude::*;
use super::{Viewshed, Map, Monster, Name, Position, WantsToMelee};
extern crate rltk;
use rltk::{Point, console};

pub struct MonsterAI {}

impl<'a> System<'a> for MonsterAI {
  type SystemData = ( WriteExpect<'a, Map>,
                      ReadExpect<'a, Point>,
                      ReadStorage<'a, Viewshed>,
                      ReadStorage<'a, Monster>,
                      ReadStorage<'a, Name>,
                      WriteStorage<'a, Position>,
                      WriteStorage<'a, WantsToMelee>);

  fn run(&mut self, data: Self::SystemData) {
    let (mut map, player_pos, mut viewshed, monster, name, mut position, mut wants_to_melee) = data;

    for (mut viewshed, _monster, mut pos) in (&mut viewshed, &monster, &mut position).join() {
      let distance = rltk::DistanceAlg::Pythagoras.distance2d(Point::new(pos.x, pos.y), *player_pos);
      if distance < 1.5 {
          wants_to_melee.insert(entity, WantsToMelee{ target: *player_entity }).expect("Unable to insert attack");
      }

    	if viewshed.visible_tiles.contains(&*player_pos) {
        let path = rltk::a_star_search(
            map.xy_idx(pos.x, pos.y) as i32,
            map.xy_idx(player_pos.x, player_pos.y) as i32,
            &mut *map
        );
        if path.success && path.steps.len()>1 {
            pos.x = path.steps[1] as i32 % map.width;
            pos.y = path.steps[1] as i32 / map.width;
            viewshed.dirty = true;
        }
    	}
    }
  }
}
