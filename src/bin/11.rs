use std::collections::HashMap;

advent_of_code::solution!(11);

pub fn part_one(input: &str) -> Option<i64> {
    Some(solve(input, 25))
}

pub fn part_two(input: &str) -> Option<i64> {
    Some(solve(input, 75))
}

enum BlinkResult {
    Single(i64),
    Double(i64, i64),
}

fn solve(input: &str, blinks: usize) -> i64 {
    let mut stone_amounts = input
        .replace("\n", "")
        .split_whitespace()
        .map(|i| (i.parse().unwrap(), 1))
        .collect::<HashMap<i64, i64>>();

    let mut blink_results = HashMap::new();
    let mut updates = Vec::new();

    for _ in 0..blinks {
        for (&stone, &amount) in stone_amounts.iter().filter(|&(_, &amount)| amount > 0) {
            let result = if let Some(cached) = blink_results.get(&stone) {
                cached
            } else {
                let new_results = if stone == 0 {
                    BlinkResult::Single(1)
                } else {
                    if (stone.ilog10() + 1) % 2 == 0 {
                        let num_of_digits = stone.ilog10() + 1;
                        let num2 = stone % 10_i64.pow(num_of_digits / 2);
                        let num1 = (stone - num2) / (10_i64.pow(num_of_digits / 2));
                        BlinkResult::Double(num1, num2)
                    } else {
                        BlinkResult::Single(stone * 2024)
                    }
                };
                blink_results.insert(stone, new_results);
                blink_results.get(&stone).unwrap()
            };

            match result {
                &BlinkResult::Single(x) => updates.push((x, amount)),
                &BlinkResult::Double(x, y) => {
                    updates.push((x, amount));
                    updates.push((y, amount));
                }
            }

            updates.push((stone, -amount));
        }

        for (stone, amount) in updates.drain(..) {
            *stone_amounts.entry(stone).or_insert(0) += amount;
        }
    }

    stone_amounts.values().sum()
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
