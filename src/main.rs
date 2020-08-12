#![feature(box_syntax)]

use num_cpus;
use oorandom::Rand32;
use rayon::prelude::*;
use std::time::SystemTime;

#[macro_use]
extern crate lazy_static;

lazy_static! {
    static ref CLOCK: SystemTime = SystemTime::now();
}

fn main() {
    const SIZE: usize = 10000;
    let mut matrix = box [0u8; SIZE * SIZE];
    let pool = rayon::ThreadPoolBuilder::new().build().unwrap();
    let mut sum = 0.0;
    let amt = 10;

    println!(
        "\t{} Physical Cores | {} Logical Cores",
        num_cpus::get_physical(),
        num_cpus::get()
    );

    println!(
        "Running {} iterations on {} threads...\n",
        amt,
        pool.current_num_threads()
    );

    let _ = pool.install(|| {
        for i in 0..amt {
            let loop_clock = SystemTime::now();

            matrix.par_iter_mut().for_each(|x| {
                let seed = CLOCK.elapsed().unwrap().as_secs();
                let mut rng = Rand32::new(seed);
                *x = rng.rand_u32() as u8;
            });

            let elapsed = loop_clock
                .elapsed()
                .expect("Could not calculate differences")
                .as_secs_f64();

            println!("[{}]\tTook: {:.4}s", i + 1, elapsed);

            sum += elapsed;
        }
    });

    println!("\nAverage: {:.4}s", sum / amt as f64);
}
