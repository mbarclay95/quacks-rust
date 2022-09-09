use log::info;
use rand::Rng;

use crate::{ChipSet, SELECTED_CHIP_SET};
use crate::chip_set::PurchasableChip;
use crate::chips::green_chip::GreenChip;
use crate::chips::is_chip::IsChip;
use crate::chips::orange_chip::OrangeChip;
use crate::chips::white_chip::WhiteChip;
use crate::players::board::Board;
use crate::players::board_space::{LAST_PLAYABLE_SPACE};
use crate::players::player_stats::PlayerStats;

#[derive(Debug)]
pub struct Player {
    pub name: &'static str,
    pub board: Board,
    pub score: i32,
    pub player_stats: PlayerStats,
    pub bag: Vec<Box<dyn IsChip>>,
    gems: usize,
    has_potion: bool,
    all_chips: Vec<Box<dyn IsChip>>,
}

impl Player {
    pub fn new(name: &'static str) -> Self {
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
        }
    }

    pub fn play_through_phase_1(&mut self) {
        self.reset_bag_and_board();
        while !self.stop_drawing() {
            self.draw_chip();
        }
        if self.is_exploded() {
            self.player_stats.times_exploded += 1;
        }
        info!("Made it to: {:?} and exploded = {}", self.board.get_current_space(), self.is_exploded());
    }

    pub fn phase_2_role_dice(&mut self) {
        let mut rng = rand::thread_rng();
        let rand_num: i32 = rng.gen_range(0..6);
        info!("Dice role : {}", rand_num);
        match rand_num {
            0 | 1 => {
                self.score += 1;
                self.player_stats.points_from_dice += 1;
            }
            2 => {
                self.score += 2;
                self.player_stats.points_from_dice += 2;
            }
            3 => {
                self.gems += 1;
                self.player_stats.gems_from_dice += 1;
            }
            4 => {
                self.board.increase_start_index(1);
                self.player_stats.start_advance_from_dice += 1;
            }
            5 => {
                self.all_chips.push(Box::new(OrangeChip::new(1)));
                self.player_stats.orange_from_dice += 1;
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
                    self.player_stats.green_activation_count += 1;
                    info!("Received a gem from green chip");
                }
                if self.board.get_played_chip(played_chips_length - 2).get_color() == "green" {
                    self.gems += 1;
                    self.player_stats.green_activation_count += 1;
                    info!("Received a gem from green chip");
                }
            }
        }
    }

    pub fn round_6_add_white_chip(&mut self) {
        self.all_chips.push(Box::new(WhiteChip::new(1)));
    }

    pub fn handle_purple_chips(&mut self) {
        match SELECTED_CHIP_SET {
            ChipSet::ChapterOne => {
                let purple_num = self.board.get_played_chips_of_color("purple").len();
                match purple_num {
                    0 => {}
                    1 => {
                        self.score += 1;
                        self.player_stats.green_activation_count += 1;
                    }
                    2 => {
                        self.score += 1;
                        self.gems += 1;
                        self.player_stats.purple_activation_count += 1;
                    }
                    _ => {
                        self.score += 2;
                        self.board.increase_start_index(1);
                        self.player_stats.purple_activation_count += 1;
                    }
                }
            }
        }
    }

    pub fn get_black_chips_value(&mut self) {
        if !self.board.get_played_chips_of_color("black").is_empty() {
            info!("Received reward from black chip");
            self.gems += 1;
            self.board.increase_start_index(1);
            self.player_stats.black_activation_count += 1;
        }
    }

    pub fn phase_4_gem_check(&mut self) {
        if self.board.get_current_space().gem {
            info!("Received a gem from board space");
            self.gems += 1;
            self.player_stats.gems_from_board += 1;
        }
    }

    pub fn phase_5_points(&mut self) {
        self.score += self.board.get_current_space().points;
        info!("Received {} points, for total of {} points", self.board.get_current_space().points, self.score);
    }

    pub fn phase_6_buy_chips(&mut self, purchasable_chips: &[PurchasableChip], final_round: bool) {
        let mut money_amount = self.board.get_current_space().money;
        if final_round {
            self.score += money_amount / 5;
            info!("Received {} points from money", money_amount / 5);
            return;
        }
        info!("Available chips: {:?}", purchasable_chips);
        if money_amount < 3 {
            return;
        }
        let mut rng = rand::thread_rng();
        let affordable_chips = purchasable_chips.iter().filter(|p| p.price <= money_amount).collect::<Vec<&PurchasableChip>>();
        let index = rng.gen_range(0..affordable_chips.len());
        let purchased_chip = affordable_chips[index];
        money_amount -= purchased_chip.price;
        let already_purchased_color = purchased_chip.chip.get_color();
        self.all_chips.push(purchased_chip.chip.clone());
        self.player_stats.increment_correct_chip_count(purchased_chip.chip.get_color());
        info!("Purchased: {:?}", purchased_chip);
        // let most_expensive_chip = purchasable_chips.iter().filter(|p| p.price <= money_amount).max_by_key(|p| p.price);
        // let mut already_purchased_color = "";
        // if let Some(purchasable_chip) = most_expensive_chip {
        //     money_amount -= purchasable_chip.price;
        //     already_purchased_color = purchasable_chip.chip.get_color();
        //     self.all_chips.push(purchasable_chip.chip.clone());
        //     self.player_stats.increment_correct_chip_count(purchasable_chip.chip.get_color());
        //     info!("Purchased: {:?}", purchasable_chip);
        // }
        let affordable_chips = purchasable_chips.iter().filter(|p| p.price <= money_amount && p.chip.get_color() != already_purchased_color).collect::<Vec<&PurchasableChip>>();
        if affordable_chips.is_empty() {
            return;
        }
        let index = rng.gen_range(0..affordable_chips.len());
        let purchased_chip = affordable_chips[index];
        self.all_chips.push(purchased_chip.chip.clone());
        self.player_stats.increment_correct_chip_count(purchased_chip.chip.get_color());
        info!("Purchased: {:?}", purchased_chip);
        // let most_expensive_chip = purchasable_chips.iter().filter(|p| p.price <= money_amount && p.chip.get_color() != already_purchased_color).max_by_key(|p| p.price);
        // if let Some(purchasable_chip) = most_expensive_chip {
        //     self.all_chips.push(purchasable_chip.chip.clone());
        //     self.player_stats.increment_correct_chip_count(purchasable_chip.chip.get_color());
        //     info!("Purchased: {:?}", purchasable_chip);
        // }
    }

    pub fn phase_7_spend_gems(&mut self, final_round: bool) {
        if self.gems > 1 {
            let mut num_of_purchases = self.gems / 2;
            info!("Spending {} gems on {} purchases", self.gems, num_of_purchases);
            if final_round {
                self.score += num_of_purchases as i32;
                self.player_stats.points_from_gems += num_of_purchases as i32;
            } else {
                if !self.has_potion && rand::random() {
                    self.has_potion = true;
                    num_of_purchases -= 1;
                    self.player_stats.bought_potions += 1;
                }
                self.board.increase_start_index(num_of_purchases);
                self.player_stats.bought_start_advances += num_of_purchases as i32;
            }
            self.gems -= num_of_purchases * 2;
        }
    }

    pub fn stop_drawing(&self) -> bool {
        if self.bag.is_empty() || self.is_exploded() {
            return true;
        }
        if self.board.get_board_position() >= LAST_PLAYABLE_SPACE {
            return true;
        }
        let white_count = self.board.get_white_count();
        info!("white count at {}", white_count);
        let mut can_explode = false;
        for white_chip in self.bag.iter().filter(|p| p.get_color() == "white").collect::<Vec<&Box<dyn IsChip>>>() {
            if white_count + white_chip.get_value() > self.board.explode_limit {
                can_explode = true;
                info!("can explode with {:?}", white_chip);
                break;
            }
        }

        can_explode
    }

    pub fn is_exploded(&self) -> bool {
        self.board.check_if_exploded()
    }

    pub fn reset_bag_and_board(&mut self) {
        self.board.reset_board();
        self.bag = self.all_chips.clone();
    }

    pub fn draw_chip(&mut self) {
        if self.bag.is_empty() {
            return;
        }
        let mut rng = rand::thread_rng();
        let index = rng.gen_range(0..self.bag.len());
        let mut drawn_chip = self.bag[index].clone();
        info!("Drawn chip: {:?}", drawn_chip);
        if drawn_chip.get_color() == "white" && self.has_potion && rand::random() {
            info!("Used potion");
            self.player_stats.num_potions_used += 1;
            self.has_potion = false;
            return;
        }
        self.bag.remove(index);

        // I think this is the only chip in the game that needs to be played before logic happens
        if matches!(SELECTED_CHIP_SET, ChipSet::ChapterOne) && drawn_chip.get_color() == "blue" {
            self.board.play_chip(&drawn_chip);
            drawn_chip.perform_chip_logic(self);
        } else {
            drawn_chip.perform_chip_logic(self);
            self.board.play_chip(&drawn_chip);
        }
    }
}
