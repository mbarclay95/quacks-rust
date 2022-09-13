use log::info;
use rand::{Rng, thread_rng};

use crate::{ChipSet, confirm, SELECTED_CHIP_SET};
use crate::chip_set::PurchasableChip;
use crate::chips::green_chip::GreenChip;
use crate::chips::is_chip::IsChip;
use crate::chips::orange_chip::OrangeChip;
use crate::chips::white_chip::WhiteChip;
use crate::players::ai::strategies::Strategies;
use crate::players::board::Board;
use crate::players::board_space::LAST_PLAYABLE_SPACE;
use crate::players::player_stats::PlayerStats;

#[derive(Debug)]
pub struct Player {
    pub name: &'static str,
    pub board: Board,
    pub score: i32,
    pub player_stats: PlayerStats,
    pub bag: Vec<Box<dyn IsChip>>,
    strategy: Strategies,
    gems: usize,
    has_potion: bool,
    pub all_chips: Vec<Box<dyn IsChip>>,
}

impl Player {
    pub fn new(name: &'static str, strategy: Strategies) -> Self {
        Player {
            name,
            board: Board::new(),
            score: 0,
            gems: 1,
            has_potion: true,
            bag: vec![],
            all_chips: vec![
                Box::new(OrangeChip::new(1)),
                Box::new(GreenChip::new(1)),
                Box::new(WhiteChip::new(1)),
                Box::new(WhiteChip::new(1)),
                Box::new(WhiteChip::new(1)),
                Box::new(WhiteChip::new(1)),
                Box::new(WhiteChip::new(2)),
                Box::new(WhiteChip::new(2)),
                Box::new(WhiteChip::new(3)),
            ],
            player_stats: PlayerStats::new(),
            strategy,
        }
    }

    pub fn is_exploded(&self) -> bool {
        self.board.check_if_exploded()
    }

    pub fn reset_bag_and_board(&mut self) {
        confirm("Reset your board");
        confirm(format!("Points {}, gems: {}", self.score, self.gems).as_str());
        self.board.reset_board();
        self.bag = self.all_chips.clone();
    }

    pub fn chip_count_of_color(&self, color: &str) -> usize {
        self.all_chips.iter().filter(|chip| chip.get_color() == color).count()
    }

    pub fn play_through_phase_1(&mut self) {
        self.reset_bag_and_board();
        while !self.stop_drawing(false) {
            self.draw_chip();
        }
        if self.is_exploded() {
            self.player_stats.times_exploded += 1.0;
        }
        info!("Made it to board index: {} and exploded = {}", self.board.get_board_position(), self.is_exploded());
    }

    pub fn stop_drawing(&self, ignore_chance_of_exploding: bool) -> bool {
        if self.bag.is_empty() || self.is_exploded() {
            confirm("You exploded");
            return true;
        }
        if self.board.get_board_position() >= LAST_PLAYABLE_SPACE {
            confirm("You are at the last space");
            return true;
        }
        if ignore_chance_of_exploding {
            return false;
        }
        let white_count = self.board.get_white_count();
        let num_of_chips_make_explode = self.get_num_of_chips_that_could_explode(white_count);
        info!("white count at {}", white_count);

        let chance_of_exploding = num_of_chips_make_explode as f32 / self.bag.len() as f32;
        info!("{:.2}% chance of exploding", chance_of_exploding * 100.0);
        let threshold = 0.22;
        confirm(format!("{:.2}% chance of exploding", chance_of_exploding * 100.0).as_str());

        chance_of_exploding > threshold
    }

    fn get_num_of_chips_that_could_explode(&self, white_count: usize) -> i32 {
        let mut num_of_chips_make_explode = 0;
        for white_chip in self.bag.iter().filter(|p| p.get_color() == "white").collect::<Vec<&Box<dyn IsChip>>>() {
            if white_count + white_chip.get_value() > self.board.explode_limit {
                num_of_chips_make_explode += 1;
            }
        }

        num_of_chips_make_explode
    }

    pub fn draw_chip(&mut self) {
        if self.bag.is_empty() {
            return;
        }
        let index = thread_rng().gen_range(0..self.bag.len());
        confirm(format!("Drawn chip: {:?}", self.bag[index]).as_str());
        let current_white_count = self.board.get_white_count();
        if self.bag[index].get_color() == "white"
            && current_white_count + self.bag[index].get_value() <= self.board.explode_limit
            && self.has_potion
            && self.strategy.potion_strategy.use_potion(&self.bag[index], self.get_num_of_chips_that_could_explode(current_white_count + self.bag[index].get_value()) > 0)
        {
            info!("Used potion");
            confirm("Potion used, put chip back");
            self.player_stats.num_potions_used += 1.0;
            self.has_potion = false;
            return;
        }
        let mut drawn_chip = self.bag.remove(index);
        info!("Drawn chip: {:?}", drawn_chip);

        // I think this is the only chip in the game that needs to be played before logic happens
        if matches!(SELECTED_CHIP_SET, ChipSet::ChapterOne) && drawn_chip.get_color() == "blue" {
            self.board.play_chip(&drawn_chip);
            drawn_chip.perform_chip_logic(self);
        } else {
            drawn_chip.perform_chip_logic(self);
            self.board.play_chip(&drawn_chip);
        }
    }

