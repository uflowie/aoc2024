use advent_of_code::indexed_chars_iter;

advent_of_code::solution!(25);

pub fn part_one(input: &str) -> Option<usize> {
    let (keys, locks) = parse(input);

    let result = keys
        .into_iter()
        .map(|k| {
            locks
                .iter()
                .filter(|l| (0..5).all(|i| k[i] + l[i] <= 5))
                .count()
        })
        .sum::<usize>();

    Some(result)
}

pub fn part_two(input: &str) -> Option<u32> {
    None
}

fn parse(input: &str) -> (Vec<[i8; 5]>, Vec<[i8; 5]>) {
    let mut keys = Vec::new();
    let mut locks = Vec::new();

    for item in input.split("\n\n") {
        let mut heights = [-1, -1, -1, -1, -1];

        for (_, col, ch) in indexed_chars_iter(item) {
            if ch == '#' {
                heights[col as usize] += 1;
            }
        }

        if item.starts_with("#####") {
            locks.push(heights);
        } else {
            keys.push(heights);
        }
    }

    (keys, locks)
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
