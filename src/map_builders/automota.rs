use crate::prelude::*;

use super::MapArchitect;

pub struct CellularAutomotaArchitect{}

impl CellularAutomotaArchitect {
    fn random_noise_map(&mut self, rng: &mut RandomNumberGenerator, map: &mut Map) {
        map.tiles.iter_mut().for_each(|t| {
            let roll = rng.range(0, 100);
            if roll > 55 {
                *t = TileType::Floor;
            } else {
                *t = TileType::Wall;
            }
        });
    }

    fn count_neighbors(&self, x: i32, y: i32, map: &Map) -> usize {
        let mut neighbors = 0;
        for iy in -1 ..= 1 {
            for ix in -1 ..= 1 {
                if !(ix == 0 && iy == 0) && map.tiles[Map::to_index(x+ix, y+iy)] == TileType::Wall {
                    neighbors += 1;
                }
            }
        }
        neighbors
    }

    fn iteration(&mut self, map: &mut Map) {
        let mut new_tiles = map.tiles.clone();
        for y in 1..MAP_HEIGHT - 1 {
            for x in 1..MAP_WIDTH - 1 {
                let neighbors = self.count_neighbors(x, y, map);
                let idx = Map::to_index(x, y);
                if neighbors > 4 || neighbors == 0 {
                    new_tiles[idx] = TileType::Wall;
                } else {
                    new_tiles[idx] = TileType::Floor;
                }
            }
        }
        map.tiles = new_tiles.clone();
    }

    fn find_start(&self, map: &Map) -> Point {
        let center = Point::new(MAP_WIDTH / 2, MAP_HEIGHT / 2);
        let closest_point = map.tiles
            .iter()
            .enumerate()
            .filter(|(_, t)| **t == TileType::Floor)
            .map(|(idx, _)| {
                (idx, DistanceAlg::Pythagoras.distance2d(center, map.index_to_point2d(idx)))
            })
            .min_by(|(_, distance), (_, distance2)| {
                distance.partial_cmp(&distance2).unwrap()
            })
            .map(|(idx, _)| idx)
            .unwrap();

        map.index_to_point2d(closest_point)
    }
}

impl MapArchitect for CellularAutomotaArchitect {
    fn build(&mut self, rng: &mut RandomNumberGenerator) -> MapBuilder {
        let mut mb = MapBuilder::new();
        self.random_noise_map(rng, &mut mb.map);
        for _ in 0..10 {
            self.iteration(&mut mb.map);
        }

        let start = self.find_start(&mb.map);
        mb.monster_spawns = mb.spawn_empty_space_monsters(&start, 50, rng);
        mb.player_start = start;
        mb.amulet_start = mb.find_most_distant();
        mb
    }   
}