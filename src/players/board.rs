use crate::chips::is_chip::IsChip;
use crate::players::board_space::{BoardSpace, LAST_PLAYABLE_SPACE};

#[derive(Debug)]
pub struct Board {
    pub explode_limit: usize,
    pub start_index: usize, // droplet
    current_position: usize,
    bonus_value: usize, // rat tail
    played_chips: Vec<PlayedSpace>,
}

#[derive(Debug)]
pub struct PlayedSpace {
    _board_space_index: usize,
    chip: Box<dyn IsChip>
}

impl Board {
    pub fn new() -> Self {
        Board {
            current_position: 1,
            start_index: 1,
            bonus_value: 0,
            explode_limit: 7,
            played_chips: vec![]
        }
    }

    pub fn get_board_position(&self) -> usize {
        self.current_position
    }

    pub fn reset_board(&mut self) {
        self.current_position = self.start_index + self.bonus_value;
        self.played_chips = vec![];
    }

    pub fn get_current_space(&self) -> &BoardSpace {
        BoardSpace::get_board_space(self.current_position).unwrap_or_else(|_| panic!("index: {}, board space out of range. \n{:?}", self.current_position, self))
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

    pub fn play_chip(&mut self, chip: &Box<dyn IsChip>) {
        let board_space_index = if self.current_position + (chip.get_value() - 1) > LAST_PLAYABLE_SPACE {
            LAST_PLAYABLE_SPACE
        } else {
            self.current_position + (chip.get_value() - 1)
        };
        self.played_chips.push(PlayedSpace {
            _board_space_index: board_space_index,
            chip: chip.clone_dyn()
        });
        if board_space_index == LAST_PLAYABLE_SPACE {
            self.current_position = LAST_PLAYABLE_SPACE + 1;
        } else {
            self.current_position += chip.get_value();
        }
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
}

impl Default for Board {
    fn default() -> Self {
        Board::new()
    }
}
