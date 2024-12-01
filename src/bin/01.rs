use itertools::Itertools;

advent_of_code::solution!(1);

pub fn part_one(input: &str) -> Option<i32> {
    let (mut lefts, mut rights): (Vec<_>, Vec<_>) = input
        .lines()
        .map_into::<LocationPair>()
        .map(|x| (x.0, x.1))
        .unzip();

    lefts.sort_unstable();
    rights.sort_unstable();

    Some(
        lefts
            .into_iter()
            .zip(rights)
            .map(|(l, r)| (l - r).abs())
            .sum(),
    )
}

pub fn part_two(input: &str) -> Option<i32> {
    let pairs: Vec<_> = input.lines().map_into::<LocationPair>().collect();

    let counts = pairs.iter().map(|pair| pair.1).counts();

    Some(
        pairs
            .iter()
            .map(|pair| pair.0)
            .map(|i| i * counts.get(&i).copied().unwrap_or_default() as i32)
            .sum(),
    )
}

struct LocationPair(i32, i32);

impl From<&str> for LocationPair {
    fn from(value: &str) -> Self {
        let mut parts = value.split_whitespace();
        Self(
            parts.next().unwrap().parse::<i32>().unwrap(),
            parts.next().unwrap().parse::<i32>().unwrap(),
        )
    }
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
