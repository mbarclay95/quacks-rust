use std::thread;
use std::thread::JoinHandle;
use std::time::Instant;

use quacks_rust::game::Game;

fn main() {
    let now = Instant::now();
    let num_of_games = 10000;
    let num_of_threads = 8;

    let mut threads = Vec::<JoinHandle<f32>>::new();
    for _ in 0..num_of_threads {
        let thread = thread::spawn(move || {
            let mut thread_results: Vec<i32> = vec![];
            let mut min = 100;
            let mut max = 0;
            for _ in 0..num_of_games / num_of_threads {
                let mut game = Game::new();
                game.play_game();
                let score = game.players[0].score;
                thread_results.push(score);
                if score < min { min = score };
                if score > max { max = score }
                // game.print_points();
                // game.print_stats();
            }
            println!("Max from thread: {}", max);
            println!("Min from thread: {}", min);
            let average = thread_results.iter().sum::<i32>() as f32 / thread_results.len() as f32;
            let standard_dev = (thread_results.iter().map(|&result| (result as f32 - average).powf(2.0)).sum::<f32>() / thread_results.len() as f32).sqrt();
            println!("Standard deviation from thread: {:.2}", standard_dev);
            println!("Expect between {:.2} and {:.2} points", average - standard_dev, average + standard_dev);
            println!();
            average
        });
        threads.push(thread);
    }

    let mut results: Vec<f32> = vec![];
    for thread in threads.into_iter() {
        results.push(thread.join().unwrap());
    }

    println!("Average score is {:.2}", results.iter().sum::<f32>() / results.len() as f32);
    println!("Elapsed: {:.2?}", now.elapsed());
}
