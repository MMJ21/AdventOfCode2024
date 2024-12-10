use std::collections::{HashMap, HashSet};
advent_of_code::solution!(8);

pub fn part_one(input: &str) -> Option<u32> {
    // Define map
    let mut antennas: HashMap<char, Vec<(i32, i32)>> = HashMap::new(); 
    let mut antinodes: HashSet<_> = HashSet::new();
    let height: i32 = input.lines().count().try_into().unwrap();
    let width: i32 = input.lines().next().unwrap().chars().count().try_into().unwrap();
    
    for (i, line) in input.lines().enumerate() {
        for (j, char) in line.chars().enumerate() {
            if char.is_alphanumeric() {
                antennas.entry(char).or_default().push((i as i32, j as i32));
            }
        }
    }

    for antenna_frequency in antennas.into_values() {
        for (i, (mut y0, mut x0)) in antenna_frequency.iter().enumerate() {
            for (mut y1, mut x1) in antenna_frequency.iter().skip(i + 1) {
                // Take inverse as well
                for _ in 0..2 {
                    let antinode_y = (2 * y0 - y1) as i32;
                    let antinode_x = (2 * x0 - x1) as i32;
                    if 0 <= antinode_y && antinode_y < height && 0 <= antinode_x && antinode_x < width {
                        antinodes.insert((antinode_y, antinode_x));
                    }

                    (y0, y1, x0, x1) = (y1, y0, x1, x0);
                }
            }
        }
    }

    Some(antinodes.len() as u32)
}

pub fn part_two(input: &str) -> Option<u32> {
    // Define map
    let mut antennas: HashMap<char, Vec<(i32, i32)>> = HashMap::new(); 
    let mut antinodes: HashSet<_> = HashSet::new();
    let height: i32 = input.lines().count().try_into().unwrap();
    let width: i32 = input.lines().next().unwrap().chars().count().try_into().unwrap();
    
    for (i, line) in input.lines().enumerate() {
        for (j, char) in line.chars().enumerate() {
            if char.is_alphanumeric() {
                antennas.entry(char).or_default().push((i as i32, j as i32));
            }
        }
    }

    for antenna_frequency in antennas.into_values() {
        for (i, (y0, x0)) in antenna_frequency.iter().enumerate() {
            for (y1, x1) in antenna_frequency.iter().skip(i + 1) {
                let mut distance_y = y1 - y0;
                let mut distance_x = x1 - x0;
                let dist = gcd(distance_y.abs(), distance_x.abs());
                distance_y /= dist;
                distance_x /= dist;

                // Take inverse as well
                for _ in 0..2 {
                    let mut y = *y0;
                    let mut x = *x0;

                    while 0 <= y && y < height && 0 <= x && x < width {
                        antinodes.insert((y, x));
                        y += distance_y;
                        x += distance_x;
                    }

                    distance_y *= -1;
                    distance_x *= -1;
                }
            }
        }
    }
    Some(antinodes.len() as u32)
}

// Function for greatest common divisor
fn gcd(mut y: i32, mut x: i32) -> i32 {
    while x > 0 {
        (y, x) = (x, y % x);
    }
    y
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(14));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(34));
    }
}
