use crate::prelude::*;

use super::MapArchitect;

pub struct EmptyArchitect {}

impl MapArchitect for EmptyArchitect {
    fn build(&mut self, rng: &mut RandomNumberGenerator) -> MapBuilder {
        let mut mb = MapBuilder::new();
        mb.fill(TileType::Floor);
        mb.player_start = Point::new(MAP_WIDTH / 2, MAP_HEIGHT / 2);
        mb.amulet_start = mb.find_most_distant();
        for _ in 0..50 {
            mb.monster_spawns.push(Point::new(
                rng.range(1, MAP_WIDTH), 
                rng.range(1, MAP_HEIGHT)));
        }
        mb
    }
}