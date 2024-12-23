use std::collections::{HashMap, VecDeque};

use advent_of_code::{
    add, find_char_index, get_index_neighbors, indexed_chars, manhattan_distance,
};
use itertools::Itertools;

advent_of_code::solution!(20);

pub fn part_one(input: &str) -> Option<usize> {
    Some(solve(input, 2, 100))
}

pub fn part_two(input: &str) -> Option<usize> {
    Some(solve(input, 20, 100))
}

fn solve(input: &str, cheat_duration: i32, good_cheat_threshold: i32) -> usize {
    let tiles = indexed_chars(input);

    let distances_from_start = get_distances_from('S', &tiles);

    let distances_from_end: HashMap<(i32, i32), i32> = get_distances_from('E', &tiles);

    let end = find_char_index(&tiles, 'E').unwrap();

    let distance_without_cheats = *distances_from_start.get(&end).unwrap();

    let good_cheats = distances_from_start
        .iter()
        .map(|(cheat_start, cost)| {
            (-cheat_duration..=cheat_duration)
                .cartesian_product(-cheat_duration..=cheat_duration)
                .map(|diff| add(*cheat_start, diff))
                .filter(|cheat_end| {
                    (2..=cheat_duration).contains(&manhattan_distance(*cheat_start, *cheat_end))
                })
                .filter(|&cheat_end| match tiles.get(&cheat_end) {
                    Some('E') | Some('.') => true,
                    _ => false,
                })
                .filter(|cheat_end| {
                    let start_to_cheat_start = *cost;
                    let cheat_start_to_cheat_end = manhattan_distance(*cheat_start, *cheat_end);
                    let cheat_end_to_end = distances_from_end.get(&cheat_end).unwrap();

                    let distance_with_cheat =
                        cheat_start_to_cheat_end + start_to_cheat_start + cheat_end_to_end;

                    distance_with_cheat <= distance_without_cheats - good_cheat_threshold
                })
                .map(move |cheat_end| (cheat_start, cheat_end))
        })
        .flatten()
        .count();

    good_cheats
}

fn get_distances_from(start: char, tiles: &HashMap<(i32, i32), char>) -> HashMap<(i32, i32), i32> {
    let start_index = find_char_index(tiles, start).unwrap();

    let mut queue = VecDeque::from([start_index]);
    let mut distances = HashMap::from([(start_index, 0)]);

    while let Some(current) = queue.pop_front() {
        let current_distance = distances[&current];

        for neighbor in get_index_neighbors(current)
            .into_iter()
            .filter(|idx| matches!(tiles.get(idx), Some(c) if c != &'#'))
        {
            if !distances.contains_key(&neighbor) {
                distances.insert(neighbor, current_distance + 1);
                queue.push_back(neighbor);
            }
        }
    }

    distances
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
