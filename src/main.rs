use quacks_rust::game::Game;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {

    let mut results: Vec<i32> = vec![];
    let num_of_games = 1;

    for _ in 0..num_of_games {
        let mut game = Game::new();
        game.play_game();
        results.push(game.players[0].score);
        game.print_points();
        game.print_stats();
    }

    println!("Average score is {}", results.iter().sum::<i32>() as f32 / results.len() as f32);

    Ok(())
}
