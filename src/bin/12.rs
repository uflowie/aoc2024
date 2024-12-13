use std::collections::{HashMap, HashSet, VecDeque};

use advent_of_code::{add, get_index_neighbors, indexed_chars};

advent_of_code::solution!(12);

pub fn part_one(input: &str) -> Option<usize> {
    let plants = indexed_chars(input);
    let mut visited_indices: HashSet<(i32, i32)> = HashSet::new();
    let mut regions = vec![];

    for &idx in plants.keys() {
        if visited_indices.contains(&idx) {
            continue;
        }
        let region = Region::new(idx, &plants);
        visited_indices.extend(&region.occupied_indices);
        regions.push(region);
    }

    Some(regions.iter().map(Region::price).sum())
}

pub fn part_two(input: &str) -> Option<usize> {
    let plants = indexed_chars(input);
    let mut visited_indices: HashSet<(i32, i32)> = HashSet::new();
    let mut regions = vec![];

    for &idx in plants.keys() {
        if visited_indices.contains(&idx) {
            continue;
        }
        let region = Region::new(idx, &plants);
        visited_indices.extend(&region.occupied_indices);
        regions.push(region);
    }

    Some(regions.iter().map(Region::discounted_price).sum())
}

struct Region {
    plant: char,
    occupied_indices: HashSet<(i32, i32)>,
}

impl Region {
    fn new(start: (i32, i32), all_plants: &HashMap<(i32, i32), char>) -> Region {
        let mut occupied_indices = HashSet::from([start]);
        let mut remaining = VecDeque::from([start]);
        let plant = *all_plants.get(&start).unwrap();

        while let Some(next) = remaining.pop_back() {
            for neighbor in get_index_neighbors(next) {
                if let Some(_) = occupied_indices.get(&neighbor) {
                    continue;
                }

                match all_plants.get(&neighbor) {
                    Some(&ch) if ch == plant => {
                        occupied_indices.insert(neighbor);
                        remaining.push_back(neighbor);
                    }
                    _ => continue,
                }
            }
        }

        Self {
            plant,
            occupied_indices,
        }
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

        let adjacent_indices: HashSet<(i32, i32)> = self
            .occupied_indices
            .iter()
            .flat_map(|&idx| get_index_neighbors(idx))
            .filter(|idx| !self.occupied_indices.contains(idx))
            .collect();

        let convex_turns = self
            .occupied_indices
            .iter()
            .map(|&idx| {
                let adjacent_neighbors: Vec<_> = get_index_neighbors(idx)
                    .into_iter()
                    .filter(|&idx| adjacent_indices.contains(&idx))
                    .collect();
                if adjacent_neighbors.len() == 4 {
                    4
                } else if adjacent_neighbors.len() == 3 {
                    2
                } else if adjacent_neighbors.len() == 2 {
                    let ((a, b), (c, d)) = (adjacent_neighbors[0], adjacent_neighbors[1]);
                    if a == c || b == d {
                        0
                    } else {
                        1
                    }
                } else {
                    0
                }
            })
            .sum::<usize>();

        let concave_turns = adjacent_indices
            .iter()
            .map(|&idx| {
                let adjacent_neighbors: Vec<_> = get_index_neighbors(idx)
                    .into_iter()
                    .filter(|&idx| self.occupied_indices.contains(&idx))
                    .collect();
                if adjacent_neighbors.len() == 4 {
                    4
                } else if adjacent_neighbors.len() == 3 {
                    2
                } else if adjacent_neighbors.len() == 2 {
                    let ((a, b), (c, d)) = (adjacent_neighbors[0], adjacent_neighbors[1]);
                    if a == c || b == d {
                        0
                    } else {
                        1
                    }
                } else {
                    0
                }
            })
            .sum::<usize>();

        // get double counts:

        let double_counts = self
            .occupied_indices
            .iter()
            .map(|idx| {
                let mut counts = 0;

                let bottom_right_neighbors = [
                    *idx,
                    add(*idx, (1, 0)),
                    add(*idx, (1, -1)),
                    add(*idx, (0, -1)),
                ]
                .map(|idx| self.occupied_indices.contains(&idx));

                if bottom_right_neighbors == [true, false, true, false] {
                    counts += 1;
                }

                let bottom_right_neighbors = [
                    *idx,
                    add(*idx, (-1, 0)),
                    add(*idx, (-1, -1)),
                    add(*idx, (0, -1)),
                ]
                .map(|idx| self.occupied_indices.contains(&idx));

                if bottom_right_neighbors == [true, false, true, false] {
                    counts += 1;
                }

                counts
            })
            .sum::<usize>()
            * 2;

        let sides = convex_turns + concave_turns - double_counts;

        println!(
            "plant: {}, convex turns: {}, concave turns: {}, price: {}, double counts: {}",
            self.plant,
            convex_turns,
            concave_turns,
            area * (sides),
            double_counts
        );

        area * (sides)
    }
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
