use advent_of_code::indexed_chars;
use rayon::iter::{IntoParallelRefIterator, ParallelIterator};
use std::collections::HashSet;

advent_of_code::solution!(6);

pub fn part_one(input: &str) -> Option<usize> {
    let visited = try_solve(input, None)?;
    Some(visited.len())
}

pub fn part_two(input: &str) -> Option<usize> {
    let visited = try_solve(input, None)?;

    let blocked_paths: Vec<_> = visited
        .par_iter()
        .filter_map(|&(x, y)| try_solve(input, Some((x, y))))
        .collect();

    Some(
        visited.len() - blocked_paths.len() - 1, /* -1 to skip starting position */
    )
}

fn try_solve(input: &str, extra_obstacle: Option<(i32, i32)>) -> Option<HashSet<(i32, i32)>> {
    let mut tiles = indexed_chars(input);
    if let Some(pos) = extra_obstacle {
        tiles.insert(pos, '#');
    }

    let mut direction = (-1, 0);
    let mut visited = HashSet::new();

    let start = tiles.iter().find(|&(_, &v)| v == '^')?.0;
    let mut current = *start;

    loop {
        let state = (current, direction);
        if visited.contains(&state) {
            return None;
        }
        visited.insert(state);

        let next = (current.0 + direction.0, current.1 + direction.1);
        match tiles.get(&next) {
            Some('.') | Some('^') => {
                current = next;
            }
            Some('#') => {
                direction = match direction {
                    (0, 1) => (1, 0),
                    (1, 0) => (0, -1),
                    (0, -1) => (-1, 0),
                    (-1, 0) => (0, 1),
                    _ => direction,
                };
            }
            _ => {
                return Some(visited.into_iter().map(|((x, y), _)| (x, y)).collect());
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::template::read_file("examples", DAY);
        assert_eq!(part_one(&input), Some(41));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::template::read_file("examples", DAY);
        assert_eq!(part_two(&input), Some(6));
    }
}
