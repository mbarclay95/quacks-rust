use std::fmt::Error;
use crate::board_space::{BoardSpace, MAX_BOARD_SPACES};
use crate::chips::is_chip::IsChip;

#[derive(Debug)]
pub struct Board {
    pub explode_limit: usize,
    start_index: usize, // droplet
    bonus_value: usize, // rat tail
    played_chips: Vec<PlayedSpace>,
}

#[derive(Debug)]
pub struct PlayedSpace {
    board_space_index: usize,
    chip: Box<dyn IsChip>
}

impl Board {
    pub fn new() -> Self {
        Board {
            start_index: 1,
            bonus_value: 0,
            explode_limit: 7,
            played_chips: vec![]
        }
    }

    pub fn get_current_space(&self) -> &BoardSpace {
        let current_position = self.get_board_space_position();
        BoardSpace::get_board_space(current_position).unwrap_or_else(|_| panic!("index: {}, board space out of range. \n{:?}", current_position, self))
    }

    pub fn get_played_chip_len(&self) -> usize {
        self.played_chips.len()
    }

    pub fn increase_start_index(&mut self, amount: usize) {
        self.start_index += amount
    }

    pub fn get_played_chip(&self, index: usize) -> &Box<dyn IsChip> {
        &self.played_chips[index].chip
    }

    pub fn play_chip(&mut self, chip: Box<dyn IsChip>) {
        let board_space_index = if self.get_board_space_position() + (chip.get_value() - 1) >= MAX_BOARD_SPACES {
            MAX_BOARD_SPACES - 1
        } else {
            self.get_board_space_position() + (chip.get_value() - 1)
        };
        self.played_chips.push(PlayedSpace {
            board_space_index,
            chip
        });
    }

    pub fn get_board_space_position(&self) -> usize {
        self.played_chips.iter().map(|played_space| played_space.chip.get_value()).reduce(|accum, value| accum + value).unwrap_or(0) + self.start_index + self.bonus_value
    }

    pub fn get_played_chips_of_color(&self, color: &str) -> Vec<&PlayedSpace> {
        self.played_chips.iter().filter(|played_space| played_space.chip.get_color() == color).collect()
    }

    pub fn get_last_played_chip(&self) -> Option<&Box<dyn IsChip>> {
        if self.played_chips.is_empty() {
            return None;
        }

        Some(self.get_played_chip(self.get_played_chip_len() - 1))
    }

    pub fn remove_last_played_chip(&mut self) -> Result<(), &'static str> {
        if self.played_chips.is_empty() {
            return Err("No played chips to remove");
        }

        self.played_chips.pop();
        Ok(())
    }

    pub fn get_white_count(&self) -> usize {
        self.get_played_chips_of_color("white").iter().map(|played_space| played_space.chip.get_value()).reduce(|accum, value| accum + value).unwrap_or(0)
    }

    pub fn check_if_exploded(&self) -> bool {
        self.get_white_count() > self.explode_limit
    }

    pub fn clear_board(&mut self) {
        self.played_chips = vec![];
    }
}

impl Default for Board {
    fn default() -> Self {
        Board::new()
    }
}
