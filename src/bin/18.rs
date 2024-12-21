use std::{
    cmp::Reverse,
    collections::{BinaryHeap, HashMap, HashSet},
};

use advent_of_code::{add, get_in_bounds_index_neighbors, manhattan_distance, NUM_RE};
use itertools::Itertools;

advent_of_code::solution!(18);

pub fn part_one(input: &str) -> Option<i32> {
    get_distance_to_exit(input, 1024)
}

pub fn part_two(input: &str) -> Option<&str> {
    let mut left = 1024;
    let mut right = input.lines().count();

    while left < right {
        let mid = left + (right - left) / 2;
        if get_distance_to_exit(input, mid).is_some() {
            left = mid + 1;
        } else {
            right = mid;
        }
    }

    Some(input.lines().take(left).last().unwrap())
}

fn get_distance_to_exit(input: &str, num_bits: usize) -> Option<i32> {
    let corrupted_coordinates: HashSet<(i32, i32)> = NUM_RE
        .find_iter(input)
        .map(|x| x.as_str().parse().unwrap())
        .tuples()
        .take(num_bits)
        .collect();

    let bounds = (71, 71);
    let start = (0, 0);
    let target = add(bounds, (-1, -1));

    let mut open_set = BinaryHeap::from([(Reverse(0), 0, start)]);
    let mut visited = HashSet::new();

    let mut g_cost_map = HashMap::new();
    g_cost_map.insert(start, 0);

    while let Some((Reverse(_), current_g, current_pos)) = open_set.pop() {
        if visited.contains(&current_pos) {
            continue;
        }

        if current_pos == target {
            return Some(current_g);
        }

        visited.insert(current_pos);

        let neighbors = get_in_bounds_index_neighbors(current_pos, bounds)
            .filter(|n| !corrupted_coordinates.contains(n));

        for neighbor in neighbors {
            let tentative_g_cost = current_g + 1;
            if tentative_g_cost < *g_cost_map.get(&neighbor).unwrap_or(&i32::MAX) {
                g_cost_map.insert(neighbor, tentative_g_cost);

                let h_cost = manhattan_distance(neighbor, target);
                let f_cost_new = tentative_g_cost + h_cost;
                open_set.push((Reverse(f_cost_new), tentative_g_cost, neighbor));
            }
        }
    }

    None
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
