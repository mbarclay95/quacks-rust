use log::info;
use crate::chip_set::{ChipSet, PurchasableChip};
use crate::players::player::Player;

#[derive(Debug)]
pub struct Game {
    pub players: Vec<Player>,
    round: i32,
    purchasable_chips: Vec<PurchasableChip>
}

impl Game {
    pub fn new() -> Self {
        Game {
            round: 1,
            purchasable_chips: ChipSet::get_starting_purchasable_chips(),
            players: vec![
                Player::new("Michael")
            ],
        }
    }

    pub fn play_game(&mut self) {
        for _ in 0..9 {
            self.play_through_round();
        }
        for player in self.players.iter_mut() {
            player.player_stats.starting_index = player.board.start_index as i32;
            player.player_stats.score = player.score;
        }
    }

    fn play_through_round(&mut self) {
        info!("\nStarting round {}", self.round);
        self.start_of_round_logic();
        self.phase_1();
        self.phase_2();
        self.phase_3();
        self.phase_4();
        self.phase_5_and_6();
        self.phase_7();
        self.round += 1;
    }

    pub fn print_points(&self) {
        for player in self.players.iter() {
            println!("Player {} has {} points", player.name, player.score);
        }
    }

    pub fn print_stats(&self) {
        for player in self.players.iter() {
            println!("{:?}", player.player_stats);
        }
    }

    fn phase_1(&mut self) {
        for player in self.players.iter_mut() {
            player.play_through_phase_1();
        }
    }

    fn phase_2(&mut self) {
        let max_board_space_option = self.players.iter().filter(|player| !player.is_exploded()).map(|player| player.board.get_board_position()).max_by_key(|position| *position);

        if let Some(max_board_space) = max_board_space_option {
            self.players.iter_mut().filter(|player| !player.is_exploded() && player.board.get_board_position() == max_board_space).for_each(|player| player.phase_2_role_dice());
        }
    }

    fn phase_3(&mut self) {
        for player in self.players.iter_mut() {
            player.handle_green_chips();
            player.handle_purple_chips();
            // this needs to be reworked
            player.get_black_chips_value()
        }
    }

    fn phase_4(&mut self) {
        for player in self.players.iter_mut() {
            player.phase_4_gem_check();
        }
    }

    fn phase_5_and_6(&mut self) {
        for player in self.players.iter_mut() {
            if player.is_exploded() {
                if self.round > 6 {
                    player.phase_5_points();
                } else {
                    player.phase_6_buy_chips(&self.purchasable_chips, self.round == 9);
                }
            } else {
                player.phase_6_buy_chips(&self.purchasable_chips, self.round == 9);
                player.phase_5_points();
            }
        }
    }

    fn phase_7(&mut self) {
        for player in self.players.iter_mut() {
            player.phase_7_spend_gems(self.round == 9);
        }
    }

    fn start_of_round_logic(&mut self) {
        match self.round {
            2 => {self.purchasable_chips.append(ChipSet::get_yellow_chips().as_mut())},
            3 => {self.purchasable_chips.append(ChipSet::get_purple_chips().as_mut())},
            6 => {
                for player in self.players.iter_mut() {
                    player.round_6_add_white_chip();
                }
            },
            _ => {}
        }
    }
}

impl Default for Game {
    fn default() -> Self {
        Game::new()
    }
}
