
#[derive(Debug)]
pub struct PlayerStats {
    pub starting_index: i32,
    pub gems_from_board: i32,
    pub orange_chips_bought: i32,
    pub green_chips_bought: i32,
    pub red_chips_bought: i32,
    pub blue_chips_bought: i32,
    pub yellow_chips_bought: i32,
    pub black_chips_bought: i32,
    pub purple_chips_bought: i32,
    pub green_activation_count: i32,
    pub black_activation_count: i32,
    pub purple_activation_count: i32,
    pub red_activation_count: i32,
    pub blue_activation_count: i32,
    pub yellow_activation_count: i32,
    pub num_potions_used: i32,
    pub bought_potions: i32,
    pub bought_start_advances: i32,
    pub score: i32,
    pub times_exploded: i32,
    pub points_from_dice: i32,
    pub gems_from_dice: i32,
    pub orange_from_dice: i32,
    pub start_advance_from_dice: i32,
    pub points_from_gems: i32,
}

impl PlayerStats {
    pub fn new() -> Self {
        PlayerStats {
            starting_index: 0,
            gems_from_board: 0,
            orange_chips_bought: 0,
            green_chips_bought: 0,
            red_chips_bought: 0,
            blue_chips_bought: 0,
            yellow_chips_bought: 0,
            black_chips_bought: 0,
            purple_chips_bought: 0,
            green_activation_count: 0,
            black_activation_count: 0,
            purple_activation_count: 0,
            red_activation_count: 0,
            blue_activation_count: 0,
            yellow_activation_count: 0,
            num_potions_used: 0,
            bought_potions: 0,
            times_exploded: 0,
            points_from_dice: 0,
            gems_from_dice: 0,
            orange_from_dice: 0,
            score: 0,
            start_advance_from_dice: 0,
            points_from_gems: 0,
            bought_start_advances: 0
        }
    }

    pub fn increment_correct_chip_count(&mut self, chip_color: &str) {
        match chip_color {
            "blue" => self.blue_chips_bought += 1,
            "black" => self.black_chips_bought += 1,
            "red" => self.red_chips_bought += 1,
            "yellow" => self.yellow_chips_bought += 1,
            "green" => self.green_chips_bought += 1,
            "purple" => self.purple_chips_bought += 1,
            "orange" => self.orange_chips_bought += 1,
            _ => {}
        }
    }
}

impl Default for PlayerStats {
    fn default() -> Self {
        PlayerStats::new()
    }
}
