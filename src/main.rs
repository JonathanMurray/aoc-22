use std::cmp::max;
use std::fs::File;
use std::io::{self, BufRead, BufReader};

fn main() {
    let file = File::open("input_2.txt").expect("Opening input file");
    let lines = BufReader::new(file).lines().map(|line| line.unwrap());
    let score = puzzle_2_predict_score(lines);
    println!("Score: {}", score);
}

fn puzzle_2_predict_score(input: impl Iterator<Item = impl Into<String>>) -> u32 {
    let mut total_score = 0;
    for line in input {
        let line = line.into();
        let chars = line.chars().into_iter().collect::<Vec<char>>();
        if let [opponent, ' ', you] = chars[..] {
            println!("{} -> {}", opponent, you);
            let opponent = match opponent {
                'A' => Shape::Rock,
                'B' => Shape::Papers,
                'C' => Shape::Scissors,
                _ => panic!("Invalid opponent move"),
            };
            let you = match you {
                'X' => Shape::Rock,
                'Y' => Shape::Papers,
                'Z' => Shape::Scissors,
                _ => panic!("Invalid response"),
            };
            total_score += you.base_score() + you.compete_against(&opponent);
        } else {
            panic!("Malformed line: {}", line);
        }
    }
    total_score
}

enum Shape {
    Rock,
    Papers,
    Scissors,
}

impl Shape {
    fn base_score(&self) -> u32 {
        match self {
            Self::Rock => 1,
            Self::Papers => 2,
            Self::Scissors => 3,
        }
    }

    fn compete_against(&self, opponent: &Self) -> u32 {
        let lost = 0;
        let draw = 3;
        let won = 6;
        match (self, opponent) {
            (Self::Rock, Self::Papers) => lost,
            (Self::Rock, Self::Scissors) => won,
            (Self::Papers, Self::Rock) => won,
            (Self::Papers, Self::Scissors) => lost,
            (Self::Scissors, Self::Rock) => lost,
            (Self::Scissors, Self::Papers) => won,
            _ => draw,
        }
    }
}

#[allow(dead_code)]
fn puzzle_1_find_most_carried_calories(
    input: impl Iterator<Item = io::Result<impl Into<String>>>,
) -> u32 {
    let mut max_elf_load = 0;
    let mut current_elf_load = 0;
    for line in input {
        let line = line.expect("Reading line").into();
        if line.is_empty() {
            max_elf_load = max(max_elf_load, current_elf_load);
            current_elf_load = 0;
        } else {
            let load: u32 = line.parse().expect("Invalid calory amount on line");
            current_elf_load += load;
        }
    }
    max_elf_load
}

#[allow(dead_code)]
fn puzzle_1_find_sum_of_top_three(
    input: impl Iterator<Item = io::Result<impl Into<String>>>,
) -> u32 {
    let mut top_three = [0, 0, 0];
    let mut current_elf_load = 0;
    for line in input {
        let line = line.expect("Reading line").into();
        if line.is_empty() {
            let (third_index, &third_load) = top_three
                .iter()
                .enumerate()
                .min_by(|(_, a), (_, b)| a.cmp(b))
                .unwrap();
            if current_elf_load > third_load {
                top_three[third_index] = current_elf_load;
            }
            current_elf_load = 0;
        } else {
            let load: u32 = line.parse().expect("Invalid calory amount on line");
            current_elf_load += load;
        }
    }
    top_three.iter().sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn most_carried() {
        let lines = vec!["3", "5", "", "6"];
        let lines = lines.into_iter().map(|line| Ok(line));
        let result = puzzle_1_find_most_carried_calories(lines);
        assert_eq!(result, 8);
    }

    #[test]
    fn sum_of_top_three() {
        let lines = vec!["3", "5", "", "6", "", "10", "10", "", "15", ""];
        let lines = lines.into_iter().map(|line| Ok(line));
        let result = puzzle_1_find_sum_of_top_three(lines);
        assert_eq!(result, 43);
    }
}
