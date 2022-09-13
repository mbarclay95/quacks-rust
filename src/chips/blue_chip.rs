use std::fmt::Formatter;

use rand::{Rng, thread_rng};

use crate::chips::is_chip::IsChip;
use crate::confirm;
use crate::players::player::Player;

#[derive(Clone, Debug)]
pub struct BlueChip {
    value: usize,
    original_value: usize,
    color: &'static str,
}

impl BlueChip {
    pub fn new(value: usize) -> Self {
        BlueChip {
            value,
            original_value: value,
            color: "blue",
        }
    }
}

impl IsChip for BlueChip {
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
        if player.stop_drawing(true) {
            confirm("Stop drawing for blue chip");
            return;
        }
        player.player_stats.blue_activation_count += 1.0;
        let mut drawn_chips: Vec<Box<dyn IsChip>> = vec![];
        for _ in 0..self.original_value {
            if player.bag.is_empty() {
                break;
            }
            let index = thread_rng().gen_range(0..player.bag.len());
            confirm(format!("Drawn chip for blue: {:?}", player.bag[index]).as_str());
            if player.bag[index].get_color() != "white" {
                drawn_chips.push(player.bag.remove(index));
            }
        }

        if !drawn_chips.is_empty() {
            let mut first_chip = drawn_chips.remove(0);
            confirm(format!("Selected chip for blue: {:?}", first_chip).as_str());
            for _ in 0..drawn_chips.len() {
                player.bag.push(drawn_chips.remove(0));
            }
            if first_chip.get_color() == "blue" {
                player.board.play_chip(&first_chip);
                first_chip.perform_chapter_one_logic(player);
            } else {
                first_chip.perform_chapter_one_logic(player);
                player.board.play_chip(&first_chip);
            }
        } else {
            confirm("No non white chips drawn");
        }
    }
}
