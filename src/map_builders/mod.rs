use crate::prelude::*;

mod empty;
mod rooms;
mod automota;
mod drunkard;
mod prefab;

pub const NUM_ROOMS: usize = (
    (MAP_WIDTH as usize * MAP_HEIGHT as usize) / 
    (SCREEN_WIDTH as usize * SCREEN_HEIGHT as usize)) * 20;

trait MapArchitect {
    fn build(&mut self, rng: &mut RandomNumberGenerator) -> MapBuilder;
}

pub struct MapBuilder {
    pub map: Map,
    pub rooms: Vec<Rect>,
    pub monster_spawns: Vec<Point>,
    pub player_start: Point,
    pub amulet_start: Point,
}

impl MapBuilder {
    pub fn new() -> Self {
        Self{
            map: Map::new(),
            rooms: Vec::new(),
            monster_spawns: Vec::new(),
            player_start: Point::zero(),
            amulet_start: Point::zero(),
        }
    }

    pub fn build(rng: &mut RandomNumberGenerator) -> Self {
        // let mut architect = automota::CellularAutomotaArchitect{};
        let mut architect: Box<dyn MapArchitect> = match rng.range(0, 5) {
            0 => {
                let stagger_distance = 400;
                let desired_floor = (MAP_WIDTH as usize * MAP_HEIGHT as usize) / 3;
                Box::new(drunkard::DrunkardsWalkArchitect{stagger_distance, desired_floor})
            },
            1 => Box::new(automota::CellularAutomotaArchitect{}),
            _ => Box::new(rooms::RoomsArchitect{})
        };
        
        let mut mb = architect.build(rng);
        prefab::apply_prefab(&mut mb, rng);
        mb
    }

    fn fill(&mut self, tile: TileType) {
        self.map.tiles.iter_mut().for_each(|t| *t = tile);
    }

    fn find_most_distant(&self) -> Point {
        let dijkstra_map = DijkstraMap::new(
            MAP_WIDTH,
            MAP_HEIGHT,
            &vec![self.map.point2d_to_index(self.player_start)],
            &self.map,
            1024.0);

        self.map.index_to_point2d(
            dijkstra_map.map
                .iter()
                .enumerate()
                .filter(|(_, dist)| *dist < &std::f32::MAX)
                .max_by(|a, b| a.1.partial_cmp(b.1).unwrap())
                .unwrap().0)
    }

    fn build_random_rooms(&mut self, rng: &mut RandomNumberGenerator) {
        while self.rooms.len() < NUM_ROOMS {
            println!("building room #{:?}", self.rooms.len() + 1);

            let room = Rect::with_size(
                rng.range(1, MAP_WIDTH - 10),
                rng.range(1, MAP_HEIGHT - 10),
                rng.range(2, 10),
                rng.range(2, 10),
            );

            let mut overlap = false;
            for r in self.rooms.iter() {
                if r.intersect(&room) {
                    overlap = true;
                }
            }

            if !overlap {
                room.for_each(|p| {
                    // TODO: should the right and bottom be reduced by 1?
                    if p.x > 0 && p.x < MAP_WIDTH && p.y > 0 && p.y < MAP_HEIGHT {
                        let idx = Map::to_index(p.x, p.y);
                        self.map.tiles[idx] = TileType::Floor;
                    }
                });
                self.rooms.push(room);
            }
        }
    }

    fn vertical_tunnel(&mut self, y1: i32, y2: i32, x: i32) {
        use std::cmp::{ min, max };
        for y in min(y1, y2) ..= max(y1, y2) {
            if let Some(idx) = self.map.try_idx(Point::new(x, y)) {
                self.map.tiles[idx] = TileType::Floor;
            }
        }
    }

    fn horizontal_tunnel(&mut self, x1: i32, x2: i32, y: i32) {
        use std::cmp::{ min, max };
        for x in min(x1, x2) ..= max(x1, x2) {
            if let Some(idx) = self.map.try_idx(Point::new(x, y)) {
                self.map.tiles[idx] = TileType::Floor;
            }
        }
    }

    fn build_corridors(&mut self, rng: &mut RandomNumberGenerator) {
        let mut rooms = self.rooms.clone();
        rooms.sort_by(|a, b| a.center().x.cmp(&b.center().x));
        for (i, room) in rooms.iter().enumerate().skip(1) {
            let prev = rooms[i-1].center();
            let curr = room.center();
            if rng.range(0,2) == 1 {
                self.horizontal_tunnel(prev.x, curr.x, prev.y);
                self.vertical_tunnel(prev.y, curr.y, curr.x);
            } else {
                self.vertical_tunnel(prev.y, curr.y, prev.x);
                self.horizontal_tunnel(prev.x, curr.x, curr.y);
            }
        }
    }
    
    fn spawn_empty_space_monsters(&self, start: &Point, num: usize, rng: &mut RandomNumberGenerator) -> Vec<Point> {
        let mut spawnable_tiles: Vec<Point> = self.map
            .tiles
            .iter()
            .enumerate()
            .filter(|(idx, t)| {
                **t == TileType::Floor && DistanceAlg::Pythagoras.distance2d(*start, self.map.index_to_point2d(*idx)) > 10.0
            })
            .map(|(idx, _)| self.map.index_to_point2d(idx))
            .collect();
        let mut spawns = Vec::new();
        for _ in 0..num {
            let target_index = rng.random_slice_index(&spawnable_tiles).unwrap();
            spawns.push(spawnable_tiles[target_index].clone());
            spawnable_tiles.remove(target_index);
        }
        spawns
    }
}
