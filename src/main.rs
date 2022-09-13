use std::thread;
use std::thread::JoinHandle;
use std::time::Instant;

use quacks_rust::game::Game;
use quacks_rust::players::player_stats::PlayerStats;

fn main() {
    let now = Instant::now();

    play_many_games();
    // play_one_game();

    println!("Elapsed: {:.2?}", now.elapsed());
}

fn play_one_game() {
    let mut game = Game::new();
    game.play_game();
    game.print_stats();
}

fn play_many_games() {
    let num_of_games = 100000;
    let num_of_threads = 32;

    let mut threads = Vec::<JoinHandle<PlayerStats>>::new();
    for _ in 0..num_of_threads {
        let thread = thread::spawn(move || {
            let mut thread_stats = PlayerStats::new();
            for _ in 0..num_of_games / num_of_threads {
                let mut game = Game::new();
                game.play_game();
                thread_stats.append_to_self(&game.players[0].player_stats);
            }
            thread_stats.get_average(num_of_games / num_of_threads);

            thread_stats
        });
        threads.push(thread);
    }

    let mut stats = PlayerStats::new();
    for thread in threads.into_iter() {
        let results = thread.join().unwrap();
        stats.append_to_self(&results);
    }

    stats.get_average(num_of_threads);
    println!("Average stats: {:?}", stats);
    // let standard_dev = results_sd.iter().sum::<f32>() / results_sd.len() as f32;
    // let standard_dev = (thread_results.iter().map(|&result| (result as f32 - average).powf(2.0)).sum::<f32>() / thread_results.len() as f32).sqrt();
    // println!("Standard deviation from thread: {:.2}", standard_dev);
    // println!("Expect between {:.2} and {:.2} points", average - standard_dev, average + standard_dev);
    // println!("Average score is {:.2}", average);
}

