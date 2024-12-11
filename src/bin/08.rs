use std::collections::{HashMap, HashSet};

use advent_of_code::{bounds, indexed_chars};
use itertools::Itertools;

advent_of_code::solution!(8);

pub fn part_one(input: &str) -> Option<usize> {
    Some(solve(input, get_antinodes))
}

pub fn part_two(input: &str) -> Option<usize> {
    Some(solve(input, get_antinodes_2))
}

fn solve(input: &str, antinode_producer: fn(&[(i32, i32)], i32, i32) -> Vec<(i32, i32)>) -> usize {
    let (max_x, max_y) = bounds(input);

    let mut frequency_groups = HashMap::new();

    let indexed_chars = indexed_chars(input);

    for (k, v) in indexed_chars.iter().filter(|(_, ch)| ch != &&'.') {
        frequency_groups
            .entry(v)
            .or_insert_with(Vec::new)
            .push(k.to_owned());
    }

    frequency_groups
        .values()
        .map(|g| antinode_producer(g, max_x, max_y))
        .flatten()
        .collect::<HashSet<_>>()
        .len()
}

fn comparisons(antennas: &[(i32, i32)]) -> impl Iterator<Item = (&(i32, i32), &(i32, i32))> {
    antennas
        .iter()
        .cartesian_product(antennas.iter())
        .filter(|(x, y)| x != y)
}

fn get_antinodes(antennas: &[(i32, i32)], max_x: i32, max_y: i32) -> Vec<(i32, i32)> {
    comparisons(antennas)
        .filter_map(|(&x, &y)| {
            let direction = (x.0 - y.0, x.1 - y.1);
            let node = (x.0 + direction.0, x.1 + direction.1);
            if node.0 >= 0 && node.0 < max_x && node.1 >= 0 && node.1 < max_y {
                Some(node)
            } else {
                None
            }
        })
        .collect()
}

fn get_antinodes_2(antennas: &[(i32, i32)], max_x: i32, max_y: i32) -> Vec<(i32, i32)> {
    comparisons(antennas)
        .map(|(&x, &y)| {
            let mut nodes = Vec::new();
            let direction = (x.0 - y.0, x.1 - y.1);
            let mut node = x;

            while node.0 >= 0 && node.0 < max_x && node.1 >= 0 && node.1 < max_y {
                nodes.push(node);
                node = (node.0 + direction.0, node.1 + direction.1);
            }

            nodes
        })
        .flatten()
        .collect()
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
