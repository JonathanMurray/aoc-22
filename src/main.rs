use std::cmp::max;
use std::fs::File;
use std::io::{self, BufRead, BufReader};

fn main() {
    let file = File::open("input_1.txt").expect("Opening input file");
    let lines = BufReader::new(file).lines();
    let max_load = puzzle_1_find_most_carried_calories(lines);
    println!("{}", max_load);
}

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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn simple() {
        let lines = vec!["3", "5", "", "6"];
        let lines = lines.into_iter().map(|line| Ok(line));
        let result = puzzle_1_find_most_carried_calories(lines);
        assert_eq!(result, 8);
    }
}
