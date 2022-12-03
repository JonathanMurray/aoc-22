use std::collections::HashSet;

#[allow(dead_code)]
pub fn find_erroneous_items(input: impl Iterator<Item = impl Into<String>>) -> u32 {
    let mut sum = 0u32;
    for line in input {
        let rucksack: Vec<char> = line.into().chars().collect();
        let item = find_erroneous_item_in_rucksack(rucksack);
        let priority = priority(item);
        sum += priority;
    }
    sum
}

#[allow(dead_code)]
pub fn find_group_badges(mut input: impl Iterator<Item = impl Into<String>>) -> u32 {
    let mut sum = 0u32;
    while let Some(first) = input.next() {
        let second = input.next().expect("Reading second group member");
        let third = input.next().expect("Reading third group member");
        let first = unique_items(first.into().chars());
        let second = unique_items(second.into().chars());
        let third = unique_items(third.into().chars());
        let badge = *first
            .intersection(&second)
            .copied()
            .collect::<HashSet<char>>()
            .intersection(&third)
            .next()
            .expect("Common badge item");
        sum += priority(badge);
    }
    sum
}

fn priority(item: char) -> u32 {
    let priority = if ('a'..='z').contains(&item) {
        item as u8 - b'a' + 1
    } else if ('A'..='Z').contains(&item) {
        item as u8 - b'A' + 27
    } else {
        panic!("Invalid item")
    };
    priority as u32
}

fn find_erroneous_item_in_rucksack(mut rucksack: Vec<char>) -> char {
    let in_first_compartment = unique_items(rucksack.drain(0..rucksack.len() / 2));
    for item in rucksack {
        if in_first_compartment.contains(&item) {
            return item;
        }
    }
    panic!("No error found in rucksack");
}

fn unique_items(items: impl Iterator<Item = char>) -> HashSet<char> {
    let mut unique: HashSet<char> = Default::default();
    for item in items {
        unique.insert(item);
    }
    unique
}
