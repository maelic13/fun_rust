mod result;

use result::Result;
use std::sync::mpsc;
use std::thread;
use std::time::{Duration, Instant};

fn play_game(cycles: u64, change_choice: bool) -> Result {
    let doors = [false, true, false];
    let mut rng = fastrand::Rng::new();
    let mut result = Result::default();

    (0..cycles).for_each(|_i| {
        if change_choice != doors[rng.u8(0..3) as usize] {
            result.successful += 1;
        } else {
            result.failed += 1;
        }
    });
    result
}

fn play_game_multithreaded(cycles: u64, change_choice: bool, cpu_count: usize) -> Result {
    if cpu_count == 1 {
        return play_game(cycles, change_choice);
    }

    let (tx, rx) = mpsc::channel();
    let mut sub_cycles = Vec::new();

    (0..cpu_count - 1).for_each(|_| {
        sub_cycles.push(cycles as u64 / cpu_count as u64);
    });
    sub_cycles.push(cycles as u64 / cpu_count as u64 + cycles as u64 % cpu_count as u64);

    sub_cycles.into_iter().for_each(|cycle| {
        let txc = tx.clone();
        thread::spawn(move || {
            txc.send(play_game(cycle, change_choice)).unwrap();
        });
    });
    let results = rx.iter().take(cpu_count);
    Result::combine(results)
}

fn print_results_to_console(
    result: Result,
    time_elapsed: Duration,
    cycles: u64,
    change_choice: bool,
) {
    println!();
    println!(
        "Change = {}. Time elapsed {:#.1?}.",
        change_choice, time_elapsed
    );
    println!(
        "{} successful tries, {} total. Success rate {:.3} %.",
        result.successful,
        result.successful + result.failed,
        result.successful as f64 / (result.successful + result.failed) as f64 * 100f64
    );
    println!(
        "Speed = {} Miter/s.",
        cycles / time_elapsed.as_millis() as u64 / 1000u64
    );
}

fn main() {
    let cpu_count = num_cpus::get();
    const CYCLES: u64 = 1000000000;

    let mut start = Instant::now();
    let mut result = play_game_multithreaded(CYCLES, false, cpu_count);
    print_results_to_console(result, start.elapsed(), CYCLES, false);

    start = Instant::now();
    result = play_game_multithreaded(CYCLES, true, cpu_count);
    print_results_to_console(result, start.elapsed(), CYCLES, true);
}
