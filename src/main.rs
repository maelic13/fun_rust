use std::sync::mpsc;
use std::thread;

use stopwatch::Stopwatch;


struct Result {
    successful: u64,
    total: u64
}


fn main() {
    let cpu_count = num_cpus::get();
    const CYCLES: u64 = 1000000000;

    let mut watch = Stopwatch::start_new();
    let result1 = play_game_multithreaded(CYCLES, false, cpu_count);
    print_results_to_console(result1, watch, CYCLES, false);

    watch.restart();
    let result2 = play_game_multithreaded(CYCLES, true, cpu_count);
    print_results_to_console(result2, watch, CYCLES, true);
}


fn play_game(cycles: u64, change_choice: bool) -> Result {
    let doors = [false, true, false];
    let rng = fastrand::Rng::new();
    let mut result = Result { successful: 0, total: 0 };

    for _i in 0..cycles {
        result.total += 1;
        if change_choice != doors[rng.usize(0..3)] {
            result.successful += 1;
        }
    }
    return result;
}


fn play_game_multithreaded(cycles: u64, change_choice: bool, cpu_count: usize) -> Result {
    if cpu_count == 1 {
        return play_game(cycles, change_choice);
    }

    let mut complete_result = Result { successful: 0, total: 0 };
    let (tx, rx) = mpsc::channel();
    let mut sub_cycles = Vec::new();

    for _ in 0..cpu_count - 1 {
        sub_cycles.push(cycles as u64 / cpu_count as u64);
    }
    sub_cycles.push(cycles as u64 / cpu_count as u64 + cycles as u64 % cpu_count as u64);

    for cycle in sub_cycles {
        let txc = tx.clone();
        thread::spawn(move || {
            txc.send(play_game(cycle, change_choice)).unwrap();
        });
    }
    for result in rx.iter().take(cpu_count) {
        complete_result.total += result.total;
        complete_result.successful += result.successful;
    }
    return complete_result;
}


fn print_results_to_console(result: Result, watch: Stopwatch, cycles: u64, change_choice: bool) {
    println!();
    println!("Change = {}. Time elapsed {:#?}", change_choice, watch.elapsed());
    println!("{} successful tries, {} total. Success rate {} %.", result.successful, result.total,
             result.successful as f64 / result.total as f64 * 100 as f64);
    println!("Speed = {} Miter/s.", cycles / watch.elapsed_ms() as u64 / 1000 as u64);
}