use std::fs::File;
use std::io::{BufRead, BufReader};

mod day_1;
mod day_2;
mod day_3;

fn main() {
    let file = File::open("input_3.txt").expect("Opening input file");
    let lines = BufReader::new(file).lines().map(|line| line.unwrap());
    println!("Answer: {}", day_3::puzzle_3_sum_of_priorities(lines));
}
