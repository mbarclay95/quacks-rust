use crate::board::Board;
use crate::chip::Chip;

pub struct Player {
    name: &'static str,
    board: Board,
    score: i32,
    gems: i32,
    potion: bool,
    bag: Vec<Chip>,
    all_chips: Vec<Chip>
}

impl Player {
    pub fn new(name: &'static str) -> Self {
        Player {
            name,
            board: Board::new(),
            score: 0,
            gems: 1,
            potion: true,
            bag: vec![],
            all_chips: vec![]
        }
    }
}
