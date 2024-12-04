use std::sync::LazyLock;

use advent_of_code::{columns, major_diagonals, minor_diagonals, rows};
use fancy_regex::Regex;
use itertools::iproduct;

advent_of_code::solution!(4);

static XMAS_RE: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"(?=(XMAS|SAMX))").unwrap());

pub fn part_one(input: &str) -> Option<usize> {
    let chars: Vec<Vec<_>> = input.lines().map(|line| line.chars().collect()).collect();

    Some(
        rows(&chars)
            .map(|c| c.collect::<String>())
            .chain(columns(&chars).map(|c| c.collect::<String>()))
            .chain(major_diagonals(&chars).map(|c| c.collect::<String>()))
            .chain(minor_diagonals(&chars).map(|c| c.collect::<String>()))
            .map(|s| XMAS_RE.find_iter(&s).collect::<Vec<_>>().len())
            .sum(),
    )
}

pub fn part_two(input: &str) -> Option<usize> {
    let chars: Vec<Vec<_>> = input.lines().map(|line| line.chars().collect()).collect();

    Some(
        iproduct!(1..chars.len() - 1, 1..chars[0].len() - 1)
            .filter(|(i, j)| {
                let i = *i;
                let j = *j;

                [
                    [chars[i - 1][j - 1], chars[i][j], chars[i + 1][j + 1]],
                    [chars[i - 1][j + 1], chars[i][j], chars[i + 1][j - 1]],
                ]
                .iter()
                .all(|d| *d == ['M', 'A', 'S'] || *d == ['S', 'A', 'M'])
            })
            .collect::<Vec<_>>()
            .len(),
    )
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
