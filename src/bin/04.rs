advent_of_code::solution!(4);

const XMAS: &str = "XMAS";

pub fn part_one(input: &str) -> Option<u32> {
    let input_as_line_vector = input.lines().collect::<Vec<&str>>();
    let mut matrix = vec![vec![' '; input_as_line_vector[0].len()]; input_as_line_vector.len()];
    let mut sum = 0;
    let mut found_xmas : bool;

    // Construct matrix from input
    for (i, line) in input_as_line_vector.iter().enumerate() {
        for (j, char) in line.chars().enumerate() {
            matrix[i][j] = char;
        }
    }

    // Run input matrix
    for i in 0..matrix.len() {
        for j in 0..matrix.len() {
            if matrix[i][j] == 'X' {
                // Horizontal
                found_xmas = true;
                for k in 1..4 {
                    if j + k >= matrix[0].len() {
                        found_xmas = false;

                        break;
                    }
                    if !(matrix[i][j + k] == XMAS.chars().nth(k).unwrap()) {
                        found_xmas = false;

                        break;
                    }
                }                
                if found_xmas {
                    sum += 1;
                }

                // Reverse Horizontal
                found_xmas = true;
                for k in 1..4 {
                    if (j as i32) - (k as i32) < 0 {
                        found_xmas = false;

                        break;
                    }
                    if !(matrix[i][j - k] == XMAS.chars().nth(k).unwrap())
                    {
                        found_xmas = false;

                        break;
                    }
                }
                if found_xmas {
                    sum += 1;
                }

                // Vertical
                found_xmas = true;
                for k in 1..4 {
                    if i + k >= matrix.len() {
                        found_xmas = false;

                        break;
                    }
                    if !(matrix[i + k][j] == XMAS.chars().nth(k).unwrap())
                    {
                        found_xmas = false;

                        break;
                    }
                }
                if found_xmas {
                    sum += 1;
                }

                // Reverse Vertical
                found_xmas = true;
                for k in 1..4 {
                    if (i as i32) - (k as i32) < 0 {
                        found_xmas = false;

                        break;
                    }
                    if !(matrix[i - k][j] == XMAS.chars().nth(k).unwrap())
                    {
                        found_xmas = false;

                        break;
                    }
                }
                if found_xmas {
                    sum += 1;
                }

                // Down-right Diagonal
                found_xmas = true;
                for k in 1..4 {
                    if i + k >= matrix.len() || j + k >= matrix[0].len() {
                        found_xmas = false;

                        break;
                    }
                    if !(matrix[i + k][j + k] == XMAS.chars().nth(k).unwrap())
                    {
                        found_xmas = false;

                        break;
                    }
                }
                if found_xmas {
                    sum += 1
                }

                // Down-left Diagonal
                found_xmas = true;
                for k in 1..4 {
                    if i + k >= matrix.len() || (j as i32) - (k as i32) < 0 {
                        found_xmas = false;

                        break;
                    }
                    if !(matrix[i + k][j - k] == XMAS.chars().nth(k).unwrap())
                    {
                        found_xmas = false;

                        break;
                    }
                }
                if found_xmas {
                    sum += 1
                }

                // Up-right Diagonal
                found_xmas = true;
                for k in 1..4 {
                    if (i as i32) - (k as i32) < 0 || j + k >= matrix[0].len() {
                        found_xmas = false;

                        break;
                    }
                    if !(matrix[i - k][j + k] == XMAS.chars().nth(k).unwrap())
                    {
                        found_xmas = false;

                        break;
                    }
                }
                if found_xmas {
                    sum += 1
                }

                // Up-left Diagonal
                found_xmas = true;
                for k in 1..4 {
                    if (i as i32) - (k as i32) < 0 || (j as i32) - (k as i32) < 0 {
                        found_xmas = false;

                        break;
                    }
                    if !(matrix[i - k][j - k] == XMAS.chars().nth(k).unwrap())
                    {
                        found_xmas = false;

                        break;
                    }
                }
                if found_xmas {
                    sum += 1
                }                
            }
        }
    }
    
    Some(sum)
}

pub fn part_two(input: &str) -> Option<u32> {
    let input_as_line_vector = input.lines().collect::<Vec<&str>>();
    let mut matrix = vec![vec![' '; input_as_line_vector[0].len()]; input_as_line_vector.len()];
    let mut sum = 0;

    // Construct matrix from input
    for (i, line) in input_as_line_vector.iter().enumerate() {
        for (j, char) in line.chars().enumerate() {
            matrix[i][j] = char;
        }
    }

    //                                      M.M
    // Since we want to find something like .A., trying to find center character, so bounds can be ignored
    //                                      S.S
    for i in 1..matrix.len() - 1 {
        for j in 1..matrix[0].len() - 1 {
            // Find certer character A to start
            if matrix[i][j] == 'A' {
                // M.M
                // .A.
                // S.S
                if matrix[i - 1][j - 1] == 'M' && matrix[i - 1][j + 1] == 'M' && matrix[i + 1][j - 1] == 'S' && matrix[i + 1][j + 1] == 'S' {
                    sum += 1;
                }
                // M.S
                // .A.
                // M.S
                else if matrix[i - 1][j - 1] == 'M' && matrix[i - 1][j + 1] == 'S' && matrix[i + 1][j - 1] == 'M' && matrix[i + 1][j + 1] == 'S' {
                    sum += 1;
                }
                // S.M
                // .A.
                // S.M
                else if matrix[i - 1][j - 1] == 'S' && matrix[i - 1][j + 1] == 'M' && matrix[i + 1][j - 1] == 'S' && matrix[i + 1][j + 1] == 'M' {
                    sum += 1;
                }
                // S.S
                // .A.
                // M.M
                else if matrix[i - 1][j - 1] == 'S' && matrix[i - 1][j + 1] == 'S' && matrix[i + 1][j - 1] == 'M' && matrix[i + 1][j + 1] == 'M' {
                    sum += 1;
                }
            }
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
        assert_eq!(result, Some(18));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(9));
    }
}
