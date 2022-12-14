pub const NUM_OF_BOARD_SPACE: usize = 54;
// the very last space is cannot be played on
pub const LAST_PLAYABLE_SPACE: usize = NUM_OF_BOARD_SPACE - 2;

static BOARD_SPACES: [BoardSpace; NUM_OF_BOARD_SPACE] = [
    BoardSpace { money: 0, points: 0, gem: false },
    BoardSpace { money: 1, points: 0, gem: false },
    BoardSpace { money: 2, points: 0, gem: false },
    BoardSpace { money: 3, points: 0, gem: false },
    BoardSpace { money: 4, points: 0, gem: false },
    BoardSpace { money: 5, points: 0, gem: true },
    BoardSpace { money: 6, points: 1, gem: false },
    BoardSpace { money: 7, points: 1, gem: false },
    BoardSpace { money: 8, points: 1, gem: false },
    BoardSpace { money: 9, points: 1, gem: true },
    BoardSpace { money: 10, points: 2, gem: false },
    BoardSpace { money: 11, points: 2, gem: false },
    BoardSpace { money: 12, points: 2, gem: false },
    BoardSpace { money: 13, points: 2, gem: true },
    BoardSpace { money: 14, points: 3, gem: false },
    BoardSpace { money: 15, points: 3, gem: false },
    BoardSpace { money: 15, points: 3, gem: true },
    BoardSpace { money: 16, points: 3, gem: false },
    BoardSpace { money: 16, points: 4, gem: false },
    BoardSpace { money: 17, points: 4, gem: false },
    BoardSpace { money: 17, points: 4, gem: true },
    BoardSpace { money: 18, points: 4, gem: false },
    BoardSpace { money: 18, points: 5, gem: false },
    BoardSpace { money: 19, points: 5, gem: false },
    BoardSpace { money: 19, points: 5, gem: true },
    BoardSpace { money: 20, points: 5, gem: false },
    BoardSpace { money: 20, points: 6, gem: false },
    BoardSpace { money: 21, points: 6, gem: false },
    BoardSpace { money: 21, points: 6, gem: true },
    BoardSpace { money: 22, points: 7, gem: false },
    BoardSpace { money: 22, points: 7, gem: true },
    BoardSpace { money: 23, points: 7, gem: false },
    BoardSpace { money: 23, points: 8, gem: false },
    BoardSpace { money: 24, points: 8, gem: false },
    BoardSpace { money: 24, points: 8, gem: true },
    BoardSpace { money: 25, points: 9, gem: false },
    BoardSpace { money: 25, points: 9, gem: true },
    BoardSpace { money: 26, points: 9, gem: false },
    BoardSpace { money: 26, points: 10, gem: false },
    BoardSpace { money: 27, points: 10, gem: false },
    BoardSpace { money: 27, points: 10, gem: true },
    BoardSpace { money: 28, points: 11, gem: false },
    BoardSpace { money: 28, points: 11, gem: true },
    BoardSpace { money: 29, points: 11, gem: false },
    BoardSpace { money: 29, points: 12, gem: false },
    BoardSpace { money: 30, points: 12, gem: false },
    BoardSpace { money: 30, points: 12, gem: true },
    BoardSpace { money: 31, points: 12, gem: false },
    BoardSpace { money: 31, points: 13, gem: false },
    BoardSpace { money: 32, points: 13, gem: false },
    BoardSpace { money: 32, points: 13, gem: true },
    BoardSpace { money: 33, points: 14, gem: false },
    BoardSpace { money: 33, points: 14, gem: true },
    BoardSpace { money: 35, points: 15, gem: false },
];

#[derive(Clone, Debug)]
pub struct BoardSpace {
    pub money: i32,
    pub points: i32,
    pub gem: bool,
}

impl BoardSpace {
    pub fn get_board_space(index: usize) -> Result<&'static BoardSpace, &'static str> {
        if index >= NUM_OF_BOARD_SPACE {
            return Err("board space out of range");
        }

        Ok(&BOARD_SPACES[index])
    }
}
