use std::fmt::{Debug, Formatter};
use crate::player::Player;
use crate::{ChipSet, SELECTED_CHIP_SET};

pub trait IsChip {
    fn clone_dyn(&self) -> Box<dyn IsChip>;
    fn debug_dyn(&self, f: &mut Formatter<'_>) -> std::fmt::Result;
    fn get_value(&self) -> usize;
    fn get_color(&self) -> &str;
    fn perform_chip_logic(&mut self, player: &mut Player) -> Option<Box<dyn IsChip>> {
        match SELECTED_CHIP_SET {
            ChipSet::ChapterOne => self.perform_chapter_one_logic(player),
        }
    }
    fn perform_chapter_one_logic(&mut self, player: &mut Player) -> Option<Box<dyn IsChip>>;
}

impl Clone for Box<dyn IsChip> {
    fn clone(&self) -> Self {
        self.clone_dyn()
    }
}

impl Debug for dyn IsChip {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        self.debug_dyn(f)
    }
}
