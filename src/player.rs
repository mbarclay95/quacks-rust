use log::info;
use crate::board::Board;
use crate::chips::green_chip::GreenChip;
use crate::chips::is_chip::IsChip;
use crate::chips::orange_chip::OrangeChip;
use crate::chips::white_chip::WhiteChip;
use rand::Rng;
use crate::{ChipSet, SELECTED_CHIP_SET};
use crate::board_space::MAX_BOARD_SPACES;
use crate::chip_set::PurchasableChip;

#[derive(Debug)]
pub struct Player {
    pub name: &'static str,
    pub board: Board,
    pub score: i32,
    gems: usize,
    potion: bool,
    pub bag: Vec<Box<dyn IsChip>>,
    all_chips: Vec<Box<dyn IsChip>>
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
            ]
        }
    }

    pub fn play_through_phase_1(&mut self) {
        self.reset_bag();
        while !self.stop_drawing() {
            self.draw_chip().unwrap();
        }
        info!("Made it to: {:?} and exploded = {}", self.board.get_current_space(), self.is_exploded());
    }

    pub fn phase_2_role_dice(&mut self) {
        let mut rng = rand::thread_rng();
        let rand_num: i32 = rng.gen_range(0..6);
        info!("Dice role : {}", rand_num);
        match rand_num {
            0 | 1 => self.score += 1,
            2 => self.score += 2,
            3 => self.gems += 1,
            4 => self.board.increase_start_index(1),
            5 => self.all_chips.push(Box::new(OrangeChip::new(1))),
            _ => {}
        }
    }

    pub fn handle_green_chips(&mut self) {
        match SELECTED_CHIP_SET {
            ChipSet::ChapterOne => {
                let played_chips_length = self.board.get_played_chip_len();
                if self.board.get_played_chip(played_chips_length - 1).get_color() == "green" {
                    self.gems += 1;
                    info!("Received a gem from green chip");
                }
                if self.board.get_played_chip(played_chips_length - 2).get_color() == "green" {
                    self.gems += 1;
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
                    0 => {},
                    1 => self.score += 1,
                    2 => {
                        self.score += 1;
                        self.gems += 1;
                    },
                    _ => {
                        self.score += 2;
                        self.board.increase_start_index(1);
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
        }
    }

    pub fn phase_4_gem_check(&mut self) {
        if self.board.get_current_space().gem {
            info!("Received a gem from board space");
            self.gems += 1;
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
        let most_expensive_chip = purchasable_chips.iter().filter(|p| p.price <= money_amount).max_by_key(|p| p.price);
        let mut already_purchased_color = "";
        if let Some(purchasable_chip) = most_expensive_chip {
            money_amount -= purchasable_chip.price;
            already_purchased_color = purchasable_chip.chip.get_color();
            self.all_chips.push(purchasable_chip.chip.clone());
            info!("Purchased: {:?}", purchasable_chip);
        }
        if money_amount < 3 {
            return;
        }
        let most_expensive_chip = purchasable_chips.iter().filter(|p| p.price <= money_amount && p.chip.get_color() != already_purchased_color).max_by_key(|p| p.price);
        if let Some(purchasable_chip) = most_expensive_chip {
            self.all_chips.push(purchasable_chip.chip.clone());
            info!("Purchased: {:?}", purchasable_chip);
        }
    }

    pub fn phase_7_spend_gems(&mut self, final_round: bool) {
        if self.gems > 1 {
            let num_of_purchases: usize = self.gems / 2;
            info!("Spending {} gems on {} purchases", self.gems, num_of_purchases);
            if final_round {
                self.score += num_of_purchases as i32;
            } else {
                self.board.increase_start_index(num_of_purchases);
            }
            self.gems -= num_of_purchases * 2;
        }
    }

    fn stop_drawing(&self) -> bool {
        if self.is_exploded() {
            return true;
        }
        if self.board.get_board_space_position() >= MAX_BOARD_SPACES - 1 {
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

    pub fn reset_bag(&mut self) {
        self.board.clear_board();
        self.bag = self.all_chips.clone();
    }

    pub fn draw_chip(&mut self) -> Result<(), &'static str> {
        if self.bag.is_empty() {
            return Err("Bag is empty");
        }
        let mut rng = rand::thread_rng();
        let index = rng.gen_range(0..self.bag.len());
        let mut drawn_chip = self.bag[index].clone();
        self.bag.remove(index);
        let additional_chip = drawn_chip.perform_chip_logic(self);
        info!("Drawn chip: {:?}", drawn_chip);
        self.board.play_chip(drawn_chip);

        if let Some(mut chip) = additional_chip {
            chip.perform_chip_logic(self);
            info!("Additional chip: {:?}", chip);
            self.board.play_chip(chip);
        }

        Ok(())
    }

}
