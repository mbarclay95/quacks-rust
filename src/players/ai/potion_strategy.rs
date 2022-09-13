use rand::random;
use crate::chips::is_chip::IsChip;

#[derive(Debug)]
pub enum PotionStrategy {
    Random,
    OnlyOn3,
    OnlyOn2,
    OnlyOn1,
    Always,
    OnlyIfChanceOfExploding
}

impl PotionStrategy {
    pub fn use_potion(&self, white_chip: &Box<dyn IsChip>, could_explode: bool) -> bool {
        match self {
            Self::Random => random(),
            Self::OnlyOn3 => white_chip.get_value() == 3,
            Self::OnlyOn2 => white_chip.get_value() == 2,
            Self::OnlyOn1 => white_chip.get_value() == 1,
            Self::Always => true,
            Self::OnlyIfChanceOfExploding => could_explode
        }
    }
}
