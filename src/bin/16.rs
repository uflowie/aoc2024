use std::cmp::Reverse;
use std::collections::{BinaryHeap, HashMap, HashSet};

use advent_of_code::{add, find_char_index, indexed_chars};

advent_of_code::solution!(16);

pub fn part_one(input: &str) -> Option<i32> {
    let (paths, end_idx) = get_best_paths(input);

    paths
        .iter()
        .filter_map(|(node, (cost, _))| {
            if node.idx == end_idx {
                Some(*cost)
            } else {
                None
            }
        })
        .next()
}

pub fn part_two(input: &str) -> Option<usize> {
    let (paths, end_idx) = get_best_paths(input);

    let unique_tiles: HashSet<_> = paths
        .iter()
        .filter(|(node, _)| node.idx == end_idx)
        .flat_map(|(_, (_, path_lists))| {
            path_lists
                .iter()
                .flat_map(|path| path.iter().map(|n| n.idx))
        })
        .collect();

    Some(unique_tiles.len())
}

fn get_best_paths(input: &str) -> (HashMap<Node, (i32, Vec<Vec<Node>>)>, (i32, i32)) {
    let tiles = indexed_chars(input);

    let start = Node {
        direction: (0, 1),
        idx: find_char_index(&tiles, 'S').unwrap(),
    };

    let end = Node {
        direction: (0, 1),
        idx: find_char_index(&tiles, 'E').unwrap(),
    };

    let mut queue = BinaryHeap::from([(Reverse(0), start.clone())]);
    let mut visited = HashSet::<Node>::new();

    let mut paths: HashMap<Node, (i32, Vec<Vec<Node>>)> = HashMap::new();
    paths.insert(start.clone(), (0, vec![vec![]]));

    while let Some((Reverse(cost), node)) = queue.pop() {
        if !visited.insert(node.clone()) {
            continue;
        }

        let next_tile = add(node.idx, node.direction);

        let candidates = [
            Some((
                cost + 1000,
                Node {
                    direction: (-node.direction.1, node.direction.0),
                    idx: node.idx,
                },
            )),
            Some((
                cost + 1000,
                Node {
                    direction: (node.direction.1, -node.direction.0),
                    idx: node.idx,
                },
            )),
            tiles.get(&next_tile).and_then(|tile| {
                if *tile == '.' || *tile == 'E' {
                    Some((
                        cost + 1,
                        Node {
                            direction: node.direction,
                            idx: next_tile,
                        },
                    ))
                } else {
                    None
                }
            }),
        ];

        let current_paths = &paths[&node].1;
        let extended_paths: Vec<Vec<Node>> = current_paths
            .iter()
            .map(|p| {
                let mut new_path = p.clone();
                new_path.push(node.clone());
                new_path
            })
            .collect();

        for (new_cost, new_node) in candidates.into_iter().flatten() {
            match paths.get(&new_node) {
                None => {
                    paths.insert(new_node.clone(), (new_cost, extended_paths.clone()));
                    queue.push((Reverse(new_cost), new_node));
                }
                Some((existing_cost, existing_paths)) => {
                    if new_cost < *existing_cost {
                        paths.insert(new_node.clone(), (new_cost, extended_paths.clone()));
                        queue.push((Reverse(new_cost), new_node));
                    } else if new_cost == *existing_cost {
                        let mut combined = existing_paths.clone();
                        combined.extend(extended_paths.clone());
                        paths.insert(new_node.clone(), (new_cost, combined));
                        queue.push((Reverse(new_cost), new_node));
                    }
                }
            }
        }
    }

    (paths, end.idx)
}

#[derive(Eq, PartialEq, Hash, PartialOrd, Ord, Clone, Debug)]
struct Node {
    direction: (i32, i32),
    idx: (i32, i32),
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
