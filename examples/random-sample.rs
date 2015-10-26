
extern crate reservoir;
extern crate rand;

use std::env::args;
use std::io::{stdin, BufRead};
use std::str::FromStr;

use reservoir::sample;

fn main() {
    let count = args().nth(1).and_then(|s| FromStr::from_str(s.as_ref()).ok()).unwrap_or(10);

    let input = stdin();
    let input_lines = input.lock().lines().map(|r| r.unwrap());
    
    let samples = sample(&mut rand::thread_rng(), count, input_lines);
    
    for sample in samples.iter() {
        println!("{}", sample);
    }
}

