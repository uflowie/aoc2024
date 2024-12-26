use std::collections::HashMap;

use itertools::Itertools;
use rayon::iter::{IntoParallelRefIterator, ParallelBridge, ParallelIterator};

advent_of_code::solution!(22);

pub fn part_one(input: &str) -> Option<i64> {
    Some(
        input
            .lines()
            .map(|line| line.parse().unwrap())
            .map(|initial| get_nth_secret_number(initial, 2000))
            .sum(),
    )
}

pub fn part_two(input: &str) -> Option<i64> {
    let diff_maps: Vec<_> = input
        .lines()
        .par_bridge()
        .map(|line| line.parse().unwrap())
        .map(|initial| get_diff_prices(initial, 2000))
        .collect();

    let result = diff_maps
        .par_iter()
        .map(|x| x.keys().par_bridge())
        .flatten()
        .map(|diff| {
            diff_maps
                .par_iter()
                .map(|diff_map| diff_map.get(diff).copied().unwrap_or_default())
                .sum()
        })
        .max();

    result
}

fn get_nth_secret_number(mut secret_number: i64, n: usize) -> i64 {
    for _ in 0..n {
        secret_number = prune(mix(secret_number, secret_number << 6));
        secret_number = prune(mix(secret_number, secret_number / 32));
        secret_number = prune(mix(secret_number, secret_number << 11));
    }

    secret_number
}

fn get_diff_prices(secret_number: i64, max: usize) -> HashMap<(i64, i64, i64, i64), i64> {
    (0..max)
        .map(|n| get_nth_secret_number(secret_number, n) % 10)
        .tuple_windows()
        .map(|(a, b)| b - a)
        .tuple_windows()
        .enumerate()
        .map(|(i, diffs)| (get_nth_secret_number(secret_number, i + 4) % 10, diffs))
        .fold(HashMap::new(), |mut acc, (price, diffs)| {
            acc.entry(diffs).or_insert(price);
            acc
        })
}

fn mix(a: i64, b: i64) -> i64 {
    a ^ b
}

fn prune(secret_number: i64) -> i64 {
    secret_number % 16777216
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
