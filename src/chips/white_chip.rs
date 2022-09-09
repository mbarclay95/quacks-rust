use std::fmt::Formatter;
use crate::chips::is_chip::IsChip;
use crate::players::player::Player;

#[derive(Clone, Debug)]
pub struct WhiteChip {
    value: usize,
    _original_value: usize,
    color: &'static str
}

impl WhiteChip {
    pub fn new(value: usize) -> Self {
        WhiteChip {
            value,
            _original_value: value,
            color: "white"
        }
    }
}

impl IsChip for WhiteChip {
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

    fn perform_chapter_one_logic(&mut self, _player: &mut Player) {
    }
}
