use crate::prelude::*;

use super::MapArchitect;

pub struct DrunkardsWalkArchitect {
    pub stagger_distance: usize,
    pub desired_floor: usize,
}

impl DrunkardsWalkArchitect {
    fn drunkard(
        &mut self,
        start: &Point,
        rng: &mut RandomNumberGenerator,
        map: &mut Map
    ) {
        let mut drunkard_pos = start.clone();
        let mut stagger_distance = 0;
        loop {
            let drunk_idx = map.point2d_to_index(drunkard_pos);
            map.tiles[drunk_idx] = TileType::Floor;
            match rng.range(0, 4) {
                0 => drunkard_pos.x -= 1,
                1 => drunkard_pos.x += 1,
                2 => drunkard_pos.y -= 1,
                _ => drunkard_pos.y += 1,
            }

            if !map.in_bounds(drunkard_pos) {
                break
            }

            stagger_distance += 1;
            if stagger_distance > self.stagger_distance {
                break
            }
        }
    }

    fn finished_digging(&self, map: &Map) -> bool {
        map.tiles.iter().filter(|t| **t == TileType::Floor).count() >= self.desired_floor
    }
}

impl MapArchitect for DrunkardsWalkArchitect {
    fn build(&mut self, rng: &mut RandomNumberGenerator) -> MapBuilder {
        let mut mb = MapBuilder::new();
        mb.fill(TileType::Wall);
        let center = Point::new(MAP_WIDTH / 2, MAP_HEIGHT / 2);
        self.drunkard(&center, rng, &mut mb.map);
        while !self.finished_digging(&mb.map) {
            self.drunkard(
                &Point::new(rng.range(1, MAP_WIDTH - 1), rng.range(1, MAP_HEIGHT - 1)),
                rng,
                &mut mb.map);

            let dijkstra_map = DijkstraMap::new(
                MAP_WIDTH,
                MAP_HEIGHT,
                &vec![mb.map.point2d_to_index(center)],
                &mb.map,
                1024.0);

            // Fill in all the tiles that are farther than 2000.0 from start
            // with walls (so there are no inaccessible cavers in the map)
            dijkstra_map.map
                .iter()
                .enumerate()
                .filter(|(_, distance)| *distance > &2000.0 )
                .for_each(|(idx, _)| mb.map.tiles[idx] = TileType::Wall);
        }

        mb.monster_spawns = mb.spawn_empty_space_monsters(&center, 50, rng);
        mb.player_start = center;
        mb.amulet_start = mb.find_most_distant();
        mb
    }
}