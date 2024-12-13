use std::collections::{HashMap, HashSet, VecDeque};

use advent_of_code::{add, get_index_neighbors, indexed_chars};

advent_of_code::solution!(12);

pub fn part_one(input: &str) -> Option<usize> {
    Some(solve(input, Region::price))
}

pub fn part_two(input: &str) -> Option<usize> {
    Some(solve(input, Region::discounted_price))
}

fn solve(input: &str, price_f: fn(&Region) -> usize) -> usize {
    let plants = indexed_chars(input);
    let mut visited_indices = HashSet::<(i32, i32)>::new();
    let mut regions = Vec::new();

    for &idx in plants.keys() {
        if visited_indices.contains(&idx) {
            continue;
        }
        let region = Region::new(idx, &plants);
        visited_indices.extend(&region.occupied_indices);
        regions.push(region);
    }

    regions.iter().map(price_f).sum()
}

struct Region {
    occupied_indices: HashSet<(i32, i32)>,
}

impl Region {
    fn new(start: (i32, i32), all_plants: &HashMap<(i32, i32), char>) -> Self {
        let mut occupied_indices = HashSet::from([start]);
        let mut remaining = VecDeque::from([start]);
        let plant = all_plants[&start];

        while let Some(next) = remaining.pop_back() {
            for neighbor in get_index_neighbors(next) {
                if occupied_indices.contains(&neighbor) {
                    continue;
                }
                if all_plants.get(&neighbor) == Some(&plant) {
                    occupied_indices.insert(neighbor);
                    remaining.push_back(neighbor);
                }
            }
        }

        Self { occupied_indices }
    }

    fn price(&self) -> usize {
        let area = self.occupied_indices.len();
        let circumference: usize = self
            .occupied_indices
            .iter()
            .map(|&idx| {
                get_index_neighbors(idx)
                    .iter()
                    .filter(|neighbor| !self.occupied_indices.contains(neighbor))
                    .count()
            })
            .sum();
        area * circumference
    }

    fn discounted_price(&self) -> usize {
        let area = self.occupied_indices.len();

        let adjacent_indices: HashSet<_> = self
            .occupied_indices
            .iter()
            .flat_map(|&idx| get_index_neighbors(idx))
            .filter(|idx| !self.occupied_indices.contains(idx))
            .collect();

        let convex_turns = calc_turns(&self.occupied_indices, &adjacent_indices);
        let concave_turns = calc_turns(&adjacent_indices, &self.occupied_indices);

        let double_counts = self
            .occupied_indices
            .iter()
            .map(|&idx| {
                let count = |offsets: [(i32, i32); 4]| {
                    let pattern: Vec<_> = offsets
                        .iter()
                        .map(|&off| self.occupied_indices.contains(&add(idx, off)))
                        .collect();
                    if pattern == [true, false, true, false] {
                        1
                    } else {
                        0
                    }
                };
                (count([(0, 0), (1, 0), (1, -1), (0, -1)])
                    + count([(0, 0), (-1, 0), (-1, -1), (0, -1)]))
                    * 2
            })
            .sum::<usize>();

        let sides = convex_turns + concave_turns - double_counts;

        area * sides
    }
}

fn calc_turns(base_set: &HashSet<(i32, i32)>, check_set: &HashSet<(i32, i32)>) -> usize {
    base_set
        .iter()
        .map(|&idx| {
            let neighbors: Vec<_> = get_index_neighbors(idx)
                .into_iter()
                .filter(|pos| check_set.contains(pos))
                .collect();
            match neighbors.len() {
                4 => 4,
                3 => 2,
                2 => {
                    let ((a, b), (c, d)) = (neighbors[0], neighbors[1]);
                    if a == c || b == d {
                        0
                    } else {
                        1
                    }
                }
                _ => 0,
            }
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(436));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(368));
    }
}
