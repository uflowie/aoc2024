use regex::Regex;
use std::ops::{Add, Mul};
use std::sync::LazyLock;

advent_of_code::solution!(7);

static EQUATION_RE: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"(\d+)").unwrap());

pub fn part_one(input: &str) -> Option<i128> {
    Some(solve(input, &[Add::add, Mul::mul]))
}

pub fn part_two(input: &str) -> Option<i128> {
    Some(solve(input, &[Add::add, Mul::mul, concat]))
}

fn solve(input: &str, operations: &[fn(i128, i128) -> i128]) -> i128 {
    input
        .lines()
        .map(|line| {
            let mut nums = EQUATION_RE
                .find_iter(line)
                .map(|m| m.as_str().parse().unwrap());

            let target = nums.next().unwrap();
            let nums: Vec<_> = nums.collect();

            if is_solvable(nums[0], target, &nums[1..], operations) {
                target
            } else {
                0
            }
        })
        .sum()
}

fn is_solvable(
    curr: i128,
    target: i128,
    remaining: &[i128],
    ops: &[fn(i128, i128) -> i128],
) -> bool {
    if remaining.is_empty() {
        curr == target
    } else {
        curr <= target
            && ops
                .iter()
                .any(|op| is_solvable(op(curr, remaining[0]), target, &remaining[1..], ops))
    }
}

fn concat(left: i128, right: i128) -> i128 {
    format!("{}{}", left, right).parse().unwrap()
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
        assert_eq!(result, None);
    }
}
