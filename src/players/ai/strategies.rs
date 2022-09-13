use crate::players::ai::chip_buying_strategy::ChipBuyingStrategy;
use crate::players::ai::gem_spending_strategy::GemSpendingStrategy;
use crate::players::ai::potion_strategy::PotionStrategy;

#[derive(Debug)]
pub struct Strategies {
    pub potion_strategy: PotionStrategy,
    pub gem_spending_strategy: GemSpendingStrategy,
    pub chip_buying_strategy: ChipBuyingStrategy,
}
