use std::cmp::max;
use std::io;

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
