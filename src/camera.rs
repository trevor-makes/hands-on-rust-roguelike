use crate::prelude::*;

pub struct Camera {
    pub left_x: i32,
    pub right_x: i32,
    pub top_y: i32,
    pub bottom_y: i32,
}

impl Camera {
    pub fn new(player_position: Point) -> Self {
        let left = player_position.x - DISPLAY_WIDTH / 2;
        let top = player_position.y - DISPLAY_HEIGHT / 2;
        Self {
            left_x: left,
            right_x: left + DISPLAY_WIDTH,
            top_y: top,
            bottom_y: top + DISPLAY_HEIGHT,
        }
    }

    pub fn on_player_move(&mut self, player_position: Point) {
        let left = player_position.x - DISPLAY_WIDTH / 2;
        let top = player_position.y - DISPLAY_HEIGHT / 2;
        self.left_x = left;
        self.right_x = left + DISPLAY_WIDTH;
        self.top_y = top;
        self.bottom_y = top + DISPLAY_HEIGHT;
    }
}