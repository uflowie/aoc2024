use std::collections::HashMap;

use advent_of_code::{add, indexed_chars_iter};

advent_of_code::solution!(15);

pub fn part_one(input: &str) -> Option<u32> {
    let mut instructions = vec![];
    let mut robot = None;
    let mut tiles = HashMap::new();

    for (i, j, ch) in indexed_chars_iter(input) {
        match ch {
            '#' => {
                tiles.insert((i, j), Tile::Wall);
            }
            'O' => {
                tiles.insert((i, j), Tile::Box);
            }
            '.' => {
                tiles.insert((i, j), Tile::Empty);
            }
            '@' => {
                robot = Some((i, j));
                tiles.insert((i, j), Tile::Robot);
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
        let tile = tiles.get()
    }

    None
}

enum Tile {
    Box,
    Robot,
    Empty,
    Wall,
}

pub fn part_two(input: &str) -> Option<u32> {
    None
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
