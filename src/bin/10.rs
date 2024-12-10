use std::collections::{HashMap, HashSet};

use advent_of_code::indexed_chars_iter;

advent_of_code::solution!(10);

const DIRECTIONS: [(i32, i32); 4] = [(0, 1), (1, 0), (0, -1), (-1, 0)];

pub fn part_one(input: &str) -> Option<usize> {
    Some(solve(input, |vec| vec.iter().collect::<HashSet<_>>().len()))
}

pub fn part_two(input: &str) -> Option<usize> {
    Some(solve(input, |vec| vec.len()))
}

fn solve(input: &str, counting_f: fn(&Vec<(i32, i32)>) -> usize) -> usize {
    let grid: HashMap<(i32, i32), i32> = indexed_chars_iter(input)
        .map(|(i, j, ch)| ((i, j), ch.to_digit(10).unwrap() as i32))
        .collect();

    grid.keys()
        .map(|&idx| {
            let mut acc = Vec::new();
            get_reachable(idx, 0, &grid, &mut acc);
            counting_f(&acc)
        })
        .sum()
}

fn get_reachable(
    idx: (i32, i32),
    target: i32,
    grid: &HashMap<(i32, i32), i32>,
    acc: &mut Vec<(i32, i32)>,
) {
    match grid.get(&idx) {
        Some(&height) if height != target => {}
        None => {}
        Some(&height) if height == 9 => {
            acc.push(idx);
        }
        Some(_) => {
            for &(dx, dy) in &DIRECTIONS {
                let new_idx = (idx.0 + dx, idx.1 + dy);
                get_reachable(new_idx, target + 1, grid, acc);
            }
        }
    }
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
