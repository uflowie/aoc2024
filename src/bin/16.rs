use std::collections::{BinaryHeap, HashMap, HashSet};

use std::cmp::Reverse;

use advent_of_code::{add, find_char_index, indexed_chars};

advent_of_code::solution!(16);

pub fn part_one(input: &str) -> Option<i32> {
    let (paths, end_idx) = get_paths(input);

    let cost = paths
        .into_iter()
        .filter_map(|(node, (cost, _))| {
            if node.idx == end_idx {
                Some(cost)
            } else {
                None
            }
        })
        .next();

    cost
}

pub fn part_two(input: &str) -> Option<usize> {
    let (paths, end_idx) = get_paths(input);

    let unique_best_tiles = paths
        .into_iter()
        .filter_map(|(node, (_, paths))| {
            if node.idx == end_idx {
                Some(paths.into_iter().flatten())
            } else {
                None
            }
        })
        .flatten()
        .map(|node| node.idx)
        .collect::<HashSet<_>>()
        .len();

    Some(unique_best_tiles)
}

fn get_paths(input: &str) -> (HashMap<Node, (i32, Vec<Vec<Node>>)>, (i32, i32)) {
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

    let mut paths = HashMap::new();
    paths.insert(start.clone(), (0, vec![vec![]]));

    while let Some((cost, node)) = queue.pop() {
        let cost = cost.0;

        if visited.contains(&node) {
            continue;
        }

        let next_tile = add(node.idx, node.direction);
        let candidates = [
            Some((
                cost + 1000,
                Node {
                    direction: (-node.direction.1, node.direction.0),
                    ..node
                },
            )),
            Some((
                cost + 1000,
                Node {
                    direction: (node.direction.1, -node.direction.0),
                    ..node
                },
            )),
            if let Some(tile) = tiles.get(&next_tile) {
                if *tile == '.' || *tile == 'E' {
                    Some((
                        cost + 1,
                        Node {
                            idx: next_tile,
                            ..node
                        },
                    ))
                } else {
                    None
                }
            } else {
                None
            },
        ];

        // get new paths
        let new_paths: Vec<Vec<Node>> = paths
            .get(&node)
            .unwrap()
            .1
            .iter()
            .map(|inner_vec| {
                let mut new_inner = inner_vec.clone();
                new_inner.push(node.clone());
                new_inner
            })
            .collect();

        // println!("{:?}", new_paths);

        for (cost, node) in candidates.into_iter().flatten() {
            let new_paths = new_paths.clone();

            if let Some((last_cost, existing_paths)) = paths.get(&node) {
                if cost > *last_cost {
                    continue;
                }

                if cost < *last_cost {
                    // replace existing paths with current paths
                    paths.insert(node.clone(), (cost, new_paths));
                    queue.push((Reverse(cost), node));
                } else if cost == *last_cost {
                    // append current paths to existing paths
                    paths.insert(
                        node.clone(),
                        (
                            cost,
                            existing_paths
                                .iter()
                                .cloned()
                                .chain(new_paths.into_iter())
                                .collect(),
                        ),
                    );
                    queue.push((Reverse(cost), node));
                }
            } else {
                paths.insert(node.clone(), (cost, new_paths));
                queue.push((Reverse(cost), node));
            }
        }

        visited.insert(node);
    }

    (paths, end.idx)
}

#[derive(Eq, Hash, PartialEq, PartialOrd, Ord, Clone, Debug)]
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
