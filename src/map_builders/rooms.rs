use crate::prelude::*;

use super::MapArchitect;

pub struct RoomsArchitect {}

impl RoomsArchitect {

}

impl MapArchitect for RoomsArchitect {
    fn build(&mut self, rng: &mut RandomNumberGenerator) -> MapBuilder {
        let mut mb = MapBuilder::new();

        println!("about to fill mb...");
        mb.fill(TileType::Wall);

        println!("about to build {:?} rooms...", NUM_ROOMS);
        mb.build_random_rooms(rng);

        println!("about to build corridors...");
        mb.build_corridors(rng);

        println!("about to set user...");
        mb.player_start = mb.rooms[0].center();
        println!("... {:?}", mb.player_start);

        println!("about to set amulet...");
        mb.amulet_start = mb.find_most_distant();
        println!("... {:?}", mb.amulet_start);

        println!("spawning monsters...");
        for room in mb.rooms.iter().skip(1) {
            mb.monster_spawns.push(room.center());
        }

        println!("about to return...");
        mb
    }    
}