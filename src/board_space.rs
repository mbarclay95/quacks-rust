use crate::{BOARD_SPACES, MAX_BOARD_SPACES};

#[derive(Clone)]
pub struct BoardSpace {
    pub money: i32,
    pub points: i32,
    pub gem: bool
}

impl BoardSpace {
    pub fn get_board_space(index: usize) -> Result<BoardSpace, &'static str> {
        if index > MAX_BOARD_SPACES {
            return Err("board space out of range");
        }

        Ok(BOARD_SPACES[index].clone())
    }
}
