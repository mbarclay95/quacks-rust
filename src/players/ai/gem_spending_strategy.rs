use rand::random;

#[derive(Debug)]
pub enum GemSpendingStrategy {
    Random,
    AlwaysPotionFirst,
    NeverPotion,
    OnlyPotion,
    NeverBuy,
    RandomPotionAlwaysAdvance
}

impl GemSpendingStrategy {
    pub fn buy_potion(&self) -> bool {
        match self {
            Self::Random | Self::RandomPotionAlwaysAdvance => random(),
            Self::AlwaysPotionFirst | Self::OnlyPotion => true,
            Self::NeverPotion | Self::NeverBuy => false,
        }
    }

    pub fn buy_start_advance(&self) -> bool {
        match self {
            Self::Random => random(),
            Self::NeverPotion | Self::AlwaysPotionFirst | Self::RandomPotionAlwaysAdvance => true,
            Self::OnlyPotion | Self::NeverBuy=> false,
        }
    }
}
