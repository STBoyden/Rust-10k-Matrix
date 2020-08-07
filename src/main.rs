#![feature(box_syntax)]

use rand::Rng;
use rand_xorshift::XorShiftRng;
use rayon::prelude::*;
// use std::sync::{Arc, Mutex};
use std::time::SystemTime;

#[macro_use]
extern crate lazy_static;

#[cfg(not(target_env = "msvc"))]
use jemallocator::Jemalloc;

#[cfg(not(target_env = "msvc"))]
#[global_allocator]
static GLOBAL: Jemalloc = Jemalloc;

lazy_static! {
    static ref CLOCK: SystemTime = SystemTime::now();
}

fn change_value(value: &mut u8) {
    let seed = CLOCK
        .elapsed()
        .expect("Could not get elapsed time")
        .as_micros()
        .to_ne_bytes();
    let mut rng: XorShiftRng = rand::SeedableRng::from_seed(seed);
    *value = rng.gen();
}

fn main() {
    let mut matrix = box [[0u8; 10000]; 10000];
    loop {
        let loop_clock = SystemTime::now();

        matrix.splitn_mut(16, |_| true).for_each(|split| {
            split.par_iter_mut().for_each(|matrix_row| {
                matrix_row.iter_mut().for_each(|x| {
                    change_value(x);
                });
            });
        });

        let elapsed = loop_clock
            .elapsed()
            .expect("Could not calculate differences")
            .as_secs_f64();

        println!("{}", elapsed);
    }
}
