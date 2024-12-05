advent_of_code::solution!(5);

pub fn part_one(input: &str) -> Option<i32> {
    let (rules, updates) = parse(input);

    Some(
        updates
            .iter()
            .filter(|update| rules.iter().all(|rule| !update.violates(rule)))
            .map(|update| update[update.len() / 2])
            .sum(),
    )
}

pub fn part_two(input: &str) -> Option<i32> {
    let (rules, updates) = parse(input);

    Some(
        updates
            .into_iter()
            .filter(|update| rules.iter().any(|rule| update.violates(rule)))
            .map(|mut update| {
                while rules.iter().any(|rule| update.violates(rule)) {
                    for rule in &rules {
                        rule.apply(&mut update);
                    }
                }
                update[update.len() / 2]
            })
            .sum(),
    )
}

fn parse(input: &str) -> (Vec<OrderingRule>, Vec<Vec<i32>>) {
    let input = input.replace("\r\n", "\n");
    let (rules_part, updates_part) = input.split_once("\n\n").unwrap();

    let rules = rules_part.lines().map(OrderingRule::from).collect();
    let updates = updates_part
        .lines()
        .map(|line| line.split(',').map(|x| x.parse().unwrap()).collect())
        .collect();

    (rules, updates)
}

trait Violate<T> {
    fn violates(&self, other: &T) -> bool;
}

impl Violate<OrderingRule> for Vec<i32> {
    fn violates(&self, rule: &OrderingRule) -> bool {
        if let Some((l, r)) = rule.find_pages(self) {
            l > r
        } else {
            false
        }
    }
}

struct OrderingRule {
    left: i32,
    right: i32,
}

impl OrderingRule {
    fn find_pages(&self, update: &Vec<i32>) -> Option<(usize, usize)> {
        let left_pos = update.iter().position(|&x| x == self.left)?;
        let right_pos = update.iter().position(|&x| x == self.right)?;
        Some((left_pos, right_pos))
    }

    fn apply(&self, update: &mut Vec<i32>) {
        if let Some((l, r)) = self.find_pages(update) {
            if l > r {
                update.swap(l, r);
            }
        }
    }
}

impl From<&str> for OrderingRule {
    fn from(value: &str) -> Self {
        let mut page_nums = value.split('|');
        Self {
            left: page_nums.next().unwrap().parse().unwrap(),
            right: page_nums.next().unwrap().parse().unwrap(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(143));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(123));
    }
}
