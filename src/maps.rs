use crate::prelude::*;

pub const MAP_WIDTH: i32 = 80;
pub const MAP_HEIGHT: i32 = 50;

const NUM_TILES: usize = (MAP_WIDTH * MAP_HEIGHT) as usize;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum TileType {
    Wall,
    Floor,
}

pub struct Map {
    pub tiles: Vec<TileType>,
    pub revealed: Vec<bool>,
}

impl Map {
    pub fn new() -> Self {
        Self{
            tiles: vec![TileType::Floor; NUM_TILES],
            revealed: vec![false; NUM_TILES],
        }
    }

    pub fn in_bounds(&self, point: Point) -> bool {
        point.x >= 0 && point.x < MAP_WIDTH && point.y >= 0 && point.y < MAP_HEIGHT
    }

    pub fn can_enter(&self, point: Point) -> bool {
        self.in_bounds(point) && self.tiles[Map::to_index(point.x, point.y)] == TileType::Floor
    }

    pub fn try_idx(&self, point: Point) -> Option<usize> {
        if self.in_bounds(point) {
            Some(Map::to_index(point.x, point.y))
        } else {
            None
        }
    }

    fn valid_exit(&self, loc: Point, delta: Point) -> Option<usize> {
        let destination = loc + delta;
        if self.in_bounds(destination) {
            if self.can_enter(destination) {
                let idx = self.point2d_to_index(destination);
                return Some(idx)
            }
        }
        None
    }

    pub fn distance(&self, idx1: usize, idx2: usize) -> f32 {
        DistanceAlg::Pythagoras.distance2d(
            self.index_to_point2d(idx1), 
            self.index_to_point2d(idx2))
    }

    pub fn to_index(x: i32, y: i32) -> usize {
        ((y * MAP_WIDTH) + x) as usize
    }
}

impl BaseMap for Map {
    fn get_available_exits(&self, idx: usize) -> SmallVec<[(usize, f32); 10]> {
        let mut exits = SmallVec::new();
        let location = self.index_to_point2d(idx);
        if let Some(idx) = self.valid_exit(location, Point::new(-1, 0)) {
            exits.push((idx, 1.0));
        }

        if let Some(idx) = self.valid_exit(location, Point::new(1, 0)) {
            exits.push((idx, 1.0));
        }

        if let Some(idx) = self.valid_exit(location, Point::new(0, -1)) {
            exits.push((idx, 1.0));
        }

        if let Some(idx) = self.valid_exit(location, Point::new(0, 1)) {
            exits.push((idx, 1.0));
        }

        exits
    }

    fn is_opaque(&self, idx: usize) -> bool {
        self.tiles[idx] != TileType::Floor
    }
}

impl Algorithm2D for Map {
    fn dimensions(&self) -> Point {
        Point::new(MAP_WIDTH, MAP_HEIGHT)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_index() {
        assert_eq!(Map::to_index(0, 0), 0);
    }
}



