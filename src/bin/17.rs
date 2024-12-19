use advent_of_code::NUM_RE;
use itertools::Itertools;

advent_of_code::solution!(17);

pub fn part_one(input: &str) -> Option<String> {
    let (register_a, instructions) = parse(input);

    let outputs = get_outputs(register_a, &instructions)
        .into_iter()
        .map(|x| x.to_string())
        .join(",");

    Some(outputs)
}

pub fn part_two(input: &str) -> Option<usize> {
    let (_, instructions) = parse(input);
    let len = instructions.len();

    let mut shift = 3 * len;
    let mut starts = vec![1 << (shift - 3)];

    for i in (0..len).rev() {
        shift -= 3;
        let target_num = instructions[i];
        let instruction_idx = i;

        let mut block_starts = vec![];
        for start in &starts {
            for j in 0..8 {
                let block_start = start + (1 << shift) * j;
                let outputs = get_outputs(block_start, &instructions);

                if outputs[instruction_idx] == target_num {
                    block_starts.push(block_start);
                }
            }
        }

        starts = block_starts;
    }

    starts.into_iter().min()
}

fn parse(input: &str) -> (usize, Box<[usize]>) {
    let mut nums = NUM_RE.find_iter(input).map(|x| x.as_str().parse().unwrap());

    let register_a = nums.next().unwrap();
    nums.next();
    nums.next();

    let instructions = nums.collect();

    (register_a, instructions)
}

fn get_outputs(mut register_a: usize, instructions: &[usize]) -> Vec<usize> {
    let mut stack_pointer = 0;
    let mut outputs = Vec::new();
    let mut register_b = 0;
    let mut register_c = 0;

    while stack_pointer + 1 < instructions.len() {
        let op_code = instructions[stack_pointer];
        let literal_operand = instructions[stack_pointer + 1];

        let operand = match op_code {
            0 | 6 | 7 | 2 | 5 => match literal_operand {
                _ if literal_operand <= 3 => literal_operand,
                4 => register_a,
                5 => register_b,
                _ => register_c,
            },
            _ => literal_operand,
        };

        match op_code {
            0 | 6 | 7 => {
                let numerator = register_a;
                let denominator = 1 << operand;
                let result = numerator / denominator;

                let register = match op_code {
                    0 => &mut register_a,
                    6 => &mut register_b,
                    _ => &mut register_c,
                };
                *register = result;
                stack_pointer += 2;
            }
            1 => {
                register_b = register_b ^ operand;
                stack_pointer += 2;
            }
            2 => {
                register_b = operand % 8;
                stack_pointer += 2;
            }
            3 => {
                stack_pointer = if register_a == 0 {
                    stack_pointer + 2
                } else {
                    operand
                };
            }
            4 => {
                register_b = register_b ^ register_c;
                stack_pointer += 2;
            }
            5 => {
                let out = operand % 8;
                outputs.push(out);
                stack_pointer += 2;
            }
            _ => panic!(),
        }
    }

    outputs
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
