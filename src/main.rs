use std::fs::File;
use std::io::{BufRead, BufReader};

mod day_1;
mod day_2;

fn main() {
    let file = File::open("input_2.txt").expect("Opening input file");
    let lines = BufReader::new(file).lines().map(|line| line.unwrap());
    let score = day_2::puzzle_2_predict_score_from_recommended_outcome(lines);
    println!("Score: {}", score);
}
