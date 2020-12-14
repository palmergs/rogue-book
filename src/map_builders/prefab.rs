use crate::prelude::*;

const FORTRESS:(&str, i32, i32) = ("
............
...######...
...#....#...
...#.M..#...
.###....###.
..M......M..
.###....###.
...#....#...
...#....#...
...######...
............
", 12, 11);

const TEMPLE:(&str, i32, i32) = ("
................
....########....
...........#....
.####...M..####.
.#............#.
.#............#.
.#....#..#....#.
.#............#.
.###..#..#..###.
...#........#...
...####MM####...
................
...#..#..#..#...
................
", 16, 14);

const APTS:(&str, i32, i32) = ("
..............................
.##########.#################.
....M...#.....#......#......#.
.#......#.....#..M...#....M.#.
.###.####.....#......#........
.#..M...#...M.#......#......#.
.#......#.....#......#......#.
.#################.##########.
..............................
", 30, 9); 

pub fn apply_prefab(mb: &mut MapBuilder, rng: &mut RandomNumberGenerator) {
    let mut placement = None;
    let dijkstra_map = DijkstraMap::new(
        MAP_WIDTH,
        MAP_HEIGHT,
        &vec![mb.map.point2d_to_index(mb.player_start)],
        &mb.map,
        1024.0);

    let prefab = match rng.range(0, 2) {
        0 => FORTRESS,
        1 => APTS,
        _ => TEMPLE,
    };

    let mut attempts = 0;
    while placement.is_none() && attempts < 10 {
        let dimensions = Rect::with_size(
            rng.range(0, MAP_WIDTH - prefab.1),
            rng.range(0, MAP_HEIGHT - prefab.2),
            prefab.1,
            prefab.2);

        let mut can_place = false;
        dimensions.for_each(|pt| {
            let idx = mb.map.point2d_to_index(pt);
            let distance = dijkstra_map.map[idx];
            if distance < 2000.0 && distance > 20.0 && mb.amulet_start != pt {
                can_place = true;
            }
        });

        if can_place {
            println!("placing prefab at {:?}", dimensions);
            placement = Some(Point::new(dimensions.x1, dimensions.y1));
            let points = dimensions.point_set();
            mb.monster_spawns.retain(|pt| !points.contains(pt));
        }

        attempts += 1;
    }

    if let Some(placement) = placement {
        let string_vec: Vec<char> = prefab.0.chars()
            .filter(|a| *a != '\r' && *a != '\n')
            .collect();
        let mut i = 0;
        for ty in placement.y .. placement.y + prefab.2 {
            for tx in placement.x .. placement.x + prefab.1 {
                let idx = Map::to_index(tx, ty);
                let c = string_vec[i];
                match c {
                    'M' => {
                        mb.map.tiles[idx] = TileType::Floor;
                        mb.monster_spawns.push(Point::new(tx, ty));
                    }
                    '.' => mb.map.tiles[idx] = TileType::Floor,
                    '#' => mb.map.tiles[idx] = TileType::Wall,
                    _ => println!("no idea what to do with {} {}x{}", c, tx, ty)
                }
                i += 1;
            }
        }
    }
}