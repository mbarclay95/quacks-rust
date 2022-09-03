use std::collections::HashMap;
use crate::BoardSpace;
use crate::chip::Chip;

pub struct Board {
    board_space_index: usize,
    start_value: i32, // droplet
    bonus_value: i32, // rat tail
    played_chips: HashMap<i32, Chip>,
}

impl Board {
    pub fn new() -> Self {
        Board {
            board_space_index: 0,
            start_value: 0,
            bonus_value: 0,
            played_chips: HashMap::new()
        }
    }

    pub fn print_current_space(&self) {
        let space = BoardSpace::get_board_space(self.board_space_index).unwrap_or_else(|_| panic!("index: {}, board space out of range", self.board_space_index));
        println!("money: {}, points: {}, gem: {}", space.money, space.points, space.gem)
    }
}

impl Default for Board {
    fn default() -> Self {
        Board::new()
    }
}
