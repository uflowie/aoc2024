use regex::Regex;

advent_of_code::solution!(3);

pub fn part_one(input: &str) -> Option<i32> {
    Some(solve(input))
}

pub fn part_two(input: &str) -> Option<i32> {
    let enabled_re = Regex::new(r"(?:^|do\(\)).*?(?:$|don't\(\))").unwrap();
    let enabled_parts = enabled_re
        .find_iter(&input.replace("\n", ""))
        .map(|m| m.as_str())
        .collect::<String>();

    Some(solve(&enabled_parts))
}

fn solve(input: &str) -> i32 {
    let re = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)").unwrap();

    re.captures_iter(&input.replace("\n", ""))
        .map(|c| c.extract())
        .map(|(_, [left, right])| left.parse::<i32>().unwrap() * right.parse::<i32>().unwrap())
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
