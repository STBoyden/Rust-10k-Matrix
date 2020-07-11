#![feature(box_syntax)]

use rand::Rng;
use rand_xorshift::XorShiftRng;
use rayon::prelude::*;
use std::time::SystemTime;

#[cfg(not(target_env = "msvc"))]
use jemallocator::Jemalloc;

#[cfg(not(target_env = "msvc"))]
#[global_allocator]
static GLOBAL: Jemalloc = Jemalloc;

fn main() {
    let clock = SystemTime::now();

    loop {
        let loop_clock = SystemTime::now();
        let mut matrix = box [[0u8; 10000]; 10000];

        matrix.par_iter_mut().for_each(|matrix_row| {
            matrix_row.par_iter_mut().for_each(|value| {
                let seed = clock
                    .elapsed()
                    .expect("Could not get elapsed time since program start")
                    .as_micros()
                    .to_ne_bytes();

                let mut rng: XorShiftRng = rand::SeedableRng::from_seed(seed);
                *value = rng.gen();
            })
        });

        let elapsed = loop_clock
            .elapsed()
            .expect("Could not calculate differences")
            .as_secs_f64();

        println!("{}", elapsed);
    }
}
