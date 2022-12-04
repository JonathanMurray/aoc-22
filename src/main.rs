use std::fs::File;
use std::io::{BufRead, BufReader};

mod day_1;
mod day_2;
mod day_3;
mod day_4;

fn main() {
    let file = File::open("input_4.txt").expect("Opening input file");
    let lines = BufReader::new(file).lines().map(|line| line.unwrap());
    println!(
        "Answer: {}",
        day_4::count_pairs_where_one_fully_contains_the_other(lines)
    );
}
