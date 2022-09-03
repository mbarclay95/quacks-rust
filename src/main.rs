use quacks_rust::board::Board;
use quacks_rust::player::Player;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {

    let player = Player::new("Michael");

    Ok(())
}
