use std::fmt::Formatter;
use rand::Rng;
use crate::chips::is_chip::IsChip;
use crate::player::Player;

#[derive(Clone, Debug)]
pub struct BlueChip {
    value: usize,
    original_value: usize,
    color: &'static str
}

impl BlueChip {
    pub fn new(value: usize) -> Self {
        BlueChip {
            value,
            original_value: value,
            color: "blue"
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

    fn perform_chapter_one_logic(&mut self, player: &mut Player) -> Option<Box<dyn IsChip>> {
        let mut drawn_chips: Vec<Box<dyn IsChip>> = vec![];
        let mut rng = rand::thread_rng();
        for _ in 0..self.original_value {
            if player.bag.is_empty() {
                break;
            }
            let index = rng.gen_range(0..player.bag.len());
            if player.bag[index].get_color() != "white" {
                drawn_chips.push(player.bag[index].clone());
                player.bag.remove(index);
            }
        }

        if !drawn_chips.is_empty() {
            let return_chip = drawn_chips[0].clone();
            for i in 1..drawn_chips.len() {
                player.bag.push(drawn_chips[i].clone());
            }
            return Some(return_chip);
        }

        None
    }
}