    pub fn phase_2_role_dice(&mut self) {
        let rand_num: i32 = thread_rng().gen_range(0..6);
        info!("Dice role : {}", rand_num);
        confirm(format!("Got dice roll of {}", rand_num).as_str());
        match rand_num {
            0 | 1 => {
                self.score += 1;
                self.player_stats.points_from_dice += 1.0;
            }
            2 => {
                self.score += 2;
                self.player_stats.points_from_dice += 2.0;
            }
            3 => {
                self.gems += 1;
                self.player_stats.gems_from_dice += 1.0;
            }
            4 => {
                self.board.increase_start_index(1);
                self.player_stats.start_advance_from_dice += 1.0;
            }
            5 => {
                self.all_chips.push(Box::new(OrangeChip::new(1)));
                self.player_stats.orange_from_dice += 1.0;
            }
            _ => {}
        }
    }

    pub fn handle_green_chips(&mut self) {
        match SELECTED_CHIP_SET {
            ChipSet::ChapterOne => {
                let played_chips_length = self.board.get_played_chip_len();
                if self.board.get_played_chip(played_chips_length - 1).get_color() == "green" {
                    self.gems += 1;
                    self.player_stats.green_activation_count += 1.0;
                    info!("Received a gem from green chip");
                    confirm("Receive gem for last green chip");
                }
                if self.board.get_played_chip(played_chips_length - 2).get_color() == "green" {
                    self.gems += 1;
                    self.player_stats.green_activation_count += 1.0;
                    info!("Received a gem from green chip");
                    confirm("Receive gem for second to last green chip");
                }
            }
        }
    }

    pub fn handle_purple_chips(&mut self) {
        match SELECTED_CHIP_SET {
            ChipSet::ChapterOne => {
                let purple_num = self.board.get_played_chips_of_color("purple").len();
                match purple_num {
                    0 => {}
                    1 => {
                        self.score += 1;
                        self.player_stats.purple_activation_count += 1.0;
                        confirm("Receive 1 point for purple chips");
                    }
                    2 => {
                        self.score += 1;
                        self.gems += 1;
                        self.player_stats.purple_activation_count += 1.0;
                        confirm("Receive 1 point and move 1 gem for purple chips");
                    }
                    _ => {
                        self.score += 2;
                        self.board.increase_start_index(1);
                        self.player_stats.purple_activation_count += 1.0;
                        confirm("Receive 2 point and move droplet 1 space for purple chips");
                    }
                }
            }
        }
    }

    pub fn get_black_chips_value(&mut self) {
        if !self.board.get_played_chips_of_color("black").is_empty() {
            info!("Received reward from black chip");
            confirm("Receive 1 gem and move droplet 1 space for black chips");
            self.gems += 1;
            self.board.increase_start_index(1);
            self.player_stats.black_activation_count += 1.0;
        }
    }

    pub fn phase_4_gem_check(&mut self) {
        if self.board.get_current_space().unwrap_or_else(|_| panic!("Board out of index: {:?}", self)).gem {
            info!("Received a gem from board space");
            confirm("Receive 1 gem from board");
            self.gems += 1;
            self.player_stats.gems_from_board += 1.0;
        }
    }

    pub fn phase_5_points(&mut self) {
        let points = self.board.get_current_space().unwrap_or_else(|_| panic!("Board out of index: {:?}", self)).points;
        self.score += points;
        confirm(format!("Receive {} points from board", points).as_str());
        info!("Received {} points, for total of {} points", points, self.score);
    }

    pub fn phase_6_final_round(&mut self) {
        let money_amount = self.board.get_current_space().unwrap_or_else(|_| panic!("Board out of index: {:?}", self)).money;
        self.score += money_amount / 5;
        confirm(format!("Receive {} points from money on board", money_amount / 5).as_str());
        info!("Received {} points from money", money_amount / 5);
    }

    pub fn phase_6_buy_chips(&mut self, purchasable_chips: &[PurchasableChip]) {
        let strategy = self.strategy.chip_buying_strategy.clone();
        strategy.buy_chips(self, purchasable_chips);
    }

    pub fn phase_7_spend_gems(&mut self, final_round: bool) {
        if self.gems > 1 {
            if final_round {
                let num_of_points = self.gems / 2;
                self.score += num_of_points as i32;
                confirm(format!("Receive {} points from gems", num_of_points).as_str());
                self.player_stats.points_from_gems += num_of_points as f32;
                self.gems -= num_of_points * 2;
            } else {
                if !self.has_potion && self.strategy.gem_spending_strategy.buy_potion() {
                    confirm("Buy your potion");
                    self.has_potion = true;
                    self.gems -= 2;
                    self.player_stats.bought_potions += 1.0;
                }
                if self.gems > 1 && self.strategy.gem_spending_strategy.buy_start_advance() {
                    let num_of_moves = self.gems / 2;
                    self.board.increase_start_index(num_of_moves);
                    confirm(format!("Buy {} number of spaces for droplet", num_of_moves).as_str());
                    self.player_stats.bought_start_advances += num_of_moves as f32;
                    self.gems -= num_of_moves * 2;
                }
            }
        }
    }

    pub fn round_6_add_white_chip(&mut self) {
        confirm("Add another 1 white chip for round 6");
        self.all_chips.push(Box::new(WhiteChip::new(1)));
    }
}
