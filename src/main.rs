use std::thread;
use std::thread::JoinHandle;
use std::time::Instant;

use quacks_rust::game::Game;

fn main() {
    let now = Instant::now();
    let num_of_games = 100000;
    let num_of_threads = 8;

    let mut threads = Vec::<JoinHandle<f32>>::new();
    for _ in 0..num_of_threads {
        let thread = thread::spawn(move || {
            let mut thread_results: Vec<i32> = vec![];
            for _ in 0..num_of_games / num_of_threads {
                let mut game = Game::new();
                game.play_game();
                thread_results.push(game.players[0].score);
                // game.print_points();
                // game.print_stats();
            }
            thread_results.iter().sum::<i32>() as f32 / thread_results.len() as f32
        });
        threads.push(thread);
    }

    let mut results: Vec<f32> = vec![];
    for thread in threads.into_iter() {
        results.push(thread.join().unwrap());
    }

    println!("Average score is {}", results.iter().sum::<f32>() / results.len() as f32);
    println!("Elapsed: {:.2?}", now.elapsed());
}
