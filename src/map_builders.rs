use crate::prelude::*;

const NUM_ROOMS: usize = 20;

pub struct MapBuilder {
    pub map: Map,
    pub rooms: Vec<Rect>,
    pub player_start: Point,
    pub amulet_start: Point,
}

impl MapBuilder {
    pub fn build(rng: &mut RandomNumberGenerator) -> Self {
        let mut mb = MapBuilder{ 
            map: Map::new(), 
            rooms: Vec::new(), 
            player_start: Point::zero(),
            amulet_start: Point::zero(),
        };

        println!("about to fill mb...");
        mb.fill(TileType::Wall);

        println!("about to build rooms...");
        mb.build_random_rooms(rng);

        println!("about to build corridors...");
        mb.build_corridors(rng);

        println!("about to set user...");
        mb.player_start = mb.rooms[0].center();
        println!("... {:?}", mb.player_start);

        println!("about to set amulet...");
        let dijkstra_map = DijkstraMap::new(
            SCREEN_WIDTH,
            SCREEN_HEIGHT,
            &vec![mb.map.point2d_to_index(mb.player_start)],
            &mb.map,
            1024.0);
        mb.amulet_start = mb.map.index_to_point2d(
            dijkstra_map.map
                .iter()
                .enumerate()
                .filter(|(_, dist)| *dist < &std::f32::MAX)
                .max_by(|a, b| a.1.partial_cmp(b.1).unwrap())
                .unwrap().0);
        println!("... {:?}", mb.amulet_start);

        println!("about to return...");
        mb
    }

    fn fill(&mut self, tile: TileType) {
        self.map.tiles.iter_mut().for_each(|t| *t = tile);
    }

    fn build_random_rooms(&mut self, rng: &mut RandomNumberGenerator) {
        while self.rooms.len() < NUM_ROOMS {
            let room = Rect::with_size(
                rng.range(1, SCREEN_WIDTH - 10),
                rng.range(1, SCREEN_HEIGHT - 10),
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
                    if p.x > 0 && p.x < SCREEN_WIDTH && p.y > 0 && p.y < SCREEN_HEIGHT {
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
}
