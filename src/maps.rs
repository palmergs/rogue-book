use crate::prelude::*;

const NUM_TILES: usize = (SCREEN_WIDTH * SCREEN_HEIGHT) as usize;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum TileType {
    Wall,
    Floor,
}

pub struct Map {
    pub tiles: Vec<TileType>,
}

impl Map {
    pub fn new() -> Self {
        Self{
            tiles: vec![TileType::Floor; NUM_TILES]
        }
    }

    pub fn in_bounds(&self, point: Point) -> bool {
        point.x >= 0 && point.x < SCREEN_WIDTH && point.y >= 0 && point.y < SCREEN_HEIGHT
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

    pub fn to_index(x: i32, y: i32) -> usize {
        ((y * SCREEN_WIDTH) + x) as usize
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



