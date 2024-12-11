use std::collections::HashMap;

advent_of_code::solution!(11);

pub fn part_one(input: &str) -> Option<i64> {
    Some(solve(input, 25))
}

pub fn part_two(input: &str) -> Option<i64> {
    Some(solve(input, 75))
}

fn solve(input: &str, blinks: usize) -> i64 {
    let mut rock_amounts = input
        .replace("\n", "")
        .split_whitespace()
        .map(|i| (i.parse().unwrap(), 1))
        .collect::<HashMap<i64, i64>>();

    let mut blink_results = HashMap::new();
    let mut updates = Vec::new();

    for _ in 0..blinks {
        updates.clear();

        for (&rock, &amount) in rock_amounts.iter().filter(|&(_, &amount)| amount > 0) {
            let results = if let Some(cached) = blink_results.get(&rock) {
                cached
            } else {
                let new_results = if rock == 0 {
                    vec![1]
                } else {
                    let rock_str = rock.to_string();
                    if rock_str.len() % 2 == 0 {
                        let mid = rock_str.len() / 2;
                        let first = rock_str[..mid].parse().unwrap();
                        let second = rock_str[mid..].parse().unwrap();
                        vec![first, second]
                    } else {
                        vec![rock * 2024]
                    }
                };
                blink_results.insert(rock, new_results);
                blink_results.get(&rock).unwrap()
            };

            for &result in results {
                updates.push((result, amount));
            }
            updates.push((rock, -amount));
        }

        for &(rock, amount) in &updates {
            *rock_amounts.entry(rock).or_insert(0) += amount;
        }
    }

    rock_amounts.values().sum()
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
