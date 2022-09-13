use log::info;
use rand::{Rng, thread_rng};

use crate::chip_set::PurchasableChip;
use crate::{ChipSet, confirm};
use crate::players::player::Player;

#[derive(Debug, Clone)]
pub enum ChipBuyingStrategy {
    Random,
    BlackThenRandom,
    PreferColor(String),
    BlackThenPreferColor(String),
    OrangeAndPreferColor(String),
    BlackThenOrangeAndPreferColor(String),
}

impl ChipBuyingStrategy {
    pub fn buy_chips(&self, player: &mut Player, purchasable_chips: &[PurchasableChip]) {
        let mut money_amount = player.board.get_current_space().unwrap_or_else(|_| panic!("Board out of index: {:?}", self)).money;
        confirm(format!("Receive {} money from board", money_amount).as_str());
        if money_amount < 3 {
            return;
        }
        info!("Available chips: {:?}", purchasable_chips);
        match self {
            Self::Random => {
                let affordable_chips = PurchasableChip::get_affordable(purchasable_chips, money_amount);
                let already_purchased_color = ChipBuyingStrategy::buy_random(affordable_chips, player, &mut money_amount);
                confirm(format!("{} money remaining", money_amount).as_str());

                let affordable_chips = PurchasableChip::get_affordable_exclude_color(purchasable_chips, money_amount, already_purchased_color);
                if affordable_chips.is_empty() {
                    return;
                }
                ChipBuyingStrategy::buy_random(affordable_chips, player, &mut money_amount);
            },
            Self::BlackThenRandom => {
                let already_purchased_color = if player.chip_count_of_color("black") == 0 && money_amount >= ChipSet::get_black_chip_price() {
                    let black_chip = PurchasableChip::get_affordable_by_color(purchasable_chips, money_amount, "black".into());
                    ChipBuyingStrategy::buy_most_expensive(black_chip, player, &mut money_amount).unwrap()
                } else {
                    let affordable_chips = PurchasableChip::get_affordable(purchasable_chips, money_amount);
                    ChipBuyingStrategy::buy_random(affordable_chips, player, &mut money_amount)
                };
                confirm(format!("{} money remaining", money_amount).as_str());

                let affordable_chips = PurchasableChip::get_affordable_exclude_color(purchasable_chips, money_amount, already_purchased_color);
                if affordable_chips.is_empty() {
                    return;
                }
                ChipBuyingStrategy::buy_random(affordable_chips, player, &mut money_amount);
            }
            Self::PreferColor(color) => {
                let mut affordable_chips = PurchasableChip::get_affordable_by_color(purchasable_chips, money_amount, color.into());
                if affordable_chips.is_empty() {
                    affordable_chips = PurchasableChip::get_affordable(purchasable_chips, money_amount);
                }
                let already_purchased_color = ChipBuyingStrategy::buy_most_expensive(affordable_chips, player, &mut money_amount).unwrap();
                confirm(format!("{} money remaining", money_amount).as_str());

                let affordable_chips = PurchasableChip::get_affordable_exclude_color(purchasable_chips, money_amount, already_purchased_color);
                if affordable_chips.is_empty() {
                    return;
                }
                ChipBuyingStrategy::buy_random(affordable_chips, player, &mut money_amount);
            },
            Self::BlackThenPreferColor(color) => {
                let already_purchased_color = if player.chip_count_of_color("black") == 0 && money_amount >= ChipSet::get_black_chip_price() {
                    let black_chip = PurchasableChip::get_affordable_by_color(purchasable_chips, money_amount, "black".into());
                    ChipBuyingStrategy::buy_most_expensive(black_chip, player, &mut money_amount).unwrap()
                } else {
                    let mut affordable_chips = PurchasableChip::get_affordable_by_color(purchasable_chips, money_amount, color.into());
                    if affordable_chips.is_empty() {
                        affordable_chips = PurchasableChip::get_affordable(purchasable_chips, money_amount);
                    }
                    ChipBuyingStrategy::buy_most_expensive(affordable_chips, player, &mut money_amount).unwrap()
                };
                confirm(format!("{} money remaining", money_amount).as_str());

                let affordable_chips = PurchasableChip::get_affordable_exclude_color(purchasable_chips, money_amount, already_purchased_color);
                if affordable_chips.is_empty() {
                    return;
                }
                ChipBuyingStrategy::buy_random(affordable_chips, player, &mut money_amount);
            },
            Self::OrangeAndPreferColor(color) => {
                let mut affordable_chips = PurchasableChip::get_affordable_by_color(purchasable_chips, money_amount, color.into());
                if affordable_chips.is_empty() {
                    affordable_chips = PurchasableChip::get_affordable(purchasable_chips, money_amount);
                }
                let already_purchased_color = ChipBuyingStrategy::buy_most_expensive(affordable_chips, player, &mut money_amount).unwrap();
                confirm(format!("{} money remaining", money_amount).as_str());

                let affordable_chips = if money_amount > 4 {
                    PurchasableChip::get_affordable_exclude_color(purchasable_chips, money_amount, already_purchased_color)
                } else {
                    PurchasableChip::get_affordable_by_color(purchasable_chips, money_amount, "orange".into())
                };
                if affordable_chips.is_empty() {
                    return;
                }
                ChipBuyingStrategy::buy_random(affordable_chips, player, &mut money_amount);
            },
            Self::BlackThenOrangeAndPreferColor(color) => {
                let already_purchased_color = if player.chip_count_of_color("black") == 0 && money_amount >= ChipSet::get_black_chip_price() {
                    let black_chip = PurchasableChip::get_affordable_by_color(purchasable_chips, money_amount, "black".into());
                    ChipBuyingStrategy::buy_most_expensive(black_chip, player, &mut money_amount).unwrap()
                } else {
                    let mut affordable_chips = PurchasableChip::get_affordable_by_color(purchasable_chips, money_amount, color.into());
                    if affordable_chips.is_empty() {
                        affordable_chips = PurchasableChip::get_affordable(purchasable_chips, money_amount);
                    }
                    ChipBuyingStrategy::buy_most_expensive(affordable_chips, player, &mut money_amount).unwrap()
                };
                confirm(format!("{} money remaining", money_amount).as_str());

                let affordable_chips = if money_amount > 4 {
                    PurchasableChip::get_affordable_exclude_color(purchasable_chips, money_amount, already_purchased_color)
                } else {
                    PurchasableChip::get_affordable_by_color(purchasable_chips, money_amount, "orange".into())
                };
                if affordable_chips.is_empty() {
                    return;
                }
                ChipBuyingStrategy::buy_random(affordable_chips, player, &mut money_amount);
            },
        }
    }

    fn buy_random(chips: Vec<&PurchasableChip>, player: &mut Player, money: &mut i32) -> String {
        let index = thread_rng().gen_range(0..chips.len());
        let purchased_chip = chips[index];
        *money -= purchased_chip.price;
        player.all_chips.push(purchased_chip.chip.clone());
        player.player_stats.increment_correct_chip_count(purchased_chip.chip.get_color());
        info!("Purchased: {:?}", purchased_chip);
        confirm(format!("Purchased chip {:?}", purchased_chip).as_str());

        purchased_chip.chip.get_color().into()
    }

    fn buy_most_expensive(chips: Vec<&PurchasableChip>, player: &mut Player, money: &mut i32) -> Option<String> {
        let max_chip_option = chips.iter().max_by_key(|chip| chip.price);
        if let Some(max_chip) = max_chip_option {
            player.all_chips.push(max_chip.chip.clone());
            player.player_stats.increment_correct_chip_count(max_chip.chip.get_color());
            confirm(format!("Purchased chip {:?}", max_chip).as_str());
            *money -= max_chip.price;

            return Some(max_chip.chip.get_color().into());
        }

        None
    }
}


