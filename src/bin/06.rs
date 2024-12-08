use std::collections::HashSet;
advent_of_code::solution!(6);

#[derive(PartialEq, Eq, Hash, Clone, Copy)]
enum Direction {
    Up,
    Down, 
    Left,
    Right
}

#[derive(Clone, Copy)]
struct GuardPos {
    x: usize,
    y: usize,
    dir: Direction,
}

pub fn part_one(input: &str) -> Option<u32> {
    let input_as_line_vector = input.lines().collect::<Vec<&str>>();
    let mut matrix = vec![vec![' '; input_as_line_vector[0].len()]; input_as_line_vector.len()];
    let mut patrolled_positions = Vec::<(usize, usize)>::new();
    let mut on_duty = true;
    let mut current_pos = GuardPos {
        x: 0,
        y: 0,
        dir: Direction::Up,
    };

    // Construct matrix from input
    for (i, line) in input_as_line_vector.iter().enumerate() {
        for (j, char) in line.chars().enumerate() {
            // Check to find guard starting position
            if char == '^' {
                current_pos.x = j;
                current_pos.y = i;
            }
            matrix[i][j] = char;
        }
    }

    while on_duty {
        if current_pos.dir == Direction::Up {
            if current_pos.y as i32 - 1 < 0 {
                on_duty = false;
            }
            else if matrix[current_pos.y - 1][current_pos.x] == '#' {
                current_pos.dir = Direction::Right;
            }
            else {
                current_pos.y -= 1;
                if !patrolled_positions.contains(&(current_pos.x, current_pos.y)) {
                    patrolled_positions.push((current_pos.x, current_pos.y));
                }
            }
        }
        if current_pos.dir == Direction::Down {
            if current_pos.y + 1 > matrix.len() - 1 {
                on_duty = false;
            }
            else if matrix[current_pos.y + 1][current_pos.x] == '#' {
                current_pos.dir = Direction::Left;
            }
            else {
                current_pos.y += 1;
                if !patrolled_positions.contains(&(current_pos.x, current_pos.y)) {
                    patrolled_positions.push((current_pos.x, current_pos.y));
                }
            }
        }
        if current_pos.dir == Direction::Left {
            if current_pos.x as i32 - 1 < 0 {
                on_duty = false;
            }
            else if matrix[current_pos.y][current_pos.x - 1] == '#' {
                current_pos.dir = Direction::Up;
            }
            else {
                current_pos.x -= 1;
                if !patrolled_positions.contains(&(current_pos.x, current_pos.y)) {
                    patrolled_positions.push((current_pos.x, current_pos.y));
                }
            }
        }
        if current_pos.dir == Direction::Right {
            if current_pos.x + 1 > matrix.len() - 1 {
                on_duty = false;
            }
            else if matrix[current_pos.y][current_pos.x + 1] == '#' {
                current_pos.dir = Direction::Down;
            }
            else {
                current_pos.x += 1;
                if !patrolled_positions.contains(&(current_pos.x, current_pos.y)) {
                    patrolled_positions.push((current_pos.x, current_pos.y));
                }
            }
        }
    }

    Some(patrolled_positions.len() as u32)
}

pub fn part_two(input: &str) -> Option<u32> {
    let input_as_line_vector = input.lines().collect::<Vec<&str>>();
    let mut matrix = vec![vec![' '; input_as_line_vector[0].len()]; input_as_line_vector.len()];
    let mut sum = 0;
    let mut start_pos = GuardPos {
        x: 0,
        y: 0,
        dir: Direction::Up,
    };

    // Construct matrix from input
    for (i, line) in input_as_line_vector.iter().enumerate() {
        for (j, char) in line.chars().enumerate() {
            // Check to find guard starting position
            if char == '^' {
                start_pos.x = j;
                start_pos.y = i;
            }
            matrix[i][j] = char;
        }
    }

    for i in 0..matrix.len() {
        for j in 0..matrix[i].len() {
            // No need to check in start position or in obstacles
            if (j, i) == (start_pos.x, start_pos.y) || matrix[i][j] == '#' {
                continue;
            }

            let mut test_matrix = matrix.clone();
            test_matrix[i][j] = '#';
            let mut current_pos = start_pos.clone();
            let mut visited_cells = HashSet::new();
            visited_cells.insert((current_pos.x, current_pos.y, current_pos.dir));

            // If we exit the matrix, it's not a loop. Else, if we visit the same cell but not facing the same direction it might not be a loop.
            loop {
                if check_if_exit(&test_matrix, &mut current_pos).is_none() {
                    break;
                }

                if visited_cells.contains(&(current_pos.x, current_pos.y, current_pos.dir)) {
                    sum += 1;
                    break;
                }

                visited_cells.insert((current_pos.x, current_pos.y, current_pos.dir));
            }
        }
    }
    Some(sum)
}

fn check_if_exit(test_matrix: &Vec<Vec<char>>, guard_position: &mut GuardPos) -> Option<(usize, usize)> {
    match guard_position.dir {
        Direction::Up => {
            if guard_position.y as i32 - 1 < 0 {
                return None;
            }
            else if test_matrix[guard_position.y - 1][guard_position.x] == '#' {
                guard_position.dir = Direction::Right;
            }
            else {
                guard_position.y -= 1;
            }
        }
        Direction::Down => {
            if guard_position.y + 1 > test_matrix.len() - 1 {
                return None;
            }
            else if test_matrix[guard_position.y + 1][guard_position.x] == '#' {
                guard_position.dir = Direction::Left;
            }
            else {
                guard_position.y += 1;
            }
        }
        Direction::Left => {
            if guard_position.x as i32 - 1 < 0 {
                return None;
            }
            else if test_matrix[guard_position.y][guard_position.x - 1] == '#' {
                guard_position.dir = Direction::Up;
            }
            else {
                guard_position.x -= 1;
            }
        }
        Direction::Right => {
            if guard_position.x + 1 > test_matrix.len() - 1 {
                return None;
            }
            else if test_matrix[guard_position.y][guard_position.x + 1] == '#' {
                guard_position.dir = Direction::Down;
            }
            else {
                guard_position.x += 1;
            }
        }
    }
    Some((guard_position.x, guard_position.y))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(41));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(6));
    }
}
