advent_of_code::solution!(1);

pub fn part_one(input: &str) -> Option<u32> {

    let mut first_numbers = Vec::new();
    let mut second_numbers = Vec::new();

    for line in input.lines() {
        let mut numbers = line.split_whitespace(); // Split by whitespace
        if let (Some(first), Some(second)) = (numbers.next(), numbers.next()) {
            if let (Ok(first_num), Ok(second_num)) = (first.parse::<i32>(), second.parse::<i32>()) {
                first_numbers.push(first_num);
                second_numbers.push(second_num);
            } else {
                println!("Error parsing numbers in line: {}", line);
            }
        } else {
            println!("Malformed line: {}", line);
        }
    }


    Some((first_numbers.sort(), second_numbers.sort()));



    // Sort both arrays
    first_numbers.sort();
    second_numbers.sort();

    // Accumulate the differences
    let total_difference: i32 = first_numbers
        .iter()
        .zip(second_numbers.iter())
        .map(|(a, b)| (a - b).abs() as i32) // Calculate absolute difference
        .sum();


   Some(total_difference as u32)
}

pub fn part_two(input: &str) -> Option<u32> {
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, 11);
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
