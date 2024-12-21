use std::collections::HashMap;

advent_of_code::solution!(19);

pub fn part_one(input: &str) -> Option<usize> {
    Some(get_ways(input).filter(|&ways| ways > 0).count())
}

pub fn part_two(input: &str) -> Option<u64> {
    Some(get_ways(input).sum())
}

fn get_ways(input: &str) -> impl Iterator<Item = u64> + '_ {
    let mut lines = input.lines();

    let towels: Box<[&str]> = lines.next().unwrap().split(", ").collect();
    lines.next().unwrap();

    lines.map(move |design| num_ways(design, &towels, &mut HashMap::new()))
}

fn num_ways<'a>(remaining: &'a str, towels: &[&str], cache: &mut HashMap<&'a str, u64>) -> u64 {
    if remaining.is_empty() {
        return 1;
    }

    if let Some(count) = cache.get(remaining) {
        return *count;
    }

    let ways = towels
        .iter()
        .filter_map(|towel| {
            if towel.len() <= remaining.len() && remaining[..towel.len()] == **towel {
                Some(num_ways(&remaining[towel.len()..], towels, cache))
            } else {
                None
            }
        })
        .sum();

    cache.insert(remaining, ways);

    ways
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
