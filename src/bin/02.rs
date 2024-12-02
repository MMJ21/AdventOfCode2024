advent_of_code::solution!(2);

pub fn part_one(input: &str) -> Option<u32> {
    Some(
        input.lines().into_iter().map(|line| {
            let values: Vec<i32> = line.split_whitespace().map(|x| {
                x.parse::<i32>().unwrap()
            }).collect();
            is_safe(&values)
        })
        .sum::<u32>()
    )
}

pub fn part_two(input: &str) -> Option<u32> {
    Some(
        input.lines().into_iter().map(|line| {
            let values: Vec<i32> = line.split_whitespace().map(|x| {
                x.parse::<i32>().unwrap()
            }).collect();
            is_safe2(&values)
        })
        .sum::<u32>()
    )   
}

fn is_safe(values: &[i32]) -> u32 {
    let asc = values[0] < values[1];
    let mut prev = values[0];
    
    for &n in &values[1..] {
        let diff = (prev - n).abs();

        if diff > 3 || (asc && prev >= n) || (!asc && prev <= n)
        {
            return 0;
        }

        prev = n;
    }
    1
}

fn is_safe2(values: &[i32]) -> u32 {
    if is_safe(values) == 1 {
        return 1;
    }
    for elem in 0..values.len() {
        let skip_elem_subset: Vec<i32> = values.iter().enumerate().filter(|(i, _)| *i != elem).map(|(_, &j)| j).collect();
        if is_safe(&skip_elem_subset) == 1 {
            return 1;
        }
    }
    0
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(4));
    }
}
