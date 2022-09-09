use std::fmt::Formatter;
use crate::chips::is_chip::IsChip;
use crate::players::player::Player;

#[derive(Clone, Debug)]
pub struct YellowChip {
    value: usize,
    _original_value: usize,
    color: &'static str
}

impl YellowChip {
    pub fn new(value: usize) -> Self {
        YellowChip {
            value,
            _original_value: value,
            color: "yellow"
        }
    }
}

impl IsChip for YellowChip {
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
        let last_played_chip_option = player.board.get_last_played_chip();
        if let Some(last_played_chip) = last_played_chip_option {
            if last_played_chip.get_color() == "white" {
                self.value += last_played_chip.get_value();
                player.bag.push(last_played_chip.clone_dyn());
                player.board.remove_last_played_chip().unwrap();
                player.player_stats.yellow_activation_count += 1;
            }
        }
    }
}
