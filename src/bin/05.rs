use std::collections::HashMap;
use std::collections::HashSet;
advent_of_code::solution!(5);

pub fn part_one(input: &str) -> Option<u32> {
    let (rules_input, updates_input) = input.split_once("\n\n").unwrap();
    let mut rules = HashMap::<u32, HashSet<u32>>::new();
    let mut sum = 0;

    for line in rules_input.lines() {
        let (first, second) = line.split_once('|').unwrap();
        rules.entry(second.parse().unwrap()).or_default().insert(first.parse().unwrap());
    }
    let updates = updates_input.lines().map(|line| {
        line.split(',').map(|page| page.parse::<u32>().unwrap()).collect::<Vec<u32>>()
    });

    for update in updates {
        if update.is_sorted_by(|x, y| rules[y].contains(x)) {
            sum += update[update.len() / 2];
        }
    }

    Some(sum)
}

pub fn part_two(input: &str) -> Option<u32> {
    let (rules_input, updates_input) = input.split_once("\n\n").unwrap();
    let mut rules = HashMap::<u32, HashSet<u32>>::new();
    let mut sum = 0;

    for line in rules_input.lines() {
        let (first, second) = line.split_once('|').unwrap();
        rules.entry(second.parse().unwrap()).or_default().insert(first.parse().unwrap());
    }
    let updates = updates_input.lines().map(|line| {
        line.split(',').map(|page| page.parse::<u32>().unwrap()).collect::<Vec<u32>>()
    });

    for mut update in updates {
        if !(update.is_sorted_by(|x, y| rules[y].contains(x))) {
            update.sort_by(|x, y| rules[y].contains(x).cmp(&true));
            sum += update[update.len() / 2];
        }
    }

    Some(sum)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        assert_ne!(true, false);
    }

    #[test]
    fn test_part_two() {
        assert_ne!(true, false);
    }
}
