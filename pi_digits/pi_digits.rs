use std::{sync::mpsc, thread};

use stopwatch::Stopwatch;

fn get_points(count: u64) -> u64 {
    let mut inside: u64 = 0;
    let rng = fastrand::Rng::new();

    (0..count).for_each(|_i| {
        let x: f64 = rng.f64();
        let y: f64 = rng.f64();

        if (x.powi(2) + y.powi(2)).sqrt() <= 1.0 {
            inside += 1;
        }
    });

    return inside;
}

fn get_pi_monte_carlo_multithreaded(digits: u32) -> f64 {
    let cpu_count = num_cpus::get();
    let base: u64 = 10;
    let points_count: u64 = base.pow(2 * digits);

    let (tx, rx) = mpsc::channel();
    let mut sub_cycles = Vec::new();

    (0..cpu_count - 1).for_each(|_| {
        sub_cycles.push(points_count as u64 / cpu_count as u64);
    });
    sub_cycles.push(
        points_count as u64 / cpu_count as u64 + points_count as u64 % cpu_count as u64);

    sub_cycles.into_iter().for_each(|sub| {
        let txc = tx.clone();
        thread::spawn(move || {
            txc.send(get_points(sub)).unwrap();
        });
    });
    let results = rx.iter().take(cpu_count);

    return 4.0 * results.sum::<u64>() as f64 / points_count as f64;
}

fn main() {
    let digits = 4;

    let mut watch = Stopwatch::start_new();
    let pi = get_pi_monte_carlo_multithreaded(digits + 1);
    watch.stop();

    let base: u32 = 10;
    let real_pi = 3.14159265358979;
    println!(
        "Real Pi: {}\nPi: {}\nTime elapsed {:#?} ms.",
        (real_pi * base.pow(digits) as f64).round() / base.pow(digits) as f64,
        (pi * base.pow(digits) as f64).round() / base.pow(digits) as f64,
        watch.elapsed().as_millis()
    );
}
