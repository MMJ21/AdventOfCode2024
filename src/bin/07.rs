advent_of_code::solution!(7);

pub fn part_one(input: &str) -> Option<usize> {
    let mut input_as_vector = Vec::<(usize, Vec<usize>)>::new();
    let sum: usize;
    // Parse input in vector
    for line in input.lines() {
        let (res, terms) = line.split_once(":").unwrap();
        input_as_vector.push((
            res.parse().unwrap(),
            terms.split_whitespace().map(|t| t.parse().unwrap()).collect()
        ));
    }

    sum = input_as_vector
        .into_iter()
        .filter(|(result, terms)| is_result_achievable_with_terms(*result, terms))
        .map(|(result, _)| result)
        .sum();

    Some(sum)
}

pub fn part_two(input: &str) -> Option<usize> {
    let mut input_as_vector = Vec::<(usize, Vec<usize>)>::new();
    let sum: usize;
    // Parse input in vector
    for line in input.lines() {
        let (res, terms) = line.split_once(":").unwrap();
        input_as_vector.push((
            res.parse().unwrap(),
            terms.split_whitespace().map(|t| t.parse().unwrap()).collect()
        ));
    }

    sum = input_as_vector
        .into_iter()
        .filter(|(result, terms)| is_result_achievable_with_terms_part2 (*result, terms))
        .map(|(result, _)| result)
        .sum();

    Some(sum)
}

fn is_result_achievable_with_terms(result: usize, terms: &[usize]) -> bool {
    // Base recursive call
    if terms.len() == 1 {
        return result == terms[0];
    }
    // Take the last term, if it's a whole divisor of the result, recurse call on the division
    // Else, if the result is bigger than the term, recurse call on the substraction
    // Else, result is not achievable
    let (&last, rest) = terms.split_last().unwrap();
    if result % last == 0 && is_result_achievable_with_terms(result / last, rest) {
        return true;
    }
    if result > last && is_result_achievable_with_terms(result - last, rest) {
        return true;
    }
    false
}

fn is_result_achievable_with_terms_part2(result: usize, terms: &[usize]) -> bool {
    // First steps as part 1
    if terms.len() == 1 {
        return result == terms[0];
    }
    let (&last_term, rest) = terms.split_last().unwrap();
    if result % last_term == 0 && is_result_achievable_with_terms_part2(result / last_term, rest) {
        return true;
    }
    if result > last_term && is_result_achievable_with_terms_part2(result - last_term, rest) {
        return true;
    }

    // If we can't divide nor subtract, take the number of digits of the last term by log10ing it and get its magnitude by raising 10 to its power.
    // Take the number of digits for the result, and get those digits by moduling it to the magnitude. If the length of the result is still greater than 
    // that of the last term, and the last digits of the result are equal to the term, recurse call dividing the result over the term's magnitude.
    let term_len = last_term.ilog10() + 1;    
    let result_len = result.ilog10() + 1;
    let term_magnitude = 10_usize.pow(term_len);
    let result_last_digits = result % term_magnitude;

    if result_len > term_len && last_term == result_last_digits && is_result_achievable_with_terms_part2(result / term_magnitude, rest) {
        return true;
    }
    false
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(3749));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(11387));
    }
}
