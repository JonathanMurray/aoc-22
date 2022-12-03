use std::collections::HashSet;

pub fn puzzle_3_sum_of_priorities(input: impl Iterator<Item = impl Into<String>>) -> u32 {
    let mut sum = 0u32;
    for line in input {
        let rucksack: Vec<char> = line.into().chars().collect();
        let item = find_erroneous_item_in_rucksack(rucksack);
        let priority = if ('a'..='z').contains(&item) {
            item as u8 - b'a' + 1
        } else if ('A'..='Z').contains(&item) {
            item as u8 - b'A' + 27
        } else {
            panic!("Invalid item")
        };
        sum += priority as u32;
    }
    sum
}

fn find_erroneous_item_in_rucksack(mut rucksack: Vec<char>) -> char {
    let mut in_first_compartment: HashSet<char> = Default::default();
    for item in rucksack.drain(0..rucksack.len() / 2) {
        in_first_compartment.insert(item);
    }
    for item in rucksack {
        if in_first_compartment.contains(&item) {
            return item;
        }
    }
    panic!("No error found in rucksack");
}
