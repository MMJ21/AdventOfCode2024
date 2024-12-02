use std::collections::HashMap;
advent_of_code::solution!(1);

pub fn part_one(input: &str) -> Option<u32> {
    let length = input.lines().count();
    let mut left_vector = Vec::with_capacity(length);
    let mut right_vector = Vec::with_capacity(length);

    for line in input.lines() {
        let values = line.split_whitespace().map(|x| x.parse::<u32>().unwrap()).collect::<Vec<u32>>();

        left_vector.push(values[0]);
        right_vector.push(values[1]);
    }

    left_vector.sort();
    right_vector.sort();

    Some(
        left_vector.iter()
            .zip(right_vector.iter())
            .map(|(left, right)| if left > right { left - right } else { right - left })
            .sum::<u32>(),
    )
}

pub fn part_two(input: &str) -> Option<u32> {
    let length = input.lines().count();
    let mut left_vector = Vec::with_capacity(length);
    let mut right_map = HashMap::with_capacity(length);

    for line in input.lines() {
        let values = line.split_whitespace().map(|x| x.parse::<u32>().unwrap()).collect::<Vec<u32>>();
        let right_value = values[1];

        left_vector.push(values[0]);
        match right_map.get(&right_value) {
            Some(value) => right_map.insert(right_value, *value + 1),
            
            None => right_map.insert(right_value, 1),
        };
    }

    Some(
        left_vector.iter()
            .map(|left| match right_map.get(&left) {
                Some (value) => *value * left,
                None => 0,
            })
            .sum::<u32>(),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(11));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(31));
    }
}
