use rand::Rng;
use quacks_rust::game::Game;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {

    let mut results: Vec<i32> = vec![];

    for _ in 0..1000 {
        let mut game = Game::new();
        game.play_game();
        // game.print_points();
        results.push(game.players[0].score);
    }

    println!("Average score is {}", results.iter().sum::<i32>() as f32 / results.len() as f32);

    Ok(())
}
