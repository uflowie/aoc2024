advent_of_code::solution!(2);

pub fn part_one(input: &str) -> Option<usize> {
    Some(solve(input, record_is_safe))
}

pub fn part_two(input: &str) -> Option<usize> {
    Some(solve(input, |record| {
        record_is_safe(record) || record_is_safe_if_one_level_is_removed(record)
    }))
}

fn solve(input: &str, f: fn(&Vec<i32>) -> bool) -> usize {
    input
        .lines()
        .map(|line| {
            line.split_whitespace()
                .map(|x| x.parse::<i32>().unwrap())
                .collect()
        })
        .filter(f)
        .collect::<Vec<_>>()
        .len()
}

fn record_is_safe(record: &Vec<i32>) -> bool {
    let diffs: Vec<i32> = record.windows(2).map(|w| w[0] - w[1]).collect();
    diffs.iter().all(|d| *d <= -1 && *d >= -3) || diffs.iter().all(|d| *d >= 1 && *d <= 3)
}

fn record_is_safe_if_one_level_is_removed(record: &Vec<i32>) -> bool {
    (0..record.len())
        .map(|i| {
            let mut v = record.clone();
            v.remove(i);
            v
        })
        .any(|record| record_is_safe(&record))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(4));
    }
}
