use std::collections::HashSet;

use advent_of_code::{add, indexed_chars_iter};

advent_of_code::solution!(15);

pub fn part_one(input: &str) -> Option<i32> {
    let mut instructions = vec![];
    let mut robot = None;
    let mut boxes = HashSet::new();
    let mut walls = HashSet::new();

    for (i, j, ch) in indexed_chars_iter(input) {
        match ch {
            '#' => {
                walls.insert((i, j));
            }
            'O' => {
                boxes.insert((i, j));
            }
            '.' => {}
            '@' => {
                robot = Some((i, j));
            }
            '^' => instructions.push((-1, 0)),
            'v' => instructions.push((1, 0)),
            '<' => instructions.push((0, -1)),
            '>' => instructions.push((0, 1)),
            _ => {}
        }
    }

    let mut robot = robot.unwrap();

    for instruction in instructions {
        let next_pos = add(robot, instruction);

        if walls.contains(&next_pos) {
            continue;
        } else if boxes.contains(&next_pos) {
            let mut next_next_pos = add(next_pos, instruction);
            while boxes.contains(&next_next_pos) {
                next_next_pos = add(next_next_pos, instruction);
            }
            if walls.contains(&next_next_pos) {
                continue;
            }
            boxes.remove(&next_pos);
            boxes.insert(next_next_pos);
            robot = next_pos;
        } else {
            robot = next_pos;
        }
    }

    let result = boxes.iter().map(|(x, y)| x * 100 + y).sum();

    Some(result)
}

pub fn part_two(input: &str) -> Option<i32> {
    let mut instructions = vec![];
    let mut robot = None;
    let mut boxes = HashSet::new();
    let mut walls = HashSet::new();

    for (i, j, ch) in indexed_chars_iter(input) {
        match ch {
            '#' => {
                walls.insert((i, j * 2));
                walls.insert((i, j * 2 + 1));
            }
            'O' => {
                boxes.insert((i, j * 2, i, j * 2 + 1));
            }
            '.' => {}
            '@' => {
                robot = Some((i, j * 2));
            }
            '^' => instructions.push((-1, 0)),
            'v' => instructions.push((1, 0)),
            '<' => instructions.push((0, -1)),
            '>' => instructions.push((0, 1)),
            _ => {}
        }
    }
    let mut robot = robot.unwrap();

    for instruction in instructions {
        let next_pos = add(robot, instruction);

        if walls.contains(&next_pos) {
            continue;
        }

        let mut boxes_to_push: Vec<(i32, i32, i32, i32)> = boxes
            .iter()
            .filter(|(x1, y1, x2, y2)| (*x1, *y1) == next_pos || (*x2, *y2) == next_pos)
            .copied()
            .collect();

        if boxes_to_push.len() == 0 {
            robot = next_pos;
        } else {
            let mut curr_boxes = boxes_to_push.clone();

            loop {
                let next_positions: HashSet<_> = curr_boxes
                    .iter()
                    .flat_map(|&(x1, y1, x2, y2)| match instruction {
                        (0, -1) => [Some(add((x1, y1), instruction)), None],
                        (0, 1) => [Some(add((x2, y2), instruction)), None],
                        _ => [
                            Some(add((x1, y1), instruction)),
                            Some(add((x2, y2), instruction)),
                        ],
                    })
                    .flatten()
                    .collect();

                // if any of them have a wall -> break
                if next_positions.iter().any(|idx| walls.contains(idx)) {
                    break;
                }

                let next_boxes: Vec<_> = boxes
                    .iter()
                    .filter(|(x1, y1, x2, y2)| {
                        next_positions.contains(&(*x1, *y1)) || next_positions.contains(&(*x2, *y2))
                    })
                    .copied()
                    .collect();

                // if all of them are free -> push existing boxes
                if next_boxes.len() == 0 {
                    let new_boxes = boxes_to_push.iter().map(|&(x1, y1, x2, y2)| {
                        (
                            x1 + instruction.0,
                            y1 + instruction.1,
                            x2 + instruction.0,
                            y2 + instruction.1,
                        )
                    });

                    for b in &boxes_to_push {
                        boxes.remove(b);
                    }

                    for b in new_boxes {
                        boxes.insert(b);
                    }

                    robot = next_pos;
                    break;
                } else {
                    curr_boxes = next_boxes;
                    for b in curr_boxes.iter().copied() {
                        boxes_to_push.push(b)
                    }
                }
            }
        }
    }
    let result = boxes.iter().map(|&(x1, y1, _, _)| x1 * 100 + y1).sum();

    Some(result)
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
