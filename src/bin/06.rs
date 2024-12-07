use std::collections::HashSet;

use advent_of_code::indexed_chars;
use rayon::iter::{IntoParallelRefIterator, ParallelIterator};

advent_of_code::solution!(6);

pub fn part_one(input: &str) -> Option<usize> {
    Some(try_solve(input, None).unwrap().len())
}

pub fn part_two(input: &str) -> Option<usize> {
    let visited = try_solve(input, None).unwrap();

    Some(
        visited.len()
            - visited
                .par_iter()
                .filter_map(|(x, y)| try_solve(input, Some((*x, *y))))
                .collect::<Vec<_>>()
                .len(),
    )
}

fn try_solve(input: &str, extra_obstacle: Option<(i32, i32)>) -> Option<HashSet<(i32, i32)>> {
    let mut tiles = indexed_chars(input);

    if let Some(pos) = extra_obstacle {
        tiles.insert(pos, '#');
    }

    let mut visited = HashSet::<((i32, i32), (i32, i32))>::new();
    let mut dir: (i32, i32) = (-1, 0);

    let curr = tiles.iter().find(|(_, &v)| v == '^');

    if curr == None {
        // hack: ignore the case where the obstacle is put on the starting position
        return Some(HashSet::default());
    }

    let mut curr = curr.unwrap().0.to_owned();

    loop {
        let visit = (curr, dir);

        if let Some(_) = visited.get(&visit) {
            return None;
        }

        visited.insert(visit);
        let next_pos = (curr.0 + dir.0, curr.1 + dir.1);

        if let Some(ch) = tiles.get(&next_pos) {
            match ch {
                '.' | '^' => curr = next_pos,
                '#' => {
                    dir = match dir {
                        (0, 1) => (1, 0),
                        (1, 0) => (0, -1),
                        (0, -1) => (-1, 0),
                        (-1, 0) => (0, 1),
                        _ => dir,
                    }
                }
                _ => {}
            }
        } else {
            return Some(visited.iter().map(|((x, y), _)| (*x, *y)).collect());
        };
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(41));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(6));
    }
}
