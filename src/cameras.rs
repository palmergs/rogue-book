use crate::prelude::*;

pub struct Camera {
    pub left_x: i32,
    pub right_x: i32,
    pub top_y: i32,
    pub bottom_y: i32,
}

impl Camera {
    pub fn new(pp: Point) -> Self {
        Self{
            left_x: pp.x - DISPLAY_WIDTH / 2,
            right_x: pp.x + DISPLAY_WIDTH / 2,
            top_y: pp.y - DISPLAY_HEIGHT / 2,
            bottom_y: pp.y + DISPLAY_HEIGHT / 2,
        }
    }

    pub fn on_play_move(&mut self, pp: Point) {
        self.left_x = pp.x - DISPLAY_WIDTH / 2;
        self.right_x = pp.x + DISPLAY_WIDTH / 2;
        self.top_y = pp.y - DISPLAY_HEIGHT / 2;
        self.bottom_y = pp.y + DISPLAY_HEIGHT / 2;
    }
}