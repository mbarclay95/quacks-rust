use crate::chips::black_chip::BlackChip;
use crate::chips::blue_chip::BlueChip;
use crate::chips::green_chip::GreenChip;
use crate::chips::is_chip::IsChip;
use crate::chips::orange_chip::OrangeChip;
use crate::chips::purple_chip::PurpleChip;
use crate::chips::red_chip::RedChip;
use crate::chips::yellow_chip::YellowChip;
use crate::SELECTED_CHIP_SET;

#[derive(Debug)]
pub enum ChipSet {
    ChapterOne
}

#[derive(Debug)]
pub struct PurchasableChip {
    pub price: i32,
    pub chip: Box<dyn IsChip>,
}

impl ChipSet {
    pub fn get_starting_purchasable_chips() -> Vec<PurchasableChip> {
        match SELECTED_CHIP_SET {
            ChipSet::ChapterOne => vec![
                // Orange
                PurchasableChip { price: 3, chip: Box::new(OrangeChip::new(1)) },
                PurchasableChip { price: 22, chip: Box::new(OrangeChip::new(6)) },
                // Green
                PurchasableChip { price: 4, chip: Box::new(GreenChip::new(1)) },
                PurchasableChip { price: 8, chip: Box::new(GreenChip::new(2)) },
                PurchasableChip { price: 14, chip: Box::new(GreenChip::new(4)) },
                // Red
                PurchasableChip { price: 6, chip: Box::new(RedChip::new(1)) },
                PurchasableChip { price: 10, chip: Box::new(RedChip::new(2)) },
                PurchasableChip { price: 16, chip: Box::new(RedChip::new(4)) },
                // Blue
                PurchasableChip { price: 5, chip: Box::new(BlueChip::new(1)) },
                PurchasableChip { price: 10, chip: Box::new(BlueChip::new(2)) },
                PurchasableChip { price: 19, chip: Box::new(BlueChip::new(4)) },
                // Black
                PurchasableChip { price: 10, chip: Box::new(BlackChip::new(1)) },
            ]
        }
    }
    
    pub fn get_black_chip_price() -> i32 {
        match SELECTED_CHIP_SET {
            ChipSet::ChapterOne => 10
        }
    }

    pub fn get_yellow_chips() -> Vec<PurchasableChip> {
        match SELECTED_CHIP_SET {
            ChipSet::ChapterOne => vec![
                // Yellow
                PurchasableChip { price: 8, chip: Box::new(YellowChip::new(1)) },
                PurchasableChip { price: 12, chip: Box::new(YellowChip::new(2)) },
                PurchasableChip { price: 18, chip: Box::new(YellowChip::new(4)) },
            ]
        }
    }

    pub fn get_purple_chips() -> Vec<PurchasableChip> {
        match SELECTED_CHIP_SET {
            ChipSet::ChapterOne => vec![
                // Purple
                PurchasableChip { price: 9, chip: Box::new(PurpleChip::new(1)) },
            ]
        }
    }
}

impl PurchasableChip {

    pub fn get_affordable(chips: &[PurchasableChip], money_amount: i32) -> Vec<&PurchasableChip> {
        chips.iter().filter(|p| p.price <= money_amount).collect::<Vec<&PurchasableChip>>()
    }

    pub fn get_affordable_exclude_color(chips: &[PurchasableChip], money_amount: i32, exclude_color: String) -> Vec<&PurchasableChip> {
        chips.iter().filter(|p| p.price <= money_amount && p.chip.get_color() != exclude_color.as_str()).collect::<Vec<&PurchasableChip>>()
    }

    pub fn get_affordable_by_color(chips: &[PurchasableChip], money_amount: i32, color: String) -> Vec<&PurchasableChip> {
        chips.iter().filter(|p| p.price <= money_amount && p.chip.get_color() == color.as_str()).collect::<Vec<&PurchasableChip>>()
    }
}
