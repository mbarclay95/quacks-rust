use std::fmt::Formatter;
use crate::chips::is_chip::IsChip;
use crate::players::player::Player;

#[derive(Clone, Debug)]
pub struct BlackChip {
    value: usize,
    _original_value: usize,
    color: &'static str
}

impl BlackChip {
    pub fn new(value: usize) -> Self {
        BlackChip {
            value,
            _original_value: value,
            color: "black"
        }
    }
}

impl IsChip for BlackChip {
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
