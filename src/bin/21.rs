use std::collections::HashMap;

advent_of_code::solution!(21);

pub fn part_one(input: &str) -> Option<usize> {
    Some(solve(input, 2))
}

pub fn part_two(input: &str) -> Option<usize> {
    Some(solve(input, 25))
}

fn solve(input: &str, num_robots: usize) -> usize {
    input
        .lines()
        .map(|target| {
            get_min_instructions_len(target, num_robots)
                * target
                    .chars()
                    .filter(|c| c.is_digit(10))
                    .collect::<String>()
                    .parse::<usize>()
                    .unwrap()
        })
        .sum()
}

fn get_min_instructions_len(target: &str, num_robots: usize) -> usize {
    let numpad_instructions = get_numpad_instructions(target);

    let mut instruction_counts = HashMap::<String, usize>::new();

    for instruction in numpad_instructions {
        instruction_counts
            .entry(instruction)
            .and_modify(|e| *e += 1)
            .or_insert(1);
    }

    for _ in 0..num_robots {
        let mut next_counts = HashMap::new();

        for (instruction, count) in instruction_counts {
            let next_instructions = get_keypad_instructions(&instruction);
            for next_instruction in next_instructions {
                next_counts
                    .entry(next_instruction)
                    .and_modify(|e| *e += count)
                    .or_insert(count);
            }
        }

        instruction_counts = next_counts;
    }

    instruction_counts
        .iter()
        .map(|(k, v)| k.len() * v)
        .sum::<usize>()
}

fn get_numpad_instructions(target: &str) -> Vec<String> {
    let mut curr = (2, 3);
    let mut instructions_set = Vec::new();

    let target: Vec<char> = target.chars().collect();
    let mut remaining = &target[0..];

    while !remaining.is_empty() {
        let mut instructions = Vec::new();

        let next = get_pos_num(remaining[0]);
        remaining = &remaining[1..];

        let (x, y) = distance(next, curr);

        let x_amount = x.abs();
        let y_amount = y.abs();

        let x_char = if x > 0 { '>' } else { '<' };
        let y_char = if y > 0 { 'v' } else { '^' };

        let x_first = (x_amount, x_char, y_amount, y_char);
        let y_first = (y_amount, y_char, x_amount, x_char);

        let (first_amount, first_char, second_amount, second_char) = if curr.1 == 3 && next.0 == 0 {
            y_first
        } else if next.1 == 3 && curr.0 == 0 {
            x_first
        } else if x < 0 {
            x_first
        } else {
            y_first
        };

        instructions.extend(std::iter::repeat(first_char).take(first_amount as usize));
        instructions.extend(std::iter::repeat(second_char).take(second_amount as usize));

        instructions.push('A');
        instructions_set.push(instructions.into_iter().collect());
        curr = next;
    }

    instructions_set
}

fn get_keypad_instructions(target: &str) -> Vec<String> {
    let mut curr = (2, 0);

    let mut instructions_set = Vec::new();

    let target: Vec<char> = target.chars().collect::<Vec<char>>();
    let mut remaining = &target[0..];

    while !remaining.is_empty() {
        let mut instructions = Vec::new();

        let next = get_pos_dir(remaining[0]);
        remaining = &remaining[1..];

        let (x, y) = distance(next, curr);

        let x_amount = x.abs();
        let y_amount = y.abs();

        let x_char = if x > 0 { '>' } else { '<' };
        let y_char = if y > 0 { 'v' } else { '^' };

        let x_first = (x_amount, x_char, y_amount, y_char);
        let y_first = (y_amount, y_char, x_amount, x_char);

        let (first_amount, first_char, second_amount, second_char) =
            if curr == (0, 1) && next.1 == 0 {
                x_first
            } else if next == (0, 1) && curr.1 == 0 {
                y_first
            } else if x < 0 {
                x_first
            } else {
                y_first
            };

        instructions.extend(std::iter::repeat(first_char).take(first_amount as usize));
        instructions.extend(std::iter::repeat(second_char).take(second_amount as usize));

        instructions.push('A');
        instructions_set.push(instructions.into_iter().collect());

        curr = next;
    }

    instructions_set
}

fn get_pos_dir(ch: char) -> (i32, i32) {
    match ch {
        '^' => (1, 0),
        '<' => (0, 1),
        'v' => (1, 1),
        '>' => (2, 1),
        'A' => (2, 0),
        _ => unreachable!(),
    }
}

fn get_pos_num(ch: char) -> (i32, i32) {
    match ch {
        '7' => (0, 0),
        '8' => (1, 0),
        '9' => (2, 0),
        '4' => (0, 1),
        '5' => (1, 1),
        '6' => (2, 1),
        '1' => (0, 2),
        '2' => (1, 2),
        '3' => (2, 2),
        '0' => (1, 3),
        'A' => (2, 3),
        _ => unreachable!(),
    }
}

fn distance(a: (i32, i32), b: (i32, i32)) -> (i32, i32) {
    (a.0 - b.0, a.1 - b.1)
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
