use std::time::Instant;
use quacks_rust::game::Game;

fn main() {
    let now = Instant::now();
    let mut results: Vec<i32> = vec![];
    let num_of_games = 1000;

    for _ in 0..num_of_games {
        let mut game = Game::new();
        game.play_game();
        results.push(game.players[0].score);
        // game.print_points();
        // game.print_stats();
    }

    println!("Average score is {}", results.iter().sum::<i32>() as f32 / results.len() as f32);
    println!("Elapsed: {:.2?}", now.elapsed());
}
