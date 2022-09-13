extern crate core;

use std::io::stdin;
use crate::chip_set::ChipSet;

pub mod game;
pub mod chips;
pub mod chip_set;
pub mod players;

pub const SELECTED_CHIP_SET: ChipSet = ChipSet::ChapterOne;
pub const PLAY_MANUAL_GAME: bool = false;

pub fn confirm(message: &str) {
    if PLAY_MANUAL_GAME {
        println!("{}", message);
        let mut ret = String::new();
        stdin().read_line(&mut ret).expect("Failed");
    }
}
