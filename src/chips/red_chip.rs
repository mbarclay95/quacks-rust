use std::fmt::Formatter;

use crate::chips::is_chip::IsChip;
use crate::players::player::Player;

#[derive(Clone, Debug)]
pub struct RedChip {
    value: usize,
    _original_value: usize,
    color: &'static str,
}

impl RedChip {
    pub fn new(value: usize) -> Self {
        RedChip {
            value,
            _original_value: value,
            color: "red",
        }
    }
}

impl IsChip for RedChip {
    fn clone_dyn(&self) -> Box<dyn IsChip> {
        Box::new(self.clone())
    }

    fn debug_dyn(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "(color: {}, value: {})", self.color, self.value)
    }

    fn get_value(&self) -> usize {
        self.value
    }

    fn get_color(&self) -> &str {
        self.color
    }

    fn perform_chapter_one_logic(&mut self, player: &mut Player) {
        let orange_count = player.board.get_played_chips_of_color("orange").len();
        match orange_count {
            0 => {}
            1 | 2 => {
                self.value += 1;
                player.player_stats.red_activation_count += 1;
            }
            3 | _ => {
                self.value += 2;
                player.player_stats.red_activation_count += 1;
            }
        }
    }
}
