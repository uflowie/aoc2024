use std::collections::{HashMap, HashSet, VecDeque};

use itertools::Itertools;

advent_of_code::solution!(23);

pub fn part_one(input: &str) -> Option<usize> {
    let graph = make_graph(input);

    let cliques = find_cliques(&graph, 3);

    Some(
        cliques
            .into_iter()
            .filter(|c| c.iter().any(|n| n.starts_with("t")))
            .count(),
    )
}

pub fn part_two(input: &str) -> Option<String> {
    let graph = make_graph(input);

    let clique = find_cliques(&graph, 13).into_iter().next().unwrap();

    Some(clique.join(", "))
}

fn make_graph(input: &str) -> HashMap<&str, Vec<&str>> {
    let mut graph: HashMap<&str, Vec<&str>> = HashMap::new();

    for line in input.lines() {
        let parts: Vec<_> = line.split("-").collect();

        graph
            .entry(parts[0])
            .and_modify(|e| e.push(parts[1]))
            .or_insert(vec![parts[1]]);
        graph
            .entry(parts[1])
            .and_modify(|e| e.push(parts[0]))
            .or_insert(vec![parts[0]]);
    }

    graph
}

fn find_cliques<'a>(graph: &'a HashMap<&'a str, Vec<&'a str>>, n: usize) -> HashSet<Vec<&'a str>> {
    let mut cliques = HashSet::new();

    for &node in graph.keys() {
        let node_neighbors = &graph[node];
        let filtered_neighbors: Vec<&str> = node_neighbors
            .iter()
            .filter(|&&neighbor| neighbor > node)
            .copied()
            .collect();

        if n == 1 {
            cliques.insert(vec![node]);
            continue;
        }

        if filtered_neighbors.len() < n - 1 {
            continue;
        }

        for combination in filtered_neighbors.iter().combinations(n - 1) {
            let mut potential_clique = vec![node];
            potential_clique.extend(combination.iter().copied());

            if all_connected(&potential_clique, graph) {
                let mut sorted_clique = potential_clique.clone();
                sorted_clique.sort();
                cliques.insert(sorted_clique);
            }
        }
    }

    cliques
}

fn all_connected(nodes: &Vec<&str>, graph: &HashMap<&str, Vec<&str>>) -> bool {
    for &node in nodes {
        match graph.get(node) {
            Some(neighbors) => {
                for &other_node in nodes {
                    if node != other_node {
                        if !neighbors.contains(&other_node) {
                            return false;
                        }
                    }
                }
            }
            None => {
                return false;
            }
        }
    }
    true
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
