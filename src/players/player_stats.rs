#[derive(Debug)]
pub struct PlayerStats {
    pub starting_index: f32,
    pub bought_start_advances: f32,
    pub num_potions_used: f32,
    pub bought_potions: f32,
    pub score: f32,
    pub times_exploded: f32,
    pub gems_from_board: f32,
    pub points_from_gems: f32,
    pub orange_chips_bought: f32,
    pub green_chips_bought: f32,
    pub green_activation_count: f32,
    pub black_chips_bought: f32,
    pub black_activation_count: f32,
    pub purple_chips_bought: f32,
    pub purple_activation_count: f32,
    pub red_chips_bought: f32,
    pub red_activation_count: f32,
    pub blue_chips_bought: f32,
    pub blue_activation_count: f32,
    pub yellow_chips_bought: f32,
    pub yellow_activation_count: f32,
    pub points_from_dice: f32,
    pub gems_from_dice: f32,
    pub orange_from_dice: f32,
    pub start_advance_from_dice: f32,
}

impl PlayerStats {
    pub fn new() -> Self {
        PlayerStats {
            starting_index: 0.0,
            gems_from_board: 0.0,
            orange_chips_bought: 0.0,
            green_chips_bought: 0.0,
            red_chips_bought: 0.0,
            blue_chips_bought: 0.0,
            yellow_chips_bought: 0.0,
            black_chips_bought: 0.0,
            purple_chips_bought: 0.0,
            green_activation_count: 0.0,
            black_activation_count: 0.0,
            purple_activation_count: 0.0,
            red_activation_count: 0.0,
            blue_activation_count: 0.0,
            yellow_activation_count: 0.0,
            num_potions_used: 0.0,
            bought_potions: 0.0,
            times_exploded: 0.0,
            points_from_dice: 0.0,
            gems_from_dice: 0.0,
            orange_from_dice: 0.0,
            score: 0.0,
            start_advance_from_dice: 0.0,
            points_from_gems: 0.0,
            bought_start_advances: 0.0,
        }
    }

    pub fn increment_correct_chip_count(&mut self, chip_color: &str) {
        match chip_color {
            "blue" => self.blue_chips_bought += 1.0,
            "black" => self.black_chips_bought += 1.0,
            "red" => self.red_chips_bought += 1.0,
            "yellow" => self.yellow_chips_bought += 1.0,
            "green" => self.green_chips_bought += 1.0,
            "purple" => self.purple_chips_bought += 1.0,
            "orange" => self.orange_chips_bought += 1.0,
            _ => {}
        }
    }

    pub fn append_to_self(&mut self, player_stats: &PlayerStats) {
        self.starting_index += player_stats.starting_index;
        self.gems_from_board += player_stats.gems_from_board;
        self.orange_chips_bought += player_stats.orange_chips_bought;
        self.green_chips_bought += player_stats.green_chips_bought;
        self.red_chips_bought += player_stats.red_chips_bought;
        self.blue_chips_bought += player_stats.blue_chips_bought;
        self.yellow_chips_bought += player_stats.yellow_chips_bought;
        self.black_chips_bought += player_stats.black_chips_bought;
        self.purple_chips_bought += player_stats.purple_chips_bought;
        self.green_activation_count += player_stats.green_activation_count;
        self.black_activation_count += player_stats.black_activation_count;
        self.purple_activation_count += player_stats.purple_activation_count;
        self.red_activation_count += player_stats.red_activation_count;
        self.blue_activation_count += player_stats.blue_activation_count;
        self.yellow_activation_count += player_stats.yellow_activation_count;
        self.num_potions_used += player_stats.num_potions_used;
        self.bought_potions += player_stats.bought_potions;
        self.times_exploded += player_stats.times_exploded;
        self.points_from_dice += player_stats.points_from_dice;
        self.gems_from_dice += player_stats.gems_from_dice;
        self.orange_from_dice += player_stats.orange_from_dice;
        self.start_advance_from_dice += player_stats.start_advance_from_dice;
        self.score += player_stats.score;
        self.points_from_gems += player_stats.points_from_gems;
        self.bought_start_advances += player_stats.bought_start_advances;
    }

    pub fn get_average(&mut self, num_of_games: i32) {
        self.starting_index /= num_of_games as f32;
        self.gems_from_board /= num_of_games as f32;
        self.orange_chips_bought /= num_of_games as f32;
        self.green_chips_bought /= num_of_games as f32;
        self.red_chips_bought /= num_of_games as f32;
        self.blue_chips_bought /= num_of_games as f32;
        self.yellow_chips_bought /= num_of_games as f32;
        self.black_chips_bought /= num_of_games as f32;
        self.purple_chips_bought /= num_of_games as f32;
        self.green_activation_count /= num_of_games as f32;
        self.black_activation_count /= num_of_games as f32;
        self.purple_activation_count /= num_of_games as f32;
        self.red_activation_count /= num_of_games as f32;
        self.blue_activation_count /= num_of_games as f32;
        self.yellow_activation_count /= num_of_games as f32;
        self.num_potions_used /= num_of_games as f32;
        self.bought_potions /= num_of_games as f32;
        self.times_exploded /= num_of_games as f32;
        self.points_from_dice /= num_of_games as f32;
        self.gems_from_dice /= num_of_games as f32;
        self.orange_from_dice /= num_of_games as f32;
        self.start_advance_from_dice /= num_of_games as f32;
        self.score /= num_of_games as f32;
        self.points_from_gems /= num_of_games as f32;
        self.bought_start_advances /= num_of_games as f32;
    }
}

impl Default for PlayerStats {
    fn default() -> Self {
        PlayerStats::new()
    }
}
