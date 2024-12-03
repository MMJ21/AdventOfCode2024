use regex::Regex;
advent_of_code::solution!(3);

pub fn part_one(input: &str) -> Option<u32> {
    let ptrn = Regex::new(r"mul\((\d+),(\d+)\)").unwrap();
    let mut sum = 0;

    for (_, [first, second]) in ptrn.captures_iter(&input).map(|elem| elem.extract()) {
        sum += first.parse::<u32>().unwrap() * second.parse::<u32>().unwrap();
    }

    Some(sum)
}

pub fn part_two(input: &str) -> Option<u32> {
    let ptrn = Regex::new(r"(?:mul\((?<first>\d+),(?<second>\d+)\)|(?<active>do\(\))|(?<inactive>don't\(\)))").unwrap();
    let mut sum = 0;
    let mut active = true;

    for capture in ptrn.captures_iter(&input) {
        if capture.name("active").is_some() {
            active = true;
        }
        else if capture.name("inactive").is_some() {
            active = false;
        }
        else if active {
            sum += &capture["first"].parse::<u32>().unwrap() * &capture["second"].parse::<u32>().unwrap();
        }
    }

    Some(sum)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(161));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(48));
    }
}
