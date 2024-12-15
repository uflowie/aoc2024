use std::{collections::HashSet, thread::sleep, time};

use advent_of_code::NUM_RE;

advent_of_code::solution!(14);

pub fn part_one(input: &str) -> Option<usize> {
    let robots: Vec<_> = input
        .lines()
        .map(Robot::<101, 103>::from)
        .map(|mut robot| {
            robot.simulate(100);
            robot
        })
        .collect();

    Robot::print(&robots);

    let quadrants: Vec<_> = robots.iter().filter_map(|robot| robot.quadrant()).collect();

    for quadrant in &quadrants {
        println!("{:?}", quadrant);
    }

    let safety_factor = [
        Quadrant::First,
        Quadrant::Second,
        Quadrant::Third,
        Quadrant::Fourth,
    ]
    .map(|quadrant| quadrants.iter().filter(|&q| *q == quadrant).count())
    .iter()
    .product::<usize>();

    Some(safety_factor)
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut robots: Vec<_> = input.lines().map(Robot::<101, 103>::from).collect();

    let mut ticks = 0;

    while !Robot::looks_like_a_christmas_tree(&robots) {
        for robot in robots.iter_mut() {
            robot.simulate(1)
        }
        ticks += 1;
    }
    Robot::print(&robots);

    Some(ticks)
}

#[derive(Debug)]
struct Robot<const WIDTH: i32, const HEIGHT: i32> {
    position: (i32, i32),
    velocity: (i32, i32),
}

#[derive(PartialEq, Debug)]
enum Quadrant {
    First,
    Second,
    Third,
    Fourth,
}

impl<const WIDTH: i32, const HEIGHT: i32> Robot<WIDTH, HEIGHT> {
    fn simulate(&mut self, ticks: i32) {
        fn wrap(value: i32, max: i32) -> i32 {
            ((value % max) + max) % max
        }

        self.position = (
            wrap(self.position.0 + self.velocity.0 * ticks, WIDTH),
            wrap(self.position.1 + self.velocity.1 * ticks, HEIGHT),
        );
    }

    fn quadrant(&self) -> Option<Quadrant> {
        let mid_x = WIDTH / 2;
        let mid_y = HEIGHT / 2;

        if self.position.0 == mid_x || self.position.1 == mid_y {
            None
        } else {
            match (self.position.0 < mid_x, self.position.1 < mid_y) {
                (true, true) => Some(Quadrant::First),
                (false, true) => Some(Quadrant::Second),
                (true, false) => Some(Quadrant::Third),
                (false, false) => Some(Quadrant::Fourth),
            }
        }
    }

    fn print(bots: &[Self]) {
        for i in 0..HEIGHT {
            for j in 0..WIDTH {
                let num_bots = bots
                    .iter()
                    .filter(|b| b.position.0 == j && b.position.1 == i)
                    .count();
                print!(
                    "{}",
                    if num_bots == 0 {
                        ".".to_owned()
                    } else {
                        num_bots.to_string()
                    }
                );
            }
            println!();
        }
    }

    fn looks_like_a_christmas_tree(robots: &[Self]) -> bool {
        let unique_positions = robots.iter().map(|b| b.position).collect::<HashSet<_>>();
        unique_positions.len() == robots.len()
    }
}

impl<const WIDTH: i32, const HEIGHT: i32> From<&str> for Robot<WIDTH, HEIGHT> {
    fn from(value: &str) -> Self {
        let nums: Vec<i32> = NUM_RE
            .find_iter(value)
            .map(|n| n.as_str().parse().unwrap())
            .collect();
        Self {
            position: (nums[0], nums[1]),
            velocity: (nums[2], nums[3]),
        }
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
