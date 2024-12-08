use regex::Regex;
use std::{
    ops::{Add, Mul},
    sync::LazyLock,
};

advent_of_code::solution!(7);

static EQUATION_RE: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"(\d+)").unwrap());

pub fn part_one(input: &str) -> Option<i128> {
    Some(solve(input, &[Add::add, Mul::mul]))
}

pub fn part_two(input: &str) -> Option<i128> {
    let concat = |l, r| format!("{}{}", l, r).parse().unwrap();
    Some(solve(input, &[Add::add, Mul::mul, concat]))
}

fn solve(input: &str, operations: &[fn(i128, i128) -> i128]) -> i128 {
    input
        .lines()
        .map(Equation::from)
        .filter(|e| e.is_solvable(operations))
        .map(|e| e.target)
        .sum()
}

struct Equation {
    target: i128,
    nums: Box<[i128]>,
}

impl Equation {
    fn is_solvable(&self, operations: &[fn(i128, i128) -> i128]) -> bool {
        Self::try_solve(self.nums[0], self.target, &self.nums[1..], operations)
    }

    fn try_solve(
        current: i128,
        target: i128,
        remaining: &[i128],
        operations: &[fn(i128, i128) -> i128],
    ) -> bool {
        if remaining.is_empty() {
            current == target
        } else {
            current <= target
                && operations.iter().any(|op| {
                    Self::try_solve(
                        op(current, remaining[0]),
                        target,
                        &remaining[1..],
                        operations,
                    )
                })
        }
    }
}

impl From<&str> for Equation {
    fn from(value: &str) -> Self {
        let mut nums = EQUATION_RE
            .find_iter(value)
            .map(|m| m.as_str().parse().unwrap());
        Self {
            target: nums.next().unwrap(),
            nums: nums.collect(),
        }
    }
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
